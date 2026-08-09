//! Direct / generic providers (`GEN_<n>` config sections). Listed under `direct`
//! by `listProviders`; upsert by `DISPNAME` (stub `addProvider&type=gen` +
//! `changeProvider` by internal `NAME`). See [`super`].

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// A direct/generic (GEN) provider.
#[resource(sync = custom, case = snake, list = get("/api?cmd=listProviders"))]
pub struct GenProvider {
    /// Display name — its identity (`DISPNAME`).
    #[key]
    #[wire(name = "DISPNAME")]
    pub dispname: String,
    /// Whether the provider is enabled.
    #[wire(name = "ENABLED")]
    pub enabled: Option<bool>,
    /// Base host URL.
    #[wire(name = "HOST")]
    pub host: Option<String>,
    /// Search path/template.
    #[wire(name = "SEARCH")]
    pub search: Option<String>,
    /// Download priority (lower = higher).
    #[wire(name = "DLPRIORITY")]
    pub dl_priority: Option<i32>,
    /// Download types this provider serves (CSV of `A`,`E`,`M`,`C`).
    #[wire(name = "DLTYPES")]
    pub dl_types: Option<String>,
}

impl CustomSync for GenProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            super::reconcile_encoded::<Self>(client, desired, execute, "direct", "gen").await
        })
    }
}
