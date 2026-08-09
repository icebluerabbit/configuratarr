use core_lib::SecretValue;
use core_macros::fields_blob;

/// HTTP CONNECT proxy configuration for indexer requests.
#[fields_blob(implementation = "HTTP", config_contract = "HTTPSettings")]
pub struct HttpConfig {
    /// Hostname or IP address of the HTTP proxy server.
    pub host: Option<String>,
    /// TCP port the HTTP proxy listens on.
    pub port: Option<i32>,
    /// Username for authenticating with the HTTP proxy.
    pub username: Option<String>,
    /// Password for authenticating with the HTTP proxy.
    pub password: Option<SecretValue>,
}
