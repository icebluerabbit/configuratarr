use core_lib::SecretValue;
use core_macros::fields_blob;

/// Ntfy notification provider configuration.
#[fields_blob(implementation = "Ntfy", config_contract = "NtfySettings")]
pub struct NtfyConfig {
    /// Base URL of the ntfy server (e.g. `https://ntfy.sh`).
    pub server_url: Option<String>,
    /// ntfy topic names to publish notifications to.
    pub topics: Vec<String>,
    /// Message priority level (1 = min … 5 = max).
    pub priority: Option<i32>,
    /// HTTP basic-auth username for the ntfy server.
    pub username: Option<String>,
    /// HTTP basic-auth password for the ntfy server.
    pub password: Option<SecretValue>,
    /// Bearer access token for ntfy authentication (alternative to username/password).
    pub access_token: Option<SecretValue>,
    /// URL opened when the notification is tapped by the user.
    pub click_url: Option<String>,
    /// ntfy message tags applied to the notification (emoji shortcodes accepted).
    pub field_tags: Vec<String>,
}
