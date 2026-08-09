use core_lib::SecretValue;
use core_macros::fields_blob;

/// CouchPotato import list — imports movies from a CouchPotato instance.
#[fields_blob(
    implementation = "CouchPotatoImport",
    config_contract = "CouchPotatoSettings"
)]
pub struct CouchPotatoConfig {
    /// Host or IP address of the CouchPotato instance.
    pub link: Option<String>,
    /// Port number the CouchPotato instance listens on.
    pub port: Option<i32>,
    /// URL path prefix if CouchPotato is served under a sub-path.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// API key for authenticating with the CouchPotato instance.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// When true, imports only movies in an active (wanted) state.
    #[wire(name = "onlyActive")]
    pub only_active: Option<bool>,
}
