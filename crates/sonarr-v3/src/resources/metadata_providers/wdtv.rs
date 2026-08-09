use core_macros::fields_blob;

/// WDTV metadata plugin — writes XML metadata and artwork for Western Digital TV players.
#[fields_blob(
    implementation = "WdtvMetadata",
    config_contract = "WdtvMetadataSettings"
)]
pub struct WdtvConfig {
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
