use core_lib::SecretValue;
use core_macros::fields_blob;

/// Freebox (FreeboxOS Download) torrent client settings.
#[fields_blob(
    implementation = "TorrentFreeboxDownload",
    config_contract = "FreeboxDownloadSettings",
    protocol = "torrent"
)]
pub struct FreeboxConfig {
    /// Hostname or IP address of the Freebox server.
    pub host: Option<String>,
    /// TCP port the Freebox API listens on.
    pub port: Option<i32>,
    /// Base URL of the Freebox HTTP API (e.g. `http://mafreebox.freebox.fr/`).
    #[wire(name = "apiUrl")]
    pub api_url: Option<String>,
    /// Application ID registered with the Freebox for OAuth-style access.
    #[wire(name = "appId")]
    pub app_id: Option<String>,
    /// Application token obtained during the Freebox authorisation flow.
    #[wire(name = "appToken")]
    pub app_token: Option<SecretValue>,
    /// Download category assigned to movie torrents in FreeboxOS.
    pub category: Option<String>,
    /// Directory the Freebox saves movie downloads to.
    #[wire(name = "destinationDirectory")]
    pub destination_directory: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentPriority")]
    pub recent_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderPriority")]
    pub older_priority: Option<i32>,
    /// Add torrents to FreeboxOS in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to the Freebox API over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
