//! Import list exclusion resource — a series explicitly excluded from Sonarr's
//! import list processing.
//!
//! In Sonarr v3 this is a plain CRUD collection keyed by TVDB series ID
//! (`tvdbId`), not a provider-shaped resource. Each exclusion prevents a
//! specific series from being added via import lists.

use core_macros::resource;

/// A series excluded from Sonarr import list processing.
#[resource(
    sync = crud,
    list = get("/api/v3/importlistexclusion"),
    create = post("/api/v3/importlistexclusion"),
    update = put("/api/v3/importlistexclusion/${self.id}"),
    delete = delete("/api/v3/importlistexclusion/${self.id}"),
)]
pub struct ImportListExclusion {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the TVDB series ID being excluded. Referenced as
    /// `${ref.import_list_exclusion.<tvdb_id>}`.
    #[key]
    pub tvdb_id: i32,
    /// Display title of the excluded series.
    pub title: Option<String>,
}
