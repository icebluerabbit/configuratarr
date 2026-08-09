use core_lib::SecretValue;
use core_macros::fields_blob;

/// LazyLibrarian application provider configuration.
#[fields_blob(
    implementation = "LazyLibrarian",
    config_contract = "LazyLibrarianSettings"
)]
pub struct LazyLibrarianConfig {
    /// Prowlarr server URL that LazyLibrarian uses to pull indexers.
    #[wire(name = "prowlarrUrl")]
    pub prowlarr_url: Option<String>,
    /// LazyLibrarian base URL.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// LazyLibrarian API key for authenticating Prowlarr's push requests.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// Newznab category IDs to sync to LazyLibrarian.
    #[wire(name = "syncCategories")]
    pub sync_categories: Vec<i32>,
}
