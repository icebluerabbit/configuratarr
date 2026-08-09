//! `/api/configuration/readarr` — Readarr app-level config and instances.

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::resource;
use serde_json::Value;

use crate::resources::arr::reconcile;

/// `/api/configuration/readarr` — Readarr-wide behaviour shared by every
/// declared instance.
///
/// The `GET` response additionally carries a server-assigned `id`, `type`, and
/// the live `instances` array; those are left unmanaged
/// (`merge(live, desired)` keeps them from the live value). Manage the
/// instances themselves via [`ReadarrInstance`].
#[resource(
    sync = singleton,
    read   = get("/api/configuration/readarr"),
    update = put("/api/configuration/readarr"),
)]
pub struct ReadarrConfig {
    /// Number of failed-import strikes a download may accrue against this app
    /// before it is treated as a failed import. `-1` disables the check.
    #[default(-1)]
    pub failed_import_max_strikes: i32,
}

/// `/api/configuration/readarr/instances` — one connected Readarr instance.
///
/// `sync = custom`: the list endpoint (`GET /api/configuration/readarr`)
/// returns an envelope object (`{ id, type, instances }`), not a bare array,
/// and `apiKey` reads back masked — see [`crate::resources::arr::reconcile`]
/// and [`crate::diff::subset`].
#[resource(sync = custom, list = get("/api/configuration/readarr"))]
pub struct ReadarrInstance {
    /// Display name — its identity (`${ref.readarr_instance.<name>}`).
    #[key]
    pub name: String,
    /// Whether Cleanuparr manages downloads against this instance.
    #[default(true)]
    pub enabled: bool,
    /// Base URL of the Readarr instance. Must parse as an absolute URI.
    pub url: String,
    /// Readarr API key. On create, the masked placeholder is rejected; on
    /// update, sending it back keeps the stored key.
    pub api_key: SecretValue,
    /// Readarr's reported API/schema version, used to pick the right request
    /// shape for this instance.
    pub version: f64,
    /// Externally reachable URL for this instance (e.g. behind a reverse
    /// proxy), used when Cleanuparr needs to hand the user a clickable link.
    pub external_url: Option<String>,
}

impl CustomSync for ReadarrInstance {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(reconcile::reconcile_instances::<Self>(
            client,
            "readarr",
            "readarr_instance",
            desired,
            refs,
            prune,
            execute,
        ))
    }
}
