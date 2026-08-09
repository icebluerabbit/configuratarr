use core_lib::SecretValue;
use core_macros::fields_blob;

/// Deluge download client settings.
#[fields_blob(
    implementation = "Deluge",
    config_contract = "DelugeSettings",
    protocol = "torrent"
)]
pub struct DelugeConfig {
    /// Hostname or IP address of the Deluge daemon.
    pub host: Option<String>,
    /// TCP port the Deluge daemon listens on.
    pub port: Option<i32>,
    /// Password for authenticating with the Deluge daemon.
    pub password: Option<SecretValue>,
    /// URL base path if Deluge is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Label assigned to movie downloads in Deluge.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Label the client moves completed downloads to after Radarr imports them.
    #[wire(name = "movieImportedCategory")]
    pub movie_imported_category: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
    /// Add torrents to Deluge in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Deluge over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
