use core_lib::SecretValue;
use core_macros::fields_blob;

/// HDBits private torrent tracker indexer.
#[fields_blob(
    implementation = "HDBits",
    config_contract = "HDBitsSettings",
    protocol = "torrent"
)]
pub struct HdBitsConfig {
    /// Base URL of the HDBits tracker.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// HDBits account username.
    pub username: String,
    /// HDBits API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: i32,
    /// Minimum seeding time in minutes Radarr must seed after download.
    #[wire(name = "seedTime")]
    pub seed_time: Option<i32>,
    /// Minimum seed ratio Radarr must reach before stopping seeding.
    #[wire(name = "seedRatio")]
    pub seed_ratio: Option<f64>,
    /// HDBits category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Language IDs to treat as multi-language releases.
    #[wire(name = "multiLanguages")]
    pub multi_languages: Vec<i32>,
    /// Tracker-specific flag IDs that a release must carry to be grabbed.
    #[wire(name = "requiredFlags")]
    pub required_flags: Vec<i32>,
    /// HDBits codec filter IDs; empty means no codec restriction.
    pub codecs: Vec<i32>,
    /// HDBits medium (source) filter IDs; empty means no medium restriction.
    pub mediums: Vec<i32>,
}
