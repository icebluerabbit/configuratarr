use core_lib::SecretValue;
use core_macros::fields_blob;

/// Headphones usenet indexer integration.
#[fields_blob(
    implementation = "Headphones",
    config_contract = "HeadphonesSettings",
    protocol = "usenet"
)]
pub struct HeadphonesConfig {
    /// Headphones account username.
    pub username: String,
    /// Headphones account password.
    pub password: SecretValue,
    /// Usenet category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Number of days before release date to start monitoring.
    #[wire(name = "earlyReleaseLimit")]
    pub early_release_limit: Option<i32>,
}
