use core_lib::SecretValue;
use core_macros::fields_blob;

/// Radarr application provider configuration.
#[fields_blob(implementation = "Radarr", config_contract = "RadarrSettings")]
pub struct RadarrConfig {
    /// Prowlarr server URL that Radarr uses to pull indexers.
    #[wire(name = "prowlarrUrl")]
    pub prowlarr_url: Option<String>,
    /// Radarr base URL.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Radarr API key for authenticating Prowlarr's push requests.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Newznab category IDs to sync to Radarr.
    #[wire(name = "syncCategories")]
    pub sync_categories: Vec<i32>,
}
