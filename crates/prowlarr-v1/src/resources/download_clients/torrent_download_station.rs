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
    /// TCP port Download Station listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Download Station.
    pub username: Option<String>,
    /// Password for authenticating with Download Station.
    pub password: Option<SecretValue>,
    /// Category label assigned to downloads in Download Station.
    pub category: Option<String>,
    /// Destination directory for completed downloads on the NAS.
    ///
    /// Note: the Prowlarr API uses the wire key `tvDirectory` for this field
    /// for historical reasons.
    #[wire(name = "tvDirectory")]
    pub station_directory: Option<String>,
    /// Connect to Download Station over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
