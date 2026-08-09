//! Sparse-update merge: live base, desired overlay, `fields[]` by name.

use core_lib::merge::merge;
use serde_json::json;

#[test]
fn desired_overrides_scalar_keeps_server_fields() {
    let live = json!({ "id": 7, "name": "old", "serverOnly": "keep" });
    let desired = json!({ "name": "new" });
    // desired changes name; id + serverOnly (unmodeled) survive; nothing dropped.
    assert_eq!(
        merge(&live, &desired),
        json!({ "id": 7, "name": "new", "serverOnly": "keep" })
    );
}

#[test]
fn omitted_field_keeps_live_value() {
    let live = json!({ "name": "x", "enable": true });
    let desired = json!({ "name": "x" }); // enable omitted → unmanaged
    assert_eq!(merge(&live, &desired)["enable"], json!(true));
}

#[test]
fn present_empty_clears() {
    let live = json!({ "name": "x", "tags": [1, 2] });
    let desired = json!({ "name": "x", "tags": [] }); // explicit clear
    assert_eq!(merge(&live, &desired)["tags"], json!([]));
}

#[test]
fn non_fields_array_replaced_wholesale() {
    let live = json!({ "tags": [1, 2, 3] });
    let desired = json!({ "tags": [9] });
    assert_eq!(merge(&live, &desired)["tags"], json!([9]));
}

#[test]
fn fields_array_merges_by_name() {
    // Provider blob: server knows `extra` we don't model; desired overrides
    // `host`, omits `extra`. `extra` must survive, `host` updates.
    let live = json!({
        "implementation": "QBittorrent",
        "fields": [
            { "name": "host", "value": "old.local" },
            { "name": "port", "value": 8080 },
            { "name": "extra", "value": "server-default" }
        ]
    });
    let desired = json!({
        "implementation": "QBittorrent",
        "fields": [
            { "name": "host", "value": "new.local" },
            { "name": "port", "value": 8080 }
        ]
    });
    let m = merge(&live, &desired);
    assert_eq!(
        m["fields"],
        json!([
            { "name": "host", "value": "new.local" },   // overridden
            { "name": "port", "value": 8080 },
            { "name": "extra", "value": "server-default" } // preserved
        ])
    );
}

#[test]
fn fields_array_appends_new_desired_entry() {
    let live = json!({ "fields": [ { "name": "host", "value": "h" } ] });
    let desired = json!({ "fields": [
        { "name": "host", "value": "h" },
        { "name": "newKey", "value": "v" }
    ] });
    assert_eq!(
        merge(&live, &desired)["fields"],
        json!([
            { "name": "host", "value": "h" },
            { "name": "newKey", "value": "v" }
        ])
    );
}

#[test]
fn nested_object_recurses() {
    let live = json!({ "cfg": { "a": 1, "b": 2 } });
    let desired = json!({ "cfg": { "b": 3 } });
    // recurse: a (server) survives, b overridden
    assert_eq!(merge(&live, &desired)["cfg"], json!({ "a": 1, "b": 3 }));
}
