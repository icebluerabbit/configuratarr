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
    /// Category (label) assigned to music downloads in Deluge.
    #[wire(name = "musicCategory")]
    pub music_category: Option<String>,
    /// Category Deluge moves completed downloads to after Lidarr imports them.
    #[wire(name = "musicImportedCategory")]
    pub music_imported_category: Option<String>,
    /// Priority for releases added in the last 14 days.
    #[wire(name = "recentMusicPriority")]
    pub recent_music_priority: Option<i32>,
    /// Priority for releases older than 14 days.
    #[wire(name = "olderMusicPriority")]
    pub older_music_priority: Option<i32>,
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
