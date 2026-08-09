//! `/api/release/profiles/duplicate` — duplicate/dedup profiles.
//!
//! A named set of boolean toggles that decide which release attributes make two
//! releases "the same" for dedup. Filters attach one via
//! `release_profile_duplicate_id`. The API exposes create + delete but **no
//! update** (`PUT` 405), so this is a `sync = custom` create-or-leave resource
//! ([`core_lib::reconcile::create_only_prune`], keyed by `name`) — edit by
//! delete+recreate. Under `--prune`, profiles the config no longer declares are
//! deleted via `DELETE /api/release/profiles/duplicate/{id}`.

use core_lib::engine;
use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// `/api/release/profiles/duplicate` — a dedup profile.
#[resource(
    sync = custom,
    case = snake,
    list = get("/api/release/profiles/duplicate")
)]
pub struct ReleaseProfileDuplicate {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.release_profile_duplicate.<name>}`).
    #[key]
    pub name: String,
    /// Treat differing protocol (torrent vs usenet) as distinct.
    pub protocol: Option<bool>,
    /// Match on the full release name.
    pub release_name: Option<bool>,
    /// Match on the release hash.
    pub hash: Option<bool>,
    /// Match on parsed title.
    pub title: Option<bool>,
    /// Match on parsed sub-title.
    pub sub_title: Option<bool>,
    /// Match on year.
    pub year: Option<bool>,
    /// Match on month.
    pub month: Option<bool>,
    /// Match on day.
    pub day: Option<bool>,
    /// Match on source (BluRay, WEB-DL, …).
    pub source: Option<bool>,
    /// Match on resolution.
    pub resolution: Option<bool>,
    /// Match on video codec.
    pub codec: Option<bool>,
    /// Match on container.
    pub container: Option<bool>,
    /// Match on dynamic range (HDR/DV).
    pub dynamic_range: Option<bool>,
    /// Match on audio.
    pub audio: Option<bool>,
    /// Match on release group.
    pub group: Option<bool>,
    /// Match on season.
    pub season: Option<bool>,
    /// Match on episode.
    pub episode: Option<bool>,
    /// Match on website/source tag.
    pub website: Option<bool>,
    /// Match on PROPER.
    pub proper: Option<bool>,
    /// Match on REPACK.
    pub repack: Option<bool>,
    /// Match on edition.
    pub edition: Option<bool>,
    /// Match on hybrid.
    pub hybrid: Option<bool>,
    /// Match on language.
    pub language: Option<bool>,
}

impl CustomSync for ReleaseProfileDuplicate {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/release/profiles/duplicate").await?;
            reconcile::create_only_prune(
                desired,
                &live,
                "name",
                prune,
                execute,
                |_name, cfg| {
                    let client = client.clone();
                    async move {
                        let wire = engine::encode_config::<Self>(&cfg)?;
                        let _: Value = client
                            .post("/api/release/profiles/duplicate", &wire)
                            .await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        client
                            .delete(&format!("/api/release/profiles/duplicate/{id}"))
                            .await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
