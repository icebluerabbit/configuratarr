//! Coverage for the provider-archetype macros added for the radarr migration:
//! `#[nested]`, `#[fields_blob]`, `#[tagged]`, `#[wire_enum]`, plus `#[default]`,
//! `#[flatten]`, and the `Json` field kind. Asserts the *descriptors* the macros
//! emit — codec logic is intentionally still deferred.

use core_lib::descriptor::CodecMeta;
use core_lib::{CodecKind, DefaultLit, Described, FieldKind, Json, SecretValue, SyncKind};
use core_macros::{fields_blob, nested, resource, tagged, wire_enum};

// --- #[nested] ---------------------------------------------------------------

/// Read-only status blob — embedded, never synced directly.
#[nested]
pub struct ProviderMessage {
    pub message: Option<String>,
    #[wire(name = "type")]
    pub message_type: Option<String>,
}

#[test]
fn nested_is_embedded_with_no_path() {
    let d = ProviderMessage::descriptor();
    assert_eq!(d.type_name, "provider_message");
    assert_eq!(d.endpoints, core_lib::Endpoints::NONE);
    assert!(matches!(d.sync, SyncKind::Embedded));
    assert!(matches!(d.codec, CodecKind::Standard));
    assert!(d.variants.is_empty());

    let mt = d.fields.iter().find(|f| f.name == "message_type").unwrap();
    assert_eq!(mt.wire_name, Some("type"));
    assert_eq!(
        d.doc,
        Some("Read-only status blob — embedded, never synced directly.")
    );
}

// --- Json field kind ---------------------------------------------------------

#[nested]
pub struct RawProvider {
    pub implementation: String,
    pub fields: Vec<Json>,
    pub blob: Option<Json>,
}

#[test]
fn json_field_kinds() {
    let d = RawProvider::descriptor();
    let fields = d.fields.iter().find(|f| f.name == "fields").unwrap();
    assert!(matches!(fields.kind, FieldKind::Vec(FieldKind::Json)));
    let blob = d.fields.iter().find(|f| f.name == "blob").unwrap();
    assert!(matches!(blob.kind, FieldKind::Optional(FieldKind::Json)));
}

// --- #[fields_blob] ----------------------------------------------------------

#[fields_blob(
    implementation = "QBittorrent",
    config_contract = "QBittorrentSettings",
    protocol = "torrent"
)]
pub struct QBittorrentConfig {
    pub host: Option<String>,
    pub password: Option<SecretValue>, // secret inferred from the type, no attr
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
}

#[test]
fn fields_blob_codec_and_meta() {
    let d = QBittorrentConfig::descriptor();
    assert_eq!(d.type_name, "q_bittorrent_config");
    assert!(matches!(d.sync, SyncKind::Embedded));
    assert!(matches!(d.codec, CodecKind::FieldsBlob));
    match &d.codec_meta {
        CodecMeta::FieldsBlob {
            implementation,
            config_contract,
            protocol,
        } => {
            assert_eq!(*implementation, "QBittorrent");
            assert_eq!(*config_contract, Some("QBittorrentSettings"));
            assert_eq!(*protocol, Some("torrent"));
        }
        _ => panic!("expected FieldsBlob meta"),
    }
    let ssl = d.fields.iter().find(|f| f.name == "use_ssl").unwrap();
    assert_eq!(ssl.wire_name, Some("useSsl"));

    // SecretValue type → secret flag, structurally a String. No wrapper, no attr.
    let pw = d.fields.iter().find(|f| f.name == "password").unwrap();
    assert!(pw.secret);
    assert!(matches!(pw.kind, FieldKind::Optional(FieldKind::String)));
    let host = d.fields.iter().find(|f| f.name == "host").unwrap();
    assert!(!host.secret);
}

// --- #[tagged] ---------------------------------------------------------------

#[tagged(by = "implementation")]
pub enum DownloadClientProvider {
    #[variant("QBittorrent")]
    QBittorrent(QBittorrentConfig),
    #[fallback]
    Unknown(RawProvider),
}

#[test]
fn tagged_variants_and_discriminator() {
    let d = DownloadClientProvider::descriptor();
    assert!(matches!(d.codec, CodecKind::TaggedByImpl));
    assert!(matches!(d.sync, SyncKind::Embedded));
    match &d.codec_meta {
        CodecMeta::TaggedByImpl { discriminator } => assert_eq!(*discriminator, "implementation"),
        _ => panic!("expected TaggedByImpl meta"),
    }
    assert!(d.fields.is_empty());
    assert_eq!(d.variants.len(), 2);

    let qb = &d.variants[0];
    assert_eq!(qb.name, "QBittorrent");
    assert_eq!(qb.wire, Some("QBittorrent"));
    assert_eq!(qb.inner_type, Some("q_bittorrent_config"));
    assert!(!qb.fallback);

    let unk = &d.variants[1];
    assert_eq!(unk.name, "Unknown");
    assert_eq!(unk.wire, None);
    assert_eq!(unk.inner_type, Some("raw_provider"));
    assert!(unk.fallback);
}

// --- #[wire_enum] ------------------------------------------------------------

#[wire_enum(rename_all = "lowercase")]
pub enum DownloadProtocol {
    Usenet,
    Torrent,
    #[fallback]
    Unknown,
}

#[test]
fn wire_enum_string_variants() {
    let d = DownloadProtocol::descriptor();
    assert!(matches!(d.codec, CodecKind::StringEnum));
    assert!(matches!(d.sync, SyncKind::Embedded));
    assert_eq!(d.variants.len(), 3);
    assert_eq!(d.variants[0].wire, Some("usenet"));
    assert_eq!(d.variants[1].wire, Some("torrent"));
    assert_eq!(d.variants[2].wire, None);
    assert!(d.variants[2].fallback);
    assert!(d.variants[0].inner_type.is_none());
}

// --- #[default] + #[flatten] -------------------------------------------------

#[resource(
    sync = crud,
    list = get("/api/v3/downloadclient"),
    create = post("/api/v3/downloadclient"),
    update = put("/api/v3/downloadclient/${self.id}"),
    delete = delete("/api/v3/downloadclient/${self.id}"),
)]
pub struct DownloadClient {
    #[flatten]
    pub common: ProviderMessage,
    /// Ref to tags — plain `Vec<i32>` carrying `#[reference(tag)]` metadata.
    #[reference(tag)]
    pub tags: Vec<i32>,
    #[default(1)]
    pub priority: i32,
    #[default(true)]
    pub enabled: bool,
    #[default("doNotPrefer")]
    pub propers: String,
}

#[test]
fn defaults_and_flatten() {
    let d = DownloadClient::descriptor();

    // #[flatten] field is carried as a nested field (splice happens in codec).
    let common = &d.fields[0];
    assert_eq!(common.name, "common");
    assert!(matches!(
        common.kind,
        FieldKind::Nested {
            type_name: "provider_message"
        }
    ));

    // #[reference(tag)] → plain Vec(Int32) + reference metadata, no Ref<T> wrapper.
    let tags = d.fields.iter().find(|f| f.name == "tags").unwrap();
    assert_eq!(tags.reference, &["tag"]);
    assert!(matches!(tags.kind, FieldKind::Vec(FieldKind::Int32)));

    let priority = d.fields.iter().find(|f| f.name == "priority").unwrap();
    assert_eq!(priority.default, Some(DefaultLit::Int(1)));
    let enabled = d.fields.iter().find(|f| f.name == "enabled").unwrap();
    assert_eq!(enabled.default, Some(DefaultLit::Bool(true)));
    let propers = d.fields.iter().find(|f| f.name == "propers").unwrap();
    assert_eq!(propers.default, Some(DefaultLit::Str("doNotPrefer")));
}
