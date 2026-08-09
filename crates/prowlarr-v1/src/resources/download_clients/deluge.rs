use core_lib::SecretValue;
use core_macros::fields_blob;

/// Deluge (via Web UI) download client settings.
#[fields_blob(
    implementation = "Deluge",
    config_contract = "DelugeSettings",
    protocol = "torrent"
)]
pub struct DelugeConfig {
    /// Hostname or IP address of the Deluge server.
    pub host: Option<String>,
    /// TCP port the Deluge web UI listens on.
    pub port: Option<i32>,
    /// Password for authenticating with Deluge.
    pub password: Option<SecretValue>,
    /// URL base path if Deluge is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in Deluge.
    pub category: Option<String>,
    /// Item priority when adding to Deluge. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Add torrents to Deluge in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Deluge over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
