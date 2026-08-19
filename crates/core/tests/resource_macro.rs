//! Integration tests for the `#[resource]` proc-macro.
//!
//! Exercises every knob on the macro surface against the types in `core_lib`:
//! type_name defaulting and override, path Some/None, sync variants, field
//! roles, doc capture, wire attributes, codec selection, FieldKind
//! classification, and the ResourceErased blanket impl.

use core_lib::descriptor::CodecMeta;
use core_lib::{
    CodecKind, Described, FieldKind, FieldRef, FieldRole, ResourceErased, SecretValue, SyncKind,
};
use core_macros::resource;

// ── ref target ───────────────────────────────────────────────────────────────

/// Minimal resource used only as a `#[reference(other)]` target type.
#[resource(
    sync = crud,
    list = get("/api/v3/other"),
    create = post("/api/v3/other"),
    update = put("/api/v3/other/${self.id}"),
    delete = delete("/api/v3/other/${self.id}"),
)]
pub struct Other {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

// ── type_name ─────────────────────────────────────────────────────────────────

#[resource(
    sync = crud,
    list = get("/api/v3/simple_resource"),
    create = post("/api/v3/simple_resource"),
    update = put("/api/v3/simple_resource/${self.id}"),
    delete = delete("/api/v3/simple_resource/${self.id}"),
)]
pub struct SimpleResource {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub label: String,
}

#[test]
fn type_name_auto_defaults_to_snake_case() {
    // No `type_name = "..."` arg — macro snake-cases the struct ident.
    assert_eq!(SimpleResource::descriptor().type_name, "simple_resource");
}

#[resource(
    type_name = "custom_type",
    sync = crud,
    list = get("/api/v3/override"),
    create = post("/api/v3/override"),
    update = put("/api/v3/override/${self.id}"),
    delete = delete("/api/v3/override/${self.id}"),
)]
pub struct OverrideMe {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub label: String,
}

#[test]
fn type_name_explicit_override() {
    assert_eq!(OverrideMe::descriptor().type_name, "custom_type");
}

// ── endpoints present / NONE ─────────────────────────────────────────────────

#[test]
fn endpoints_present_when_declared() {
    let e = SimpleResource::descriptor().endpoints;
    assert_eq!(e.create.unwrap().path, "/api/v3/simple_resource");
    assert_eq!(e.delete.unwrap().path, "/api/v3/simple_resource/${self.id}");
}

/// A resource that declares no endpoints — every slot defaults to `None`.
#[resource(sync = crud)]
pub struct NoPath {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

#[test]
fn endpoints_none_when_omitted() {
    assert_eq!(NoPath::descriptor().endpoints, core_lib::Endpoints::NONE);
}

// ── SyncKind variants ─────────────────────────────────────────────────────────

#[resource(sync = crud)]
pub struct CrudRes {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

#[resource(sync = singleton)]
pub struct SingletonRes {
    pub id: Option<i32>,
}

/// String form of the sync argument should parse identically to the bare ident.
#[resource(sync = "crud")]
pub struct CrudStrRes {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

#[test]
fn sync_kind_crud() {
    assert!(matches!(CrudRes::descriptor().sync, SyncKind::Crud));
}

#[test]
fn sync_kind_singleton() {
    assert!(matches!(
        SingletonRes::descriptor().sync,
        SyncKind::Singleton
    ));
}

#[test]
fn sync_kind_string_form_parses() {
    // `sync = "crud"` must produce the same SyncKind as the bare `sync = crud`.
    assert!(matches!(CrudStrRes::descriptor().sync, SyncKind::Crud));
}

// ── field roles ───────────────────────────────────────────────────────────────

#[test]
fn field_id_role_and_read_only() {
    let d = SimpleResource::descriptor();
    assert_eq!(d.fields[0].name, "id");
    assert!(matches!(d.fields[0].role, FieldRole::Id));
    // #[id] implies read_only — id fields are server-assigned, never sent on write.
    assert!(d.fields[0].read_only);
}

#[test]
fn field_key_role_not_read_only() {
    let d = SimpleResource::descriptor();
    assert_eq!(d.fields[1].name, "label");
    assert!(matches!(d.fields[1].role, FieldRole::Key));
    assert!(!d.fields[1].read_only);
}

#[resource(sync = crud)]
pub struct WithNormal {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    pub count: i32,
}

#[test]
fn field_normal_role() {
    let d = WithNormal::descriptor();
    assert_eq!(d.fields.len(), 3);
    assert_eq!(d.fields[2].name, "count");
    assert!(matches!(d.fields[2].role, FieldRole::Normal));
    assert!(!d.fields[2].read_only);
}

#[test]
fn field_count_and_declaration_order() {
    let d = SimpleResource::descriptor();
    assert_eq!(d.fields.len(), 2);
    assert_eq!(d.fields[0].name, "id");
    assert_eq!(d.fields[1].name, "label");
}

// ── doc comments ─────────────────────────────────────────────────────────────

/// A well-documented resource describing itself.
#[resource(sync = crud)]
pub struct Documented {
    /// The server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// The user-visible display name.
    #[key]
    pub name: String,
}

#[test]
fn struct_doc_captured_in_descriptor() {
    assert_eq!(
        Documented::descriptor().doc,
        Some("A well-documented resource describing itself.")
    );
}

#[test]
fn field_doc_captured_in_descriptor() {
    let d = Documented::descriptor();
    assert_eq!(d.fields[0].doc, Some("The server-assigned id."));
    assert_eq!(d.fields[1].doc, Some("The user-visible display name."));
}

// ── wire attributes ───────────────────────────────────────────────────────────

#[resource(sync = crud)]
pub struct Wired {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    /// Overrides the wire key for irregular API spellings.
    #[wire(name = "configContract")]
    pub config_contract: String,
    /// API-computed field — present on read, skipped on write.
    #[wire(read_only)]
    pub implementation_name: String,
}

#[test]
fn wire_name_override() {
    let d = Wired::descriptor();
    let f = d
        .fields
        .iter()
        .find(|f| f.name == "config_contract")
        .unwrap();
    assert_eq!(f.wire_name, Some("configContract"));
    assert!(!f.read_only); // wire_name alone does not imply read_only
}

#[test]
fn wire_read_only_flag() {
    let d = Wired::descriptor();
    let f = d
        .fields
        .iter()
        .find(|f| f.name == "implementation_name")
        .unwrap();
    assert!(f.read_only);
    assert_eq!(f.wire_name, None); // no name override on this one
}

// ── codec selection ───────────────────────────────────────────────────────────

#[resource(sync = crud)]
#[codec(
    FieldsBlob,
    implementation = "QBittorrent",
    config_contract = "QBittorrentSettings",
    protocol = "torrent"
)]
pub struct QBittorrentConfig {
    #[key]
    pub name: String,
}

#[test]
fn codec_fields_blob_kind_and_meta() {
    let d = QBittorrentConfig::descriptor();
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
        _ => panic!("expected CodecMeta::FieldsBlob, got something else"),
    }
}

#[resource(sync = crud)]
#[codec(TaggedByImpl, discriminator = "implementation")]
pub enum ProviderEnum {}

#[test]
fn codec_tagged_by_impl_kind_and_meta() {
    let d = ProviderEnum::descriptor();
    assert!(matches!(d.codec, CodecKind::TaggedByImpl));
    assert_eq!(d.type_name, "provider_enum");
    match &d.codec_meta {
        CodecMeta::TaggedByImpl { discriminator } => {
            assert_eq!(*discriminator, "implementation");
        }
        _ => panic!("expected CodecMeta::TaggedByImpl"),
    }
    // Enum variants carry no fields on the parent descriptor.
    assert_eq!(d.fields.len(), 0);
}

#[resource(sync = crud)]
pub struct StandardResource {
    #[key]
    pub name: String,
}

#[test]
fn codec_default_is_standard() {
    let d = StandardResource::descriptor();
    assert!(matches!(d.codec, CodecKind::Standard));
    assert!(matches!(d.codec_meta, CodecMeta::Standard));
}

// ── FieldKind classification ──────────────────────────────────────────────────

/// Resource with one field per structural shape, so we can assert each FieldKind.
#[resource(sync = crud)]
pub struct AllKinds {
    /// Option<i32> — server id, never sent on write.
    #[id]
    pub id: Option<i32>,
    /// Plain bool.
    pub flag: bool,
    /// Vec<String> — basic collection.
    pub tags: Vec<String>,
    /// Plain String — env/file template resolved before decode.
    pub tmpl: String,
    /// SecretValue — credential field; kind is String, secret flag is true.
    pub secret: SecretValue,
    /// Plain i32 with reference metadata — resolved from ${ref.other.<key>}.
    #[reference(other)]
    pub target: i32,
    /// Vec<i32> with reference metadata — multi-valued ref.
    #[reference(other)]
    pub targets: Vec<i32>,
    /// Option<i32> with reference metadata — nullable ref.
    #[reference(other)]
    pub opt_target: Option<i32>,
    /// String FK naming two targets — a polymorphic ref, resolved from either
    /// ${ref.other.<key>} or ${ref.second.<key>}.
    #[reference(other, second)]
    pub poly_target: Option<String>,
}

fn field_kind_of(name: &str) -> &'static FieldKind {
    AllKinds::descriptor()
        .fields
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("no field named `{name}` on AllKinds"))
        .kind
}

#[test]
fn field_kind_opt_int32() {
    // Option<i32> — optional scalar
    match field_kind_of("id") {
        FieldKind::Optional(inner) => assert!(matches!(**inner, FieldKind::Int32)),
        other => panic!("expected Optional(Int32), got {other:?}"),
    }
}

#[test]
fn field_kind_bool() {
    assert!(matches!(*field_kind_of("flag"), FieldKind::Bool));
}

#[test]
fn field_kind_vec_string() {
    match field_kind_of("tags") {
        FieldKind::Vec(inner) => assert!(matches!(**inner, FieldKind::String)),
        other => panic!("expected Vec(String), got {other:?}"),
    }
}

#[test]
fn field_kind_interpolated_string() {
    // tmpl is now a plain String — interpolation is resolved on the Value tree before decode
    assert!(matches!(*field_kind_of("tmpl"), FieldKind::String));
}

#[test]
fn field_kind_secret() {
    // SecretValue fields have kind = String, distinguished by the secret flag on the descriptor
    assert!(matches!(*field_kind_of("secret"), FieldKind::String));
    let f = AllKinds::descriptor()
        .fields
        .iter()
        .find(|f| f.name == "secret")
        .unwrap();
    assert!(f.secret);
}

#[test]
fn field_reference_accepts_multiple_targets() {
    // A polymorphic FK (one field, several possible target types) records every
    // target, so `apply_order` gets an edge to each of them.
    let f = AllKinds::descriptor()
        .fields
        .iter()
        .find(|f| f.name == "poly_target")
        .unwrap();
    assert_eq!(f.reference, &["other", "second"]);
}

#[test]
fn field_reference_empty_when_absent() {
    let f = AllKinds::descriptor()
        .fields
        .iter()
        .find(|f| f.name == "secret")
        .unwrap();
    assert!(f.reference.is_empty());
}

#[test]
fn field_kind_ref() {
    // target is a plain i32 with #[reference(other)] — kind is Int32, reference metadata holds the type
    assert!(matches!(*field_kind_of("target"), FieldKind::Int32));
    let f = AllKinds::descriptor()
        .fields
        .iter()
        .find(|f| f.name == "target")
        .unwrap();
    assert_eq!(f.reference, &["other"]);
}

#[test]
fn field_kind_vec_ref() {
    // targets is Vec<i32> with #[reference(other)] — kind is Vec(Int32), reference metadata holds the type
    match field_kind_of("targets") {
        FieldKind::Vec(inner) => assert!(matches!(**inner, FieldKind::Int32)),
        other => panic!("expected Vec(Int32), got {other:?}"),
    }
    let f = AllKinds::descriptor()
        .fields
        .iter()
        .find(|f| f.name == "targets")
        .unwrap();
    assert_eq!(f.reference, &["other"]);
}

#[test]
fn field_kind_opt_ref() {
    // opt_target is Option<i32> with #[reference(other)] — kind is Optional(Int32)
    match field_kind_of("opt_target") {
        FieldKind::Optional(inner) => assert!(matches!(**inner, FieldKind::Int32)),
        other => panic!("expected Optional(Int32), got {other:?}"),
    }
    let f = AllKinds::descriptor()
        .fields
        .iter()
        .find(|f| f.name == "opt_target")
        .unwrap();
    assert_eq!(f.reference, &["other"]);
}

// ── ResourceErased blanket impl ───────────────────────────────────────────────

#[test]
fn resource_erased_field_names_roles_and_scalar_values() {
    let val = AllKinds {
        id: Some(42),
        flag: true,
        tags: vec!["hd".to_string()],
        tmpl: "${env.FOO}".to_string(),
        secret: SecretValue::new("test-secret".to_string()),
        target: 0,
        targets: vec![99],
        opt_target: None,
        poly_target: None,
    };

    let erased = val.descriptor_erased();
    let fields: Vec<_> = erased.fields.iter.collect();

    // Total count matches the struct definition.
    assert_eq!(fields.len(), 9);

    // Declaration order is preserved.
    assert_eq!(fields[0].name, "id");
    assert_eq!(fields[1].name, "flag");
    assert_eq!(fields[2].name, "tags");
    assert_eq!(fields[3].name, "tmpl");
    assert_eq!(fields[4].name, "secret");
    assert_eq!(fields[5].name, "target");
    assert_eq!(fields[6].name, "targets");
    assert_eq!(fields[7].name, "opt_target");

    // Role and read_only for the id field.
    assert!(matches!(fields[0].role, FieldRole::Id));
    assert!(fields[0].read_only);

    // FieldRef variant for id: OptInt32 pointing at Some(42).
    if let FieldRef::OptInt32(v) = &fields[0].value {
        assert_eq!(**v, Some(42));
    } else {
        panic!("expected FieldRef::OptInt32 for `id`, got something else");
    }

    // FieldRef variant for flag: Bool pointing at true.
    if let FieldRef::Bool(v) = &fields[1].value {
        assert!(**v);
    } else {
        panic!("expected FieldRef::Bool for `flag`");
    }

    // target is a plain i32; ref metadata lives in the descriptor, not the FieldRef
    if let FieldRef::Int32(v) = &fields[5].value {
        assert_eq!(**v, 0);
    } else {
        panic!("expected FieldRef::Int32 for `target`");
    }
}

// ── Option<Vec<T>> ───────────────────────────────────────────────────────────

/// A list the user may omit entirely. Distinct from `Vec<T>`, which always
/// encodes — so an undeclared `Vec<T>` sends `[]` and clobbers whatever the
/// server holds.
#[resource(sync = crud, list = get("/api/v3/optvec"))]
struct OptVecResource {
    #[key]
    name: String,
    tags: Option<Vec<String>>,
    ids: Option<Vec<i64>>,
    /// The always-encoding counterpart, for contrast.
    always: Vec<String>,
}

#[test]
fn option_vec_classifies_as_optional_of_vec() {
    let by_name = |n: &str| {
        OptVecResource::descriptor()
            .fields
            .iter()
            .find(|f| f.name == n)
            .expect("field exists")
    };

    assert!(matches!(
        by_name("tags").kind,
        FieldKind::Optional(FieldKind::Vec(FieldKind::String)),
    ));
    assert!(matches!(
        by_name("ids").kind,
        FieldKind::Optional(FieldKind::Vec(FieldKind::Int64)),
    ));
    assert!(matches!(
        by_name("always").kind,
        FieldKind::Vec(FieldKind::String)
    ));
}

/// The reason the shape exists: `None` omits the key, an empty `Vec` doesn't.
#[test]
fn none_omits_the_key_but_an_empty_vec_still_encodes() {
    let omitted = core_lib::engine::encode(&OptVecResource {
        name: "n".into(),
        tags: None,
        ids: None,
        always: Vec::new(),
    })
    .expect("encodes");

    assert!(omitted.get("tags").is_none(), "None must omit the key");
    assert!(omitted.get("ids").is_none(), "None must omit the key");
    assert_eq!(
        omitted.get("always"),
        Some(&serde_json::json!([])),
        "a plain Vec always encodes — this is what Option<Vec<_>> exists to avoid"
    );

    let present = core_lib::engine::encode(&OptVecResource {
        name: "n".into(),
        tags: Some(Vec::new()),
        ids: Some(vec![7, 9]),
        always: vec!["a".into()],
    })
    .expect("encodes");

    assert_eq!(present.get("tags"), Some(&serde_json::json!([])));
    assert_eq!(present.get("ids"), Some(&serde_json::json!([7, 9])));
}

#[test]
fn option_vec_round_trips_through_the_wire_codec() {
    let wire = serde_json::json!({ "name": "n", "tags": ["x", "y"], "ids": [1], "always": [] });
    let decoded: OptVecResource = core_lib::engine::decode(&wire).expect("decodes");

    assert_eq!(
        decoded.tags.as_deref(),
        Some(["x".to_string(), "y".to_string()].as_slice())
    );
    assert_eq!(decoded.ids.as_deref(), Some([1i64].as_slice()));
    assert_eq!(
        core_lib::engine::encode(&decoded).expect("re-encodes"),
        wire
    );
}

/// Doc-gen already understood the nested kind; this pins that it keeps
/// reporting the element type rather than a bare "any".
#[test]
fn option_vec_documents_its_element_type_and_is_not_required() {
    let docs = core_lib::engine::field_docs::<OptVecResource>();
    let tags = docs.iter().find(|d| d.name == "tags").expect("documented");

    assert_eq!(tags.type_label, "array of string");
    assert!(!tags.required, "an optional list is never required");
}

// ── id shape ─────────────────────────────────────────────────────────────────

/// Every *arr: an integer `#[id]`.
#[resource(sync = crud, list = get("/api/v3/intid"))]
struct IntIdResource {
    #[id]
    id: Option<i32>,
    #[key]
    name: String,
}

/// Jellyfin/Komga/Audiobookshelf: an opaque string `#[id]`.
#[resource(sync = crud, list = get("/api/v1/strid"))]
struct StrIdResource {
    #[id]
    id: Option<String>,
    #[key]
    name: String,
}

/// No `#[id]` at all — a resource that never models the server's id.
#[resource(sync = crud, list = get("/api/v1/noid"))]
struct NoIdResource {
    #[key]
    name: String,
}

/// `id_shape` picks the placeholder type a preview substitutes for a
/// not-yet-created ref. Getting it wrong doesn't degrade gracefully: an integer
/// `-1` handed to a `String` FK fails decode and aborts the whole plan.
#[test]
fn id_shape_reads_the_id_field() {
    use core_lib::IdShape;
    use core_lib::engine::id_shape;

    assert_eq!(id_shape::<IntIdResource>(), IdShape::Int);
    assert_eq!(id_shape::<StrIdResource>(), IdShape::Str);
    // No `#[id]` falls back to the historical behaviour, which is right for
    // every *arr — hence the rule that a string-id resource must declare one.
    assert_eq!(id_shape::<NoIdResource>(), IdShape::Int);
}
