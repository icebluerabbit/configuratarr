use core_macros::fields_blob;

/// Roksbox metadata plugin — writes XML metadata and artwork for Roksbox media players.
#[fields_blob(
    implementation = "RoksboxMetadata",
    config_contract = "RoksboxMetadataSettings"
)]
pub struct RoksboxConfig {
    /// Write movie-level metadata files.
    #[wire(name = "movieMetadata")]
    pub movie_metadata: Option<bool>,
    /// Download and store movie-level artwork.
    #[wire(name = "movieImages")]
    pub movie_images: Option<bool>,
}
