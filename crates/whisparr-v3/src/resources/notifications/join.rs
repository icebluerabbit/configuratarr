use core_lib::SecretValue;
use core_macros::fields_blob;

/// Join notification provider configuration.
#[fields_blob(implementation = "Join", config_contract = "JoinSettings")]
pub struct JoinConfig {
    /// Join API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Legacy comma-separated device ids. Deprecated by the API in favour of
    /// `device_names`; kept so an existing definition round-trips.
    #[wire(name = "deviceIds")]
    pub device_ids: Option<String>,
    /// Comma-separated target device names; leave empty to send to all devices.
    #[wire(name = "deviceNames")]
    pub device_names: Option<String>,
    /// Notification priority level.
    pub priority: Option<i32>,
}
