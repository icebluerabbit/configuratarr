use core_lib::SecretValue;
use core_macros::fields_blob;

/// PushBullet notification provider configuration.
#[fields_blob(implementation = "PushBullet", config_contract = "PushBulletSettings")]
pub struct PushbulletConfig {
    /// PushBullet access token used for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Target device identifiers to receive the push notification; leave empty
    /// to send to all devices.
    #[wire(name = "deviceIds")]
    pub device_ids: Vec<String>,
    /// PushBullet channel tags to publish the notification to.
    #[wire(name = "channelTags")]
    pub channel_tags: Vec<String>,
    /// Sender device identifier shown as the push source.
    #[wire(name = "senderId")]
    pub sender_id: Option<String>,
}
