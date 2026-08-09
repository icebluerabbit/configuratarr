//! Torznab providers (`Torznab_<n>` config sections). Upsert by `DISPNAME` via
//! `addProvider&type=torznab` / `changeProvider`. See [`super`].

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::resource;
use serde_json::Value;

/// A Torznab (torrent-over-newznab) provider.
#[resource(sync = custom, case = snake, list = get("/api?cmd=listProviders"))]
pub struct TorznabProvider {
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
    /// Provider API key.
    #[wire(name = "API")]
    pub api: Option<SecretValue>,
    /// eBook category ids (CSV).
    #[wire(name = "BOOKCAT")]
    pub book_cat: Option<String>,
    /// Magazine category ids (CSV).
    #[wire(name = "MAGCAT")]
    pub mag_cat: Option<String>,
    /// Audiobook category ids (CSV).
    #[wire(name = "AUDIOCAT")]
    pub audio_cat: Option<String>,
    /// Comic category ids (CSV).
    #[wire(name = "COMICCAT")]
    pub comic_cat: Option<String>,
    /// Use the extended API (`1`/`0`).
    #[wire(name = "EXTENDED")]
    pub extended: Option<i32>,
    /// Download priority (lower = higher).
    #[wire(name = "DLPRIORITY")]
    pub dl_priority: Option<i32>,
    /// Download types this provider serves (CSV of `A`,`E`,`M`,`C`).
    #[wire(name = "DLTYPES")]
    pub dl_types: Option<String>,
    /// Minimum seeders to accept a torrent.
    #[wire(name = "SEEDERS")]
    pub seeders: Option<i32>,
    /// Required seed ratio.
    #[wire(name = "SEED_RATIO")]
    pub seed_ratio: Option<f64>,
    /// Required seed duration (minutes).
    #[wire(name = "SEED_DURATION")]
    pub seed_duration: Option<i32>,
}

impl CustomSync for TorznabProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            super::reconcile_encoded::<Self>(client, desired, execute, "torznab", "torznab").await
        })
    }
}
