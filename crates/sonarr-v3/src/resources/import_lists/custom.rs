use core_macros::fields_blob;

/// Custom import list — imports series from a custom URL compatible with the Sonarr list format.
#[fields_blob(implementation = "CustomImport", config_contract = "CustomSettings")]
pub struct CustomConfig {
    /// Base URL of the custom list endpoint.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
}
