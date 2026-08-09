use core_lib::SecretValue;
use core_macros::fields_blob;

/// ntfy notification provider configuration.
#[fields_blob(implementation = "Ntfy", config_contract = "NtfySettings")]
pub struct NtfyConfig {
    /// ntfy server URL (default: https://ntfy.sh).
    pub server_url: String,
    /// ntfy authentication username.
    pub username: Option<String>,
    /// ntfy authentication password.
    pub password: Option<SecretValue>,
    /// ntfy access token (alternative to username/password).
    pub access_token: Option<SecretValue>,
    /// Notification priority level (1 = min … 5 = max).
    pub priority: Option<i32>,
    /// URL opened when the notification is clicked.
    pub click_url: Option<String>,
    /// ntfy topics to publish to.
    pub topics: Vec<String>,
    /// Tag emojis attached to the notification.
    pub field_tags: Vec<String>,
}
