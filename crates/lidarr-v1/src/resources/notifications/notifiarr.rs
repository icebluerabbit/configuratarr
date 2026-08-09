use core_lib::SecretValue;
use core_macros::fields_blob;

/// Notifiarr notification provider configuration.
///
/// Notifiarr is a Lidarr-specific notification integration.
#[fields_blob(implementation = "Notifiarr", config_contract = "NotifiarrSettings")]
pub struct NotifiarrConfig {
    /// Notifiarr API key.
    pub api_key: SecretValue,
}
