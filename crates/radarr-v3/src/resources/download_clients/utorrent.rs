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
    /// Category assigned to movie downloads in uTorrent.
    #[wire(name = "movieCategory")]
    pub movie_category: Option<String>,
    /// Category the client moves completed downloads to after Radarr imports them.
    #[wire(name = "movieImportedCategory")]
    pub movie_imported_category: Option<String>,
    /// Priority for movies released in the last 14 days.
    #[wire(name = "recentMoviePriority")]
    pub recent_movie_priority: Option<i32>,
    /// Priority for movies released more than 14 days ago.
    #[wire(name = "olderMoviePriority")]
    pub older_movie_priority: Option<i32>,
    /// Initial state. 0 = Start, 1 = ForceStart, 2 = Pause, 3 = Stop
    /// Note: the Terraform provider uses the intentional typo "intialState" (missing 'i')
    /// as the API field name for uTorrent.
    #[wire(name = "intialState")]
    pub initial_state: Option<i32>,
    /// URL base path if uTorrent is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Connect to uTorrent over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
