use core_lib::SecretValue;
use core_macros::fields_blob;

/// Email notification provider configuration.
#[fields_blob(implementation = "Email", config_contract = "EmailSettings")]
pub struct EmailConfig {
    /// SMTP server hostname or IP address.
    pub server: String,
    /// SMTP server port number (default 587).
    pub port: i32,
    /// Encryption mode: 0 = preferred, 1 = always, 2 = never.
    #[wire(name = "useEncryption")]
    pub use_encryption: i32,
    /// SMTP authentication username.
    pub username: Option<String>,
    /// SMTP authentication password.
    pub password: Option<SecretValue>,
    /// Sender email address shown in the From header.
    pub from: String,
    /// Primary recipient email addresses.
    pub to: Vec<String>,
    /// Carbon-copy recipient email addresses.
    pub cc: Vec<String>,
    /// Blind carbon-copy recipient email addresses.
    pub bcc: Vec<String>,
}
