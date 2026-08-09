use core_lib::SecretValue;
use core_macros::fields_blob;

/// Pushover notification provider configuration.
#[fields_blob(implementation = "Pushover", config_contract = "PushoverSettings")]
pub struct PushoverConfig {
    /// Pushover application API token.
    pub api_key: SecretValue,
    /// Pushover user or group key identifying the recipient.
    pub user_key: SecretValue,
    /// Notification priority (-2 = lowest … 2 = emergency).
    pub priority: Option<i32>,
    /// Notification sound name played on the device.
    pub sound: Option<String>,
    /// Target device names; leave empty to send to all registered devices.
    pub devices: Vec<String>,
    /// Retry interval in seconds for emergency-priority notifications.
    pub retry: Option<i32>,
    /// Expiration time in seconds after which emergency retries stop.
    pub expire: Option<i32>,
}
