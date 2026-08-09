use core_lib::SecretValue;
use core_macros::fields_blob;

/// Gazelle-based private torrent tracker indexer (e.g. OPS, RED clones).
#[fields_blob(
    implementation = "Gazelle",
    config_contract = "GazelleSettings",
    protocol = "torrent"
)]
pub struct GazelleConfig {
    /// Base URL of the Gazelle tracker.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Tracker account username.
    pub username: String,
    /// Tracker account password.
    pub password: SecretValue,
    /// Use freeleech tokens automatically when grabbing.
    #[wire(name = "useFreeleechToken")]
    pub use_freeleech_token: Option<bool>,
    /// Number of days before release date to start monitoring.
    #[wire(name = "earlyReleaseLimit")]
    pub early_release_limit: Option<i32>,
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
