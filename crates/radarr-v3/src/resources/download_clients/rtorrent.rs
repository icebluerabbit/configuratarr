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
    /// Label assigned to movie torrents in rTorrent.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Directory rTorrent saves movie downloads to.
    #[wire(name = "movieDirectory")]
    pub movie_directory: Option<String>,
    /// Label the client moves completed downloads to after Radarr imports them.
    #[wire(name = "movieImportedCategory")]
    pub movie_imported_category: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
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
