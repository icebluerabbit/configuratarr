use core_macros::fields_blob;

/// CustomScript notification provider configuration.
#[fields_blob(
    implementation = "CustomScript",
    config_contract = "CustomScriptSettings"
)]
pub struct CustomScriptConfig {
    /// Absolute filesystem path to the script to execute.
    pub path: String,
    /// Additional arguments passed to the script on invocation.
    pub arguments: Option<String>,
}
