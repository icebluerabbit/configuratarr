use core_macros::resource;

use crate::resources::import_lists::ImportListProvider;
use crate::resources::provider::Provider;

/// An import list syncs series from an external source into Sonarr's library.
///
/// Each import list pairs a shared envelope (identity, monitoring preferences,
/// root folder, quality profile, series type) with a typed per-implementation
/// settings blob (the `config` field).
// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list   = get("/api/v3/importlist"),
    create = post("/api/v3/importlist?forceSave=true"),
    update = put("/api/v3/importlist/${self.id}?forceSave=true"),
    delete = delete("/api/v3/importlist/${self.id}"),
)]
pub struct ImportList {
    /// Identity (id + name), tag refs, and read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// Typed per-implementation settings stored in the API's `fields[]` blob.
    #[flatten]
    pub config: ImportListProvider,
    /// Whether Sonarr automatically adds series found on this list.
    pub enable_automatic_add: bool,
    /// Whether Sonarr searches for missing episodes of added series.
    pub search_for_missing_episodes: bool,
    /// Episode monitoring strategy applied when series are added
    /// (e.g. `"all"`, `"future"`, `"missing"`, `"pilot"`, `"firstSeason"`).
    pub should_monitor: Option<String>,
    /// How newly released episodes are monitored after the series is added
    /// (`"all"` or `"none"`).
    pub monitor_new_items: Option<String>,
    /// Root folder where series added by this list are placed.
    pub root_folder_path: Option<String>,
    /// Quality profile assigned to series added by this list.
    #[reference(quality_profile)]
    pub quality_profile_id: i32,
    /// Series type for added series (`"standard"`, `"daily"`, or `"anime"`).
    pub series_type: Option<String>,
    /// Whether to organise episodes into per-season subfolders.
    pub season_folder: bool,
    /// API-reported list category (`"program"`, `"plex"`, `"trakt"`, …); read-only.
    #[wire(read_only)]
    pub list_type: Option<String>,
    /// Display sort order of this list in the UI.
    pub list_order: i32,
    /// Minimum interval between list refreshes, enforced by the API; read-only.
    #[wire(read_only)]
    pub min_refresh_interval: Option<String>,
}
