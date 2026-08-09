use core_macros::fields_blob;

/// Slack notification provider configuration.
#[fields_blob(implementation = "Slack", config_contract = "SlackSettings")]
pub struct SlackConfig {
    /// Slack incoming webhook URL.
    pub web_hook_url: String,
    /// Display name for the webhook bot.
    pub username: String,
    /// Emoji name or image URL to use as the bot's icon (e.g. `:ghost:`).
    pub icon: Option<String>,
    /// Slack channel to post to, overriding the webhook's default channel.
    pub channel: Option<String>,
}
