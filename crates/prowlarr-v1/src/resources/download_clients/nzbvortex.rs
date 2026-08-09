use core_lib::SecretValue;
use core_macros::fields_blob;

/// NZBVortex download client settings.
#[fields_blob(
    implementation = "NzbVortex",
    config_contract = "NzbVortexSettings",
    protocol = "usenet"
)]
pub struct NzbvortexConfig {
    /// Hostname or IP address of the NZBVortex server.
    pub host: Option<String>,
    /// TCP port the NZBVortex server listens on.
    pub port: Option<i32>,
    /// API key used to authenticate with NZBVortex.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
    /// URL base path if NZBVortex is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in NZBVortex.
    pub category: Option<String>,
    /// Item priority when adding to NZBVortex. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
}
