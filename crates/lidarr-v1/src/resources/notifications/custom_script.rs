use core_macros::fields_blob;

/// Custom script notification provider configuration.
#[fields_blob(
    implementation = "CustomScript",
    config_contract = "CustomScriptSettings"
)]
pub struct CustomScriptConfig {
    /// Absolute path to the script to execute on notification events.
    pub path: String,
    /// Arguments passed to the script on invocation.
    pub arguments: Option<String>,
}
