//! *arr fields-blob codec for provider/specification variant structs.
//!
//! Wire shape: `{ "implementation": "...", "configContract": "...",
//! "fields": [ { "name": "host", "value": ... }, ... ] }`. Each typed Rust
//! field becomes one `{name, value}` entry, keyed by its `wire_name` override
//! or snake_case identifier (fields-blob does NOT camelCase). The
//! `implementation` / `configContract` discriminators come from the
//! descriptor's [`CodecMeta::FieldsBlob`].

use serde_json::{Map, Value};

use crate::codec::standard;
use crate::described::Described;
use crate::descriptor::CodecMeta;

pub fn encode<T: Described>(value: &T) -> anyhow::Result<Value> {
    let desc = T::descriptor();
    let CodecMeta::FieldsBlob {
        implementation,
        config_contract,
        ..
    } = &desc.codec_meta
    else {
        anyhow::bail!("fields_blob::encode on a non-FieldsBlob descriptor");
    };

    let mut fields = Vec::new();
    for f in desc.fields {
        if f.read_only {
            continue;
        }
        if let Some(v) = standard::field_to_json(&(f.get)(value))? {
            let mut entry = Map::new();
            entry.insert(
                "name".to_string(),
                Value::String(standard::wire_key(f.name, f.wire_name, desc.case)),
            );
            entry.insert("value".to_string(), v);
            fields.push(Value::Object(entry));
        }
    }

    let mut obj = Map::new();
    obj.insert(
        "implementation".to_string(),
        Value::String((*implementation).to_string()),
    );
    if let Some(c) = config_contract {
        obj.insert(
            "configContract".to_string(),
            Value::String((*c).to_string()),
        );
    }
    obj.insert("fields".to_string(), Value::Array(fields));
    Ok(Value::Object(obj))
}

pub fn decode<T: Described>(value: &Value) -> anyhow::Result<T> {
    // Index the {name, value} array by name.
    let mut by_name: std::collections::HashMap<&str, &Value> = std::collections::HashMap::new();
    if let Some(arr) = value.get("fields").and_then(Value::as_array) {
        for item in arr {
            if let (Some(name), Some(val)) =
                (item.get("name").and_then(Value::as_str), item.get("value"))
            {
                by_name.insert(name, val);
            }
        }
    }

    let mut out = T::empty();
    let desc = T::descriptor();
    for f in desc.fields {
        let key = standard::wire_key(f.name, f.wire_name, desc.case);
        let Some(jv) = by_name.get(key.as_str()) else {
            continue;
        };
        if jv.is_null() {
            continue;
        }
        let fv = standard::json_to_field_value(f.kind, f.secret, jv)
            .map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
        (f.set)(&mut out, fv).map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
    }
    Ok(out)
}
