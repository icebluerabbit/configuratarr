use core_lib::SecretValue;
use core_macros::fields_blob;

/// Telegram notification provider configuration.
#[fields_blob(implementation = "Telegram", config_contract = "TelegramSettings")]
pub struct TelegramConfig {
    /// Telegram bot token issued by BotFather.
    #[wire(name = "botToken")]
    pub bot_token: SecretValue,
    /// Target chat, group, or channel ID to send messages to.
    #[wire(name = "chatId")]
    pub chat_id: String,
    /// Topic (message thread) ID for supergroup forums. Must be greater than 1.
    #[wire(name = "topicId")]
    pub topic_id: Option<i32>,
    /// Send the notification silently (no sound or alert on the recipient's device).
    #[wire(name = "sendSilently")]
    pub send_silently: Option<bool>,
    /// Prefix the message title with the application name.
    #[wire(name = "includeAppNameInTitle")]
    pub include_app_name_in_title: Option<bool>,
    /// Prefix the message title with this instance's name.
    #[wire(name = "includeInstanceNameInTitle")]
    pub include_instance_name_in_title: Option<bool>,
    /// Metadata link types to append to the message body.
    #[wire(name = "metadataLinks")]
    pub metadata_links: Vec<i32>,
}
