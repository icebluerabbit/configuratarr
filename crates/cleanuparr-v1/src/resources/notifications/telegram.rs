use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::resource;
use serde_json::Value;

/// `/api/configuration/notification_providers/telegram` — a Telegram
/// notification provider. See `crate::resources::notifications` for the
/// shared read/write shape and the global-uniqueness note on `name`.
#[resource(sync = custom, list = get("/api/configuration/notification_providers"))]
pub struct TelegramProvider {
    /// Server-assigned id (GUID). Not stable across updates — an update
    /// deletes and recreates the row — so never key on it.
    #[id]
    pub id: Option<String>,
    /// Display name. Required and unique across *all* notification provider
    /// types, not just Telegram ones.
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
    /// Telegram bot token, at least 10 characters. Masked on read.
    pub bot_token: Option<SecretValue>,
    /// Target chat id (integer as a string; negative for groups).
    pub chat_id: Option<String>,
    /// Forum topic id (integer as a string), for chats with topics enabled.
    pub topic_id: Option<String>,
    /// Send the message without a notification sound.
    pub send_silently: Option<bool>,
}

impl CustomSync for TelegramProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(super::reconcile_provider::<Self>(
            client, desired, prune, execute, "telegram", "Telegram",
        ))
    }
}
