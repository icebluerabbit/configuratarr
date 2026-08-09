//! Collection-resource for Lidarr tags. The simplest archetype: a server id
//! and a natural key label.

use core_macros::resource;

/// A label applied to artists, albums, import lists, indexers, download clients,
/// notifications, etc.
#[resource(
    sync = crud,
    list = get("/api/v1/tag"),
    create = post("/api/v1/tag"),
    update = put("/api/v1/tag/${self.id}"),
    delete = delete("/api/v1/tag/${self.id}"),
)]
pub struct Tag {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the name referenced in `${ref.tag.<label>}`.
    #[key]
    pub label: String,
}
