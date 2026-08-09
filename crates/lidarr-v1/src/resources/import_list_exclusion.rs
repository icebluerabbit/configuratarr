//! Import list exclusion resource — an artist explicitly excluded from Lidarr's
//! import list processing.
//!
//! In Lidarr v1 this is a plain CRUD collection keyed by the MusicBrainz artist
//! ID (`foreignId`), not a provider-shaped resource. Each exclusion prevents a
//! specific artist from being added via import lists. This differs from Sonarr's
//! equivalent which uses a numeric TVDB id; Lidarr uses a string foreign id.

use core_macros::resource;

/// An artist excluded from Lidarr import list processing.
#[resource(
    sync = crud,
    list = get("/api/v1/importlistexclusion"),
    create = post("/api/v1/importlistexclusion"),
    update = put("/api/v1/importlistexclusion/${self.id}"),
    delete = delete("/api/v1/importlistexclusion/${self.id}"),
)]
pub struct ImportListExclusion {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the MusicBrainz artist ID being excluded. Referenced as
    /// `${ref.import_list_exclusion.<foreign_id>}`.
    #[key]
    pub foreign_id: String,
    /// Display name of the excluded artist.
    pub artist_name: Option<String>,
}
