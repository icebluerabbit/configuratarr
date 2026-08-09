use core_macros::fields_blob;

/// Nyaa anime torrent indexer.
#[fields_blob(
    implementation = "Nyaa",
    config_contract = "NyaaSettings",
    protocol = "torrent"
)]
pub struct NyaaConfig {
    /// Base URL of the Nyaa indexer.
    #[wire(name = "baseUrl")]
    pub base_url: String,
    /// Extra query string parameters appended verbatim to every API request.
    #[wire(name = "additionalParameters")]
    pub additional_parameters: Option<String>,
    /// Search anime releases using the standard Sonarr title format.
    #[wire(name = "animeStandardFormatSearch")]
    pub anime_standard_format_search: Option<bool>,
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
