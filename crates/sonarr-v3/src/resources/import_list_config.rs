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
    /// Action taken when a series is removed from all import lists: `disabled`, `logOnly`, `keepAndUnmonitor`, or `keepAndTag`.
    pub list_sync_level: Option<String>,
    /// Tag applied to series when `list_sync_level` is `keepAndTag`; resolved from `${ref.tag.<label>}`.
    #[reference(tag)]
    pub list_sync_tag: Option<i32>,
}
