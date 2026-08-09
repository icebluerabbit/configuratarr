use core_lib::SecretValue;
use core_macros::fields_blob;

/// Deluge torrent client settings.
#[fields_blob(
    implementation = "Deluge",
    config_contract = "DelugeSettings",
    protocol = "torrent"
)]
pub struct DelugeConfig {
    /// Hostname or IP address of the Deluge daemon.
    pub host: Option<String>,
    /// TCP port the Deluge web UI listens on.
    pub port: Option<i32>,
    /// Password for authenticating with the Deluge web UI.
    pub password: Option<SecretValue>,
    /// Category (label) assigned to TV series downloads in Deluge.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Category Deluge moves completed downloads to after Sonarr imports them.
    #[wire(name = "tvImportedCategory")]
    pub tv_imported_category: Option<String>,
    /// Priority for episodes aired in the last 14 days.
    #[wire(name = "recentTvPriority")]
    pub recent_tv_priority: Option<i32>,
    /// Priority for episodes aired more than 14 days ago.
    #[wire(name = "olderTvPriority")]
    pub older_tv_priority: Option<i32>,
    /// URL base path if Deluge is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add torrents to Deluge in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Deluge over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
