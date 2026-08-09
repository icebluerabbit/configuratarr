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
    /// Strip the release year from search queries before sending them to the indexer.
    #[wire(name = "removeYear")]
    pub remove_year: bool,
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: i32,
    /// Minimum seeding time in minutes Radarr must seed after download.
    #[wire(name = "seedTime")]
    pub seed_time: Option<i32>,
    /// Minimum seed ratio Radarr must reach before stopping seeding.
    #[wire(name = "seedRatio")]
    pub seed_ratio: Option<f64>,
    /// Extra query string parameters appended verbatim to every API request.
    #[wire(name = "additionalParameters")]
    pub additional_parameters: Option<String>,
    /// Torznab category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Language IDs to treat as multi-language releases.
    #[wire(name = "multiLanguages")]
    pub multi_languages: Vec<i32>,
    /// Tracker-specific flag IDs that a release must carry to be grabbed.
    #[wire(name = "requiredFlags")]
    pub required_flags: Vec<i32>,
}
