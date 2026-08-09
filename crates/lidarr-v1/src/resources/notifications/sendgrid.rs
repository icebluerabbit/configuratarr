use core_lib::SecretValue;
use core_macros::fields_blob;

/// SendGrid email notification provider configuration.
#[fields_blob(implementation = "Sendgrid", config_contract = "SendgridSettings")]
pub struct SendgridConfig {
    /// SendGrid API key.
    pub api_key: SecretValue,
    /// Sender email address.
    pub from: String,
    /// Recipient email addresses.
    pub recipients: Vec<String>,
}
