use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::{resource, wire_enum};
use serde_json::Value;

/// Apprise delivery mode: talk to a running `apprise-api` container, or shell
/// out to the `apprise` CLI directly.
#[wire_enum]
pub enum AppriseMode {
    /// API mode: POST to a running `apprise-api` container.
    Api,
    /// CLI mode: invoke the `apprise` command-line tool directly.
    Cli,
    /// Unknown or future mode not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// `/api/configuration/notification_providers/apprise` — an Apprise
/// notification provider. See `crate::resources::notifications` for the
/// shared read/write shape and the global-uniqueness note on `name`.
#[resource(sync = custom, list = get("/api/configuration/notification_providers"))]
pub struct AppriseProvider {
    /// Server-assigned id (GUID). Not stable across updates — an update
    /// deletes and recreates the row — so never key on it.
    #[id]
    pub id: Option<String>,
    /// Display name. Required and unique across *all* notification provider
    /// types, not just Apprise ones.
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
    /// Whether to talk to an `apprise-api` container (`Api`) or shell out to
    /// the `apprise` CLI (`Cli`).
    pub mode: Option<AppriseMode>,
    /// API mode: base URL of the `apprise-api` container.
    pub url: Option<String>,
    /// API mode: configuration key, at least 2 characters. Masked on read.
    pub key: Option<SecretValue>,
    /// Comma-separated Apprise tag expression restricting which configured
    /// URLs a notification is sent to.
    pub tags: Option<String>,
    /// CLI mode: one Apprise service URL per line. Masked on read down to
    /// the scheme (e.g. `discord://••••••••`).
    pub service_urls: Option<SecretValue>,
}

impl CustomSync for AppriseProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(super::reconcile_provider::<Self>(
            client, desired, prune, execute, "apprise", "Apprise",
        ))
    }
}
