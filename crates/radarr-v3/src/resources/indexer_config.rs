use core_macros::resource;

/// `/api/v3/config/indexer` — global indexer and RSS sync settings.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/indexer"),
    update = put("/api/v3/config/indexer/${self.id}"),
)]
pub struct IndexerConfig {
    #[id]
    pub id: Option<i32>,
    /// Minimum age in minutes a Usenet release must be before Radarr will grab it.
    pub minimum_age: i32,
    /// Maximum release size in MB that Radarr will grab; 0 = unlimited.
    pub maximum_size: i32,
    /// Usenet retention period in days; 0 = unlimited.
    pub retention: i32,
    /// Interval in minutes between RSS feed syncs; 0 = disable RSS sync.
    #[default(60)]
    pub rss_sync_interval: i32,
    /// Prefers releases flagged by indexers (e.g. freeleech on torrents) when scoring candidates.
    pub prefer_indexer_flags: bool,
    /// Number of days before (`-`) or after (`+`) a movie's availability date to start searching.
    pub availability_delay: i32,
    /// Allows grabbing releases that contain hardcoded (burned-in) subtitles.
    pub allow_hardcoded_subs: bool,
    /// Comma-separated list of subtitle language codes whose hardcoded releases are permitted.
    pub whitelisted_hardcoded_subs: Option<String>,
}
