use core_macros::fields_blob;

/// Kodi / XBMC metadata plugin — writes NFO files and artwork alongside movies.
#[fields_blob(
    implementation = "XbmcMetadata",
    config_contract = "XbmcMetadataSettings"
)]
pub struct XbmcConfig {
    /// Write movie-level NFO metadata files.
    #[wire(name = "movieMetadata")]
    pub movie_metadata: Option<bool>,
    /// Include the tmdb/imdb url inside NFO files.
    #[wire(name = "movieMetadataURL")]
    pub movie_metadata_url: Option<bool>,
    /// Language to write metadata in, if available.
    #[wire(name = "movieMetadataLanguage")]
    pub movie_metadata_language: Option<i32>,
    /// Download and store movie-level artwork.
    #[wire(name = "movieImages")]
    pub movie_images: Option<bool>,
    /// Write metadata to movie.nfo instead of the default `<movie-filename>.nfo`.
    #[wire(name = "useMovieNfo")]
    pub use_movie_nfo: Option<bool>,
    /// Write the collection name to the .nfo file.
    #[wire(name = "addCollectionName")]
    pub add_collection_name: Option<bool>,
}
