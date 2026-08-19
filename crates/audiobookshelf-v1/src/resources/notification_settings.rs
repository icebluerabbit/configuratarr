//! `/api/notifications` — the Apprise-delivery settings singleton
//! (`NotificationSettings`), managed alongside — but distinct from — the
//! per-rule [`crate::resources::notification::Notification`] collection that
//! shares the same endpoint.
//!
//! `PATCH /api/notifications` only ever reads three flat keys —
//! `appriseApiUrl`, `maxFailedAttempts`, `maxNotificationQueue` (the exact set
//! the OpenAPI spec's inline PATCH request body declares; nothing else is
//! writable through this route, not even `notificationDelay` despite it
//! living right next to the other two in the read shape) — but their read
//! counterparts arrive nested one level down, under `.settings.*` in the
//! `GET /api/notifications` envelope (`{ data: {...}, settings:
//! NotificationSettings }`). A plain `sync = singleton` merges the flat
//! desired keys onto the envelope **root**, which never matches `.settings.*`
//! and would PATCH on every apply — forever. So this is `sync = custom`: GET
//! once, read `.settings`, presence-mask the desired config to what the user
//! wrote (`engine::config_present_to_wire`, same masking a stock `singleton`
//! gets for free), and PATCH only when something the user declared actually
//! differs from `.settings`.
use core_lib::Json;
use core_lib::engine;
use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// `/api/notifications` (read: the `.settings` half of the envelope; write:
/// `PATCH /api/notifications`, three keys only) — Apprise delivery settings.
#[resource(sync = custom, list = get("/api/notifications"))]
pub struct NotificationSettings {
    /// Always the literal `notification-settings` — this singleton's id,
    /// unrelated to any individual notification rule's id. Read-only.
    #[id]
    pub id: Option<String>,
    /// Apprise backend type; currently always `api` (only the Apprise
    /// API-server integration is implemented). Read-only — not one of the
    /// three keys `PATCH /api/notifications` accepts.
    #[wire(name = "appriseType", read_only)]
    pub apprise_type: Option<String>,
    /// Base URL of the Apprise API server notifications are POSTed to.
    /// Notifications are inert while this is unset; setting it to `null`
    /// disables them.
    // `#[wire(null)]`: a declared `null` has to reach the PATCH body as an
    // explicit null, not be dropped, or it can never clear the setting.
    #[wire(name = "appriseApiUrl", null)]
    pub apprise_api_url: Option<String>,
    /// Every configured notification rule, as `GET` reports them. Manage them
    /// under `notifications` at the top level, not here.
    // Opaque `Json` rather than a typed mirror: read-only from this resource,
    // kept only so the struct documents the full read shape.
    #[wire(read_only)]
    pub notifications: Option<Json>,
    /// Consecutive failures after which a rule auto-disables. API default
    /// `5` when unset.
    #[wire(name = "maxFailedAttempts")]
    pub max_failed_attempts: Option<i32>,
    /// Max events queued while a prior notification send is in flight; once
    /// full, further events are dropped. API default `20` when unset.
    #[wire(name = "maxNotificationQueue")]
    pub max_notification_queue: Option<i32>,
    /// Milliseconds delayed between processing queued notifications. API
    /// default `1000`. Read-only here — **not configurable via any route**;
    /// `PATCH /api/notifications` does not accept it despite it sitting
    /// right next to `max_failed_attempts`/`max_notification_queue` in the
    /// read shape.
    #[wire(name = "notificationDelay", read_only)]
    pub notification_delay: Option<i32>,
}

/// `want` (a declared wire value) vs `have` (the live value at the same key,
/// if any) — numeric-insensitive so an `i32` desired value compares equal to
/// whatever JSON number shape the live settings carries it as; a `have` that's
/// simply absent only counts as in sync when the declared value is itself
/// `null` (the `apprise_api_url` disable case).
fn field_in_sync(want: &Value, have: Option<&Value>) -> bool {
    match have {
        None => want.is_null(),
        Some(h) => match (want.as_f64(), h.as_f64()) {
            (Some(a), Some(b)) => a == b,
            _ => want == h,
        },
    }
}

/// Every key the presence-masked `wire` declares already matches `.settings`
/// at the same key.
fn in_sync(wire: &Value, live_settings: &Value) -> bool {
    let Some(obj) = wire.as_object() else {
        return true;
    };
    obj.iter()
        .all(|(k, want)| field_in_sync(want, live_settings.get(k)))
}

impl CustomSync for NotificationSettings {
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

            // Decode into the typed struct (validate, drop unknown keys),
            // presence-mask to declared keys, emit camelCase wire — the same
            // engine step a stock `singleton` gets for free.
            let wire = engine::config_present_to_wire::<Self>(cfg)?;

            let envelope: Value = client.get("/api/notifications").await?;
            let live_settings = envelope.get("settings").cloned().unwrap_or(Value::Null);

            if in_sync(&wire, &live_settings) {
                return Ok(vec![Change::unchanged("notification_settings")]);
            }

            if execute {
                // Exactly the flat body the route accepts — `wire` only ever
                // carries `appriseApiUrl`/`maxFailedAttempts`/
                // `maxNotificationQueue`, since every other field on this
                // struct is `#[wire(read_only)]` and so never presence-masked
                // in.
                let _: Value = client.patch("/api/notifications", &wire).await?;
            }
            Ok(vec![Change::updated("notification_settings")])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn in_sync_numeric_insensitive() {
        let wire = json!({"maxFailedAttempts": 5});
        let live = json!({"maxFailedAttempts": 5.0});
        assert!(in_sync(&wire, &live));
    }

    #[test]
    fn absent_live_key_only_in_sync_when_declared_null() {
        assert!(in_sync(&json!({"appriseApiUrl": null}), &json!({})));
        assert!(!in_sync(&json!({"maxFailedAttempts": 5}), &json!({})));
    }

    #[test]
    fn in_sync_catches_a_real_diff() {
        let wire = json!({"appriseApiUrl": "http://apprise.local"});
        let live = json!({"appriseApiUrl": "http://other.local"});
        assert!(!in_sync(&wire, &live));
    }
}
