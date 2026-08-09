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
    /// Category assigned to TV series downloads in NZBVortex.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Priority for episodes aired in the last 14 days.
    #[wire(name = "recentTvPriority")]
    pub recent_tv_priority: Option<i32>,
    /// Priority for episodes aired more than 14 days ago.
    #[wire(name = "olderTvPriority")]
    pub older_tv_priority: Option<i32>,
    /// URL base path if NZBVortex is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
}
