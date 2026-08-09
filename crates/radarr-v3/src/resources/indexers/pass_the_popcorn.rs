use core_lib::SecretValue;
use core_macros::fields_blob;

/// PassThePopcorn private torrent tracker indexer.
#[fields_blob(
    implementation = "PassThePopcorn",
    config_contract = "PassThePopcornSettings",
    protocol = "torrent"
)]
pub struct PassThePopcornConfig {
    /// Base URL of the PassThePopcorn tracker.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// PTP API username used alongside the API key.
    #[wire(name = "aPIUser")]
    pub api_user: Option<String>,
    /// PTP API key for authentication.
    #[wire(name = "aPIKey")]
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
    /// Language IDs to treat as multi-language releases.
    #[wire(name = "multiLanguages")]
    pub multi_languages: Vec<i32>,
    /// Tracker-specific flag IDs that a release must carry to be grabbed.
    #[wire(name = "requiredFlags")]
    pub required_flags: Vec<i32>,
}
