use core_macros::resource;

/// `/api/v3/config/importlist` — import list sync level configuration.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/importlist"),
    update = put("/api/v3/config/importlist/${self.id}"),
)]
pub struct ImportListConfig {
    #[id]
    pub id: Option<i32>,
    /// Action taken when a movie is removed from all import lists: e.g. `disabled`, `logOnly`, `removeAndKeep`, or `removeAndDelete`.
    pub list_sync_level: Option<String>,
}
