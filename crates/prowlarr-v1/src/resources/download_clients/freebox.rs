use core_lib::SecretValue;
use core_macros::fields_blob;

/// Freebox (FreeboxOS integrated downloader) download client settings.
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
    /// URL of the Freebox API endpoint.
    #[wire(name = "apiUrl")]
    pub api_url: Option<String>,
    /// Application identifier registered with the Freebox OS.
    #[wire(name = "appId")]
    pub app_id: Option<String>,
    /// Application token for authenticating with the Freebox OS.
    #[wire(name = "appToken")]
    pub app_token: Option<SecretValue>,
    /// Category label assigned to downloads on the Freebox.
    pub category: Option<String>,
    /// Destination directory for completed downloads on the Freebox.
    #[wire(name = "destinationDirectory")]
    pub destination_directory: Option<String>,
    /// Item priority when adding to the Freebox. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Add torrents to the Freebox in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to the Freebox over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
