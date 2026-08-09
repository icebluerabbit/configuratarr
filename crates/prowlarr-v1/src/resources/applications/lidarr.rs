use core_lib::SecretValue;
use core_macros::fields_blob;

/// Lidarr application provider configuration.
#[fields_blob(implementation = "Lidarr", config_contract = "LidarrSettings")]
pub struct LidarrConfig {
    /// Prowlarr server URL that Lidarr uses to pull indexers.
    #[wire(name = "prowlarrUrl")]
    pub prowlarr_url: Option<String>,
    /// Lidarr base URL.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Lidarr API key for authenticating Prowlarr's push requests.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Newznab category IDs to sync to Lidarr.
    #[wire(name = "syncCategories")]
    pub sync_categories: Vec<i32>,
}
