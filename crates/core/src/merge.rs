//! Sparse-update merge — the body to PUT back to the API.
//!
//! REST wants a full object on update, but desired-state omits server-managed
//! fields. So the body is `merge(live, desired)`: live base, desired wins. An
//! omitted key keeps its live value; an explicit `[]`/`""` still clears it.
//!
//! Structural, on the wire `Value`: objects overlay key-by-key; the provider
//! `fields: [{name, value}]` array merges **by name** (live-only entries kept —
//! the server fills fields we don't model). The `fields` rule is the one
//! *arr-ism; it lives here because the blob is flattened into a Standard
//! resource that doesn't know it carries it.

use serde_json::{Map, Value};

/// The wire key of the provider configuration blob across the *arr family.
const FIELDS_KEY: &str = "fields";

/// Merge `desired` over `live`, producing the body to write back.
pub fn merge(live: &Value, desired: &Value) -> Value {
    match (live, desired) {
        (Value::Object(l), Value::Object(d)) => Value::Object(merge_objects(l, d)),
        // scalars and non-`fields` arrays: desired is authoritative.
        _ => desired.clone(),
    }
}

fn merge_objects(live: &Map<String, Value>, desired: &Map<String, Value>) -> Map<String, Value> {
    let mut out = live.clone();
    for (k, dv) in desired {
        let merged = match live.get(k) {
            Some(lv) if k == FIELDS_KEY => merge_fields(lv, dv),
            Some(lv) => merge(lv, dv),
            None => dv.clone(),
        };
        out.insert(k.clone(), merged);
    }
    out
}

/// Merge two `[{name, value}, ...]` arrays by `name`: live order preserved,
/// desired entries override matching names, desired-only names appended,
/// live-only names kept. Falls back to "desired wins" if either side isn't the
/// expected shape.
fn merge_fields(live: &Value, desired: &Value) -> Value {
    let (Some(live_arr), Some(desired_arr)) = (live.as_array(), desired.as_array()) else {
        return desired.clone();
    };

    let name_of = |v: &Value| v.get("name").and_then(Value::as_str).map(str::to_string);

    let mut out: Vec<Value> = Vec::with_capacity(live_arr.len() + desired_arr.len());
    let mut seen: Vec<String> = Vec::new();

    for lv in live_arr {
        match name_of(lv) {
            Some(name) => {
                // desired override for this name, if present
                let chosen = desired_arr
                    .iter()
                    .find(|dv| name_of(dv).as_deref() == Some(name.as_str()))
                    .unwrap_or(lv);
                out.push(chosen.clone());
                seen.push(name);
            }
            None => out.push(lv.clone()),
        }
    }
    // desired-only names, in desired order
    for dv in desired_arr {
        match name_of(dv) {
            Some(name) if !seen.contains(&name) => out.push(dv.clone()),
            _ => {}
        }
    }
    Value::Array(out)
}
