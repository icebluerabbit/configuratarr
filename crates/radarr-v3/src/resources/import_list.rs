use core_macros::resource;

use crate::resources::import_lists::ImportListProvider;
use crate::resources::provider::Provider;

// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list = get("/api/v3/importlist"),
    create = post("/api/v3/importlist?forceSave=true"),
    update = put("/api/v3/importlist/${self.id}?forceSave=true"),
    delete = delete("/api/v3/importlist/${self.id}"),
)]
pub struct ImportList {
    /// Identity (id + name), tag refs, read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: ImportListProvider,
    /// Whether the import list is active and will be processed.
    pub enabled: bool,
    /// Whether movies from this list are automatically added to Radarr.
    pub enable_auto: bool,
    /// Monitoring strategy applied to added movies (e.g. `"movieOnly"`, `"movieAndCollection"`).
    pub monitor: Option<String>,
    /// Root folder where movies added by this list are placed.
    pub root_folder_path: Option<String>,
    /// Quality profile assigned to movies added by this list.
    #[reference(quality_profile)]
    pub quality_profile_id: i32,
    /// Whether to trigger an immediate search when a movie is added.
    pub search_on_add: bool,
    /// Minimum availability status before a movie is considered (e.g. `"announced"`, `"inCinemas"`, `"released"`).
    pub minimum_availability: Option<String>,
    /// API-reported list type category; read-only.
    #[wire(read_only)]
    pub list_type: Option<String>,
    /// Display sort order of this list in the UI.
    pub list_order: i32,
    /// Minimum interval between list refreshes; enforced by the API, read-only.
    #[wire(read_only)]
    pub min_refresh_interval: Option<String>,
}
