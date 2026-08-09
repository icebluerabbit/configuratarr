use core_macros::fields_blob;

/// Fanzub anime usenet indexer.
#[fields_blob(
    implementation = "Fanzub",
    config_contract = "FanzubSettings",
    protocol = "usenet"
)]
pub struct FanzubConfig {
    /// Base URL of the Fanzub indexer (defaults to the public instance if absent).
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// Search anime releases using the standard Sonarr title format.
    #[wire(name = "animeStandardFormatSearch")]
    pub anime_standard_format_search: Option<bool>,
}
