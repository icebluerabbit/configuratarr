use core_lib::SecretValue;
use core_macros::fields_blob;

/// rTorrent (via XML-RPC/SCGI) download client settings.
#[fields_blob(
    implementation = "RTorrent",
    config_contract = "RTorrentSettings",
    protocol = "torrent"
)]
pub struct RTorrentConfig {
    /// Hostname or IP address of the rTorrent SCGI/HTTP endpoint.
    pub host: Option<String>,
    /// TCP port the rTorrent SCGI or HTTP interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with rTorrent (used when fronted by a web server).
    pub username: Option<String>,
    /// Password for authenticating with rTorrent (used when fronted by a web server).
    pub password: Option<SecretValue>,
    /// Label assigned to music torrents in rTorrent.
    #[wire(name = "musicCategory")]
    pub music_category: Option<String>,
    /// Directory rTorrent saves music downloads to.
    #[wire(name = "musicDirectory")]
    pub music_directory: Option<String>,
    /// Label rTorrent moves completed downloads to after Lidarr imports them.
    #[wire(name = "musicImportedCategory")]
    pub music_imported_category: Option<String>,
    /// Priority for releases added in the last 14 days.
    #[wire(name = "recentMusicPriority")]
    pub recent_music_priority: Option<i32>,
    /// Priority for releases older than 14 days.
    #[wire(name = "olderMusicPriority")]
    pub older_music_priority: Option<i32>,
    /// URL base path if rTorrent is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add torrents to rTorrent in a stopped state rather than starting immediately.
    #[wire(name = "addStopped")]
    pub add_stopped: Option<bool>,
    /// Connect to rTorrent over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
