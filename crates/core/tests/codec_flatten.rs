//! A provider-shaped resource: a flattened identity envelope + own fields + a
//! secret. Exercises flatten hoisting, key-through-flatten, nested decode —
//! none of which `Tag` touches.

use core_lib::{SecretValue, engine};
use core_macros::{nested, resource};

/// Shared identity envelope, flattened into the parent on the wire.
#[nested]
pub struct Common {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

#[resource(
    sync = crud,
    list = get("/api/v3/downloadclient"),
    create = post("/api/v3/downloadclient"),
    update = put("/api/v3/downloadclient/${self.id}"),
    delete = delete("/api/v3/downloadclient/${self.id}"),
)]
pub struct Thing {
    #[flatten]
    pub common: Common,
    pub enable: bool,
    pub api_key: SecretValue,
}

#[test]
fn flatten_hoists_to_parent_on_encode() {
    let t = Thing {
        common: Common {
            id: Some(5),
            name: "qbit".into(),
        },
        enable: true,
        api_key: SecretValue::new("k".into()),
    };
    let v = engine::encode(&t).unwrap();
    // name/enable/apiKey are TOP-LEVEL — no "common" wrapper; id read-only.
    assert_eq!(
        v,
        serde_json::json!({ "name": "qbit", "enable": true, "apiKey": "k" })
    );
}

#[test]
fn flatten_decodes_from_parent() {
    let api = serde_json::json!({ "id": 5, "name": "qbit", "enable": true, "apiKey": "k" });
    let t: Thing = engine::decode(&api).unwrap();
    assert_eq!(t.common.id, Some(5));
    assert_eq!(t.common.name, "qbit");
    assert!(t.enable);
    assert_eq!(t.api_key.expose(), "k");
}

#[test]
fn key_is_discoverable_through_flatten() {
    // The engine must find the natural key even though it lives in the envelope.
    assert_eq!(engine::key_wire_name::<Thing>(), Some("name".to_string()));
}
