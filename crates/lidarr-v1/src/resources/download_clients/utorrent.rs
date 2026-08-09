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
    /// Category assigned to music downloads in uTorrent.
    #[wire(name = "musicCategory")]
    pub music_category: Option<String>,
    /// Category uTorrent moves completed downloads to after Lidarr imports them.
    #[wire(name = "musicImportedCategory")]
    pub music_imported_category: Option<String>,
    /// Priority for releases added in the last 14 days.
    #[wire(name = "recentMusicPriority")]
    pub recent_music_priority: Option<i32>,
    /// Priority for releases older than 14 days.
    #[wire(name = "olderMusicPriority")]
    pub older_music_priority: Option<i32>,
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
