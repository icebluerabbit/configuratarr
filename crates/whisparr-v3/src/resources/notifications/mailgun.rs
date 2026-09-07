use core_lib::SecretValue;
use core_macros::fields_blob;

/// Mailgun notification provider configuration.
///
/// The eros provider class is `MailGun` (capital G) while its settings class
/// stayed `MailgunSettings` — hence the mismatched discriminator pair below.
#[fields_blob(implementation = "MailGun", config_contract = "MailgunSettings")]
pub struct MailgunConfig {
    /// Mailgun API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Use the EU Mailgun API endpoint instead of the US endpoint.
    #[wire(name = "useEuEndpoint")]
    pub use_eu_endpoint: Option<bool>,
    /// Sender email address shown in the From header.
    pub from: String,
    /// Mailgun sending domain registered in your account.
    #[wire(name = "senderDomain")]
    pub sender_domain: String,
    /// Recipient email addresses.
    pub recipients: Vec<String>,
}
