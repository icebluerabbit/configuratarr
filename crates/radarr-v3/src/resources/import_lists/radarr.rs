use core_lib::SecretValue;
use core_macros::fields_blob;

/// Radarr import list — imports movies from another Radarr instance.
#[fields_blob(implementation = "RadarrImport", config_contract = "RadarrSettings")]
pub struct RadarrConfig {
    /// Base URL of the source Radarr instance.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// API key for authenticating with the source Radarr instance.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Profile IDs to import from — stored as a JSON array of ints.
    #[wire(name = "profileIds")]
    pub profile_ids: Vec<i32>,
    /// Tag IDs to filter by — stored as a JSON array of ints.
    #[wire(name = "tagIds")]
    pub tag_ids: Vec<i32>,
}
