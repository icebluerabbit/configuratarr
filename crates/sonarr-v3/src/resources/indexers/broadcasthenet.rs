use core_lib::SecretValue;
use core_macros::fields_blob;

/// BroadcastheNet private torrent tracker indexer.
#[fields_blob(
    implementation = "BroadcastheNet",
    config_contract = "BroadcastheNetSettings",
    protocol = "torrent"
)]
pub struct BroadcastheNetConfig {
    /// Base URL of the BroadcastheNet tracker API.
    #[wire(name = "baseUrl")]
    pub base_url: String,
    /// API key for authenticating requests to BroadcastheNet.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: Option<i32>,
    /// Minimum seeding time in minutes after a season-pack download.
    #[wire(name = "seasonPackSeedTime")]
    pub season_pack_seed_time: Option<i32>,
    /// Minimum seeding time in minutes Sonarr must seed after download.
    #[wire(name = "seedTime")]
    pub seed_time: Option<i32>,
    /// Minimum seed ratio Sonarr must reach before stopping seeding.
    #[wire(name = "seedRatio")]
    pub seed_ratio: Option<f64>,
}
