use core_lib::SecretValue;
use core_macros::fields_blob;

/// Telegram notification provider configuration.
#[fields_blob(implementation = "Telegram", config_contract = "TelegramSettings")]
pub struct TelegramConfig {
    /// Telegram bot token issued by BotFather.
    pub bot_token: SecretValue,
    /// Target chat, group, or channel ID to send messages to.
    pub chat_id: String,
    /// Topic (message thread) ID for supergroup forums. Must be greater than 1.
    // Integer, not string: Radarr's `TelegramSettings.TopicId` is `int?`
    // (`src/NzbDrone.Core/Notifications/Telegram/TelegramSettings.cs`). A string
    // here never compares equal to the number the server returns, so the field
    // would report a diff and re-`PUT` on every apply.
    pub topic_id: Option<i32>,
    /// Send the notification silently (no sound or alert on the recipient's device).
    pub send_silently: Option<bool>,
}
