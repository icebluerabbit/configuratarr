use core_lib::SecretValue;
use core_macros::fields_blob;

/// Email (SMTP) notification provider configuration.
#[fields_blob(implementation = "Email", config_contract = "EmailSettings")]
pub struct EmailConfig {
    /// SMTP server hostname or IP address.
    pub server: String,
    /// SMTP server port number.
    pub port: Option<i32>,
    /// Encryption mode: 0 = none, 1 = SSL/TLS, 2 = STARTTLS.
    pub use_encryption: Option<i32>,
    /// Sender email address shown in the From header.
    pub from: String,
    /// SMTP authentication username.
    pub username: Option<String>,
    /// SMTP authentication password.
    pub password: Option<SecretValue>,
    /// Primary recipient email addresses.
    pub to: Vec<String>,
    /// Carbon-copy recipient email addresses.
    #[wire(name = "cC")]
    pub cc: Vec<String>,
    /// Blind carbon-copy recipient email addresses.
    pub bcc: Vec<String>,
}
