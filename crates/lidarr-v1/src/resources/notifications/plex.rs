use core_lib::SecretValue;
use core_macros::fields_blob;

/// Plex Media Server notification provider configuration.
#[fields_blob(implementation = "PlexServer", config_contract = "PlexServerSettings")]
pub struct PlexConfig {
    /// Plex server hostname or IP address.
    pub host: String,
    /// Plex server port.
    pub port: Option<i32>,
    /// Plex authentication token (X-Plex-Token).
    pub auth_token: SecretValue,
    /// Connect to the Plex server over HTTPS.
    pub use_ssl: Option<bool>,
    /// Trigger a Plex music library refresh after a track is imported.
    pub update_library: Option<bool>,
}
