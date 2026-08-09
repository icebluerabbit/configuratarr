use core_macros::fields_blob;

/// Roksbox metadata plugin — writes XML metadata and artwork for Roksbox media players.
#[fields_blob(
    implementation = "RoksboxMetadata",
    config_contract = "RoksboxMetadataSettings"
)]
pub struct RoksboxConfig {
    /// Download and store series-level artwork.
    #[wire(name = "seriesImages")]
    pub series_images: Option<bool>,
    /// Download and store season-level artwork.
    #[wire(name = "seasonImages")]
    pub season_images: Option<bool>,
    /// Write episode-level metadata files.
    #[wire(name = "episodeMetadata")]
    pub episode_metadata: Option<bool>,
    /// Download and store episode-level artwork (thumbnails).
    #[wire(name = "episodeImages")]
    pub episode_images: Option<bool>,
}
