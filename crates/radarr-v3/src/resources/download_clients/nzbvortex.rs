use core_lib::SecretValue;
use core_macros::fields_blob;

/// NZBVortex download client settings.
#[fields_blob(
    implementation = "Nzbvortex",
    config_contract = "NzbvortexSettings",
    protocol = "usenet"
)]
pub struct NzbvortexConfig {
    /// Hostname or IP address of the NZBVortex server.
    pub host: Option<String>,
    /// TCP port the NZBVortex server listens on.
    pub port: Option<i32>,
    /// API key used to authenticate with NZBVortex.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// URL base path if NZBVortex is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
}
