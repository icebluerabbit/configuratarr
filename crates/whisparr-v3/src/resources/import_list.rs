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
    /// Whether items from this list are automatically added to Whisparr.
    pub enable_auto: bool,
    /// Monitoring strategy applied to added items.
    /// One of `movieOnly`, `movieAndScene`, `sceneOnly`, `none`.
    pub monitor: Option<String>,
    /// Root folder where items added by this list are placed.
    pub root_folder_path: Option<String>,
    /// Quality profile assigned to items added by this list.
    #[reference(quality_profile)]
    pub quality_profile_id: i32,
    /// Whether to trigger an immediate search when an item is added.
    pub search_on_add: bool,
    /// API-reported list type category; read-only.
    /// One of `program`, `stashDB`, `tmdb`, `other`, `advanced`.
    #[wire(read_only)]
    pub list_type: Option<String>,
    /// Display sort order of this list in the UI.
    pub list_order: i32,
    /// Minimum interval between list refreshes; enforced by the API, read-only.
    #[wire(read_only)]
    pub min_refresh_interval: Option<String>,
    /// Timestamp of the last successful sync of this list; server-set, read-only.
    #[wire(read_only)]
    pub last_info_sync: Option<String>,
}
