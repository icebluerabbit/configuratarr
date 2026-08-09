use core_lib::SecretValue;
use core_macros::fields_blob;

/// Lidarr import list — imports artists from another Lidarr instance.
#[fields_blob(implementation = "LidarrImport", config_contract = "LidarrSettings")]
pub struct LidarrImportConfig {
    /// Base URL of the source Lidarr instance (e.g. `"http://lidarr:8686"`).
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// API key for authenticating with the source Lidarr instance.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Quality profile IDs to filter by on the source Lidarr instance.
    #[wire(name = "profileIds")]
    pub profile_ids: Vec<i32>,
    /// Tag IDs to filter by on the source Lidarr instance.
    #[wire(name = "tagIds")]
    pub tag_ids: Vec<i32>,
}
