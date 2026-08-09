use core_lib::SecretValue;
use core_macros::fields_blob;

/// Emby (MediaBrowser) notification provider configuration.
#[fields_blob(
    implementation = "MediaBrowser",
    config_contract = "MediaBrowserSettings"
)]
pub struct EmbyConfig {
    /// Emby server hostname or IP address.
    pub host: String,
    /// Emby server HTTP port.
    pub port: i32,
    /// Emby API key for authentication.
    pub api_key: SecretValue,
    /// Connect to Emby over HTTPS.
    pub use_ssl: Option<bool>,
    /// Send an on-screen notification to Emby users on events.
    pub notify: Option<bool>,
    /// Trigger an Emby library refresh after a movie is imported.
    pub update_library: Option<bool>,
}
