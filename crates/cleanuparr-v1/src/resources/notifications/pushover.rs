use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::{resource, wire_enum};
use serde_json::Value;

/// Pushover message priority.
#[wire_enum]
pub enum PushoverPriority {
    /// No sound or vibration; the message is only shown in the Pushover app.
    Lowest,
    /// No sound or vibration by default, but shown in notifications.
    Low,
    /// Default priority: normal sound/vibration.
    Normal,
    /// Bypasses the user's quiet hours.
    High,
    /// Repeats until acknowledged. Requires `retry` and `expire`.
    Emergency,
    /// Unknown or future priority not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// `/api/configuration/notification_providers/pushover` — a Pushover
/// notification provider. See `crate::resources::notifications` for the
/// shared read/write shape and the global-uniqueness note on `name`.
#[resource(sync = custom, list = get("/api/configuration/notification_providers"))]
pub struct PushoverProvider {
    /// Server-assigned id (GUID). Not stable across updates — an update
    /// deletes and recreates the row — so never key on it.
    #[id]
    pub id: Option<String>,
    /// Display name. Required and unique across *all* notification provider
    /// types, not just Pushover ones.
    #[key]
    pub name: String,
    /// Whether this provider is active.
    #[default(true)]
    pub is_enabled: bool,
    /// Notify when a download is struck for a failed import.
    pub on_failed_import_strike: Option<bool>,
    /// Notify when a download is struck for stalling.
    pub on_stalled_strike: Option<bool>,
    /// Notify when a download is struck for being too slow.
    pub on_slow_strike: Option<bool>,
    /// Notify when a queue item is deleted.
    pub on_queue_item_deleted: Option<bool>,
    /// Notify when a download is cleaned (seeding finished / orphaned).
    pub on_download_cleaned: Option<bool>,
    /// Notify when a download's category changes.
    pub on_category_changed: Option<bool>,
    /// Notify when a search is triggered.
    pub on_search_triggered: Option<bool>,
    /// Notify when a search grabs an item.
    pub on_search_item_grabbed: Option<bool>,
    /// Pushover application API token. Masked on read.
    pub api_token: Option<SecretValue>,
    /// Pushover user or group key. Masked on read.
    pub user_key: Option<SecretValue>,
    /// Device names to target (letters, digits, underscore and hyphen only).
    /// Empty targets every device on the account.
    pub devices: Vec<String>,
    /// Message priority.
    pub priority: Option<PushoverPriority>,
    /// Built-in Pushover sound name (`pushover`, `bike`, `siren`, `none`, …)
    /// or a custom one.
    pub sound: Option<String>,
    /// Seconds between repeat notifications. Required for `Emergency`
    /// priority; at least 30 seconds.
    pub retry: Option<i32>,
    /// Seconds before Pushover stops repeating an `Emergency` notification.
    /// Required for `Emergency` priority; at most 10800 seconds.
    pub expire: Option<i32>,
    /// Pushover tags attached to the notification.
    pub tags: Vec<String>,
}

impl CustomSync for PushoverProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(super::reconcile_provider::<Self>(
            client, desired, prune, execute, "pushover", "Pushover",
        ))
    }
}
