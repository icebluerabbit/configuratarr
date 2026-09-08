use core_macros::fields_blob;

/// WDTV metadata plugin — writes XML metadata and artwork for Western Digital TV players.
#[fields_blob(
    implementation = "WdtvMetadata",
    config_contract = "WdtvMetadataSettings"
)]
pub struct WdtvConfig {
    /// Write movie-level metadata files.
    #[wire(name = "movieMetadata")]
    pub movie_metadata: Option<bool>,
    /// Download and store movie-level artwork.
    #[wire(name = "movieImages")]
    pub movie_images: Option<bool>,
}
