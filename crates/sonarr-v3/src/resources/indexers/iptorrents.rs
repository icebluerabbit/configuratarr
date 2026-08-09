use core_macros::fields_blob;

/// IPTorrents private torrent tracker indexer (RSS-only; passkey embedded in URL).
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
