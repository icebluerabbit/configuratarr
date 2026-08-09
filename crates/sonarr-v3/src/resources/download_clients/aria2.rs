use core_lib::SecretValue;
use core_macros::fields_blob;

/// Aria2 download client settings.
#[fields_blob(
    implementation = "Aria2",
    config_contract = "Aria2Settings",
    protocol = "torrent"
)]
pub struct Aria2Config {
    /// Hostname or IP address of the Aria2 RPC server.
    pub host: Option<String>,
    /// TCP port the Aria2 RPC server listens on.
    pub port: Option<i32>,
    /// Secret token for authenticating with the Aria2 RPC interface.
    #[wire(name = "secretToken")]
    pub secret_token: Option<SecretValue>,
    /// Path to the Aria2 JSON-RPC endpoint (default: `/rpc`).
    #[wire(name = "rpcPath")]
    pub rpc_path: Option<String>,
    /// Connect to the Aria2 RPC server over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}
