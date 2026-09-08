use core_lib::SecretValue;
use core_macros::fields_blob;

/// Newznab usenet indexer (also used for Prowlarr proxy).
#[fields_blob(
    implementation = "Newznab",
    config_contract = "NewznabSettings",
    protocol = "usenet"
)]
pub struct NewznabConfig {
    /// Base URL of the Newznab indexer.
    #[wire(name = "baseUrl")]
    pub base_url: String,
    /// URL path to the Newznab API endpoint, appended to base_url.
    #[wire(name = "apiPath")]
    pub api_path: Option<String>,
    /// API key for authenticating requests to the Newznab indexer.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Newznab category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Extra query string parameters appended verbatim to every API request.
    #[wire(name = "additionalParameters")]
    pub additional_parameters: Option<String>,
    /// Language IDs to treat as multi-language releases.
    #[wire(name = "multiLanguages")]
    pub multi_languages: Vec<i32>,
    /// Strip the release year from search queries before sending them to the indexer.
    #[wire(name = "removeYear")]
    pub remove_year: bool,
    /// Download outcomes (e.g. executables, potentially dangerous files) that
    /// should be treated as a failed grab.
    #[wire(name = "failDownloads")]
    pub fail_downloads: Vec<i32>,
}
