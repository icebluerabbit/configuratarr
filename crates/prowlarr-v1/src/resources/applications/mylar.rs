use core_lib::SecretValue;
use core_macros::fields_blob;

/// Mylar application provider configuration.
#[fields_blob(implementation = "Mylar", config_contract = "MylarSettings")]
pub struct MylarConfig {
    /// Prowlarr server URL that Mylar uses to pull indexers.
    #[wire(name = "prowlarrUrl")]
    pub prowlarr_url: Option<String>,
    /// Mylar base URL.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Mylar API key for authenticating Prowlarr's push requests.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Newznab category IDs to sync to Mylar.
    #[wire(name = "syncCategories")]
    pub sync_categories: Vec<i32>,
}
