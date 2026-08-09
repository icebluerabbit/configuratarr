use core_macros::fields_blob;

/// Plex RSS import list — imports series from a Plex RSS feed URL.
#[fields_blob(
    implementation = "PlexRssImport",
    config_contract = "PlexRssImportSettings"
)]
pub struct PlexRssConfig {
    /// Full URL of the Plex RSS feed to import from.
    pub url: Option<String>,
}
