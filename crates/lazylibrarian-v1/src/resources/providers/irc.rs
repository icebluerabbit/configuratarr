//! IRC providers (`IRC_<n>` config sections). Upsert by `DISPNAME` (stub
//! `addProvider` + `changeProvider` by internal `NAME`); the host key is `SERVER`
//! (not `HOST`). See [`super`].

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// An IRC provider.
#[resource(sync = custom, case = snake, list = get("/api?cmd=listProviders"))]
pub struct IrcProvider {
    /// Display name — its identity (`DISPNAME`).
    #[key]
    #[wire(name = "DISPNAME")]
    pub dispname: String,
    /// Whether the provider is enabled.
    #[wire(name = "ENABLED")]
    pub enabled: Option<bool>,
    /// IRC server host.
    #[wire(name = "SERVER")]
    pub server: Option<String>,
    /// Channel to join.
    #[wire(name = "CHANNEL")]
    pub channel: Option<String>,
    /// Bot nick to search under.
    #[wire(name = "BOTNICK")]
    pub bot_nick: Option<String>,
    /// Search command prefix (default `@search`).
    #[wire(name = "SEARCH")]
    pub search: Option<String>,
    /// Download priority (lower = higher).
    #[wire(name = "DLPRIORITY")]
    pub dl_priority: Option<i32>,
    /// Download types this provider serves (CSV of `A`,`E`,`M`,`C`).
    #[wire(name = "DLTYPES")]
    pub dl_types: Option<String>,
}

impl CustomSync for IrcProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            super::reconcile_encoded::<Self>(client, desired, execute, "irc", "irc").await
        })
    }
}
