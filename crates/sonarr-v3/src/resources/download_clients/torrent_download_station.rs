use core_lib::SecretValue;
use core_macros::fields_blob;

/// Synology Download Station (torrent) download client settings.
#[fields_blob(
    implementation = "TorrentDownloadStation",
    config_contract = "DownloadStationSettings",
    protocol = "torrent"
)]
pub struct TorrentDownloadStationConfig {
    /// Hostname or IP address of the Synology NAS running Download Station.
    pub host: Option<String>,
    /// TCP port the Synology DSM web interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Synology DSM.
    pub username: Option<String>,
    /// Password for authenticating with Synology DSM.
    pub password: Option<SecretValue>,
    /// Shared folder or category assigned to TV series downloads.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Directory Download Station saves TV series downloads to.
    #[wire(name = "tvDirectory")]
    pub tv_directory: Option<String>,
    /// Connect to Synology DSM over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
