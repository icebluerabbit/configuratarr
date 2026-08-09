use core_macros::resource;

/// `/api/configuration/blacklist_sync` — periodically publishes a blacklist
/// file that seeds *arr download-client blocklists.
#[resource(
    sync = singleton,
    read = get("/api/configuration/blacklist_sync"),
    update = put("/api/configuration/blacklist_sync"),
)]
pub struct BlacklistSync {
    /// Enables the blacklist sync job.
    pub enabled: Option<bool>,
    /// http(s) URL or an existing local file path. Required when `enabled`.
    pub blacklist_path: Option<String>,
}
