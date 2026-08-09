use core_lib::SecretValue;
use core_macros::fields_blob;

/// Sonarr import list — imports series from another Sonarr instance.
#[fields_blob(implementation = "SonarrImport", config_contract = "SonarrSettings")]
pub struct SonarrConfig {
    /// Base URL of the source Sonarr instance (e.g. `"http://sonarr:8989"`).
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// API key for authenticating with the source Sonarr instance.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Quality profile IDs to filter by on the source Sonarr instance.
    #[wire(name = "profileIds")]
    pub profile_ids: Vec<i32>,
    /// Language profile IDs to filter by on the source Sonarr instance.
    #[wire(name = "languageProfileIds")]
    pub language_profile_ids: Vec<i32>,
    /// Tag IDs to filter by on the source Sonarr instance.
    #[wire(name = "tagIds")]
    pub tag_ids: Vec<i32>,
}
