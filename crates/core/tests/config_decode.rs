//! `engine::decode_config` — the user-config ingest codec.
//!
//! Users author the typed, flat shape keyed by snake_case `field.name`. This is
//! distinct from the wire codec (camelCase). These tests pin that distinction:
//! the same snake_case input that `decode_config` accepts is *not* what the wire
//! `decode` expects.

use core_lib::{SecretValue, engine};
use core_macros::{fields_blob, nested, resource, tagged, wire_enum};

#[resource(
    sync = crud,
    list = get("/api/v3/server"),
    create = post("/api/v3/server"),
    update = put("/api/v3/server/${self.id}"),
    delete = delete("/api/v3/server/${self.id}"),
)]
pub struct Server {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    /// Multiword field with a camelCase wire override — config users write the
    /// snake form, the wire form writes `useSsl`.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    pub api_key: SecretValue,
    pub port: Option<i32>,
}

#[test]
fn decodes_snake_case_config() {
    let cfg = serde_json::json!({
        "name": "main",
        "use_ssl": true,
        "api_key": "s3cr3t",
        "port": 8080
    });
    let s: Server = engine::decode_config(&cfg).unwrap();
    assert_eq!(s.name, "main");
    assert_eq!(s.use_ssl, Some(true));
    assert_eq!(s.api_key.expose(), "s3cr3t");
    assert_eq!(s.port, Some(8080));
    assert!(s.id.is_none()); // server-assigned, never in config
}

#[test]
fn config_keys_are_snake_not_wire() {
    // The wire codec expects `useSsl`; feeding it the snake form leaves the
    // field at its default. Proves config-decode keys differ from wire-decode.
    let snake = serde_json::json!({ "name": "x", "use_ssl": true, "api_key": "k" });
    let via_config: Server = engine::decode_config(&snake).unwrap();
    let via_wire: Server = engine::decode(&snake).unwrap();
    assert_eq!(via_config.use_ssl, Some(true));
    assert_eq!(via_wire.use_ssl, None); // wire didn't see `useSsl`
}

#[test]
fn omitted_fields_keep_defaults() {
    let cfg = serde_json::json!({ "name": "min", "api_key": "k" });
    let s: Server = engine::decode_config(&cfg).unwrap();
    assert_eq!(s.name, "min");
    assert_eq!(s.use_ssl, None);
    assert_eq!(s.port, None);
}

// --- flatten + nested-struct config recursion --------------------------------

#[nested]
pub struct Common {
    #[key]
    pub name: String,
    pub enable: bool,
}

#[resource(
    sync = crud,
    list = get("/api/v3/widget"),
    create = post("/api/v3/widget"),
    update = put("/api/v3/widget/${self.id}"),
    delete = delete("/api/v3/widget/${self.id}"),
)]
pub struct Widget {
    #[flatten]
    pub common: Common,
    pub port: Option<i32>,
}

#[test]
fn flatten_reads_parent_fields() {
    // Flattened struct's fields appear at the parent level in config, and the
    // nested struct decodes through the config codec (snake keys all the way).
    let cfg = serde_json::json!({ "name": "w", "enable": true, "port": 9 });
    let w: Widget = engine::decode_config(&cfg).unwrap();
    assert_eq!(w.common.name, "w");
    assert!(w.common.enable);
    assert_eq!(w.port, Some(9));
}

// --- provider config: tagged enum + fields-blob + wire enum, all flat --------

#[wire_enum(rename_all = "lowercase")]
pub enum Protocol {
    Usenet,
    Torrent,
    #[fallback]
    Unknown,
}

#[nested]
pub struct Provider {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    #[reference(tag)]
    pub tags: Vec<i32>,
}

#[fields_blob(
    implementation = "QBittorrent",
    config_contract = "QBittorrentSettings"
)]
pub struct QBittorrentConfig {
    pub host: Option<String>,
    pub port: Option<i32>,
    pub password: Option<SecretValue>,
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}

#[tagged(by = "implementation")]
pub enum DownloadClientProvider {
    #[variant("QBittorrent")]
    QBittorrent(QBittorrentConfig),
    #[fallback]
    Unknown(RawProvider),
}

#[nested]
pub struct RawProvider {
    pub implementation: String,
}

#[resource(
    sync = crud,
    list = get("/api/v3/downloadclient"),
    create = post("/api/v3/downloadclient"),
    update = put("/api/v3/downloadclient/${self.id}"),
    delete = delete("/api/v3/downloadclient/${self.id}"),
)]
pub struct DownloadClient {
    #[flatten]
    pub common: Provider,
    #[flatten]
    pub config: DownloadClientProvider,
    pub enable: bool,
    pub protocol: Protocol,
}

#[test]
fn provider_decodes_from_flat_config() {
    // The whole provider — envelope, tagged config, wire-enum protocol — is one
    // flat snake_case object. The user never writes the fields-blob array.
    let cfg = serde_json::json!({
        "name": "qbit",
        "tags": [3, 5],
        "implementation": "QBittorrent",
        "host": "qbit.local",
        "port": 8080,
        "password": "hunter2",
        "use_ssl": false,
        "enable": true,
        "protocol": "torrent"
    });
    let dc: DownloadClient = engine::decode_config(&cfg).unwrap();
    assert_eq!(dc.common.name, "qbit");
    assert_eq!(dc.common.tags, vec![3, 5]);
    assert!(dc.enable);
    assert!(matches!(dc.protocol, Protocol::Torrent));
    match dc.config {
        DownloadClientProvider::QBittorrent(c) => {
            assert_eq!(c.host.as_deref(), Some("qbit.local"));
            assert_eq!(c.port, Some(8080));
            assert_eq!(c.password.unwrap().expose(), "hunter2");
            assert_eq!(c.use_ssl, Some(false)); // snake `use_ssl`, not wire `useSsl`
        }
        _ => panic!("expected QBittorrent variant"),
    }
}

#[test]
fn provider_unknown_implementation_hits_fallback() {
    let cfg = serde_json::json!({
        "name": "weird",
        "implementation": "SomethingNew",
        "enable": false,
        "protocol": "usenet"
    });
    let dc: DownloadClient = engine::decode_config(&cfg).unwrap();
    assert!(matches!(dc.config, DownloadClientProvider::Unknown(_)));
    assert!(matches!(dc.protocol, Protocol::Usenet));
}
