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
    /// Category (label) assigned to TV series downloads in Transmission.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Directory Transmission saves TV series downloads to.
    #[wire(name = "tvDirectory")]
    pub tv_directory: Option<String>,
    /// Priority for episodes aired in the last 14 days.
    #[wire(name = "recentTvPriority")]
    pub recent_tv_priority: Option<i32>,
    /// Priority for episodes aired more than 14 days ago.
    #[wire(name = "olderTvPriority")]
    pub older_tv_priority: Option<i32>,
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
