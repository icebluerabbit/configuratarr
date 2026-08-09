use core_lib::SecretValue;
use core_macros::fields_blob;

/// Join notification provider configuration.
#[fields_blob(implementation = "Join", config_contract = "JoinSettings")]
pub struct JoinConfig {
    /// Join API key for authentication.
    pub api_key: Option<SecretValue>,
    /// Comma-separated target device names; leave empty to send to all devices.
    pub device_names: Option<String>,
    /// Notification priority level.
    pub priority: Option<i32>,
}
