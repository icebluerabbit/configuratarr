use core_lib::SecretValue;
use core_macros::fields_blob;

/// Sonarr application provider configuration.
#[fields_blob(implementation = "Sonarr", config_contract = "SonarrSettings")]
pub struct SonarrConfig {
    /// Prowlarr server URL that Sonarr uses to pull indexers.
    #[wire(name = "prowlarrUrl")]
    pub prowlarr_url: Option<String>,
    /// Sonarr base URL.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Sonarr API key for authenticating Prowlarr's push requests.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Standard Newznab category IDs to sync to Sonarr.
    #[wire(name = "syncCategories")]
    pub sync_categories: Vec<i32>,
    /// Anime-specific Newznab category IDs to sync to Sonarr.
    #[wire(name = "animeSyncCategories")]
    pub anime_sync_categories: Vec<i32>,
}
