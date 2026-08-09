use core_lib::SecretValue;
use core_macros::fields_blob;

/// FileList private torrent tracker indexer.
#[fields_blob(
    implementation = "FileList",
    config_contract = "FileListSettings",
    protocol = "torrent"
)]
pub struct FileListConfig {
    /// Base URL of the FileList tracker (optional; has a built-in default).
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// FileList account username.
    pub username: String,
    /// FileList passkey for API authentication.
    pub passkey: SecretValue,
    /// TV category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Anime category IDs to include in searches.
    #[wire(name = "animeCategories")]
    pub anime_categories: Vec<i32>,
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
