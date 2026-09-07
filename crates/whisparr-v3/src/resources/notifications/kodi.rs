use core_lib::SecretValue;
use core_macros::fields_blob;

/// Kodi (XBMC) notification provider configuration.
#[fields_blob(implementation = "Xbmc", config_contract = "XbmcSettings")]
pub struct KodiConfig {
    /// Kodi hostname or IP address.
    pub host: String,
    /// Kodi JSON-RPC HTTP port (default 8080).
    pub port: i32,
    /// Connect to Kodi over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// URL base path for the Kodi JSON-RPC endpoint (default `/jsonrpc`).
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Kodi authentication username.
    pub username: Option<String>,
    /// Kodi authentication password.
    pub password: Option<SecretValue>,
    /// Duration in seconds to display the on-screen notification (minimum 2).
    #[wire(name = "displayTime")]
    pub display_time: Option<i32>,
    /// Display an on-screen notification in Kodi on events.
    pub notify: Option<bool>,
    /// Trigger a Kodi video library update after a scene is imported.
    #[wire(name = "updateLibrary")]
    pub update_library: Option<bool>,
    /// Trigger a Kodi video library clean after a scene is deleted.
    #[wire(name = "cleanLibrary")]
    pub clean_library: Option<bool>,
    /// Always update the library on every event, not just import events.
    #[wire(name = "alwaysUpdate")]
    pub always_update: Option<bool>,
}
