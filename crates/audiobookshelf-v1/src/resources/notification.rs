//! `/api/notifications` — one configured notification rule.
//!
//! Audiobookshelf serves the notification **event catalog** and the
//! **settings singleton** (which nests the actual rule array) behind the same
//! `GET /api/notifications` envelope: `{ data: { events: [...] }, settings:
//! NotificationSettings }`. The rules this resource manages live at
//! `.settings.notifications`, not at the envelope's top level.
//!
//! Mutating a single rule — `POST /api/notifications` (create), `PATCH
//! /api/notifications/{id}` (update), `DELETE /api/notifications/{id}`
//! (delete) — returns the **whole** `NotificationSettings` object, not the
//! mutated rule, so none of those responses are useful for diffing.
//!
//! A rule has no name of its own; its identity is the composite `(eventName,
//! libraryId)` (audiobookshelf allows at most one rule per event per library,
//! plus one "all libraries" rule per event). [`core_lib::reconcile::upsert_prune`]
//! matches by a single string key, so `reconcile` stamps a synthetic
//! `"{eventName}@{libraryId|global}"` under a `_identity` key onto *copies* of
//! the desired/live wire values purely for that matching — [`strip_identity`]
//! removes it again before anything reaches the wire, and the create/update
//! closures always send the encoded resource body, never the keyed copy.
//!
//! `sync = custom` because of both quirks above (nested list, non-item
//! mutation responses); `list = get("/api/notifications")` is declared for
//! doc-gen / descriptor completeness — the hook re-fetches it itself since it
//! must reach into `.settings.notifications`, not the bare array a `sync =
//! crud` list step expects.
//!
//! [`in_sync`] ignores `id` (the synthetic-key source, never on the wire
//! anyway) and the server-computed firing-history fields (`lastFiredAt`,
//! `lastAttemptFailed`, `numConsecutiveFailedAttempts`, `numTimesFired`,
//! `createdAt`) — everything else must match exactly.

use core_lib::engine;
use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// `/api/notifications` — one rule: fire `event_name` (optionally scoped to
/// `library_id`) to `urls` via Apprise.
#[resource(sync = custom, list = get("/api/notifications"))]
pub struct Notification {
    /// Server-generated id (uuidv4), assigned on create. Never sent — the
    /// [`CustomSync`] hook keys off the composite `(event_name, library_id)`
    /// instead and reads this back off the live item when it needs the URL
    /// path (`PATCH`/`DELETE /api/notifications/{id}`).
    #[id]
    pub id: Option<String>,
    /// Restrict this rule to one library; unset applies to all libraries.
    /// Part of this rule's identity alongside `event_name` — audiobookshelf
    /// allows one rule per event per library, plus one "all libraries" rule
    /// per event.
    #[reference(library)]
    #[wire(name = "libraryId", null)]
    pub library_id: Option<String>,
    /// The event that fires this rule. This rule's other identity component.
    /// One of `onPodcastEpisodeDownloaded`, `onBackupCompleted`,
    /// `onBackupFailed`, `onRSSFeedFailed`, `onRSSFeedDisabled`, `onTest`.
    #[key]
    pub event_name: String,
    /// Apprise URLs (<https://github.com/caronc/apprise>) the payload is
    /// POSTed to via the configured Apprise API. Must be non-empty for the
    /// rule to actually fire.
    pub urls: Vec<String>,
    /// Title template; `{{variable}}` placeholders are filled from the firing
    /// event's data. Absent from a fixture that doesn't override it —
    /// audiobookshelf may substitute its own per-event default text, which
    /// this resource has no way to read back and compare against.
    pub title_template: Option<String>,
    /// Body template; `{{variable}}` placeholders are filled from the firing
    /// event's data. Same per-event-default caveat as `title_template`.
    pub body_template: Option<String>,
    /// Whether this rule fires. Toggling disabled → enabled resets
    /// `last_fired_at`, `last_attempt_failed`, and
    /// `num_consecutive_failed_attempts` server-side. Defaults to `false`
    /// (the API's own POST default) so an omitted value still round-trips
    /// against a live `false` rather than a missing key.
    #[default(false)]
    pub enabled: bool,
    /// Free-form client-UI styling hint — the server does not validate it
    /// against its own documented enum (`info`/`success`/`warning`/
    /// `failure`). Omitted ⇒ wire `null` (`#[wire(null)]`), matching the
    /// server's own "defaults to null, not `info`, if omitted" behavior.
    #[wire(name = "type", null)]
    pub notification_type: Option<String>,
    /// Unix ms timestamp of the last delivery attempt, or unset if never
    /// fired (or just re-enabled). Read-only.
    #[wire(name = "lastFiredAt", read_only)]
    pub last_fired_at: Option<i64>,
    /// Whether the last delivery attempt failed. Read-only.
    #[wire(read_only)]
    pub last_attempt_failed: Option<bool>,
    /// Consecutive failure count; once it reaches
    /// `NotificationSettings.max_failed_attempts` the rule is auto-disabled.
    /// Read-only.
    #[wire(read_only)]
    pub num_consecutive_failed_attempts: Option<i32>,
    /// Total delivery attempts, success or failure. Read-only.
    #[wire(read_only)]
    pub num_times_fired: Option<i32>,
    /// Unix ms timestamp set when the rule was created. Read-only.
    #[wire(read_only)]
    pub created_at: Option<i64>,
}

/// Wire keys [`in_sync`] never compares: `_identity` is our own synthetic
/// matching key (never reaches the wire — see [`strip_identity`]); the rest
/// are server-computed firing history no desired config can express.
const IGNORED_KEYS: &[&str] = &[
    "_identity",
    "id",
    "lastFiredAt",
    "lastAttemptFailed",
    "numConsecutiveFailedAttempts",
    "numTimesFired",
    "createdAt",
];

/// `v`'s object with [`IGNORED_KEYS`] removed, for [`in_sync`]'s comparison.
fn stripped_for_compare(v: &Value) -> Value {
    let mut obj = v.as_object().cloned().unwrap_or_default();
    for k in IGNORED_KEYS {
        obj.remove(*k);
    }
    Value::Object(obj)
}

/// Every field but the ignored ones must match exactly — audiobookshelf
/// round-trips a created/updated rule verbatim (no redaction, no
/// server-rewritten sub-values), so plain equality (not a subset test) is the
/// right idempotency predicate here.
fn in_sync(desired: &Value, live: &Value) -> bool {
    stripped_for_compare(desired) == stripped_for_compare(live)
}

/// `"{eventName}@{libraryId|global}"` — this rule's composite identity, used
/// only to key the [`reconcile::upsert_prune`] match. Never written to the
/// wire under this name; see [`with_identity`] / [`strip_identity`].
fn identity_of(v: &Value) -> String {
    let event = v.get("eventName").and_then(Value::as_str).unwrap_or("");
    let library = v
        .get("libraryId")
        .and_then(Value::as_str)
        .unwrap_or("global");
    format!("{event}@{library}")
}

/// Stamp `_identity` onto a copy of `v` (an object) for [`reconcile::upsert_prune`]
/// to match on.
fn with_identity(mut v: Value) -> Value {
    let identity = identity_of(&v);
    if let Some(obj) = v.as_object_mut() {
        obj.insert("_identity".to_string(), Value::String(identity));
    }
    v
}

/// Remove the synthetic `_identity` key before a value reaches the wire.
fn strip_identity(mut v: Value) -> Value {
    if let Some(obj) = v.as_object_mut() {
        obj.remove("_identity");
    }
    v
}

impl CustomSync for Notification {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            // The envelope's `.settings.notifications` is the live rule list
            // — the bare `GET` response is `{ data: {...}, settings: {...} }`,
            // not an array.
            let envelope: Value = client.get("/api/notifications").await?;
            let live: Vec<Value> = envelope
                .get("settings")
                .and_then(|s| s.get("notifications"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            // Full typed wire per desired rule (id is read-only, so absent
            // here regardless).
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            let desired_keyed: Vec<Value> = wire.into_iter().map(with_identity).collect();
            let live_keyed: Vec<Value> = live.into_iter().map(with_identity).collect();

            reconcile::upsert_prune(
                &desired_keyed,
                &live_keyed,
                "_identity",
                in_sync,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let body = strip_identity(w);
                        let _: Value = client.post("/api/notifications", &body).await?;
                        Ok(())
                    }
                },
                |l, w| {
                    let client = client.clone();
                    // The server matches the rule to update by the body's
                    // `id`, not the path — it does not fall back to `{id}` in
                    // the URL — so the update must carry it explicitly.
                    let id = l
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    let mut body = strip_identity(w);
                    reconcile::echo(&mut body, "id", l);
                    async move {
                        let _: Value = client
                            .patch(&format!("/api/notifications/{id}"), &body)
                            .await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    async move {
                        client.delete(&format!("/api/notifications/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identity_uses_global_when_library_unset() {
        assert_eq!(
            identity_of(&json!({"eventName": "onBackupFailed"})),
            "onBackupFailed@global"
        );
        assert_eq!(
            identity_of(&json!({"eventName": "onBackupFailed", "libraryId": "lib-1"})),
            "onBackupFailed@lib-1"
        );
    }

    #[test]
    fn in_sync_ignores_firing_history_and_identity() {
        let desired = json!({
            "_identity": "onBackupFailed@global",
            "eventName": "onBackupFailed",
            "libraryId": null,
            "urls": ["http://example/notify"],
            "enabled": true,
        });
        let live = json!({
            "_identity": "onBackupFailed@global",
            "id": "abc-123",
            "eventName": "onBackupFailed",
            "libraryId": null,
            "urls": ["http://example/notify"],
            "enabled": true,
            "lastFiredAt": 1_700_000_000_000i64,
            "lastAttemptFailed": false,
            "numConsecutiveFailedAttempts": 0,
            "numTimesFired": 3,
            "createdAt": 1_699_000_000_000i64,
        });
        assert!(in_sync(&desired, &live));
    }

    #[test]
    fn in_sync_catches_a_real_diff() {
        let desired = json!({"eventName": "onBackupFailed", "enabled": true});
        let live = json!({"eventName": "onBackupFailed", "enabled": false});
        assert!(!in_sync(&desired, &live));
    }
}
