use core_macros::resource;

/// Application sync profile — controls how Prowlarr syncs indexers to connected
/// applications (Sonarr, Radarr, etc.).
#[resource(
    sync = crud,
    list = get("/api/v1/appprofile"),
    create = post("/api/v1/appprofile"),
    update = put("/api/v1/appprofile/${self.id}"),
    delete = delete("/api/v1/appprofile/${self.id}"),
)]
pub struct AppProfile {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the display name for this application profile.
    #[key]
    pub name: String,
    /// Enables RSS feed syncing for indexers using this profile.
    pub enable_rss: bool,
    /// Enables automatic (monitored) search syncing for indexers using this profile.
    pub enable_automatic_search: bool,
    /// Enables interactive (manual) search syncing for indexers using this profile.
    pub enable_interactive_search: bool,
    /// Minimum number of seeders required when syncing torrent indexers.
    pub minimum_seeders: i32,
}
