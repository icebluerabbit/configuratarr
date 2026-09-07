use core_lib::SecretValue;
use core_macros::fields_blob;

/// Signal notification provider configuration, via a signal-cli REST API
/// gateway. Eros-only — this provider has no radarr counterpart.
#[fields_blob(implementation = "Signal", config_contract = "SignalSettings")]
pub struct SignalConfig {
    /// Hostname or IP address of the signal-cli REST API gateway.
    pub host: String,
    /// TCP port the signal-cli REST API listens on.
    pub port: i32,
    /// Connect to the gateway over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// Registered Signal phone number messages are sent from.
    #[wire(name = "senderNumber")]
    pub sender_number: SecretValue,
    /// Recipient group id or phone number.
    #[wire(name = "receiverId")]
    pub receiver_id: String,
    /// HTTP basic-auth username for the gateway.
    #[wire(name = "authUsername")]
    pub auth_username: Option<String>,
    /// HTTP basic-auth password for the gateway.
    #[wire(name = "authPassword")]
    pub auth_password: Option<SecretValue>,
}
