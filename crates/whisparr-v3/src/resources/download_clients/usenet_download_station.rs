use core_lib::SecretValue;
use core_macros::fields_blob;

/// Synology Download Station (usenet) download client settings.
///
/// eros keeps the C# `DownloadStationSettings` class's Sonarr-derived `Tv*`
/// field naming (`tvCategory`/`tvDirectory`) rather than Radarr's
/// `movieCategory`/`movieDirectory` — do not rename these to `movie*`.
#[fields_blob(
    implementation = "UsenetDownloadStation",
    config_contract = "DownloadStationSettings",
    protocol = "usenet"
)]
pub struct UsenetDownloadStationConfig {
    /// Hostname or IP address of the Synology NAS running Download Station.
    pub host: Option<String>,
    /// TCP port the Synology DSM web interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Synology DSM.
    pub username: Option<String>,
    /// Password for authenticating with Synology DSM.
    pub password: Option<SecretValue>,
    /// Connect to Synology DSM over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// Category assigned to downloads in Download Station.
    #[wire(name = "tvCategory")]
    pub tv_category: Option<String>,
    /// Directory Download Station saves downloads to.
    #[wire(name = "tvDirectory")]
    pub tv_directory: Option<String>,
}
