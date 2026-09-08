use core_macros::fields_blob;

/// CustomScript notification provider configuration.
#[fields_blob(
    implementation = "CustomScript",
    config_contract = "CustomScriptSettings"
)]
pub struct CustomScriptConfig {
    /// Absolute filesystem path to the script to execute.
    pub path: String,
    /// Legacy argument string. No longer supported — the API rejects a
    /// non-empty value; kept so an existing definition round-trips.
    pub arguments: Option<String>,
}
