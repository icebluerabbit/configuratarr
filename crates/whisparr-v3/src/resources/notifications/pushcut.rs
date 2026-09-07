use core_lib::SecretValue;
use core_macros::fields_blob;

/// Pushcut notification provider configuration. Eros-only — this provider has
/// no radarr counterpart.
#[fields_blob(implementation = "Pushcut", config_contract = "PushcutSettings")]
pub struct PushcutConfig {
    /// Name of the Pushcut notification definition to trigger.
    #[wire(name = "notificationName")]
    pub notification_name: String,
    /// Pushcut API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Deliver as a time-sensitive notification (bypasses focus modes on iOS).
    #[wire(name = "timeSensitive")]
    pub time_sensitive: Option<bool>,
    /// Attach the scene poster image to the notification.
    #[wire(name = "includePoster")]
    pub include_poster: Option<bool>,
    /// Metadata link types to append to the message body.
    #[wire(name = "metadataLinks")]
    pub metadata_links: Vec<i32>,
}
