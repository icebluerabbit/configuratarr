use core_lib::SecretValue;
use core_macros::fields_blob;

/// Signal notification provider configuration.
///
/// Routes notifications through a self-hosted signal-cli REST API bridge.
#[fields_blob(implementation = "Signal", config_contract = "SignalSettings")]
pub struct SignalConfig {
    /// Hostname or IP address of the signal-cli REST API host.
    pub host: String,
    /// HTTP port of the signal-cli REST API.
    pub port: Option<i32>,
    /// Phone number registered in signal-cli that sends messages.
    pub sender_number: SecretValue,
    /// Phone number or group ID that receives the notification messages.
    pub receiver_id: String,
    /// Connect to the signal-cli REST API over HTTPS.
    pub use_ssl: Option<bool>,
    /// HTTP basic-auth username for the signal-cli REST API.
    pub auth_username: Option<String>,
    /// HTTP basic-auth password for the signal-cli REST API.
    pub auth_password: Option<SecretValue>,
}
