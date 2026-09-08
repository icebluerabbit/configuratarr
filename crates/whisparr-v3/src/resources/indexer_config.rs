use core_macros::resource;

/// `/api/v3/config/indexer` — global indexer, RSS sync, and eros search-tuning
/// settings. eros indexers can search by studio code/title/date in addition to
/// the standard release title search, and the extra fields here tune that.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/indexer"),
    update = put("/api/v3/config/indexer/${self.id}"),
)]
pub struct IndexerConfig {
    #[id]
    pub id: Option<i32>,
    /// Minimum age in minutes a Usenet release must be before Whisparr will grab it.
    pub minimum_age: i32,
    /// Maximum release size in MB that Whisparr will grab; 0 = unlimited.
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
    /// Includes the studio's release code (e.g. scene id) in indexer searches.
    pub search_studio_code: bool,
    /// Restricts indexer searches to the release title only, skipping studio/date variants.
    pub search_title_only: bool,
    /// Includes the release date alongside the title in indexer searches.
    pub search_title_date: bool,
    /// Includes the release date alongside the studio name in indexer searches.
    pub search_studio_date: bool,
    /// Includes the studio name alongside the title in indexer searches.
    pub search_studio_title: bool,
    /// Date format used when building date-based search queries: `yymmdd`, `ddmmyyyy`, or `both`.
    pub search_date_format: String,
    /// Studio name format used when building studio-based search queries: `original`, `clean`, or `both`.
    pub search_studio_format: String,
}
