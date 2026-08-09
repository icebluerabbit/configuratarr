use core_lib::SecretValue;
use core_macros::fields_blob;

/// Mailgun notification provider configuration.
#[fields_blob(implementation = "Mailgun", config_contract = "MailgunSettings")]
pub struct MailgunConfig {
    /// Mailgun API key.
    pub api_key: SecretValue,
    /// Mailgun sending domain.
    pub sender_domain: String,
    /// Sender email address.
    pub from: String,
    /// Recipient email addresses.
    pub recipients: Vec<String>,
    /// Use the Mailgun EU (European) API endpoint instead of the US endpoint.
    pub use_eu_endpoint: Option<bool>,
}
