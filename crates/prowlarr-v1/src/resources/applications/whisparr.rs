use core_lib::SecretValue;
use core_macros::fields_blob;

/// Whisparr application provider configuration.
#[fields_blob(implementation = "Whisparr", config_contract = "WhisparrSettings")]
pub struct WhisparrConfig {
    /// Prowlarr server URL that Whisparr uses to pull indexers.
    #[wire(name = "prowlarrUrl")]
    pub prowlarr_url: Option<String>,
    /// Whisparr base URL.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Whisparr API key for authenticating Prowlarr's push requests.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Newznab category IDs to sync to Whisparr.
    #[wire(name = "syncCategories")]
    pub sync_categories: Vec<i32>,
}
