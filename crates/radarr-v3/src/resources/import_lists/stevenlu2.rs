use core_macros::fields_blob;

/// StevenLu2 import list — imports from StevenLu's scored popular movies list.
#[fields_blob(
    implementation = "Stevenlu2Import",
    config_contract = "Stevenlu2Settings"
)]
pub struct StevenLu2Config {
    /// Source list selector for the StevenLu2 feed (integer enum).
    pub source: Option<i32>,
    /// Minimum score threshold for a movie to be included.
    #[wire(name = "minScore")]
    pub min_score: Option<i32>,
}
