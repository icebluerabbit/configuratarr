use core_lib::SecretValue;
use core_macros::fields_blob;

/// TorrentLeech private torrent tracker indexer.
#[fields_blob(
    implementation = "Torrentleech",
    config_contract = "TorrentleechSettings",
    protocol = "torrent"
)]
pub struct TorrentLeechConfig {
    /// Base URL of the TorrentLeech tracker (optional; has a built-in default).
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// TorrentLeech API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: Option<i32>,
    /// Minimum seeding time in minutes Lidarr must seed after download.
    #[wire(name = "seedTime")]
    pub seed_time: Option<i32>,
    /// Minimum seeding time in minutes for a full discography download.
    #[wire(name = "discographySeedTime")]
    pub discography_seed_time: Option<i32>,
    /// Minimum seed ratio Lidarr must reach before stopping seeding.
    #[wire(name = "seedRatio")]
    pub seed_ratio: Option<f64>,
}
