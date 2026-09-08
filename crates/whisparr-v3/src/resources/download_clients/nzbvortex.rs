use core_lib::SecretValue;
use core_macros::fields_blob;

/// NZBVortex download client settings.
///
/// eros's class/settings names are capital-V `NzbVortex`/`NzbVortexSettings`
/// (radarr's crate uses `Nzbvortex`/`NzbvortexSettings` — do not copy that
/// casing here). eros's settings also carry no `useSsl` field and use
/// `tvCategory`, not `movieCategory`.
#[fields_blob(
    implementation = "NzbVortex",
    config_contract = "NzbVortexSettings",
    protocol = "usenet"
)]
pub struct NzbVortexConfig {
    /// Hostname or IP address of the NZBVortex server.
    pub host: Option<String>,
    /// TCP port the NZBVortex server listens on.
    pub port: Option<i32>,
    /// URL base path if NZBVortex is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// API key used to authenticate with NZBVortex.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Group/category assigned to downloads in NZBVortex.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
}
