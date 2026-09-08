use core_macros::fields_blob;

/// Kometa metadata plugin — writes `poster.jpg`/`background.jpg` artwork alongside movies.
#[fields_blob(
    implementation = "KometaMetadata",
    config_contract = "KometaMetadataSettings"
)]
pub struct KometaConfig {
    /// Download and store movie-level artwork.
    #[wire(name = "movieImages")]
    pub movie_images: Option<bool>,
}
