use core_lib::SecretValue;
use core_macros::fields_blob;

/// Vuze torrent client settings.
///
/// Vuze uses the same config contract as Transmission ("TransmissionSettings").
#[fields_blob(
    implementation = "Vuze",
    config_contract = "TransmissionSettings",
    protocol = "torrent"
)]
pub struct VuzeConfig {
    /// Hostname or IP address of the Vuze remote UI server.
    pub host: Option<String>,
    /// TCP port the Vuze remote interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Vuze.
    pub username: Option<String>,
    /// Password for authenticating with Vuze.
    pub password: Option<SecretValue>,
    /// Category (label) assigned to movie downloads in Vuze.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Directory Vuze saves movie downloads to.
    #[wire(name = "movieDirectory")]
    pub movie_directory: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
    /// URL base path if Vuze is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add torrents to Vuze in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Vuze over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
