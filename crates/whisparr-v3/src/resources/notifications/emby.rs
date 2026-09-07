use core_lib::SecretValue;
use core_macros::fields_blob;

/// Emby / Jellyfin (MediaBrowser) notification provider configuration.
#[fields_blob(
    implementation = "MediaBrowser",
    config_contract = "MediaBrowserSettings"
)]
pub struct EmbyConfig {
    /// Emby/Jellyfin server hostname or IP address.
    pub host: String,
    /// Emby/Jellyfin server HTTP port (default 8096).
    pub port: i32,
    /// Connect to Emby/Jellyfin over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// URL base path when Emby/Jellyfin is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Emby/Jellyfin API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Send an on-screen notification to Emby/Jellyfin users on events.
    pub notify: Option<bool>,
    /// Trigger an Emby/Jellyfin library refresh after a scene is imported.
    #[wire(name = "updateLibrary")]
    pub update_library: Option<bool>,
    /// Whisparr-side path prefix to rewrite when Emby/Jellyfin mounts the
    /// library at a different location.
    #[wire(name = "mapFrom")]
    pub map_from: Option<String>,
    /// Emby/Jellyfin-side path prefix that `map_from` is rewritten to.
    #[wire(name = "mapTo")]
    pub map_to: Option<String>,
}
