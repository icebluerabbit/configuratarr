use core_macros::fields_blob;

/// IPTorrents private torrent tracker indexer (RSS-only).
#[fields_blob(
    implementation = "IPTorrents",
    config_contract = "IPTorrentsSettings",
    protocol = "torrent"
)]
pub struct IpTorrentsConfig {
    /// RSS feed URL including the user's passkey (provided by IPTorrents).
    #[wire(name = "baseUrl")]
    pub base_url: String,
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
