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
    /// URL base path if Transmission is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in Transmission.
    pub category: Option<String>,
    /// Directory Transmission saves downloads to.
    pub directory: Option<String>,
    /// Item priority when adding to Transmission. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Add torrents to Transmission in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Transmission over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
