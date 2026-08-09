use core_lib::SecretValue;
use core_macros::fields_blob;

/// Notifiarr notification provider configuration.
#[fields_blob(implementation = "Notifiarr", config_contract = "NotifiarrSettings")]
pub struct NotifiarrConfig {
    /// Notifiarr API key for authentication.
    pub api_key: SecretValue,
}
