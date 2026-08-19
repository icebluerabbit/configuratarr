//! Bindery's `/api/v1/setting` key/value table.
//!
//! There is no bulk-config endpoint with a fixed shape: the table is
//! schema-less, keyed by free-form dotted strings (`import.mode`,
//! `library.defaultRootFolderId`, …), read one-at-a-time with
//! `GET /api/v1/setting/{key}` (or in bulk with `GET /api/v1/setting`, a bare
//! `[{key, value, updatedAt}, ...]` array) and written with
//! `PUT /api/v1/setting/{key}` (`{"value": "<string>"}`). Every value is
//! stored and returned **as a string** — booleans are `"true"`/`"false"`,
//! numbers are decimal strings, durations are Go duration strings.
//!
//! So — like lazylibrarian's `config` — this is a `sync = custom` singleton:
//! one [`Setting`] struct with a field per *validated* key (see below), each
//! carrying its literal dotted key as an explicit `#[wire(name = "...")]`.
//! `engine::encode_config::<Setting>` then yields exactly the `{key: value}`
//! pairs the PUT endpoint wants, typed on the Rust side (`bool`, `i64`,
//! `String`, `SecretValue`) and stringified for the wire by [`scalar`] — the
//! same shape lazylibrarian's `scalar`/`same` pair exists for, minus the
//! `""`-means-false convention (bindery writes literal `"true"`/`"false"`).
//!
//! **Only the validated keys are modelled** — this is a closed set, read out
//! of the `PUT /api/v1/setting/{key}` spec description (not invented): every
//! key that endpoint's `validateSettingValue` rejects bad values for, plus
//! the two secret keys the endpoint deliberately allows through
//! (`hardcover.api_token`, `calibre.plugin_api_key`). Unlisted keys pass the
//! server through unvalidated and unchanged — deliberately **not** modelled
//! as an open blob (the task's "closed/open" rule: a documented, closed key
//! set gets typed fields + docs, not a generic map).
//!
//! ## Behaviour the spec forces
//!
//! - **Secret keys never read back.** `GET /api/v1/setting/{key}` 404s for a
//!   secret key exactly the same as for a key that was never set (the secret
//!   check runs *before* the lookup), and the bulk `GET /api/v1/setting`
//!   simply omits secret rows — not masked, *absent*. So `hardcover.api_token`
//!   and `calibre.plugin_api_key` (the only two secrets this endpoint will
//!   write) can never be diffed against live state: `reconcile` doesn't try,
//!   and instead always writes them when the config declares a value, always
//!   reporting `Updated` — never `Unchanged`, even when the value hasn't
//!   changed. `abs.api_key` and `grimmory.api_key` are real secrets too, but
//!   this endpoint refuses to write them (403) — they belong to their own
//!   connection-test-backed resources, not here.
//! - **`prune` is meaningless and ignored.** `DELETE /api/v1/setting/{key}`
//!   reverts a key to its built-in default and 403s unconditionally on every
//!   secret key (including the two PUT allows) — there is no safe, generic
//!   "unset" this reconcile can drive, so it never deletes and does not
//!   consult the `prune` flag.
//! - **Two keys only take effect after a restart:** `search.interval` and
//!   `hardcover.sync_interval`. Applying them updates the stored value
//!   immediately; the running server keeps using its old interval until
//!   restarted.
//! - **`library.defaultRootFolderId` is `Option<i64>` + `#[reference(root_folder)]`**,
//!   not a `String`, even though every other field here is typed to its
//!   semantic shape and then stringified for the wire. A `${ref.root_folder.*}`
//!   resolves to an integer [`core_lib::RefId`]; substituting that `Int` into a
//!   `String`-typed field fails decode (`expected string, got <n>`) before
//!   [`scalar`] ever runs — [`scalar`]'s job is exactly to turn that resolved
//!   integer back into the decimal string the PUT body wants.

use std::collections::HashMap;

use core_lib::apply::Change;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine};
use core_macros::resource;
use serde_json::{Value, json};

/// Bindery settings — every validated key (all optional: present in config =
/// manage that key, absent = leave it alone). See the module docs for the
/// secret-key and `prune` caveats.
#[resource(sync = custom, case = snake, list = get("/api/v1/setting"))]
pub struct Setting {
    // ── hardcover.* ─────────────────────────────────────────────────────────
    /// `hardcover.api_token` — Hardcover API token. One of the two secret keys
    /// this endpoint allows writing (rejected 403 for every other secret);
    /// validated to reject control characters. **Never reads back** — GET
    /// 404s for a secret key exactly as for an unset one — so a configured
    /// value always reports `Updated`, every apply.
    #[wire(name = "hardcover.api_token")]
    pub hardcover_api_token: Option<SecretValue>,
    /// `hardcover.enhanced_series_enabled` — case-insensitive `true`/`false`.
    #[wire(name = "hardcover.enhanced_series_enabled")]
    pub hardcover_enhanced_series_enabled: Option<bool>,
    /// `hardcover.sync_interval` — Go duration string, must be within
    /// `[1h, 168h]`. Takes effect only after a restart.
    #[wire(name = "hardcover.sync_interval")]
    pub hardcover_sync_interval: Option<String>,

    // ── abs.* (Audiobookshelf) ──────────────────────────────────────────────
    /// `abs.enabled` — case-insensitive `true`/`false`.
    #[wire(name = "abs.enabled")]
    pub abs_enabled: Option<bool>,
    /// `abs.base_url` — must pass `abs.ValidateBaseURLSecure` (an SSRF-safe
    /// http/https URL).
    #[wire(name = "abs.base_url")]
    pub abs_base_url: Option<String>,

    // ── calibre.* ────────────────────────────────────────────────────────────
    /// `calibre.library_path` — must stat to an existing directory. Validation
    /// **stats the local filesystem as a side effect** of applying.
    #[wire(name = "calibre.library_path")]
    pub calibre_library_path: Option<String>,
    /// `calibre.binary_path` — must be an existing, regular, executable file.
    #[wire(name = "calibre.binary_path")]
    pub calibre_binary_path: Option<String>,
    /// `calibre.mode` — one of `off`, `calibredb`, `plugin`.
    #[wire(name = "calibre.mode")]
    pub calibre_mode: Option<String>,
    /// `calibre.push_path_remap` — must match the `from:to[,from:to]` grammar.
    #[wire(name = "calibre.push_path_remap")]
    pub calibre_push_path_remap: Option<String>,
    /// `calibre.plugin_url` — must be an http/https URL whose host passes the
    /// outbound SSRF policy (link-local and cloud-metadata addresses blocked).
    #[wire(name = "calibre.plugin_url")]
    pub calibre_plugin_url: Option<String>,
    /// `calibre.plugin_api_key` — the other secret key this endpoint allows
    /// writing. No format validation beyond being a secret; like
    /// `hardcover.api_token`, it **never reads back** (GET 404s identically to
    /// unset), so it always reports `Updated`.
    #[wire(name = "calibre.plugin_api_key")]
    pub calibre_plugin_api_key: Option<SecretValue>,

    // ── cwa.* ───────────────────────────────────────────────────────────────
    /// `cwa.ingest_path` — must be an existing directory. Validation **stats
    /// the local filesystem as a side effect** of applying.
    #[wire(name = "cwa.ingest_path")]
    pub cwa_ingest_path: Option<String>,

    // ── import.* ────────────────────────────────────────────────────────────
    /// `import.mode` — one of `auto`, `move`, `copy`, `hardlink`, `external`.
    #[wire(name = "import.mode")]
    pub import_mode: Option<String>,
    /// `import.drop_folder` — must be an existing directory. Validation
    /// **stats the local filesystem as a side effect** of applying.
    #[wire(name = "import.drop_folder")]
    pub import_drop_folder: Option<String>,
    /// `import.drop_layout` — one of `flat`, `templated`.
    #[wire(name = "import.drop_layout")]
    pub import_drop_layout: Option<String>,
    /// `import.drop_link_mode` — one of `copy`, `hardlink`.
    #[wire(name = "import.drop_link_mode")]
    pub import_drop_link_mode: Option<String>,
    /// `import.drop_pair_gating` — case-insensitive `true`/`false`.
    #[wire(name = "import.drop_pair_gating")]
    pub import_drop_pair_gating: Option<bool>,
    /// `import.drop_pair_gating_timeout_hours` — a positive integer.
    #[wire(name = "import.drop_pair_gating_timeout_hours")]
    pub import_drop_pair_gating_timeout_hours: Option<i64>,

    // ── default.* ───────────────────────────────────────────────────────────
    /// `default.media_type` — one of `ebook`, `audiobook`, `both`.
    #[wire(name = "default.media_type")]
    pub default_media_type: Option<String>,
    /// `default.media_type_strict` — case-insensitive `true`/`false`.
    #[wire(name = "default.media_type_strict")]
    pub default_media_type_strict: Option<bool>,

    // ── author.* ────────────────────────────────────────────────────────────
    /// `author.default_monitor_mode` — one of `all`, `future`, `latest`,
    /// `none` (`series` monitoring is per-author only, not settable here).
    #[wire(name = "author.default_monitor_mode")]
    pub author_default_monitor_mode: Option<String>,
    /// `author.default_monitor_latest_count` — a positive integer.
    #[wire(name = "author.default_monitor_latest_count")]
    pub author_default_monitor_latest_count: Option<i64>,

    // ── library.* ───────────────────────────────────────────────────────────
    /// `library.defaultRootFolderId` — must be a positive integer identifying
    /// an existing root folder. A ref, not a free-typed value — see the module
    /// docs for why this is `i64` rather than `String` like its siblings.
    #[wire(name = "library.defaultRootFolderId")]
    #[reference(root_folder)]
    pub library_default_root_folder_id: Option<i64>,

    // ── metadata.* ──────────────────────────────────────────────────────────
    /// `metadata.primary_provider` — one of `openlibrary`, `dnb`.
    #[wire(name = "metadata.primary_provider")]
    pub metadata_primary_provider: Option<String>,

    // ── search.* ────────────────────────────────────────────────────────────
    /// `search.interval` — Go duration string, must be within `[1h, 168h]`.
    /// Takes effect only after a restart.
    #[wire(name = "search.interval")]
    pub search_interval: Option<String>,
}

/// The two secret keys this endpoint allows writing (every other secret key
/// is refused with 403). See the module docs: neither ever reads back, so
/// `reconcile` never diffs them — it always writes and reports `Updated`.
const WRITABLE_SECRET_KEYS: &[&str] = &["hardcover.api_token", "calibre.plugin_api_key"];

fn is_writable_secret(key: &str) -> bool {
    WRITABLE_SECRET_KEYS.contains(&key)
}

/// Render a JSON scalar as the string `PUT /api/v1/setting/{key}` expects.
/// Bindery stores every value as a string on the wire — literal `"true"` /
/// `"false"` for booleans (no lazylibrarian-style `""`-means-false
/// convention), decimal for numbers.
fn scalar(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

impl CustomSync for Setting {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(vec![]);
            };
            let wire = engine::encode_config::<Self>(cfg)?;
            let Some(fields) = wire.as_object() else {
                return Ok(vec![]);
            };

            // One bulk read covers every non-secret key at once — secret rows
            // are simply absent (not masked), which is exactly why they're
            // handled separately below instead of falling out of this map.
            let current: Vec<Value> = client.get("/api/v1/setting").await?;
            let have: HashMap<&str, &str> = current
                .iter()
                .filter_map(|row| Some((row.get("key")?.as_str()?, row.get("value")?.as_str()?)))
                .collect();

            let mut changes = Vec::with_capacity(fields.len());
            for (key, val) in fields {
                let want = scalar(val);

                if is_writable_secret(key) {
                    if execute {
                        put_setting(client, key, &want).await?;
                    }
                    changes.push(Change::updated(key.clone()));
                    continue;
                }

                if have.get(key.as_str()).copied().unwrap_or("") == want.as_str() {
                    changes.push(Change::unchanged(key.clone()));
                    continue;
                }
                if execute {
                    put_setting(client, key, &want).await?;
                }
                changes.push(Change::updated(key.clone()));
            }
            Ok(changes)
        })
    }
}

async fn put_setting(client: &HttpClient, key: &str, value: &str) -> anyhow::Result<()> {
    client
        .put(
            &format!("/api/v1/setting/{key}"),
            &json!({ "value": value }),
        )
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_stringifies_bool_and_number() {
        assert_eq!(scalar(&json!(true)), "true");
        assert_eq!(scalar(&json!(false)), "false");
        assert_eq!(scalar(&json!("auto")), "auto");
        assert_eq!(scalar(&json!(77)), "77");
        assert_eq!(scalar(&json!(null)), "");
    }

    #[test]
    fn is_writable_secret_matches_exactly_the_two_documented_keys() {
        assert!(is_writable_secret("hardcover.api_token"));
        assert!(is_writable_secret("calibre.plugin_api_key"));
        assert!(!is_writable_secret("abs.api_key"));
        assert!(!is_writable_secret("grimmory.api_key"));
        assert!(!is_writable_secret("import.mode"));
    }
}
