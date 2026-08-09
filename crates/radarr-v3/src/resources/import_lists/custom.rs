use core_macros::fields_blob;

/// Custom (RadarrList) import list — imports from a custom Radarr list URL.
#[fields_blob(
    implementation = "RadarrListImport",
    config_contract = "RadarrListSettings"
)]
pub struct CustomConfig {
    /// URL of the custom Radarr-compatible list endpoint.
    pub url: Option<String>,
}
