use core_lib::SecretValue;
use core_macros::fields_blob;

/// NZBGet download client settings.
#[fields_blob(
    implementation = "Nzbget",
    config_contract = "NzbgetSettings",
    protocol = "usenet"
)]
pub struct NzbgetConfig {
    /// Hostname or IP address of the NZBGet server.
    pub host: Option<String>,
    /// TCP port the NZBGet web UI listens on.
    pub port: Option<i32>,
    /// Username for authenticating with NZBGet.
    pub username: Option<String>,
    /// Password for authenticating with NZBGet.
    pub password: Option<SecretValue>,
    /// URL base path if NZBGet is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Category label assigned to downloads in NZBGet.
    pub category: Option<String>,
    /// Item priority when adding to NZBGet. `0` = Last, `1` = First.
    #[wire(name = "itemPriority")]
    pub item_priority: Option<i32>,
    /// Add NZBs to NZBGet in a paused state.
    #[wire(name = "addPaused")]
    pub add_paused: Option<bool>,
    /// Connect to NZBGet over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
