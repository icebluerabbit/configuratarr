//! Standard JSON object codec.
//!
//! snake_case Rust field name → camelCase wire key by default; `wire_name`
//! overrides it. Read-only fields (`#[id]`, `#[wire(read_only)]`) are skipped on
//! encode. `Option::None` is omitted (unless `#[wire(null)]`, which emits an
//! explicit null); `#[wire(int)]` renders a bool as `0`/`1`. Secrets are exposed
//! to their plaintext string at this boundary. Nested resources recurse via the
//! erased view.

use serde_json::{Map, Number, Value};

use crate::described::Described;
use crate::descriptor::{Case, FieldDescriptor};
use crate::field::FieldRef;

/// Encode a resource to its wire JSON object.
pub fn encode<T: Described>(value: &T) -> anyhow::Result<Value> {
    let desc = T::descriptor();
    let mut map = Map::new();
    for f in desc.fields {
        emit_field(&mut map, f, (f.get)(value), desc.case)?;
    }
    Ok(Value::Object(map))
}

/// Write one field into the parent object. Read-only fields are skipped;
/// `#[flatten]` nested fields have their keys hoisted into the parent rather
/// than nested under their own key.
fn emit_field<T: 'static>(
    map: &mut Map<String, Value>,
    f: &FieldDescriptor<T>,
    r: FieldRef<'_>,
    case: Case,
) -> anyhow::Result<()> {
    if f.read_only {
        return Ok(());
    }
    // `#[fields_map]`: a `{name: value}` object rendered as the *arr
    // `[{name, value}]` provider blob under this field's wire key.
    if f.fields_map {
        let arr = match &r {
            FieldRef::Json(v) => fields_map_to_array(v),
            FieldRef::OptJson(o) => match o.as_ref() {
                Some(v) => fields_map_to_array(v),
                None => return Ok(()),
            },
            _ => anyhow::bail!("#[fields_map] requires a Json field on `{}`", f.name),
        };
        map.insert(wire_key(f.name, f.wire_name, case), Value::Array(arr));
        return Ok(());
    }
    if f.flatten
        && let FieldRef::Nested(n) = &r
    {
        if let Value::Object(m) = n.encode_self()? {
            map.extend(m);
        }
        return Ok(());
    }
    // `VecNested` owns a boxed iterator → encode it here, by value; everything
    // else goes through the borrowing `field_to_json`.
    match r {
        FieldRef::VecNested(it) => {
            let mut arr = Vec::new();
            for n in it {
                arr.push(n.encode_self()?);
            }
            map.insert(wire_key(f.name, f.wire_name, case), Value::Array(arr));
        }
        other => match field_to_json(&other)? {
            Some(v) => {
                map.insert(
                    wire_key(f.name, f.wire_name, case),
                    coerce_wire(v, f.as_int),
                );
            }
            // `#[wire(null)]`: keep the key as an explicit null instead of
            // dropping it (a `None` optional the API wants present).
            None if f.emit_none => {
                map.insert(wire_key(f.name, f.wire_name, case), Value::Null);
            }
            None => {}
        },
    }
    Ok(())
}

/// Apply `#[wire(int)]`: a bool renders as the integer `0`/`1`. Anything else is
/// unchanged.
fn coerce_wire(v: Value, as_int: bool) -> Value {
    match (as_int, &v) {
        (true, Value::Bool(b)) => Value::Number(i64::from(*b).into()),
        _ => v,
    }
}

/// Convert one borrowed field value to JSON. `Ok(None)` means "omit this key"
/// (a `None` optional).
pub(crate) fn field_to_json(r: &FieldRef<'_>) -> anyhow::Result<Option<Value>> {
    let v = match r {
        FieldRef::Bool(b) => Value::Bool(**b),
        FieldRef::Int32(i) => Value::Number((**i).into()),
        FieldRef::Int64(i) => Value::Number((**i).into()),
        FieldRef::Float64(f) => num_f64(**f)?,
        FieldRef::String(s) => Value::String((*s).clone()),

        FieldRef::OptBool(o) => return Ok((**o).map(Value::Bool)),
        FieldRef::OptInt32(o) => return Ok((**o).map(|i| Value::Number(i.into()))),
        FieldRef::OptInt64(o) => return Ok((**o).map(|i| Value::Number(i.into()))),
        FieldRef::OptFloat64(o) => return (**o).map(num_f64).transpose(),
        FieldRef::OptString(o) => return Ok((*o).clone().map(Value::String)),

        FieldRef::VecBool(s) => Value::Array(s.iter().map(|b| Value::Bool(*b)).collect()),
        FieldRef::VecInt32(s) => {
            Value::Array(s.iter().map(|i| Value::Number((*i).into())).collect())
        }
        FieldRef::VecInt64(s) => {
            Value::Array(s.iter().map(|i| Value::Number((*i).into())).collect())
        }
        FieldRef::VecString(s) => Value::Array(s.iter().cloned().map(Value::String).collect()),

        FieldRef::Secret(sv) => Value::String(sv.expose().to_string()),
        FieldRef::OptSecret(o) => {
            return Ok((**o)
                .as_ref()
                .map(|sv| Value::String(sv.expose().to_string())));
        }

        FieldRef::Nested(n) => n.encode_self()?,
        // `VecNested` owns its iterator and is consumed by the standard codec's
        // `emit_field`; it never reaches the borrowing `field_to_json`.
        FieldRef::VecNested(_) => {
            anyhow::bail!(
                "vec-nested fields are encoded by the standard codec, not the fields-blob"
            )
        }

        FieldRef::Json(v) => (**v).clone(),
        FieldRef::OptJson(o) => return Ok((**o).clone()),
        FieldRef::VecJson(s) => Value::Array(s.to_vec()),
    };
    Ok(Some(v))
}

/// Splay a `{name: value}` object into the *arr `[{name, value}]` blob shape.
/// A non-object (e.g. null) yields an empty array.
fn fields_map_to_array(v: &Value) -> Vec<Value> {
    match v.as_object() {
        Some(obj) => obj
            .iter()
            .map(|(k, val)| {
                let mut e = Map::new();
                e.insert("name".to_string(), Value::String(k.clone()));
                e.insert("value".to_string(), val.clone());
                Value::Object(e)
            })
            .collect(),
        None => Vec::new(),
    }
}

/// Collect the *arr `[{name, value}]` blob back into a `{name: value}` object.
/// Entries missing `name`/`value` are skipped.
fn fields_array_to_map(arr: &[Value]) -> Value {
    let mut obj = Map::new();
    for item in arr {
        if let (Some(name), Some(val)) =
            (item.get("name").and_then(Value::as_str), item.get("value"))
        {
            obj.insert(name.to_string(), val.clone());
        }
    }
    Value::Object(obj)
}

fn num_f64(x: f64) -> anyhow::Result<Value> {
    Number::from_f64(x)
        .map(Value::Number)
        .ok_or_else(|| anyhow::anyhow!("non-finite float cannot be encoded as JSON: {x}"))
}

/// Wire key: explicit `wire_name` override, else the field name cased per the
/// resource's [`Case`] — camelCase (default), PascalCase (.NET-style APIs), or
/// Snake (identity — the field name is already the wire key).
pub(crate) fn wire_key(name: &str, wire_name: Option<&str>, case: Case) -> String {
    match wire_name {
        Some(w) => w.to_string(),
        None => match case {
            Case::Camel => to_camel_case(name),
            Case::Pascal => to_pascal_case(name),
            // Identity: the wire key is the snake field name (bazarr).
            Case::Snake => name.to_string(),
        },
    }
}

fn to_camel_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = false;
    for c in s.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.extend(c.to_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// PascalCase = camelCase with the first character upper-cased. Single casing
/// base, so both spellings stay in sync.
fn to_pascal_case(s: &str) -> String {
    let camel = to_camel_case(s);
    let mut chars = camel.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => camel,
    }
}

/// Decode a resource from its wire JSON object.
///
/// Builds an empty instance, then populates each present (non-null) field via
/// its `set` closure. `#[default]` is deliberately not applied — this decodes
/// the server's full wire object; default-filling is the config codec's job
/// ([`super::config`]), on ingest.
pub fn decode<T: Described>(value: &Value) -> anyhow::Result<T> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("standard::decode expected a JSON object"))?;

    let mut out = T::empty();
    let desc = T::descriptor();
    let case = desc.case;
    for f in desc.fields {
        // A flattened nested struct reads its fields from the parent object.
        if f.flatten {
            let fv = crate::field::FieldValue::Nested(value.clone());
            (f.set)(&mut out, fv).map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
            continue;
        }
        // `#[fields_map]`: collect the `[{name, value}]` blob into a map.
        if f.fields_map {
            let key = wire_key(f.name, f.wire_name, case);
            let map = obj
                .get(&key)
                .and_then(Value::as_array)
                .map(|arr| fields_array_to_map(arr))
                .unwrap_or_else(|| Value::Object(Map::new()));
            (f.set)(&mut out, crate::field::FieldValue::Json(map))
                .map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
            continue;
        }
        let key = wire_key(f.name, f.wire_name, case);
        let Some(jv) = obj.get(&key) else { continue };
        if jv.is_null() {
            continue;
        }
        // `#[wire(int)]`: the wire carries `0`/`1` for a bool field — map it back
        // before the kind-driven conversion (which expects a JSON bool).
        let int_bool;
        let jv = if f.as_int && jv.is_number() {
            int_bool = Value::Bool(jv.as_i64().is_some_and(|n| n != 0));
            &int_bool
        } else {
            jv
        };
        let fv = json_to_field_value(f.kind, f.secret, jv)
            .map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
        (f.set)(&mut out, fv).map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
    }
    Ok(out)
}

/// Convert a JSON value to the owned [`FieldValue`] matching `kind`.
pub(crate) fn json_to_field_value(
    kind: &crate::field::FieldKind,
    secret: bool,
    jv: &Value,
) -> anyhow::Result<crate::field::FieldValue> {
    use crate::field::{FieldKind as K, FieldValue as V};

    Ok(match kind {
        K::Bool => V::Bool(jv.as_bool().ok_or_else(|| err("bool", jv))?),
        K::Int32 => V::Int32(jv.as_i64().ok_or_else(|| err("int", jv))? as i32),
        K::Int64 => V::Int64(jv.as_i64().ok_or_else(|| err("int", jv))?),
        K::Float64 => V::Float64(jv.as_f64().ok_or_else(|| err("float", jv))?),
        K::String if secret => V::Secret(jv.as_str().ok_or_else(|| err("string", jv))?.to_string()),
        K::String => V::String(jv.as_str().ok_or_else(|| err("string", jv))?.to_string()),

        K::Optional(inner) => match &**inner {
            K::Bool => V::OptBool(Some(jv.as_bool().ok_or_else(|| err("bool", jv))?)),
            K::Int32 => V::OptInt32(Some(jv.as_i64().ok_or_else(|| err("int", jv))? as i32)),
            K::Int64 => V::OptInt64(Some(jv.as_i64().ok_or_else(|| err("int", jv))?)),
            K::Float64 => V::OptFloat64(Some(jv.as_f64().ok_or_else(|| err("float", jv))?)),
            K::String if secret => {
                V::Secret(jv.as_str().ok_or_else(|| err("string", jv))?.to_string())
            }
            K::String => V::OptString(Some(
                jv.as_str().ok_or_else(|| err("string", jv))?.to_string(),
            )),
            K::Json => V::Json(jv.clone()),
            // `Option<Nested>` — the `set` closure wraps the decoded value in `Some`.
            K::Nested { .. } => V::Nested(jv.clone()),
            other => anyhow::bail!("optional decode unsupported for {other:?}"),
        },

        K::Vec(inner) => {
            let arr = jv.as_array().ok_or_else(|| err("array", jv))?;
            match &**inner {
                K::Bool => V::VecBool(collect(arr, |x| x.as_bool(), "bool")?),
                K::Int32 => V::VecInt32(collect(arr, |x| x.as_i64().map(|i| i as i32), "int")?),
                K::Int64 => V::VecInt64(collect(arr, |x| x.as_i64(), "int")?),
                K::String => {
                    V::VecString(collect(arr, |x| x.as_str().map(str::to_string), "string")?)
                }
                K::Json => V::VecJson(arr.clone()),
                K::Nested { .. } => V::VecNested(arr.clone()),
                other => anyhow::bail!("vec decode unsupported for {other:?}"),
            }
        }

        K::Json => V::Json(jv.clone()),

        // Hand the sub-object to the field's `set`, which knows the concrete
        // nested type and calls its decode.
        K::Nested { .. } => V::Nested(jv.clone()),
    })
}

fn err(ty: &str, jv: &Value) -> anyhow::Error {
    anyhow::anyhow!("expected {ty}, got {jv}")
}

fn collect<T>(arr: &[Value], f: impl Fn(&Value) -> Option<T>, ty: &str) -> anyhow::Result<Vec<T>> {
    arr.iter().map(|x| f(x).ok_or_else(|| err(ty, x))).collect()
}
