use core_lib::SecretValue;
use core_macros::fields_blob;

/// NZBGet download client settings.
#[fields_blob(
    implementation = "Nzbget",
    config_contract = "NzbgetSettings",
    protocol = "usenet"
)]
pub struct NzbgetConfig {
    /// Hostname or IP address of the NZBGet server.
    pub host: Option<String>,
    /// TCP port the NZBGet web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with NZBGet.
    pub username: Option<String>,
    /// Password for authenticating with NZBGet.
    pub password: Option<SecretValue>,
    /// Category assigned to music downloads in NZBGet.
    #[wire(name = "musicCategory")]
    pub music_category: Option<String>,
    /// Priority for releases added in the last 14 days.
    #[wire(name = "recentMusicPriority")]
    pub recent_music_priority: Option<i32>,
    /// Priority for releases older than 14 days.
    #[wire(name = "olderMusicPriority")]
    pub older_music_priority: Option<i32>,
    /// URL base path if NZBGet is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Add downloads to NZBGet in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to NZBGet over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
