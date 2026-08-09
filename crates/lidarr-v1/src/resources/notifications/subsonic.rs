use core_lib::SecretValue;
use core_macros::fields_blob;

/// Subsonic/Airsonic notification provider configuration.
///
/// This provider is Lidarr-specific — it updates and notifies a Subsonic-compatible
/// music streaming server after tracks are imported or retagged.
#[fields_blob(implementation = "Subsonic", config_contract = "SubsonicSettings")]
pub struct SubsonicConfig {
    /// Subsonic server hostname or IP address.
    pub host: String,
    /// Subsonic server port.
    pub port: Option<i32>,
    /// Subsonic authentication username.
    pub username: String,
    /// Subsonic authentication password.
    pub password: SecretValue,
    /// URL base path for the Subsonic server (e.g. `/subsonic`).
    pub url_base: Option<String>,
    /// Connect to the Subsonic server over HTTPS.
    pub use_ssl: Option<bool>,
    /// Send an on-screen notification to the Subsonic server on events.
    pub notify: Option<bool>,
    /// Trigger a Subsonic library update after a track is imported.
    pub update_library: Option<bool>,
}
