use core_lib::SecretValue;
use core_macros::fields_blob;

/// Readarr application provider configuration.
#[fields_blob(implementation = "Readarr", config_contract = "ReadarrSettings")]
pub struct ReadarrConfig {
    /// Prowlarr server URL that Readarr uses to pull indexers.
    #[wire(name = "prowlarrUrl")]
    pub prowlarr_url: Option<String>,
    /// Readarr base URL.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Readarr API key for authenticating Prowlarr's push requests.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Newznab category IDs to sync to Readarr.
    #[wire(name = "syncCategories")]
    pub sync_categories: Vec<i32>,
}
