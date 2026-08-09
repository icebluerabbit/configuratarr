use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::{resource, wire_enum};
use serde_json::Value;

/// How the ntfy provider authenticates against the ntfy server.
#[wire_enum]
pub enum NtfyAuthenticationType {
    /// No authentication.
    None,
    /// HTTP basic auth via `username`/`password`.
    BasicAuth,
    /// Bearer auth via `access_token`.
    AccessToken,
    /// Unknown or future authentication type not yet modelled by this
    /// version.
    #[fallback]
    Unknown,
}

/// ntfy notification priority.
#[wire_enum]
pub enum NtfyPriority {
    /// Lowest priority.
    Min,
    /// Below-default priority.
    Low,
    /// Default priority.
    Default,
    /// Above-default priority.
    High,
    /// Highest priority (triggers ntfy's most intrusive delivery).
    Max,
    /// Unknown or future priority not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// `/api/configuration/notification_providers/ntfy` — an ntfy.sh
/// notification provider. See `crate::resources::notifications` for the
/// shared read/write shape and the global-uniqueness note on `name`.
#[resource(sync = custom, list = get("/api/configuration/notification_providers"))]
pub struct NtfyProvider {
    /// Server-assigned id (GUID). Not stable across updates — an update
    /// deletes and recreates the row — so never key on it.
    #[id]
    pub id: Option<String>,
    /// Display name. Required and unique across *all* notification provider
    /// types, not just ntfy ones.
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
    /// Base URL of the ntfy server (self-hosted or `https://ntfy.sh`).
    pub server_url: Option<String>,
    /// Topics to publish to. At least one non-blank topic is required.
    pub topics: Vec<String>,
    /// Authentication method against the ntfy server.
    pub authentication_type: Option<NtfyAuthenticationType>,
    /// Username, required for `BasicAuth`.
    pub username: Option<String>,
    /// Password, required for `BasicAuth`. Masked on read.
    pub password: Option<SecretValue>,
    /// Access token, required for `AccessToken`. Masked on read.
    pub access_token: Option<SecretValue>,
    /// Message priority.
    pub priority: Option<NtfyPriority>,
    /// ntfy tags/emoji shown alongside the notification.
    pub tags: Vec<String>,
}

impl CustomSync for NtfyProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(super::reconcile_provider::<Self>(
            client, desired, prune, execute, "ntfy", "Ntfy",
        ))
    }
}
