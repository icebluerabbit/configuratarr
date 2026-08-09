use core_lib::SecretValue;
use core_macros::fields_blob;

/// qBittorrent download client settings.
#[fields_blob(
    implementation = "QBittorrent",
    config_contract = "QBittorrentSettings",
    protocol = "torrent"
)]
pub struct QBittorrentConfig {
    /// Hostname or IP address of the qBittorrent server.
    pub host: Option<String>,
    /// TCP port the qBittorrent web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with qBittorrent.
    pub username: Option<String>,
    /// Password for authenticating with qBittorrent.
    pub password: Option<SecretValue>,
    /// URL base path if qBittorrent is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in qBittorrent.
    pub category: Option<String>,
    /// Item priority when adding to qBittorrent. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Initial torrent state. `0` = Start, `1` = ForceStart, `2` = Pause.
    #[wire(name = "initialState")]
    pub initial_state: Option<i32>,
    /// Connect to qBittorrent over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
