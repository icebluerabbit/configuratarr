use core_lib::SecretValue;
use core_macros::fields_blob;

/// Pushbullet notification provider configuration.
#[fields_blob(implementation = "PushBullet", config_contract = "PushBulletSettings")]
pub struct PushbulletConfig {
    /// Pushbullet API key.
    pub api_key: SecretValue,
    /// Optional Pushbullet sender identity (device or channel).
    pub sender_id: Option<String>,
    /// Target device IDs to send the notification to.
    pub device_ids: Vec<String>,
    /// Pushbullet channel tags to publish the notification to.
    pub channel_tags: Vec<String>,
}
