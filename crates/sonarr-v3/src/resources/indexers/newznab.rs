use core_lib::SecretValue;
use core_macros::fields_blob;

/// Newznab usenet indexer (also used for Prowlarr usenet proxy).
#[fields_blob(
    implementation = "Newznab",
    config_contract = "NewznabSettings",
    protocol = "usenet"
)]
pub struct NewznabConfig {
    /// Base URL of the Newznab indexer.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// URL path to the Newznab API endpoint, appended to base_url.
    #[wire(name = "apiPath")]
    pub api_path: Option<String>,
    /// API key for authenticating requests to the Newznab indexer.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Extra query string parameters appended verbatim to every API request.
    #[wire(name = "additionalParameters")]
    pub additional_parameters: Option<String>,
    /// Search anime releases using the standard Sonarr title format.
    #[wire(name = "animeStandardFormatSearch")]
    pub anime_standard_format_search: Option<bool>,
    /// Newznab category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Anime category IDs to include in searches.
    #[wire(name = "animeCategories")]
    pub anime_categories: Vec<i32>,
}
