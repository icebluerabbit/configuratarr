use core_lib::SecretValue;
use core_macros::fields_blob;

/// PushBullet notification provider configuration.
#[fields_blob(implementation = "PushBullet", config_contract = "PushBulletSettings")]
pub struct PushbulletConfig {
    /// PushBullet API key for authentication.
    pub api_key: SecretValue,
    /// Sender device identifier shown as the push source.
    pub sender_id: Option<String>,
    /// Target device identifiers to receive the push notification.
    pub device_ids: Vec<String>,
    /// PushBullet channel tags to publish the notification to.
    pub channel_tags: Vec<String>,
}
