use core_lib::SecretValue;
use core_macros::fields_blob;

/// Simplepush notification provider configuration.
#[fields_blob(implementation = "Simplepush", config_contract = "SimplepushSettings")]
pub struct SimplepushConfig {
    /// Simplepush key (device identifier).
    pub key: SecretValue,
    /// Custom event name for filtering notifications in the Simplepush app.
    pub event: Option<String>,
}
