use core_lib::SecretValue;
use core_macros::fields_blob;

/// uTorrent download client settings.
#[fields_blob(
    implementation = "UTorrent",
    config_contract = "UTorrentSettings",
    protocol = "torrent"
)]
pub struct UTorrentConfig {
    /// Hostname or IP address of the uTorrent web UI.
    pub host: Option<String>,
    /// TCP port the uTorrent web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with uTorrent.
    pub username: Option<String>,
    /// Password for authenticating with uTorrent.
    pub password: Option<SecretValue>,
    /// Category assigned to TV series downloads in uTorrent.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Category uTorrent moves completed downloads to after Sonarr imports them.
    #[wire(name = "tvImportedCategory")]
    pub tv_imported_category: Option<String>,
    /// Priority for episodes aired in the last 14 days.
    #[wire(name = "recentTvPriority")]
    pub recent_tv_priority: Option<i32>,
    /// Priority for episodes aired more than 14 days ago.
    #[wire(name = "olderTvPriority")]
    pub older_tv_priority: Option<i32>,
    /// Initial torrent state. 0 = Start, 1 = ForceStart, 2 = Pause, 3 = Stop.
    /// Note: the API field name is intentionally misspelled as "intialState".
    #[wire(name = "intialState")]
    pub initial_state: Option<i32>,
    /// URL base path if uTorrent is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Connect to uTorrent over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
