use core_macros::resource;

use crate::resources::import_lists::ImportListProvider;
use crate::resources::provider::Provider;

/// An import list syncs artists and albums from an external source into Lidarr's library.
///
/// Each import list pairs a shared envelope (identity, monitoring preferences,
/// root folder, quality profile, metadata profile) with a typed per-implementation
/// settings blob (the `config` field).
// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list   = get("/api/v1/importlist"),
    create = post("/api/v1/importlist?forceSave=true"),
    update = put("/api/v1/importlist/${self.id}?forceSave=true"),
    delete = delete("/api/v1/importlist/${self.id}"),
)]
pub struct ImportList {
    /// Identity (id + name), tag refs, and read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// Typed per-implementation settings stored in the API's `fields[]` blob.
    #[flatten]
    pub config: ImportListProvider,
    /// Whether Lidarr automatically adds artists found on this list.
    pub enable_automatic_add: bool,
    /// Whether to monitor items already present in Lidarr's library when the
    /// list is refreshed.
    pub should_monitor_existing: bool,
    /// Whether to search for missing tracks of artists added by this list.
    pub should_search: bool,
    /// What to monitor when an artist is added from this list
    /// (`"none"`, `"specificAlbum"`, or `"entireArtist"`).
    pub should_monitor: Option<String>,
    /// How newly released albums are monitored after the artist is added
    /// (`"all"`, `"none"`, or `"new"`).
    pub monitor_new_items: Option<String>,
    /// Root folder where artists added by this list are placed.
    pub root_folder_path: Option<String>,
    /// Quality profile assigned to artists added by this list.
    #[reference(quality_profile)]
    pub quality_profile_id: i32,
    /// Metadata profile assigned to artists added by this list.
    #[reference(metadata_profile)]
    pub metadata_profile_id: i32,
    /// API-reported list category (`"program"`, `"spotify"`, `"lastFm"`, …); read-only.
    #[wire(read_only)]
    pub list_type: Option<String>,
    /// Display sort order of this list in the UI.
    pub list_order: i32,
    /// Minimum interval between list refreshes, enforced by the API; read-only.
    #[wire(read_only)]
    pub min_refresh_interval: Option<String>,
}
