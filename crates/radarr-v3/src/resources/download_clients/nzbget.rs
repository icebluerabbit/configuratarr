use core_lib::SecretValue;
use core_macros::fields_blob;

/// NZBGet download client settings.
#[fields_blob(
    implementation = "Nzbget",
    config_contract = "NzbgetSettings",
    protocol = "usenet"
)]
pub struct NzbgetConfig {
    /// Hostname or IP address of the NZBGet server.
    pub host: Option<String>,
    /// TCP port the NZBGet web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with NZBGet.
    pub username: Option<String>,
    /// Password for authenticating with NZBGet.
    pub password: Option<SecretValue>,
    /// Category assigned to movie downloads in NZBGet.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
    /// URL base path if NZBGet is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add downloads to NZBGet in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to NZBGet over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
