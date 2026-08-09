use core_lib::SecretValue;
use core_macros::fields_blob;

/// FileList private torrent tracker indexer.
#[fields_blob(
    implementation = "FileList",
    config_contract = "FileListSettings",
    protocol = "torrent"
)]
pub struct FileListConfig {
    /// Base URL of the FileList tracker.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// FileList account username.
    pub username: String,
    /// FileList account passkey used for API authentication.
    pub passkey: Option<SecretValue>,
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: i32,
    /// Minimum seeding time in minutes Radarr must seed after download.
    #[wire(name = "seedTime")]
    pub seed_time: Option<i32>,
    /// Minimum seed ratio Radarr must reach before stopping seeding.
    #[wire(name = "seedRatio")]
    pub seed_ratio: Option<f64>,
    /// FileList category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Language IDs to treat as multi-language releases.
    #[wire(name = "multiLanguages")]
    pub multi_languages: Vec<i32>,
    /// Tracker-specific flag IDs that a release must carry to be grabbed.
    #[wire(name = "requiredFlags")]
    pub required_flags: Vec<i32>,
}
