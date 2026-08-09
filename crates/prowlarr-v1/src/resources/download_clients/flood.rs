use core_lib::SecretValue;
use core_macros::fields_blob;

/// Flood (rTorrent web UI) download client settings.
#[fields_blob(
    implementation = "Flood",
    config_contract = "FloodSettings",
    protocol = "torrent"
)]
pub struct FloodConfig {
    /// Hostname or IP address of the Flood server.
    pub host: Option<String>,
    /// TCP port the Flood web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Flood.
    pub username: Option<String>,
    /// Password for authenticating with Flood.
    pub password: Option<SecretValue>,
    /// Directory Flood saves downloaded files to.
    pub destination: Option<String>,
    /// URL base path if Flood is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add torrents to Flood in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Flood over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// Tags applied to the torrent in Flood (string labels, not Prowlarr tag ids).
    #[wire(name = "fieldTags")]
    pub field_tags: Vec<String>,
    /// Additional Prowlarr-managed metadata tags appended to the torrent (integer codes).
    #[wire(name = "additionalTags")]
    pub additional_tags: Vec<i32>,
}
