use core_macros::fields_blob;

/// Roksbox metadata plugin — writes artwork and track-level metadata alongside media.
#[fields_blob(
    implementation = "RoksboxMetadata",
    config_contract = "RoksboxMetadataSettings"
)]
pub struct RoksboxConfig {
    /// Download and store artist-level artwork.
    #[wire(name = "artistImages")]
    pub artist_images: Option<bool>,
    /// Download and store album-level artwork.
    #[wire(name = "albumImages")]
    pub album_images: Option<bool>,
    /// Write track-level metadata files.
    #[wire(name = "trackMetadata")]
    pub track_metadata: Option<bool>,
}
