//! `/api/v1/importlist` — a configured import list (e.g. a Hardcover shelf)
//! that syncs works into Bindery.
//!
//! `sync = custom`, keyed by `name`, GET lists / `POST /api/v1/importlist`
//! creates / `PUT /api/v1/importlist/{live.id}` updates / `--prune` deletes via
//! `DELETE /api/v1/importlist/{live.id}`
//! ([`core_lib::reconcile::upsert_prune`]). This can't be `sync = crud` for
//! three reasons:
//!
//! 1. `apiKey` is `writeOnly` — every response blanks it, so a declared
//!    credential would differ from live on *every* apply and churn forever
//!    under plain merge-equality. [`in_sync`] excludes it from the diff
//!    entirely, which means **secret rotation is undetectable**: changing only
//!    `api_key` in config produces no plan. Don't be fooled into "fixing" this
//!    with a fake always-update — the API gives no way to tell "still the old
//!    key" from "rotated", so an update triggered for an unrelated reason still
//!    carries the desired `api_key` along (harmless), but `api_key` alone never
//!    triggers one.
//! 2. The update body is a *different* schema (`UpdateImportListRequest`):
//!    `account` is create-only (not present in the update schema at all — see
//!    below), and there's an update-only `clearApiKey` flag this resource does
//!    not model (there is no declarative way to express "clear the secret" in
//!    this config shape, only "set" or "leave").
//! 3. `ownerUserId` is tri-state in the API (absent = leave, `null` = clear,
//!    a number = set) but the engine's config codec can't express that on a
//!    plain `Option<i64>`: [`core_lib::codec::config`]'s struct decode treats an
//!    explicit `null` identically to an absent key (both skip the field), so
//!    "clear the owner" cannot be declared through this resource — only "leave"
//!    or "set to a specific user" are reachable. Flagging this rather than
//!    faking a distinction the codec can't make.
//!
//! `in_sync` ignores `apiKey`, `apiKeyConfigured`, `account`, `lastSyncAt`,
//! `createdAt`, `updatedAt`, `id` — the first for reason 1 above, the rest
//! because they're either server-owned display state or (for `account`)
//! write-once and not worth flagging as drift we can never correct.
//!
//! FKs: `rootFolderId` → a managed root folder (`${ref.root_folder.<path>}`),
//! `qualityProfileId` → a managed quality profile
//! (`${ref.quality_profile.<name>}`), `ownerUserId` → a managed user
//! (`${ref.user.<username>}`).

use core_lib::{
    CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine, reconcile,
};
use core_macros::resource;
use serde_json::Value;

/// `/api/v1/importlist` — a configured import list.
#[resource(sync = custom, list = get("/api/v1/importlist"))]
pub struct ImportList {
    /// Server-assigned id. Read-only.
    #[id]
    pub id: Option<i64>,
    /// Display name — its identity (`${ref.import_list.<name>}`).
    #[key]
    pub name: String,
    /// Provider kind. Not validated server-side, but only `hardcover` has a
    /// syncer wired; an empty value is defaulted to `csv` on create (a
    /// no-op list with no syncer).
    #[wire(name = "type")]
    #[default("csv")]
    pub list_type: String,
    /// Source locator. For `hardcover` lists this is the list slug (matched
    /// against the account's Hardcover lists), not an absolute URL.
    pub url: Option<String>,
    /// Provider credential (e.g. the Hardcover API token). `writeOnly` on the
    /// API — every response blanks it; read `api_key_configured` instead. See
    /// the module doc for why this makes rotation undetectable.
    pub api_key: Option<SecretValue>,
    /// Whether a provider credential is currently stored, reported by the
    /// API in place of the blanked `apiKey`. Read-only.
    #[wire(read_only)]
    pub api_key_configured: Option<bool>,
    /// Provider-side account identity the list belongs to (e.g. the
    /// Hardcover username the token was loaded with). Settable on create;
    /// **not patchable** via the update route — excluded from [`in_sync`] so
    /// drift here is never (falsely) flagged as fixable.
    pub account: Option<String>,
    /// Root folder assigned to content synced from this list.
    #[reference(root_folder)]
    pub root_folder_id: Option<i64>,
    /// Quality profile stamped on authors created by this list's sync.
    #[reference(quality_profile)]
    pub quality_profile_id: Option<i64>,
    /// Bindery user who owns books/authors synced from this list. `None`
    /// (absent) means "leave as-is" here — see the module doc; there is no
    /// way to declare "clear to global" through this field.
    #[reference(user)]
    pub owner_user_id: Option<i64>,
    /// Pins the format synced books are created as: `ebook`, `audiobook`, or
    /// `both`. Empty/absent keeps the format derived from the source.
    pub media_type: Option<String>,
    /// Whether newly-synced works are monitored.
    pub monitor_new: Option<bool>,
    /// Whether works from this list are automatically added.
    pub auto_add: Option<bool>,
    /// Whether the list is active; disabled lists are skipped by the
    /// scheduler and reject a manual sync.
    pub enabled: Option<bool>,
    /// Timestamp of the last successful sync, reported by the API.
    /// Read-only.
    #[wire(read_only)]
    pub last_sync_at: Option<String>,
    /// Row creation time, reported by the API. Read-only.
    #[wire(read_only)]
    pub created_at: Option<String>,
    /// Row last-update time, reported by the API. Read-only.
    #[wire(read_only)]
    pub updated_at: Option<String>,
}

/// Wire keys ignored when deciding whether a live import list already matches
/// desired — see the module doc for why each is excluded.
const IGNORED_ON_DIFF: &[&str] = &[
    "apiKey",
    "apiKeyConfigured",
    "account",
    "lastSyncAt",
    "createdAt",
    "updatedAt",
    "id",
];

/// Idempotency predicate for [`reconcile::upsert_prune`]: every key `desired`
/// declares (other than [`IGNORED_ON_DIFF`]) must already match `live`, value
/// for value. A key `desired` omits is only in sync when `live` is likewise
/// absent/null for it — `desired` is already the full encoded wire shape, so an
/// omitted key means "unset", not "don't care".
fn in_sync(desired: &Value, live: &Value) -> bool {
    let (Value::Object(want), Value::Object(have)) = (desired, live) else {
        return desired == live;
    };
    want.iter()
        .filter(|(k, _)| !IGNORED_ON_DIFF.contains(&k.as_str()))
        .all(|(k, wv)| match have.get(k) {
            Some(hv) => hv == wv,
            None => wv.is_null(),
        })
}

/// Keys `UpdateImportListRequest` accepts, projected out of the full encoded
/// wire body. `account` is deliberately excluded — it isn't part of the update
/// schema (create-only), and `clearApiKey` is never set (see the module doc).
const UPDATE_KEYS: &[&str] = &[
    "name",
    "type",
    "url",
    "rootFolderId",
    "qualityProfileId",
    "monitorNew",
    "autoAdd",
    "enabled",
    "mediaType",
    "ownerUserId",
    "apiKey",
];

/// Build the `UpdateImportListRequest` body from the full encoded wire form.
fn update_projection(wire: &Value) -> Value {
    let mut out = serde_json::Map::new();
    if let Value::Object(obj) = wire {
        for key in UPDATE_KEYS {
            if let Some(v) = obj.get(*key) {
                out.insert((*key).to_string(), v.clone());
            }
        }
    }
    Value::Object(out)
}

impl CustomSync for ImportList {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/v1/importlist").await?;
            // Full desired wire, incl. `account` (create-only, still part of
            // the create body) and `apiKey` (writeOnly, still sendable).
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert_prune(
                &wire,
                &live,
                "name",
                in_sync,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/v1/importlist", &w).await?;
                        Ok(())
                    }
                },
                |l, w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    let body = update_projection(&w);
                    async move {
                        let _: Value = client
                            .put(&format!("/api/v1/importlist/{id}"), &body)
                            .await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        client.delete(&format!("/api/v1/importlist/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
