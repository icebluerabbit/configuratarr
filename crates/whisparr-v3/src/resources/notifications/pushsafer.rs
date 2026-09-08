use core_lib::SecretValue;
use core_macros::fields_blob;

/// Pushsafer notification provider configuration. Eros-only — this provider has
/// no radarr counterpart.
#[fields_blob(implementation = "Pushsafer", config_contract = "PushsaferSettings")]
pub struct PushsaferConfig {
    /// Pushsafer private or alias key used for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Device group id or list of device ids; leave empty to send to all devices.
    #[wire(name = "deviceIds")]
    pub device_ids: Vec<String>,
    /// Notification priority level.
    pub priority: Option<i32>,
    /// Retry interval in seconds for emergency-priority notifications
    /// (60 … 10800).
    pub retry: Option<i32>,
    /// Expiration time in seconds after which emergency retries stop
    /// (60 … 10800).
    pub expire: Option<i32>,
    /// Notification sound number 0-62; leave empty for the device default.
    pub sound: Option<String>,
    /// Vibration pattern 1-3; leave empty for the device default.
    pub vibration: Option<String>,
    /// Icon number 1-181; leave empty for the default Pushsafer icon.
    pub icon: Option<String>,
    /// Icon colour in hex format (e.g. `#ff0000`); leave empty for the default.
    #[wire(name = "iconColor")]
    pub icon_color: Option<String>,
}
