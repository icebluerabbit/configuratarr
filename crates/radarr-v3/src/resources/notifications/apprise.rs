use core_lib::SecretValue;
use core_macros::fields_blob;

/// Apprise notification provider configuration.
#[fields_blob(implementation = "Apprise", config_contract = "AppriseSettings")]
pub struct AppriseConfig {
    /// Base URL of the Apprise API server.
    pub server_url: String,
    /// Comma-separated stateless Apprise notification URLs (e.g. `slack://…`).
    pub stateless_urls: Option<String>,
    /// Notification type/category identifier sent to Apprise.
    pub notification_type: Option<i32>,
    /// HTTP basic-auth username for the Apprise server.
    pub auth_username: Option<String>,
    /// HTTP basic-auth password for the Apprise server.
    #[wire(name = "authPassword")]
    pub auth_password: Option<SecretValue>,
    /// Apprise persistent-store configuration key.
    #[wire(name = "configurationKey")]
    pub configuration_key: Option<String>,
    /// Tag filters applied to the Apprise notification dispatch.
    pub field_tags: Vec<String>,
}
