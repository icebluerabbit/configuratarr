use core_lib::SecretValue;
use core_macros::fields_blob;

/// NZBVortex download client settings.
#[fields_blob(
    implementation = "Nzbvortex",
    config_contract = "NzbvortexSettings",
    protocol = "usenet"
)]
pub struct NzbvortexConfig {
    /// Hostname or IP address of the NZBVortex server.
    pub host: Option<String>,
    /// TCP port the NZBVortex server listens on.
    pub port: Option<i32>,
    /// API key used to authenticate with NZBVortex.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Category assigned to music downloads in NZBVortex.
    #[wire(name = "musicCategory")]
    pub music_category: Option<String>,
    /// Priority for releases added in the last 14 days.
    #[wire(name = "recentMusicPriority")]
    pub recent_music_priority: Option<i32>,
    /// Priority for releases older than 14 days.
    #[wire(name = "olderMusicPriority")]
    pub older_music_priority: Option<i32>,
    /// URL base path if NZBVortex is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
}
