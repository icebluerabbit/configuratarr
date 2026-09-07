use core_lib::SecretValue;
use core_macros::fields_blob;

/// SABnzbd download client settings.
#[fields_blob(
    implementation = "Sabnzbd",
    config_contract = "SabnzbdSettings",
    protocol = "usenet"
)]
pub struct SabnzbdConfig {
    /// Hostname or IP address of the SABnzbd server.
    pub host: Option<String>,
    /// TCP port the SABnzbd web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with SABnzbd.
    pub username: Option<String>,
    /// Password for authenticating with SABnzbd.
    pub password: Option<SecretValue>,
    /// SABnzbd API key used as an alternative to username/password auth.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Category assigned to movie downloads in SABnzbd.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
    /// URL base path if SABnzbd is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Connect to SABnzbd over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
