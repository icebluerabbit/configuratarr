use core_lib::SecretValue;
use core_macros::fields_blob;

/// Webhook notification provider configuration.
#[fields_blob(implementation = "Webhook", config_contract = "WebhookSettings")]
pub struct WebhookConfig {
    /// Webhook endpoint URL that receives the HTTP request.
    pub url: String,
    /// HTTP method to use: 1 = POST, 2 = PUT.
    pub method: i32,
    /// HTTP basic-auth username sent with the request.
    pub username: Option<String>,
    /// HTTP basic-auth password sent with the request.
    pub password: Option<SecretValue>,
}
