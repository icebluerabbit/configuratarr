use core_lib::SecretValue;
use core_macros::fields_blob;

/// Transmission download client settings.
#[fields_blob(
    implementation = "Transmission",
    config_contract = "TransmissionSettings",
    protocol = "torrent"
)]
pub struct TransmissionConfig {
    /// Hostname or IP address of the Transmission server.
    pub host: Option<String>,
    /// TCP port the Transmission RPC interface listens on.
    pub port: Option<i32>,
    /// Username for authenticating with Transmission.
    pub username: Option<String>,
    /// Password for authenticating with Transmission.
    pub password: Option<SecretValue>,
    /// Category (label) assigned to music downloads in Transmission.
    #[wire(name = "musicCategory")]
    pub music_category: Option<String>,
    /// Directory Transmission saves music downloads to.
    #[wire(name = "musicDirectory")]
    pub music_directory: Option<String>,
    /// Priority for releases added in the last 14 days.
    #[wire(name = "recentMusicPriority")]
    pub recent_music_priority: Option<i32>,
    /// Priority for releases older than 14 days.
    #[wire(name = "olderMusicPriority")]
    pub older_music_priority: Option<i32>,
    /// URL base path if Transmission is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add torrents to Transmission in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to Transmission over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
