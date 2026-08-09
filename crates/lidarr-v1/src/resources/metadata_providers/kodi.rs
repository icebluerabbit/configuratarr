use core_macros::fields_blob;

/// Kodi / XBMC metadata plugin — writes NFO files and artwork alongside media.
#[fields_blob(
    implementation = "XbmcMetadata",
    config_contract = "XbmcMetadataSettings"
)]
pub struct KodiConfig {
    /// Write artist-level metadata files.
    #[wire(name = "artistMetadata")]
    pub artist_metadata: Option<bool>,
    /// Write album-level metadata files.
    #[wire(name = "albumMetadata")]
    pub album_metadata: Option<bool>,
    /// Download and store artist-level artwork.
    #[wire(name = "artistImages")]
    pub artist_images: Option<bool>,
    /// Download and store album-level artwork.
    #[wire(name = "albumImages")]
    pub album_images: Option<bool>,
}
