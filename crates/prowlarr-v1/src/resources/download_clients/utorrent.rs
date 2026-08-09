use core_lib::SecretValue;
use core_macros::fields_blob;

/// µTorrent (uTorrent) download client settings.
#[fields_blob(
    implementation = "UTorrent",
    config_contract = "UTorrentSettings",
    protocol = "torrent"
)]
pub struct UTorrentConfig {
    /// Hostname or IP address of the µTorrent server.
    pub host: Option<String>,
    /// TCP port the µTorrent web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with µTorrent.
    pub username: Option<String>,
    /// Password for authenticating with µTorrent.
    pub password: Option<SecretValue>,
    /// URL base path if µTorrent is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in µTorrent.
    pub category: Option<String>,
    /// Item priority when adding to µTorrent. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Initial state on add. `0` = Start, `1` = ForceStart, `2` = Pause, `3` = Stop.
    ///
    /// Note: the Prowlarr API wire key is `intialState` (intentional upstream typo).
    #[wire(name = "intialState")]
    pub initial_state: Option<i32>,
    /// Connect to µTorrent over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
