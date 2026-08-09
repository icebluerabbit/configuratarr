use core_lib::SecretValue;
use core_macros::fields_blob;

/// Join notification provider configuration.
#[fields_blob(implementation = "Join", config_contract = "JoinSettings")]
pub struct JoinConfig {
    /// Join API key.
    pub api_key: SecretValue,
    /// Target device names to deliver the notification to.
    pub device_names: Option<String>,
    /// Notification priority level.
    pub priority: Option<i32>,
}
