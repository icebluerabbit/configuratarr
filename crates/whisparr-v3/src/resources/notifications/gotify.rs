use core_lib::SecretValue;
use core_macros::fields_blob;

/// Gotify notification provider configuration.
#[fields_blob(implementation = "Gotify", config_contract = "GotifySettings")]
pub struct GotifyConfig {
    /// Gotify server URL (e.g. `http://gotify.example.com`).
    pub server: String,
    /// Gotify application token used to publish messages.
    #[wire(name = "appToken")]
    pub app_token: SecretValue,
    /// Message priority level sent with each notification (default 5).
    pub priority: Option<i32>,
    /// Attach the scene poster image to the notification.
    #[wire(name = "includeMoviePoster")]
    pub include_movie_poster: Option<bool>,
    /// Metadata link types to append to the message body.
    #[wire(name = "metadataLinks")]
    pub metadata_links: Vec<i32>,
    /// Metadata link type used for the message's primary link. Must be one of
    /// the types selected in `metadata_links` when that list is non-empty.
    #[wire(name = "preferredMetadataLink")]
    pub preferred_metadata_link: Option<i32>,
}
