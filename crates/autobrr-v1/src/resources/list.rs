//! `/api/lists` — autobrr lists: pull wanted titles from an *arr instance or an
//! external source (Trakt/MDBList/…) and auto-attach them to filters.
//!
//! `sync = custom`, keyed by `name`, create + update by id (`POST /api/lists`,
//! `PUT /api/lists/{id}`), and `--prune` deletes lists the config no longer
//! declares via `DELETE /api/lists/{id}`. Idempotency is a structural-subset test
//! over the **readable** fields ([`readable_matches`]): `api_key` is redacted on
//! read, and `filters` is *write-only* (the list endpoint never echoes the filter
//! attachment — see [`readable_matches`]), so both are excluded from the diff. The
//! create/update/prune skeleton is [`core_lib::reconcile::upsert_prune`].
//!
//! FKs: `client_id` → a managed download client (`${ref.download_client.<name>}`,
//! required for the *arr list types); `filters[].id` → managed filters
//! (`${ref.filter.<name>}`). autobrr requires at least one filter for *arr-type
//! lists and a `url` for the external list types.

use core_lib::{
    CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine, reconcile,
};
use core_macros::resource;
use serde_json::Value;

use crate::diff;
use crate::resources::list_filter::ListFilter;

/// `/api/lists` — a configured list.
#[resource(sync = custom, case = snake, list = get("/api/lists"))]
pub struct List {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.list.<name>}`).
    #[key]
    pub name: String,
    /// List type: `RADARR`, `SONARR`, `LIDARR`, `READARR`, `WHISPARR` (*arr
    /// types, need `client_id`), or `MDBLIST`, `METACRITIC`, `PLAINTEXT`, `TRAKT`,
    /// `STEAM`, `ANILIST` (external types, need `url`).
    #[wire(name = "type")]
    pub list_type: String,
    /// Whether the list is active.
    pub enabled: Option<bool>,
    /// Download client to pull wanted titles from, for the *arr list types
    /// (`${ref.download_client.<name>}`).
    #[reference(download_client)]
    pub client_id: Option<i32>,
    /// Source URL, for the external list types.
    pub url: Option<String>,
    /// Extra HTTP headers to send when fetching an external list.
    pub headers: Vec<String>,
    /// API key for the external source, where required.
    pub api_key: Option<SecretValue>,
    /// Filters this list feeds into (`${ref.filter.<name>}`). autobrr requires at
    /// least one for the *arr list types.
    pub filters: Vec<ListFilter>,
    /// Match on the release name rather than the parsed title.
    pub match_release: Option<bool>,
    /// Only include titles carrying these tags.
    pub tags_included: Vec<String>,
    /// Exclude titles carrying these tags.
    pub tags_excluded: Vec<String>,
    /// Include unmonitored titles (*arr types).
    pub include_unmonitored: Option<bool>,
    /// Include alternate titles (*arr types).
    pub include_alternate_titles: Option<bool>,
    /// Append the year to the matched title.
    pub include_year: Option<bool>,
    /// Skip autobrr's title clean/sanitize pass.
    pub skip_clean_sanitize: Option<bool>,
}

/// Idempotency predicate. autobrr's list endpoint (`GET /api/lists`) **never
/// returns the filter attachment** — the find-all query hardcodes `filters: []`
/// (there is no `GET /api/lists/{id}` either), so `filters` is a *write-only*
/// field, exactly like an indexer's `settings`. Diffing on it would report a
/// perpetual drift (declared `[{id}]` vs live `[]`) and re-`PUT` on every apply —
/// and a list update immediately 500s on a never-refreshed list (autobrr's
/// `FindByID` scans a NULL `last_refresh_time` into a non-nullable `*time.Time`).
/// So exclude `filters` from the diff: a list is in sync when its **readable**
/// fields match. A filters-only edit therefore isn't detected (re-sent only when
/// another field drifts) — the same trade-off as indexer `settings`.
fn readable_matches(wire: &Value, live: &Value) -> bool {
    let mut readable = wire.clone();
    if let Some(obj) = readable.as_object_mut() {
        obj.remove("filters");
    }
    diff::subset(&readable, live)
}

impl CustomSync for List {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/lists").await?;
            // Full desired wire (refs already resolved to ids; id is read-only).
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert_prune(
                &wire,
                &live,
                "name",
                readable_matches,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/lists", &w).await?;
                        Ok(())
                    }
                },
                |l, mut w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    // autobrr's list update decodes the whole list from the *body*
                    // and never reads the `{listID}` path param, so it looks the
                    // list up by the body's `id` (`FindByID` → "no rows" 500 when
                    // absent). `#[id]` omits it on encode, so echo the live id back.
                    reconcile::echo(&mut w, "id", l);
                    async move {
                        let _: Value = client.put(&format!("/api/lists/{id}"), &w).await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        client.delete(&format!("/api/lists/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
