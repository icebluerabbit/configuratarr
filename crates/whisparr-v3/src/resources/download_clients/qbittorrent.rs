use core_lib::SecretValue;
use core_macros::fields_blob;

/// qBittorrent download client settings.
#[fields_blob(
    implementation = "QBittorrent",
    config_contract = "QBittorrentSettings",
    protocol = "torrent"
)]
pub struct QBittorrentConfig {
    /// Hostname or IP address of the qBittorrent server.
    pub host: Option<String>,
    /// TCP port the qBittorrent web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with qBittorrent.
    pub username: Option<String>,
    /// Password for authenticating with qBittorrent.
    pub password: Option<SecretValue>,
    /// Category assigned to movie downloads in qBittorrent.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Category the client moves completed downloads to after Whisparr imports them.
    #[wire(name = "movieImportedCategory")]
    pub movie_imported_category: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
    /// 0 = Start, 1 = ForceStart, 2 = Pause
    #[wire(name = "initialState")]
    pub initial_state: Option<i32>,
    /// URL base path if qBittorrent is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Connect to qBittorrent over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// Download pieces in sequential order to enable early playback.
    #[wire(name = "sequentialOrder")]
    pub sequential_order: Option<bool>,
    /// Prioritise downloading the first and last pieces of each file first.
    #[wire(name = "firstAndLast")]
    pub first_and_last: Option<bool>,
    /// How the downloaded content is laid out on disk. 0 = Default, 1 = Original, 2 = Subfolder.
    #[wire(name = "contentLayout")]
    pub content_layout: Option<i32>,
}
