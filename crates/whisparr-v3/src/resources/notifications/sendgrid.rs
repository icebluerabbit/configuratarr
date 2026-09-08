use core_lib::SecretValue;
use core_macros::fields_blob;

/// SendGrid notification provider configuration.
///
/// Eros names both the provider class and its settings contract with a capital
/// G (`SendGrid`/`SendGridSettings`); radarr's are `Sendgrid`/`SendgridSettings`.
#[fields_blob(implementation = "SendGrid", config_contract = "SendGridSettings")]
pub struct SendgridConfig {
    /// SendGrid API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Sender email address shown in the From header.
    pub from: String,
    /// Recipient email addresses.
    pub recipients: Vec<String>,
}
