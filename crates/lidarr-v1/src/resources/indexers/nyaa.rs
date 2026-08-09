use core_macros::fields_blob;

/// Nyaa torrent indexer for anime/music releases.
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
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: Option<i32>,
    /// Minimum seeding time in minutes Lidarr must seed after download.
    #[wire(name = "seedTime")]
    pub seed_time: Option<i32>,
    /// Minimum seed ratio Lidarr must reach before stopping seeding.
    #[wire(name = "seedRatio")]
    pub seed_ratio: Option<f64>,
}
