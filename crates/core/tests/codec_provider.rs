//! The full provider shape: flatten envelope + tagged enum + fields-blob
//! variant + wire-enum. Round-trips the real *arr download-client wire format.

use core_lib::{SecretValue, engine};
use core_macros::{fields_blob, nested, resource, tagged, wire_enum};

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

fn sample() -> DownloadClient {
    DownloadClient {
        common: Provider {
            id: Some(1),
            name: "qbit".into(),
            tags: vec![3, 5],
        },
        config: DownloadClientProvider::QBittorrent(QBittorrentConfig {
            host: Some("qbit.local".into()),
            port: Some(8080),
            password: Some(SecretValue::new("hunter2".into())),
            use_ssl: Some(false),
        }),
        enable: true,
        protocol: Protocol::Torrent,
    }
}

#[test]
fn encodes_to_arr_wire_shape() {
    let v = engine::encode(&sample()).unwrap();
    assert_eq!(
        v,
        serde_json::json!({
            // envelope (id read-only → dropped)
            "name": "qbit",
            "tags": [3, 5],
            // tagged enum → fields-blob, hoisted to top level
            "implementation": "QBittorrent",
            "configContract": "QBittorrentSettings",
            "fields": [
                { "name": "host", "value": "qbit.local" },
                { "name": "port", "value": 8080 },
                { "name": "password", "value": "hunter2" },
                { "name": "useSsl", "value": false }
            ],
            // own fields; protocol wire-enum → bare string
            "enable": true,
            "protocol": "torrent"
        })
    );
}

#[test]
fn decodes_from_arr_wire_shape() {
    let api = serde_json::json!({
        "id": 1,
        "name": "qbit",
        "tags": [3, 5],
        "implementation": "QBittorrent",
        "configContract": "QBittorrentSettings",
        "fields": [
            { "name": "host", "value": "qbit.local" },
            { "name": "port", "value": 8080 },
            { "name": "password", "value": "hunter2" },
            { "name": "useSsl", "value": false }
        ],
        "enable": true,
        "protocol": "torrent"
    });
    let dc: DownloadClient = engine::decode(&api).unwrap();
    assert_eq!(dc.common.id, Some(1));
    assert_eq!(dc.common.name, "qbit");
    assert_eq!(dc.common.tags, vec![3, 5]);
    assert!(dc.enable);
    assert!(matches!(dc.protocol, Protocol::Torrent));
    match dc.config {
        DownloadClientProvider::QBittorrent(c) => {
            assert_eq!(c.host.as_deref(), Some("qbit.local"));
            assert_eq!(c.port, Some(8080));
            assert_eq!(c.password.unwrap().expose(), "hunter2");
            assert_eq!(c.use_ssl, Some(false));
        }
        _ => panic!("expected QBittorrent variant"),
    }
}

#[test]
fn unknown_implementation_hits_fallback() {
    let api = serde_json::json!({
        "name": "weird",
        "implementation": "SomethingNew",
        "enable": false,
        "protocol": "usenet"
    });
    let dc: DownloadClient = engine::decode(&api).unwrap();
    assert!(matches!(dc.config, DownloadClientProvider::Unknown(_)));
    assert!(matches!(dc.protocol, Protocol::Usenet));
}
