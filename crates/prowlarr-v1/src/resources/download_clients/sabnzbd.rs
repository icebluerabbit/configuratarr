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
    /// SABnzbd API key used as an alternative to username/password auth.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Username for authenticating with SABnzbd.
    pub username: Option<String>,
    /// Password for authenticating with SABnzbd.
    pub password: Option<SecretValue>,
    /// URL base path if SABnzbd is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in SABnzbd.
    pub category: Option<String>,
    /// Item priority when adding to SABnzbd. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Connect to SABnzbd over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
