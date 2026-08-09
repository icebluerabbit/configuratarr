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
    /// Category assigned to TV series downloads in SABnzbd.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Priority for episodes aired in the last 14 days.
    #[wire(name = "recentTvPriority")]
    pub recent_tv_priority: Option<i32>,
    /// Priority for episodes aired more than 14 days ago.
    #[wire(name = "olderTvPriority")]
    pub older_tv_priority: Option<i32>,
    /// URL base path if SABnzbd is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Connect to SABnzbd over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
