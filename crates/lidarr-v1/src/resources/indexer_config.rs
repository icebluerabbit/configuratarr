use core_macros::resource;

/// `/api/v1/config/indexer` — global indexer and RSS sync settings.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/indexer"),
    update = put("/api/v1/config/indexer/${self.id}"),
)]
pub struct IndexerConfig {
    #[id]
    pub id: Option<i32>,
    /// Minimum age in minutes a Usenet release must be before Lidarr will grab it.
    pub minimum_age: i32,
    /// Maximum release size in MB that Lidarr will grab; 0 = unlimited.
    pub maximum_size: i32,
    /// Usenet retention period in days; 0 = unlimited.
    pub retention: i32,
    /// Interval in minutes between RSS feed syncs; 0 = disable RSS sync.
    #[default(60)]
    pub rss_sync_interval: i32,
}
