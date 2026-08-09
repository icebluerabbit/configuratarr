//! `/api/configuration/lidarr` — Lidarr app-level config and instances.

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::resource;
use serde_json::Value;

use crate::resources::arr::reconcile;

/// `/api/configuration/lidarr` — Lidarr-wide behaviour shared by every
/// declared instance.
///
/// The `GET` response additionally carries a server-assigned `id`, `type`, and
/// the live `instances` array; those are left unmanaged
/// (`merge(live, desired)` keeps them from the live value). Manage the
/// instances themselves via [`LidarrInstance`].
#[resource(
    sync = singleton,
    read   = get("/api/configuration/lidarr"),
    update = put("/api/configuration/lidarr"),
)]
pub struct LidarrConfig {
    /// Number of failed-import strikes a download may accrue against this app
    /// before it is treated as a failed import. `-1` disables the check.
    #[default(-1)]
    pub failed_import_max_strikes: i32,
}

/// `/api/configuration/lidarr/instances` — one connected Lidarr instance.
///
/// `sync = custom`: the list endpoint (`GET /api/configuration/lidarr`)
/// returns an envelope object (`{ id, type, instances }`), not a bare array,
/// and `apiKey` reads back masked — see [`crate::resources::arr::reconcile`]
/// and [`crate::diff::subset`].
#[resource(sync = custom, list = get("/api/configuration/lidarr"))]
pub struct LidarrInstance {
    /// Display name — its identity (`${ref.lidarr_instance.<name>}`).
    #[key]
    pub name: String,
    /// Whether Cleanuparr manages downloads against this instance.
    #[default(true)]
    pub enabled: bool,
    /// Base URL of the Lidarr instance. Must parse as an absolute URI.
    pub url: String,
    /// Lidarr API key. On create, the masked placeholder is rejected; on
    /// update, sending it back keeps the stored key.
    pub api_key: SecretValue,
    /// Lidarr's reported API/schema version, used to pick the right request
    /// shape for this instance.
    pub version: f64,
    /// Externally reachable URL for this instance (e.g. behind a reverse
    /// proxy), used when Cleanuparr needs to hand the user a clickable link.
    pub external_url: Option<String>,
}

impl CustomSync for LidarrInstance {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(reconcile::reconcile_instances::<Self>(
            client,
            "lidarr",
            "lidarr_instance",
            desired,
            refs,
            prune,
            execute,
        ))
    }
}
