//! `#[fields_map]` — author the open *arr provider settings as a plain map
//! (`fields: { baseUrl: ... }`) while the wire form stays the API's
//! `fields: [{ name, value }]` blob. Covers splay-on-encode, collect-on-decode,
//! the config (snake/map) ingest path, value-type fidelity, and the flattened
//! provider envelope it's actually used through.

use core_lib::{Json, engine};
use core_macros::{nested, resource};
use serde_json::json;

/// Shared identity envelope, flattened into the parent (id + natural key).
#[nested]
pub struct Common {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

/// A raw provider whose open settings live in a `#[fields_map]` object.
#[nested]
pub struct RawCfg {
    pub implementation: String,
    #[wire(name = "configContract")]
    pub config_contract: Option<String>,
    #[fields_map]
    pub fields: Json,
}

/// Indexer-shaped resource: identity envelope + raw provider config, both
/// flattened, plus an own scalar.
#[resource(
    sync = crud,
    list = get("/api/v1/indexer"),
    create = post("/api/v1/indexer"),
    update = put("/api/v1/indexer/${self.id}"),
    delete = delete("/api/v1/indexer/${self.id}"),
)]
pub struct Indexer {
    #[flatten]
    pub common: Common,
    #[flatten]
    pub config: RawCfg,
    pub enable: bool,
}

fn sample() -> Indexer {
    Indexer {
        common: Common {
            id: Some(7),
            name: "NZBgeek".into(),
        },
        config: RawCfg {
            implementation: "Newznab".into(),
            config_contract: Some("NewznabSettings".into()),
            fields: json!({
                "baseUrl": "https://api.nzbgeek.info",
                "apiKey": "deadbeef",
                "categories": [5000, 5070],
            }),
        },
        enable: true,
    }
}

#[test]
fn encode_splays_map_into_blob_array() {
    let wire = engine::encode(&sample()).unwrap();
    // Top-level (flatten): name, implementation, configContract, enable + the
    // `fields` blob as a [{name,value}] array (id is read-only, omitted).
    assert_eq!(wire["name"], json!("NZBgeek"));
    assert_eq!(wire["implementation"], json!("Newznab"));
    assert_eq!(wire["configContract"], json!("NewznabSettings"));
    assert_eq!(wire["enable"], json!(true));

    let fields = wire["fields"].as_array().expect("fields is an array");
    assert_eq!(fields.len(), 3, "one entry per map key: {fields:?}");
    // Order-independent: each map entry became a {name, value} object,
    // preserving the value's JSON type (string / array).
    let by_name: std::collections::HashMap<&str, &serde_json::Value> = fields
        .iter()
        .map(|e| (e["name"].as_str().unwrap(), &e["value"]))
        .collect();
    assert_eq!(by_name["baseUrl"], &json!("https://api.nzbgeek.info"));
    assert_eq!(by_name["apiKey"], &json!("deadbeef"));
    assert_eq!(by_name["categories"], &json!([5000, 5070]));
}

#[test]
fn decode_collects_blob_into_map() {
    let wire = json!({
        "id": 7,
        "name": "NZBgeek",
        "implementation": "Newznab",
        "configContract": "NewznabSettings",
        "enable": true,
        "fields": [
            { "name": "baseUrl", "value": "https://api.nzbgeek.info", "type": "textbox" },
            { "name": "apiKey", "value": "deadbeef" },
            { "name": "categories", "value": [5000, 5070] },
        ],
    });
    let idx: Indexer = engine::decode(&wire).unwrap();
    assert_eq!(idx.common.id, Some(7));
    assert_eq!(idx.config.implementation, "Newznab");
    // The [{name,value}] blob collapses back into a {name: value} map — the
    // extra per-entry metadata (`type`) is dropped.
    assert_eq!(
        idx.config.fields,
        json!({
            "baseUrl": "https://api.nzbgeek.info",
            "apiKey": "deadbeef",
            "categories": [5000, 5070],
        })
    );
}

#[test]
fn round_trip_map_to_blob_to_map() {
    let original = sample();
    let wire = engine::encode(&original).unwrap();
    let back: Indexer = engine::decode(&wire).unwrap();
    assert_eq!(back.config.fields, original.config.fields);
    assert_eq!(back.config.implementation, original.config.implementation);
}

#[test]
fn config_ingest_accepts_yaml_map_then_encodes_to_blob() {
    // What a user writes (snake_case `field.name` keys; `fields` is a plain map).
    let cfg = json!({
        "name": "NZBgeek",
        "implementation": "Newznab",
        "config_contract": "NewznabSettings",
        "enable": true,
        "fields": {
            "baseUrl": "https://api.nzbgeek.info",
            "apiKey": "deadbeef",
            "categories": [5000, 5070],
        },
    });
    let idx: Indexer = engine::decode_config(&cfg).unwrap();
    assert_eq!(
        idx.config.fields["baseUrl"],
        json!("https://api.nzbgeek.info")
    );

    // …and it renders to the API blob form on the wire.
    let wire = engine::encode(&idx).unwrap();
    let fields = wire["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 3);
    assert!(
        fields
            .iter()
            .any(|e| e["name"] == json!("apiKey") && e["value"] == json!("deadbeef"))
    );
}

#[test]
fn empty_or_absent_blob_is_an_empty_map() {
    // No `fields` key on the wire → empty map, not an error.
    let wire = json!({
        "id": 1, "name": "x", "implementation": "Newznab", "enable": false,
    });
    let idx: Indexer = engine::decode(&wire).unwrap();
    assert_eq!(idx.config.fields, json!({}));

    // Empty map → empty array on encode.
    let mut e = sample();
    e.config.fields = json!({});
    let w = engine::encode(&e).unwrap();
    assert_eq!(w["fields"], json!([]));
}

#[test]
fn null_blob_encodes_as_empty_array() {
    let mut e = sample();
    e.config.fields = serde_json::Value::Null;
    let w = engine::encode(&e).unwrap();
    assert_eq!(w["fields"], json!([]), "null map → [] not an error");
}
