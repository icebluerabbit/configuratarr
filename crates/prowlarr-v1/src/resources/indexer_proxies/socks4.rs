use core_lib::SecretValue;
use core_macros::fields_blob;

/// SOCKS4 proxy configuration for indexer requests.
#[fields_blob(implementation = "Socks4", config_contract = "Socks4Settings")]
pub struct Socks4Config {
    /// Hostname or IP address of the SOCKS4 proxy server.
    pub host: Option<String>,
    /// TCP port the SOCKS4 proxy listens on.
    pub port: Option<i32>,
    /// Username for authenticating with the SOCKS4 proxy.
    pub username: Option<String>,
    /// Password for authenticating with the SOCKS4 proxy.
    pub password: Option<SecretValue>,
}
