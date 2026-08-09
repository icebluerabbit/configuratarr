use core_lib::SecretValue;
use core_macros::fields_blob;

/// Vuze download client settings.
///
/// Vuze shares the `TransmissionSettings` config contract with Transmission.
#[fields_blob(
    implementation = "Vuze",
    config_contract = "TransmissionSettings",
    protocol = "torrent"
)]
pub struct VuzeConfig {
    /// Hostname or IP address of the Vuze server.
    pub host: Option<String>,
    /// TCP port the Vuze RPC interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Vuze.
    pub username: Option<String>,
    /// Password for authenticating with Vuze.
    pub password: Option<SecretValue>,
    /// URL base path if Vuze is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in Vuze.
    pub category: Option<String>,
    /// Directory Vuze saves downloads to.
    pub directory: Option<String>,
    /// Item priority when adding to Vuze. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Add torrents to Vuze in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Vuze over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
