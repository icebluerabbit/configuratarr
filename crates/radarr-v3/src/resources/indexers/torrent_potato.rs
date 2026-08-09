use core_lib::SecretValue;
use core_macros::fields_blob;

/// TorrentPotato legacy torrent indexer.
#[fields_blob(
    implementation = "TorrentPotato",
    config_contract = "TorrentPotatoSettings",
    protocol = "torrent"
)]
pub struct TorrentPotatoConfig {
    /// Base URL of the TorrentPotato-compatible indexer.
    #[wire(name = "baseUrl")]
    pub base_url: String,
    /// Username for TorrentPotato authentication.
    pub user: Option<String>,
    /// Passkey for TorrentPotato authentication.
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
    /// Language IDs to treat as multi-language releases.
    #[wire(name = "multiLanguages")]
    pub multi_languages: Vec<i32>,
    /// Tracker-specific flag IDs that a release must carry to be grabbed.
    #[wire(name = "requiredFlags")]
    pub required_flags: Vec<i32>,
}
