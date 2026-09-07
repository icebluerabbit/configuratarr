use core_macros::fields_blob;

/// MediaBrowser (Emby) metadata plugin — writes metadata files alongside movies.
#[fields_blob(
    implementation = "MediaBrowserMetadata",
    config_contract = "MediaBrowserMetadataSettings"
)]
pub struct MediaBrowserConfig {
    /// Write movie-level metadata files.
    #[wire(name = "movieMetadata")]
    pub movie_metadata: Option<bool>,
}
