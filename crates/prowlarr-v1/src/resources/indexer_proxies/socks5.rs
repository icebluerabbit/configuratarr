use core_lib::SecretValue;
use core_macros::fields_blob;

/// SOCKS5 proxy configuration for indexer requests.
#[fields_blob(implementation = "Socks5", config_contract = "Socks5Settings")]
pub struct Socks5Config {
    /// Hostname or IP address of the SOCKS5 proxy server.
    pub host: Option<String>,
    /// TCP port the SOCKS5 proxy listens on.
    pub port: Option<i32>,
    /// Username for authenticating with the SOCKS5 proxy.
    pub username: Option<String>,
    /// Password for authenticating with the SOCKS5 proxy.
    pub password: Option<SecretValue>,
}
