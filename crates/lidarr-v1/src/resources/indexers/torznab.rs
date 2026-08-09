use core_lib::SecretValue;
use core_macros::fields_blob;

/// Torznab torrent indexer (also used for Prowlarr torrent proxy).
#[fields_blob(
    implementation = "Torznab",
    config_contract = "TorznabSettings",
    protocol = "torrent"
)]
pub struct TorznabConfig {
    /// Base URL of the Torznab indexer.
    #[wire(name = "baseUrl")]
    pub base_url: String,
    /// URL path to the Torznab API endpoint, appended to base_url.
    #[wire(name = "apiPath")]
    pub api_path: Option<String>,
    /// API key for authenticating requests to the Torznab indexer.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Extra query string parameters appended verbatim to every API request.
    #[wire(name = "additionalParameters")]
    pub additional_parameters: Option<String>,
    /// Torznab category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: Option<i32>,
    /// Minimum seeding time in minutes Lidarr must seed after download.
    #[wire(name = "seedTime")]
    pub seed_time: Option<i32>,
    /// Minimum seed ratio Lidarr must reach before stopping seeding.
    #[wire(name = "seedRatio")]
    pub seed_ratio: Option<f64>,
}
