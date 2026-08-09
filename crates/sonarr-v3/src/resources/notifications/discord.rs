use core_macros::fields_blob;

/// Discord notification provider configuration.
#[fields_blob(implementation = "Discord", config_contract = "DiscordSettings")]
pub struct DiscordConfig {
    /// Discord incoming webhook URL.
    pub web_hook_url: String,
    /// Display name override for the webhook bot.
    pub username: Option<String>,
    /// Avatar image URL for the webhook bot.
    pub avatar: Option<String>,
    /// Author name shown in the Discord embed header.
    pub author: Option<String>,
    /// Field indices included in grab-event notification embeds.
    pub grab_fields: Vec<i32>,
    /// Field indices included in import-event notification embeds.
    pub import_fields: Vec<i32>,
}
