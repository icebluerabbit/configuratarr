//! RSS / wishlist providers (`RSS_<n>` config sections). Upsert by `DISPNAME`
//! (stub `addProvider` + `changeProvider` by internal `NAME`). See [`super`].

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// An RSS / wishlist provider.
#[resource(sync = custom, case = snake, list = get("/api?cmd=listProviders"))]
pub struct RssProvider {
    /// Display name — its identity (`DISPNAME`).
    #[key]
    #[wire(name = "DISPNAME")]
    pub dispname: String,
    /// Whether the provider is enabled.
    #[wire(name = "ENABLED")]
    pub enabled: Option<bool>,
    /// Feed host URL.
    #[wire(name = "HOST")]
    pub host: Option<String>,
    /// Download priority (lower = higher).
    #[wire(name = "DLPRIORITY")]
    pub dl_priority: Option<i32>,
    /// Download types this provider serves (CSV of `A`,`E`,`M`,`C`).
    #[wire(name = "DLTYPES")]
    pub dl_types: Option<String>,
    /// Download-client label to tag grabs from this feed.
    #[wire(name = "LABEL")]
    pub label: Option<String>,
}

impl CustomSync for RssProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            super::reconcile_encoded::<Self>(client, desired, execute, "rss", "rss").await
        })
    }
}
