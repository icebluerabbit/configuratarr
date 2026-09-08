use core_lib::SecretValue;
use core_macros::fields_blob;

/// Apprise notification provider configuration.
#[fields_blob(implementation = "Apprise", config_contract = "AppriseSettings")]
pub struct AppriseConfig {
    /// Base URL of the Apprise API server (e.g. `http://localhost:8000`).
    #[wire(name = "serverUrl")]
    pub server_url: String,
    /// Apprise persistent-store configuration key. Mutually exclusive with
    /// `stateless_urls`; allowed characters are `a-z`, `0-9` and `-`.
    #[wire(name = "configurationKey")]
    pub configuration_key: Option<String>,
    /// Comma-separated stateless Apprise notification URLs (e.g. `slack://…`).
    /// Mutually exclusive with `configuration_key`.
    #[wire(name = "statelessUrls")]
    pub stateless_urls: Option<String>,
    /// Notification type/category identifier sent to Apprise (0 = Info).
    #[wire(name = "notificationType")]
    pub notification_type: Option<i32>,
    /// Tag filters applied to the Apprise notification dispatch. Not supported
    /// when `stateless_urls` is used.
    // Renamed from `tags`: the shared `Provider` envelope already flattens a
    // `tags` field (tag-id references) into the same config namespace, so a
    // second `tags` here would be unreachable. Wire name is unchanged.
    #[wire(name = "tags")]
    pub message_tags: Vec<String>,
    /// Attach the scene poster image to the notification.
    #[wire(name = "includePoster")]
    pub include_poster: Option<bool>,
    /// HTTP basic-auth username for the Apprise server.
    #[wire(name = "authUsername")]
    pub auth_username: Option<String>,
    /// HTTP basic-auth password for the Apprise server.
    #[wire(name = "authPassword")]
    pub auth_password: Option<SecretValue>,
}
