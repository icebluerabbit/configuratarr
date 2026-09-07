use core_lib::SecretValue;
use core_macros::fields_blob;

/// Ntfy notification provider configuration.
#[fields_blob(implementation = "Ntfy", config_contract = "NtfySettings")]
pub struct NtfyConfig {
    /// Base URL of the ntfy server (e.g. `https://ntfy.sh`).
    #[wire(name = "serverUrl")]
    pub server_url: String,
    /// Bearer access token for ntfy authentication (alternative to
    /// username/password).
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
    /// HTTP basic-auth username for the ntfy server. The eros C# property is
    /// `UserName`, so the wire key is `userName` (radarr's is `username`).
    #[wire(name = "userName")]
    pub user_name: Option<String>,
    /// HTTP basic-auth password for the ntfy server.
    pub password: Option<SecretValue>,
    /// Message priority level (1 = min … 5 = max, default 3).
    pub priority: Option<i32>,
    /// ntfy topic names to publish notifications to.
    pub topics: Vec<String>,
    /// ntfy message tags applied to the notification (emoji shortcodes accepted).
    // Renamed from `tags`: the shared `Provider` envelope already flattens a
    // `tags` field (tag-id references) into the same config namespace, so a
    // second `tags` here would be unreachable. Wire name is unchanged.
    #[wire(name = "tags")]
    pub message_tags: Vec<String>,
    /// URL opened when the notification is tapped by the user.
    #[wire(name = "clickUrl")]
    pub click_url: Option<String>,
}
