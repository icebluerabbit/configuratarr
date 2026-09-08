use core_macros::fields_blob;

/// Discord notification provider configuration.
#[fields_blob(implementation = "Discord", config_contract = "DiscordSettings")]
pub struct DiscordConfig {
    /// Discord incoming webhook URL.
    #[wire(name = "webHookUrl")]
    pub web_hook_url: String,
    /// Display name override for the webhook bot.
    pub username: Option<String>,
    /// Avatar image URL for the webhook bot.
    pub avatar: Option<String>,
    /// Author name shown in the Discord embed header.
    pub author: Option<String>,
    /// Field indices included in grab-event notification embeds.
    #[wire(name = "grabFields")]
    pub grab_fields: Vec<i32>,
    /// Field indices included in import-event notification embeds.
    #[wire(name = "importFields")]
    pub import_fields: Vec<i32>,
    /// Field indices included in manual-interaction-required notification embeds.
    #[wire(name = "manualInteractionFields")]
    pub manual_interaction_fields: Vec<i32>,
}
