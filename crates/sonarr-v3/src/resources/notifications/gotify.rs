use core_lib::SecretValue;
use core_macros::fields_blob;

/// Gotify notification provider configuration.
#[fields_blob(implementation = "Gotify", config_contract = "GotifySettings")]
pub struct GotifyConfig {
    /// Gotify server URL (e.g. `http://gotify.example.com`).
    pub server: String,
    /// Gotify application token used to publish messages.
    pub app_token: SecretValue,
    /// Message priority level sent with each notification.
    pub priority: Option<i32>,
}
