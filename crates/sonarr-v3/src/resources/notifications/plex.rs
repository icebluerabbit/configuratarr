use core_lib::SecretValue;
use core_macros::fields_blob;

/// Plex Media Server notification provider configuration.
#[fields_blob(implementation = "PlexServer", config_contract = "PlexServerSettings")]
pub struct PlexConfig {
    /// Plex Media Server hostname or IP address.
    pub host: String,
    /// Plex Media Server HTTP port.
    pub port: i32,
    /// Plex authentication token (X-Plex-Token).
    pub auth_token: SecretValue,
    /// Connect to Plex over HTTPS.
    pub use_ssl: Option<bool>,
    /// Trigger a Plex library section refresh after an episode is imported.
    pub update_library: Option<bool>,
}
