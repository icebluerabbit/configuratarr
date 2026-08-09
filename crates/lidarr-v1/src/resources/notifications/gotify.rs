use core_lib::SecretValue;
use core_macros::fields_blob;

/// Gotify notification provider configuration.
#[fields_blob(implementation = "Gotify", config_contract = "GotifySettings")]
pub struct GotifyConfig {
    /// Gotify server URL.
    pub server: String,
    /// Gotify application token.
    pub app_token: SecretValue,
    /// Notification priority level.
    pub priority: Option<i32>,
}
