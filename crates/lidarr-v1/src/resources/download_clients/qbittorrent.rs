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
    /// Category assigned to music downloads in qBittorrent.
    #[wire(name = "musicCategory")]
    pub music_category: Option<String>,
    /// Category qBittorrent moves completed downloads to after Lidarr imports them.
    #[wire(name = "musicImportedCategory")]
    pub music_imported_category: Option<String>,
    /// Priority for releases added in the last 14 days.
    #[wire(name = "recentMusicPriority")]
    pub recent_music_priority: Option<i32>,
    /// Priority for releases older than 14 days.
    #[wire(name = "olderMusicPriority")]
    pub older_music_priority: Option<i32>,
    /// Initial torrent state. 0 = Start, 1 = ForceStart, 2 = Pause.
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
}
