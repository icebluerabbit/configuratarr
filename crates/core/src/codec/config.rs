//! User-config codec — the ingest side, mirror image of the wire codec.
//!
//! Users author desired state in the *typed, flat* shape keyed by **snake_case
//! `field.name`** (the Rust field names), not the API's camelCase wire keys:
//!
//! ```yaml
//! name: qbit
//! host: qbit.local
//! use_ssl: false
//! ```
//!
//! That same struct is later encoded to the wire form (camelCase, provider
//! fields-blob arrays, …) for the API. Interpolation (`${env}` / `${file}` /
//! `${ref}`) is resolved on the `Value` before this decode runs.
//!
//! Differences from [`super::standard::decode`]:
//! * keys by `field.name` (snake) rather than `wire_key` (camelCase),
//! * nested structs recurse via the config codec, carried as
//!   [`crate::field::FieldValue::NestedConfig`],
//! * provider/tagged enums dispatch on the user-written discriminator field and
//!   decode the matched variant's inner config flat.

use serde_json::Value;

use crate::codec::{CodecKind, standard};
use crate::described::Described;
use crate::descriptor::CodecMeta;
use crate::field::{FieldKind, FieldValue};

/// Decode a resource from its user-config JSON (snake_case, typed, flat).
pub fn decode<T: Described>(value: &Value) -> anyhow::Result<T> {
    match T::descriptor().codec {
        CodecKind::Standard | CodecKind::FieldsBlob => decode_struct(value),
        CodecKind::TaggedByImpl => decode_tagged(value),
        CodecKind::StringEnum => decode_string_enum(value),
        CodecKind::Custom => T::custom_decode(value),
    }
}

/// Provider/tagged enum: read the discriminator field the user wrote (e.g.
/// `implementation: QBittorrent`), dispatch to the matching variant, decode its
/// inner config flat. Mirror of [`super::tagged_by_impl::decode`] but the inner
/// decode is the config codec.
fn decode_tagged<T: Described>(value: &Value) -> anyhow::Result<T> {
    let discriminator = match &T::descriptor().codec_meta {
        CodecMeta::TaggedByImpl { discriminator } => *discriminator,
        _ => anyhow::bail!("config::decode_tagged on a non-TaggedByImpl descriptor"),
    };
    let disc = value
        .get(discriminator)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("config: missing discriminator `{discriminator}`"))?;
    T::decode_config_variant(disc, value)?
        .ok_or_else(|| anyhow::anyhow!("config: no variant for `{disc}` and no #[fallback]"))
}

/// Unit enum: a bare string, identical to the wire form.
fn decode_string_enum<T: Described>(value: &Value) -> anyhow::Result<T> {
    let s = value
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("config: expected a string for a unit enum, got {value}"))?;
    T::decode_config_variant(s, value)?
        .ok_or_else(|| anyhow::anyhow!("config: unknown value `{s}` and no #[fallback]"))
}

/// Decode → re-encode → keep only the keys the user wrote. A singleton's typed
/// struct fills omitted fields with type defaults that would clobber live values
/// on merge; masking to present keys makes "omitted = unmanaged" hold
/// (validation + casing still come from the full roundtrip).
///
/// Recurses into a nested single object (`Nested` / `Option<Nested>`, via
/// [`FieldDescriptor::nested_present`](crate::descriptor::FieldDescriptor::nested_present))
/// so its inner keys are masked too; a `Vec<Nested>` has no per-element presence
/// mask and is copied whole. Not for the `fields` blob (provider keys live inside
/// the `[{name, value}]` array, not as top-level keys).
pub fn present_to_wire<T: Described>(cfg: &Value) -> anyhow::Result<Value> {
    let decoded = decode::<T>(cfg)?;
    let full = crate::engine::encode(&decoded)?;
    let cfg_obj = cfg
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("config::present_to_wire expected a JSON object"))?;
    let full_obj = full.as_object().cloned().unwrap_or_default();

    let mut out = serde_json::Map::new();
    let desc = T::descriptor();
    for f in desc.fields {
        if f.flatten {
            continue;
        }
        let Some(cfg_v) = cfg_obj.get(f.name) else {
            continue;
        };
        let wk = standard::wire_key(f.name, f.wire_name, desc.case);
        match f.nested_present {
            // A nested single object: recurse so the *inner* keys are also masked
            // to what the user wrote — not the whole struct with type defaults.
            //
            // Gated on the config value actually being an object. The macro
            // can't tell a `#[wire_enum]` type from a `#[nested]` struct — both
            // are just a named type in the AST — so it emits `nested_present`
            // for a wire-enum field too, whose config value is a bare *string*.
            // Recursing into that would fail the object check at the top of this
            // function; there is nothing to mask in a scalar anyway.
            Some(mask) if cfg_v.is_object() => {
                out.insert(wk, mask(cfg_v)?);
            }
            // Scalar (incl. wire-enum) / Vec-nested / null: copy the fully-encoded
            // value as-is.
            _ => {
                if let Some(v) = full_obj.get(&wk) {
                    out.insert(wk, v.clone());
                }
            }
        }
    }
    Ok(Value::Object(out))
}

/// Flat-struct config decode: build an empty instance, then populate each field
/// whose snake_case `name` key is present.
fn decode_struct<T: Described>(value: &Value) -> anyhow::Result<T> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("config::decode expected a JSON object"))?;

    let mut out = T::empty();
    for f in T::descriptor().fields {
        // A flattened nested struct reads its fields from the parent object.
        if f.flatten {
            (f.set)(&mut out, FieldValue::NestedConfig(value.clone()))
                .map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
            continue;
        }
        let Some(jv) = obj.get(f.name) else {
            // Absent — apply the declared `#[default(...)]` if any (the user
            // omitting a field means "use its default", not "type zero").
            if let Some(lit) = f.default
                && let Some(fv) = default_field_value(f.kind, lit)
            {
                (f.set)(&mut out, fv).map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
            }
            continue;
        };
        if jv.is_null() {
            continue;
        }
        let fv = to_field_value(f.kind, f.secret, jv)
            .map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
        (f.set)(&mut out, fv).map_err(|e| anyhow::anyhow!("field `{}`: {e}", f.name))?;
    }
    Ok(out)
}

/// Build the [`FieldValue`] for an absent field from its `#[default(..)]`
/// literal, coerced to the field's scalar kind. `None` for shapes a literal
/// default doesn't apply to (optionals, vecs, nested).
fn default_field_value(kind: &FieldKind, lit: crate::DefaultLit) -> Option<FieldValue> {
    use crate::DefaultLit as L;
    use FieldKind as K;
    Some(match (kind, lit) {
        (K::Bool, L::Bool(b)) => FieldValue::Bool(b),
        (K::Int32, L::Int(i)) => FieldValue::Int32(i as i32),
        (K::Int64, L::Int(i)) => FieldValue::Int64(i),
        (K::Float64, L::Float(f)) => FieldValue::Float64(f),
        (K::String, L::Str(s)) => FieldValue::String(s.to_string()),
        _ => return None,
    })
}

/// Convert a JSON value to a [`FieldValue`]. Reuses the wire codec's scalar
/// conversion; only nested fields differ — they carry `NestedConfig` so the
/// inner type decodes through the config codec, not the wire codec.
fn to_field_value(kind: &FieldKind, secret: bool, jv: &Value) -> anyhow::Result<FieldValue> {
    use FieldKind as K;
    match kind {
        K::Nested { .. } => Ok(FieldValue::NestedConfig(jv.clone())),
        K::Optional(inner) if matches!(**inner, K::Nested { .. }) => {
            Ok(FieldValue::NestedConfig(jv.clone()))
        }
        K::Vec(inner) if matches!(**inner, K::Nested { .. }) => {
            let arr = jv
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("expected an array, got {jv}"))?;
            Ok(FieldValue::VecNestedConfig(arr.clone()))
        }
        _ => standard::json_to_field_value(kind, secret, jv),
    }
}
