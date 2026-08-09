//! Notification providers.
//!
//! Bazarr's notifiers (Apprise-backed) write through `/api/system/settings` (the
//! same POST as [`Settings`]) but as a **side-channel** form field, not a
//! `settings-*` key:
//! * `notifications-providers` — one repeated field per provider, each a **JSON
//!   string** `{"name", "enabled", "url"}`, with **sparse-update-by-`name`**
//!   semantics (bazarr updates only the submitted providers and leaves the rest
//!   of its catalogue untouched — unlike `languages-profiles`, which full-replaces).
//!
//! They're read back from the `notifications.providers` list embedded in the
//! `/api/system/settings` GET (there is no dedicated notifications endpoint). The
//! per-provider JSON shape (`enabled` defaulting to `false`, `url` an explicit
//! null) is produced by the [`Notifier`] descriptor via `engine::encode`, so the
//! desired matches the read shape and the diff stays structural. This stays
//! `sync = custom` only for the *write contract* — the JSON-string-per-item side
//! channel and sparse-update-by-`name` diff — not for any hand-rolled
//! translation, like [`Languages`].
//!
//! [`Settings`]: crate::resources::settings::Settings
//! [`Languages`]: crate::resources::languages::Languages

use std::collections::HashMap;

use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::resource;
use serde_json::Value;

use crate::resources::notifier::Notifier;

const SETTINGS_PATH: &str = "/api/system/settings";

/// Notification providers, reconciled sparsely by `name`: only the providers you
/// declare are updated; bazarr's other providers are left as they are. Written
/// via the settings POST.
#[resource(sync = custom, list = get("/api/system/settings"))]
pub struct Notifications {
    /// Notification providers to manage (sparse, keyed by `name`). Each declared
    /// provider is updated; providers you don't list are left untouched.
    pub providers: Vec<Notifier>,
}

/// Translate one config provider into bazarr's exact stored JSON shape
/// (`{name, enabled, url}`) via the [`Notifier`] descriptor — `enabled` defaults
/// to `false` and `url` renders as an explicit null (see the struct's field
/// attributes), so no hand-rolled JSON is needed.
fn wire_provider(p: &Value) -> anyhow::Result<Value> {
    engine::encode_config::<Notifier>(p)
}

/// Every declared provider already matches the live catalogue entry of the same
/// `name` (a sparse, per-name comparison — providers not declared are ignored).
/// `declared` is the already-wired form (see [`wire_provider`]).
fn providers_synced(declared: &[Value], live: &[Value]) -> bool {
    let by_name: HashMap<&str, &Value> = live
        .iter()
        .filter_map(|p| p.get("name").and_then(Value::as_str).map(|n| (n, p)))
        .collect();
    declared.iter().all(|want| {
        let name = want.get("name").and_then(Value::as_str).unwrap_or("");
        match by_name.get(name) {
            None => false,
            Some(live) => {
                live.get("enabled").unwrap_or(&Value::Null)
                    == want.get("enabled").unwrap_or(&Value::Null)
                    && live.get("url").unwrap_or(&Value::Null)
                        == want.get("url").unwrap_or(&Value::Null)
            }
        }
    })
}

/// The `notifications.providers` list from a `/api/system/settings` response.
fn live_providers(settings: &Value) -> Vec<Value> {
    settings
        .get("notifications")
        .and_then(|n| n.get("providers"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

impl CustomSync for Notifications {
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
            let Some(declared) = cfg.get("providers").and_then(Value::as_array) else {
                return Ok(vec![Change::unchanged("notifications")]);
            };
            if declared.is_empty() {
                return Ok(vec![Change::unchanged("notifications")]);
            }

            let wired: Vec<Value> = declared
                .iter()
                .map(wire_provider)
                .collect::<anyhow::Result<_>>()?;

            let settings: Value = client.get(SETTINGS_PATH).await?;
            if providers_synced(&wired, &live_providers(&settings)) {
                return Ok(vec![Change::unchanged("notifications")]);
            }

            // One `notifications-providers` field per declared provider, each a
            // JSON string — bazarr's `getlist` + `json.loads` per-item contract.
            let pairs: Vec<(String, String)> = wired
                .iter()
                .map(|w| ("notifications-providers".to_string(), w.to_string()))
                .collect();
            if execute {
                let _: Value = client.post_form(SETTINGS_PATH, &pairs).await?;
            }

            Ok(vec![
                Change::updated("notifications").with("providers", declared.len().to_string()),
            ])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn wire_provider_matches_bazarr_stored_shape() {
        assert_eq!(
            wire_provider(&json!({ "name": "Discord", "enabled": true, "url": "discord://x/y" }))
                .unwrap(),
            json!({ "name": "Discord", "enabled": true, "url": "discord://x/y" })
        );
    }

    #[test]
    fn omitted_enabled_defaults_false_and_url_null() {
        assert_eq!(
            wire_provider(&json!({ "name": "Telegram" })).unwrap(),
            json!({ "name": "Telegram", "enabled": false, "url": null })
        );
    }

    #[test]
    fn synced_is_sparse_by_name() {
        let live = json!([
            { "name": "Discord", "enabled": true, "url": "discord://x/y" },
            { "name": "Bark", "enabled": false, "url": null },
        ]);
        let live = live.as_array().unwrap();

        let wire = |v: Value| vec![wire_provider(&v).unwrap()];

        // Declared matches its live entry; the other provider is ignored.
        assert!(providers_synced(
            &wire(json!({ "name": "Discord", "enabled": true, "url": "discord://x/y" })),
            live
        ));
        // A differing field is out of sync.
        assert!(!providers_synced(
            &wire(json!({ "name": "Discord", "enabled": false, "url": "discord://x/y" })),
            live
        ));
        // An unknown provider name is out of sync (bazarr has no such catalogue entry live).
        assert!(!providers_synced(
            &wire(json!({ "name": "Nope", "enabled": true })),
            live
        ));
    }
}
