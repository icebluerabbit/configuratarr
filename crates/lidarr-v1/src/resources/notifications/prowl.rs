use core_lib::SecretValue;
use core_macros::fields_blob;

/// Prowl notification provider configuration.
#[fields_blob(implementation = "Prowl", config_contract = "ProwlSettings")]
pub struct ProwlConfig {
    /// Prowl API key.
    pub api_key: SecretValue,
    /// Notification priority (-2 = very low … 2 = emergency).
    pub priority: Option<i32>,
}
