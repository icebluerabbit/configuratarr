use core_lib::SecretValue;
use core_macros::fields_blob;

/// Kodi (XBMC) notification provider configuration.
#[fields_blob(implementation = "Xbmc", config_contract = "XbmcSettings")]
pub struct KodiConfig {
    /// Kodi hostname or IP address.
    pub host: String,
    /// Kodi JSON-RPC HTTP port.
    pub port: i32,
    /// Kodi authentication username.
    pub username: Option<String>,
    /// Kodi authentication password.
    pub password: Option<SecretValue>,
    /// Connect to Kodi over HTTPS.
    pub use_ssl: Option<bool>,
    /// Display an on-screen notification in Kodi on events.
    pub notify: Option<bool>,
    /// Duration in milliseconds to display the on-screen notification.
    pub display_time: Option<i32>,
    /// Trigger a Kodi video library update after an episode is imported.
    pub update_library: Option<bool>,
    /// Trigger a Kodi video library clean after an episode file is deleted.
    pub clean_library: Option<bool>,
    /// Always update the library on every event, not just import events.
    pub always_update: Option<bool>,
}
