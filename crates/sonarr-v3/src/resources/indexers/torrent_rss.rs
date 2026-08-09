use core_macros::fields_blob;

/// Generic torrent RSS feed indexer (RSS-only).
#[fields_blob(
    implementation = "TorrentRssIndexer",
    config_contract = "TorrentRssIndexerSettings",
    protocol = "torrent"
)]
pub struct TorrentRssConfig {
    /// URL of the torrent RSS feed.
    #[wire(name = "baseUrl")]
    pub base_url: String,
    /// Session cookie sent with RSS requests for authenticated feeds.
    pub cookie: Option<String>,
    /// Allow releases that report a size of zero bytes.
    #[wire(name = "allowZeroSize")]
    pub allow_zero_size: Option<bool>,
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
