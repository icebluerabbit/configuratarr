use core_lib::SecretValue;
use core_macros::fields_blob;

/// rTorrent (via XML-RPC/SCGI) download client settings.
#[fields_blob(
    implementation = "RTorrent",
    config_contract = "RTorrentSettings",
    protocol = "torrent"
)]
pub struct RTorrentConfig {
    /// Hostname or IP address of the rTorrent SCGI/HTTP endpoint.
    pub host: Option<String>,
    /// TCP port the rTorrent SCGI or HTTP interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with rTorrent (used when fronted by a web server).
    pub username: Option<String>,
    /// Password for authenticating with rTorrent (used when fronted by a web server).
    pub password: Option<SecretValue>,
    /// URL base path if rTorrent is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Label assigned to torrents in rTorrent.
    pub category: Option<String>,
    /// Directory rTorrent saves downloads to.
    pub directory: Option<String>,
    /// Item priority when adding to rTorrent. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Add torrents to rTorrent in a stopped state rather than starting immediately.
    #[wire(name = "addStopped")]
    pub add_stopped: Option<bool>,
    /// Connect to rTorrent over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
