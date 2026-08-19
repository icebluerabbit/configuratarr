//! `/api/v1/settings` — Komga's global settings singleton.
//!
//! This cannot be a plain `sync = singleton`: the **read** shape
//! (`SettingsDto`, `GET /api/v1/settings`) and the **write** shape
//! (`SettingsUpdateDto`, `PATCH /api/v1/settings`) disagree on three fields.
//! `kepubifyPath`, `serverContextPath` and `serverPort` come back as a
//! `SettingMultiSourceString`/`SettingMultiSourceInteger`-shaped object
//! (`{configurationSource, databaseSource, effectiveValue}`, exposing *where*
//! the value comes from — a config file vs the database) but are written as
//! bare scalars. A
//! singleton's structural merge would diff an object against a scalar and
//! PATCH on every apply. `renewRememberMeKey` makes the mismatch worse: it
//! exists only on `SettingsUpdateDto` — a write-only action ("rotate the
//! remember-me signing key now") with no read counterpart to compare against
//! at all.
//!
//! So [`Settings`] is `sync = custom`, over one `GET`/`PATCH` pair:
//!
//! 1. `GET` the live document once.
//! 2. Flatten it: the three multi-source fields collapse to their
//!    `effectiveValue`; every other field is already a plain scalar.
//! 3. Encode the declared config through the descriptor
//!    ([`engine::encode_config`]) — decode validates + drops unknown keys,
//!    then the standard codec re-encodes. Every field here is a plain
//!    `Option<_>` with no `#[default]`, so a key the user didn't write
//!    decodes to `None` and the encode step omits it — the wire object that
//!    comes back already contains exactly the keys the user wrote.
//! 4. Diff that sparse wire object against the flattened live document,
//!    key by key. `renewRememberMeKey` has no live counterpart to diff
//!    against, so it is treated as an action: declaring it `true` fires the
//!    rotation on every apply (not idempotent — an inherent property of a
//!    write-only action, not a bug); declaring it `false` (or omitting it)
//!    is a no-op, since there is no "un-rotate" to converge toward.
//! 5. If anything differs, send **one** `PATCH` carrying only the differing
//!    keys and emit **one** [`Change`] for the whole singleton. `execute`
//!    gates the write exactly like every other custom hook — a preview
//!    (`execute = false`) computes and reports the diff but sends nothing.
//!
//! `prune` is meaningless for a keyless singleton (there is nothing to
//! delete) and is ignored.

use core_lib::engine;
use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::{resource, wire_enum};
use serde_json::Value;

const SETTINGS_PATH: &str = "/api/v1/settings";

/// Live-document keys whose `GET` shape is a `SettingMultiSource*` object
/// (`{configurationSource, databaseSource, effectiveValue}`) rather than a
/// bare scalar. [`flatten_live`] collapses each to its `effectiveValue` so it
/// can be diffed against the flat scalar the write side sends.
const MULTI_SOURCE_KEYS: [&str; 3] = ["kepubifyPath", "serverContextPath", "serverPort"];

/// Wire key of the one field that exists only on the write side
/// (`SettingsUpdateDto.renewRememberMeKey`) — a write-only action with no
/// `SettingsDto` counterpart, so it can never be read back and diffed.
const ACTION_ONLY_KEY: &str = "renewRememberMeKey";

/// Allowed thumbnail size values (`SettingsDto.thumbnailSize` /
/// `SettingsUpdateDto.thumbnailSize`).
#[wire_enum(rename_all = "UPPERCASE")]
pub enum ThumbnailSize {
    /// Use the server's default thumbnail size.
    Default,
    /// Medium-sized thumbnails.
    Medium,
    /// Large thumbnails.
    Large,
    /// Extra-large thumbnails.
    Xlarge,
    /// A size not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// Komga's global settings (`/api/v1/settings`). Every field is `Option`:
/// present = manage it, absent = leave Komga's current value alone.
#[resource(sync = custom, list = get("/api/v1/settings"))]
pub struct Settings {
    /// Delete collections left empty after a library scan.
    pub delete_empty_collections: Option<bool>,
    /// Delete read lists left empty after a library scan.
    pub delete_empty_read_lists: Option<bool>,
    /// Path to the `kepubify` binary, used to convert EPUB to KEPUB for Kobo
    /// devices. Deprecated on the write side by Komga itself.
    pub kepubify_path: Option<String>,
    /// Port used for the Kobo OPDS/sync proxy.
    pub kobo_port: Option<i32>,
    /// Enable the Kobo sync proxy.
    pub kobo_proxy: Option<bool>,
    /// Days a "remember me" session stays valid.
    pub remember_me_duration_days: Option<i64>,
    /// Write-only action: rotate the "remember me" signing key. Declaring
    /// `true` fires the rotation on every apply; there is no read-back state
    /// to converge toward, so this is never itself "in sync".
    pub renew_remember_me_key: Option<bool>,
    /// Path prefix Komga is served under (reverse-proxy sub-path).
    pub server_context_path: Option<String>,
    /// HTTP port Komga listens on.
    pub server_port: Option<i32>,
    /// Size of the background task worker pool.
    pub task_pool_size: Option<i32>,
    /// Default thumbnail size for newly generated thumbnails.
    pub thumbnail_size: Option<ThumbnailSize>,
}

/// Collapse the three `SettingMultiSource*` objects in a live `GET
/// /api/v1/settings` document down to their `effectiveValue`, so the result
/// is shaped like the flat `SettingsUpdateDto` write side and can be diffed
/// key-for-key against it. Every other key is copied verbatim.
fn flatten_live(live: &Value) -> Value {
    let mut out = live.as_object().cloned().unwrap_or_default();
    for key in MULTI_SOURCE_KEYS {
        if let Some(v) = out.get(key) {
            let effective = v.get("effectiveValue").cloned().unwrap_or(Value::Null);
            out.insert(key.to_string(), effective);
        }
    }
    Value::Object(out)
}

/// Numeric-insensitive equality — the live document and our encoded wire
/// values are both plain JSON scalars here, but comparing via `as_f64` first
/// avoids any `Number` representation mismatch (e.g. `i64` vs `u64` internal
/// storage) between what Komga returned and what we just encoded.
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a.as_f64(), b.as_f64()) {
        (Some(x), Some(y)) => x == y,
        _ => a == b,
    }
}

/// Diff the sparse desired wire object (only the keys the user wrote) against
/// the flattened live document, returning the subset that differ. The
/// write-only `renewRememberMeKey` action has no live counterpart: `true`
/// always "differs" (fires the rotation), `false`/absent never does.
fn diff(wire: &Value, live_flat: &Value) -> serde_json::Map<String, Value> {
    let mut changed = serde_json::Map::new();
    let Some(wire_obj) = wire.as_object() else {
        return changed;
    };
    for (key, want) in wire_obj {
        if key == ACTION_ONLY_KEY {
            if want.as_bool() == Some(true) {
                changed.insert(key.clone(), want.clone());
            }
            continue;
        }
        let have = live_flat.get(key);
        let in_sync = have.is_some_and(|h| values_equal(want, h));
        if !in_sync {
            changed.insert(key.clone(), want.clone());
        }
    }
    changed
}

impl CustomSync for Settings {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(Vec::new());
            };

            // Sparse by construction: every field is a plain `Option` with no
            // `#[default]`, so an absent config key decodes to `None` and the
            // standard encode omits it — this is already "only what the user
            // wrote", no separate presence mask needed.
            let wire = engine::encode_config::<Self>(cfg)?;

            let live: Value = client.get(SETTINGS_PATH).await?;
            let live_flat = flatten_live(&live);

            let changed = diff(&wire, &live_flat);
            if changed.is_empty() {
                return Ok(vec![Change::unchanged("settings")]);
            }

            if execute {
                let body = Value::Object(changed.clone());
                client.patch(SETTINGS_PATH, &body).await?;
            }

            let fields = changed.keys().cloned().collect::<Vec<_>>().join(", ");
            Ok(vec![Change::updated("settings").with("fields", fields)])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn flatten_live_collapses_multi_source_fields() {
        let live = json!({
            "deleteEmptyCollections": true,
            "kepubifyPath": {
                "configurationSource": "/etc/kepubify",
                "databaseSource": null,
                "effectiveValue": "/etc/kepubify"
            },
            "serverPort": {
                "configurationSource": 8080,
                "databaseSource": null,
                "effectiveValue": 25600
            },
        });
        let flat = flatten_live(&live);
        assert_eq!(flat["deleteEmptyCollections"], json!(true));
        assert_eq!(flat["kepubifyPath"], json!("/etc/kepubify"));
        assert_eq!(flat["serverPort"], json!(25600));
    }

    #[test]
    fn diff_only_reports_declared_and_differing_keys() {
        let live = json!({
            "deleteEmptyCollections": true,
            "taskPoolSize": 4,
            "thumbnailSize": "DEFAULT",
        });
        // Nothing declared differs.
        assert!(diff(&json!({ "deleteEmptyCollections": true }), &live).is_empty());
        // A declared key that differs is reported.
        let changed = diff(&json!({ "taskPoolSize": 8 }), &live);
        assert_eq!(changed.get("taskPoolSize"), Some(&json!(8)));
        // A key never declared is never reported, even though it "differs".
        assert!(!changed.contains_key("thumbnailSize"));
    }

    #[test]
    fn renew_remember_me_key_is_action_only() {
        let live = json!({});
        assert!(diff(&json!({ "renewRememberMeKey": false }), &live).is_empty());
        let changed = diff(&json!({ "renewRememberMeKey": true }), &live);
        assert_eq!(changed.get("renewRememberMeKey"), Some(&json!(true)));
    }
}
