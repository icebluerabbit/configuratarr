use core_macros::fields_blob;

/// Kodi / XBMC metadata plugin — writes NFO files and artwork alongside media.
#[fields_blob(
    implementation = "XbmcMetadata",
    config_contract = "XbmcMetadataSettings"
)]
pub struct KodiConfig {
    /// Write series-level NFO metadata files.
    #[wire(name = "seriesMetadata")]
    pub series_metadata: Option<bool>,
    /// Include series metadata URLs inside NFO files.
    #[wire(name = "seriesMetadataUrl")]
    pub series_metadata_url: Option<bool>,
    /// Download and store series-level artwork.
    #[wire(name = "seriesImages")]
    pub series_images: Option<bool>,
    /// Download and store season-level artwork.
    #[wire(name = "seasonImages")]
    pub season_images: Option<bool>,
    /// Write episode-level NFO metadata files.
    #[wire(name = "episodeMetadata")]
    pub episode_metadata: Option<bool>,
    /// Download and store episode-level artwork (thumbnails).
    #[wire(name = "episodeImages")]
    pub episode_images: Option<bool>,
}
