//! Standard codec — encode. First end-to-end use of a descriptor to produce
//! wire JSON. Validates the IR against real serialization.

use core_lib::{Described, SecretValue, engine};
use core_macros::resource;

#[resource(
    sync = crud,
    list = get("/api/v3/tag"),
    create = post("/api/v3/tag"),
    update = put("/api/v3/tag/${self.id}"),
    delete = delete("/api/v3/tag/${self.id}"),
)]
pub struct Tag {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub label: String,
}

#[test]
fn tag_encodes_without_read_only_id() {
    let t = Tag {
        id: Some(3),
        label: "4k".to_string(),
    };
    let v = engine::encode(&t).unwrap();
    // id is #[id] → read-only → omitted on write; label is the only key.
    assert_eq!(v, serde_json::json!({ "label": "4k" }));
}

#[test]
fn empty_is_type_defaults() {
    let t = Tag::empty();
    assert_eq!(t.id, None);
    assert_eq!(t.label, "");
}

#[test]
fn tag_decodes_from_api_json() {
    // Shape of testdata/tag/api.json — read-only id IS read on decode.
    let json = serde_json::json!({ "id": 3, "label": "4k" });
    let t: Tag = engine::decode(&json).unwrap();
    assert_eq!(t.id, Some(3));
    assert_eq!(t.label, "4k");
}

#[test]
fn tag_round_trips() {
    // decode an API payload → encode for a write → id dropped, value preserved.
    let api = serde_json::json!({ "id": 7, "label": "hd" });
    let t: Tag = engine::decode(&api).unwrap();
    let wire = engine::encode(&t).unwrap();
    assert_eq!(wire, serde_json::json!({ "label": "hd" }));
}

#[resource(
    sync = singleton,
    read = get("/api/v3/config"),
    update = put("/api/v3/config"),
)]
pub struct Cfg {
    #[id]
    pub id: Option<i32>,
    pub recycle_bin_cleanup_days: i32,
    pub chmod_folder: Option<String>,
    #[wire(name = "useSsl")]
    pub use_ssl: bool,
    pub api_key: SecretValue,
    #[wire(read_only)]
    pub computed: Option<String>,
}

#[test]
fn camelcase_skips_and_secret() {
    let c = Cfg {
        id: Some(1),
        recycle_bin_cleanup_days: 7,
        chmod_folder: None,                          // None → omitted
        use_ssl: true,                               // wire override
        api_key: SecretValue::new("hunter2".into()), // secret → plaintext at the wire
        computed: Some("x".into()),                  // read-only → omitted on write
    };
    let v = engine::encode(&c).unwrap();
    assert_eq!(
        v,
        serde_json::json!({
            "recycleBinCleanupDays": 7,
            "useSsl": true,
            "apiKey": "hunter2"
        })
    );
}

/// `case = snake` — the wire key is the field name verbatim (for APIs whose JSON
/// is snake_case, e.g. bazarr), not camelCased.
#[resource(sync = singleton, case = snake, read = get("/s"), update = put("/s"))]
pub struct SnakeCfg {
    pub use_sonarr: bool,
    pub minimum_score: i32,
}

#[test]
fn snake_case_keeps_field_names_verbatim() {
    let c = SnakeCfg {
        use_sonarr: true,
        minimum_score: 90,
    };
    let v = engine::encode(&c).unwrap();
    assert_eq!(
        v,
        serde_json::json!({ "use_sonarr": true, "minimum_score": 90 })
    );
}

/// `#[wire(null)]` keeps a `None` optional as an explicit null; `#[wire(int)]`
/// renders a bool as `0`/`1`. Both compose (bazarr's `originalFormat`).
#[resource(sync = singleton, read = get("/w"), update = put("/w"))]
pub struct WireOpts {
    #[wire(null)]
    pub note: Option<String>,
    #[wire(int)]
    pub flag: bool,
    #[wire(null, int)]
    pub tri: Option<bool>,
}

#[test]
fn wire_null_and_int_encode() {
    let none = WireOpts {
        note: None,
        flag: false,
        tri: None,
    };
    assert_eq!(
        engine::encode(&none).unwrap(),
        serde_json::json!({ "note": null, "flag": 0, "tri": null })
    );

    let some = WireOpts {
        note: Some("x".into()),
        flag: true,
        tri: Some(false),
    };
    assert_eq!(
        engine::encode(&some).unwrap(),
        serde_json::json!({ "note": "x", "flag": 1, "tri": 0 })
    );
}

#[test]
fn wire_int_decodes_back_to_bool() {
    // The wire carries `0`/`1` ints for the bool fields; decode maps them back.
    let wire = serde_json::json!({ "note": null, "flag": 1, "tri": 0 });
    let d: WireOpts = engine::decode(&wire).unwrap();
    assert_eq!(d.note, None);
    assert!(d.flag);
    assert_eq!(d.tri, Some(false));
}
