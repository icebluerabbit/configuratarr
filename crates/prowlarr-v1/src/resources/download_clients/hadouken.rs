use core_lib::SecretValue;
use core_macros::fields_blob;

/// Hadouken download client settings.
#[fields_blob(
    implementation = "Hadouken",
    config_contract = "HadoukenSettings",
    protocol = "torrent"
)]
pub struct HadoukenConfig {
    /// Hostname or IP address of the Hadouken server.
    pub host: Option<String>,
    /// TCP port the Hadouken web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Hadouken.
    pub username: Option<String>,
    /// Password for authenticating with Hadouken.
    pub password: Option<SecretValue>,
    /// URL base path if Hadouken is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in Hadouken.
    pub category: Option<String>,
    /// Connect to Hadouken over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
