use core_lib::SecretValue;
use core_macros::fields_blob;

/// Transmission download client settings.
#[fields_blob(
    implementation = "Transmission",
    config_contract = "TransmissionSettings",
    protocol = "torrent"
)]
pub struct TransmissionConfig {
    /// Hostname or IP address of the Transmission server.
    pub host: Option<String>,
    /// TCP port the Transmission RPC interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Transmission.
    pub username: Option<String>,
    /// Password for authenticating with Transmission.
    pub password: Option<SecretValue>,
    /// Category (label) assigned to movie downloads in Transmission.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Directory Transmission saves movie downloads to.
    #[wire(name = "movieDirectory")]
    pub movie_directory: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
    /// URL base path if Transmission is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add torrents to Transmission in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Transmission over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
