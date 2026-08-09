use core_lib::SecretValue;
use core_macros::fields_blob;

/// Redacted (RED) private torrent tracker indexer.
#[fields_blob(
    implementation = "Redacted",
    config_contract = "RedactedSettings",
    protocol = "torrent"
)]
pub struct RedactedConfig {
    /// Redacted API key for authentication.
    #[wire(name = "apiKey")]
    pub api_key: SecretValue,
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
