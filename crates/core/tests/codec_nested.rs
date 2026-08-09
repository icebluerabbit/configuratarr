//! `Vec<Nested>` and `Option<Nested>` round-trips — the paths that used to be
//! stubbed (`VecNested` encoded as `[]`, both `set`s bailed). A `quality_profile`
//! with `items: Vec<QualityProfileItem>` was silently dropping its content.

use core_lib::engine;
use core_macros::{nested, resource};
use serde_json::json;

#[nested]
pub struct Item {
    pub label: String,
    pub score: Option<i32>,
}

#[resource(
    sync = crud,
    list = get("/api/v3/profile"),
    create = post("/api/v3/profile"),
    update = put("/api/v3/profile/${self.id}"),
    delete = delete("/api/v3/profile/${self.id}"),
)]
pub struct Profile {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    pub primary: Option<Item>,
    pub items: Vec<Item>,
}

#[test]
fn vec_and_option_nested_wire_roundtrip() {
    let wire = json!({
        "id": 1,
        "name": "p",
        "primary": { "label": "a", "score": 5 },
        "items": [ { "label": "x" }, { "label": "y", "score": 9 } ]
    });

    let p: Profile = engine::decode(&wire).unwrap();
    // The vec actually decoded its elements (not empty, the old bug).
    assert_eq!(p.items.len(), 2);
    assert_eq!(p.items[1].label, "y");
    assert_eq!(p.items[1].score, Some(9));
    assert_eq!(p.primary.as_ref().unwrap().label, "a");

    let out = engine::encode(&p).unwrap();
    assert_eq!(
        out["items"].as_array().unwrap().len(),
        2,
        "vec must encode its elements, not []"
    );
    assert_eq!(out["items"][0]["label"], "x");
    assert_eq!(out["items"][1]["score"], 9);
    assert_eq!(out["primary"]["label"], "a");
}

#[test]
fn vec_and_option_nested_config_decode() {
    // Config (snake) decodes the nested elements through the config codec.
    let cfg = json!({
        "name": "p",
        "primary": { "label": "a" },
        "items": [ { "label": "x", "score": 3 } ]
    });

    let p: Profile = engine::decode_config(&cfg).unwrap();
    assert_eq!(p.items.len(), 1);
    assert_eq!(p.items[0].score, Some(3));
    assert_eq!(p.primary.as_ref().unwrap().label, "a");

    let out = engine::encode(&p).unwrap();
    assert_eq!(out["items"][0]["score"], 3);
}

#[test]
fn empty_and_absent_nested() {
    // Absent optional → None; empty vec → [].
    let p: Profile = engine::decode(&json!({ "name": "p", "items": [] })).unwrap();
    assert!(p.primary.is_none());
    assert!(p.items.is_empty());
    let out = engine::encode(&p).unwrap();
    assert_eq!(out["items"], json!([]));
    assert!(
        out.get("primary").is_none(),
        "None optional-nested is omitted"
    );
}
