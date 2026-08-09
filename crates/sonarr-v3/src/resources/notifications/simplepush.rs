use core_lib::SecretValue;
use core_macros::fields_blob;

/// Simplepush notification provider configuration.
#[fields_blob(implementation = "Simplepush", config_contract = "SimplepushSettings")]
pub struct SimplepushConfig {
    /// Simplepush API key identifying the recipient device.
    pub key: SecretValue,
    /// Custom event name for categorizing notifications in Simplepush.
    pub event: Option<String>,
}
