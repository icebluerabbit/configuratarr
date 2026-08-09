//! `application/x-www-form-urlencoded` **write** serialization.
//!
//! A structural wire-`Value` helper, not a codec — like [`crate::merge`], it
//! lives outside `codec/` because the engine never dispatches to it: a
//! [`CodecKind`](crate::CodecKind) governs both encode *and* decode over one wire
//! shape, but form-write APIs (e.g. bazarr) typically **read JSON and write a
//! form** — the two directions differ. So this is a write-only helper, driven by
//! a `sync = custom` reconcile hook and paired with
//! [`HttpClient::post_form`](core_http::HttpClient::post_form).
//!
//! [`flatten`] turns a (already-encoded, e.g. via [`crate::engine::encode`]) wire
//! object into the flat `prefix-key-subkey` pairs a form endpoint wants: nested
//! objects are flattened by joining keys with `-`, and a list emits one pair per
//! element (form list semantics; an empty list emits a single empty value so the
//! server stores the cleared list).

use serde_json::Value;

/// One JSON scalar as its form-value string. The common convention: booleans as
/// `"true"`/`"false"`, numbers as their decimal text, strings verbatim.
fn scalar(v: &Value) -> String {
    match v {
        Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        // null / nested containers are handled by `flatten`, never reach here.
        _ => String::new(),
    }
}

/// Flatten a wire object into `prefix`-rooted form pairs, joining nested keys
/// with `-`. Scalars render via the common convention (bool → `"true"`/`"false"`,
/// numbers decimal, strings verbatim — a service wanting a different rendering
/// pre-renders the value to a string first); a list emits one pair per element
/// (empty list → one empty value). Objects nested inside arrays are not supported
/// (encode those to a JSON-string field instead).
///
/// `flatten(&json!({"general": {"use_sonarr": true}}), "settings")`
/// → `[("settings-general-use_sonarr", "true")]`.
pub fn flatten(value: &Value, prefix: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    push(&mut out, prefix, value);
    out
}

fn push(out: &mut Vec<(String, String)>, key: &str, value: &Value) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let child = if key.is_empty() {
                    k.clone()
                } else {
                    format!("{key}-{k}")
                };
                push(out, &child, v);
            }
        }
        Value::Array(items) if items.is_empty() => {
            out.push((key.to_string(), String::new()));
        }
        Value::Array(items) => {
            for item in items {
                out.push((key.to_string(), scalar(item)));
            }
        }
        Value::Null => {}
        scalar_val => out.push((key.to_string(), scalar(scalar_val))),
    }
}
