//! Top-level engine functions. Each dispatches to a codec module based on the
//! resource's declared [`crate::CodecKind`].
//!
//! Engine functions are the only public way to encode, decode, or resolve a
//! resource. The trait surface stays minimal — [`crate::Described`] is the
//! single user-facing trait; engine free functions cover everything else.

use crate::codec::{self, CodecKind};
use crate::described::{Described, ResourceErased};

/// Encode a resource to its wire JSON form using the codec declared in its
/// descriptor.
pub fn encode<T: Described>(value: &T) -> anyhow::Result<serde_json::Value> {
    match T::descriptor().codec {
        CodecKind::Standard => codec::standard::encode(value),
        CodecKind::FieldsBlob => codec::fields_blob::encode(value),
        CodecKind::TaggedByImpl => codec::tagged_by_impl::encode(value),
        CodecKind::StringEnum => codec::string_enum::encode(value),
        CodecKind::Custom => value.custom_encode(),
    }
}

/// Decode a resource from its wire JSON form.
pub fn decode<T: Described>(value: &serde_json::Value) -> anyhow::Result<T> {
    match T::descriptor().codec {
        CodecKind::Standard => codec::standard::decode(value),
        CodecKind::FieldsBlob => codec::fields_blob::decode(value),
        CodecKind::TaggedByImpl => codec::tagged_by_impl::decode(value),
        CodecKind::StringEnum => codec::string_enum::decode(value),
        CodecKind::Custom => T::custom_decode(value),
    }
}

/// Decode a resource from its **user-config** JSON — the typed, flat shape keyed
/// by snake_case `field.name`. The ingest counterpart to [`decode`] (which reads
/// the API's camelCase wire form). See [`crate::codec::config`].
pub fn decode_config<T: Described>(value: &serde_json::Value) -> anyhow::Result<T> {
    codec::config::decode(value)
}

/// Config → wire in one step: [`decode_config`] then [`encode`]. The full
/// (non-masked) counterpart to [`config_present_to_wire`] — the recurring
/// `encode(&decode_config::<T>(cfg)?)` a `sync = custom` hook runs to turn one
/// resolved config entry into its API wire shape.
pub fn encode_config<T: Described>(value: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
    encode(&decode_config::<T>(value)?)
}

/// Config → wire keeping only the keys the user wrote (presence-masked). For
/// singletons / partial config — see [`crate::codec::config::present_to_wire`].
pub fn config_present_to_wire<T: Described>(
    value: &serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    codec::config::present_to_wire::<T>(value)
}

/// Decode a whole service instance from one config-file entry. Thin wrapper over
/// the macro-emitted [`crate::Service::from_config`]; the registry (core-config)
/// dispatches the `type:` tag to the concrete `S` and calls this.
pub fn decode_service_config<S: crate::Service>(value: &serde_json::Value) -> anyhow::Result<S> {
    S::from_config(value)
}

/// The wire name of a resource's natural key (`FieldRole::Key`), descending
/// through `#[flatten]` envelopes. `None` for keyless resources (singletons).
/// Used by the planner to match desired vs live by key.
pub fn key_wire_name<T: Described>() -> Option<String> {
    find_key(T::empty().descriptor_erased())
}

/// Every resource type this resource references (`#[reference(t)]`), descending
/// through nested/`#[flatten]` structs — so a ref on a flattened envelope (e.g.
/// `Provider.tags`) is still seen. Drives apply ordering.
///
/// Walks the **static** descriptor (no instance): each field contributes its own
/// `#[reference]` and, for a nested field, its inner type's targets via the
/// macro-emitted [`FieldDescriptor::nested_reference_targets`]. That accessor is
/// how a `Vec<Nested>`/`Option<Nested>` FK is reached — an `empty()` instance's
/// list/option is empty, so an instance walk can't see the element type (the bug
/// that let `filter.indexers[].id → indexer` escape the apply-order graph).
/// `#[flatten]` fields are nested, so their FKs (e.g. the *arr `Provider.tags`)
/// recurse too. Unlike a doc-tree walk, read-only fields are **not** skipped.
///
/// `seen` (by resource type name) guards self-/mutually-recursive nested types —
/// e.g. radarr's `QualityProfileItem`, whose `items` nest the same type.
pub fn reference_targets<T: Described>() -> Vec<&'static str> {
    let mut out = Vec::new();
    let mut seen = Vec::new();
    collect_reference_targets::<T>(&mut out, &mut seen);
    out
}

/// Collect `T`'s `#[reference]` targets (its own + those of its nested types)
/// into `out`, recursing via the macro-emitted per-field accessors. `seen` tracks
/// visited type names so a cyclic nesting terminates. Public because the
/// generated `nested_reference_targets` closures re-enter it for the inner type.
pub fn collect_reference_targets<T: Described>(
    out: &mut Vec<&'static str>,
    seen: &mut Vec<&'static str>,
) {
    let tn = T::descriptor().type_name;
    if seen.contains(&tn) {
        return;
    }
    seen.push(tn);
    for f in T::descriptor().fields {
        for r in f.reference {
            out.push(r);
        }
        if let Some(nested) = f.nested_reference_targets {
            nested(out, seen);
        }
    }
}

/// Wire-key names of this resource's secret fields, descending through
/// `#[flatten]`. Used by [`crate::plan::Plan::render`] to redact credentials.
/// Secrets inside a provider `fields` blob aren't listed — the renderer never
/// prints complex values, so they're covered by that policy.
pub fn secret_wire_keys<T: Described>() -> Vec<String> {
    let mut out = Vec::new();
    collect_secret_keys(T::empty().descriptor_erased(), &mut out);
    out
}

/// One documented field, for config-doc generation. The `Documented`-style
/// data the engine can extract from a descriptor without an instance.
pub struct FieldDoc {
    /// snake_case config key the user writes.
    pub name: &'static str,
    /// The field's `///` comment.
    pub doc: Option<&'static str>,
    /// Human type label (`string`, `integer`, `array of ...`, a ref target, …).
    pub type_label: String,
    /// True when the user must supply it (not optional, not a vec, no default).
    pub required: bool,
    /// Declared `#[default(..)]`, if any.
    pub default: Option<crate::descriptor::DefaultLit>,
    /// `#[reference(<type>)]` target, if this is a ref.
    pub reference: &'static [&'static str],
    pub secret: bool,
    /// The nested type name, if this field is a nested struct/enum (so the
    /// renderer can link the type label to that type's section).
    pub nested_type: Option<&'static str>,
}

/// A resource's documentation: flat fields, provider (tagged-enum) blocks
/// discovered through `#[flatten]`, nested types it references, and — for a
/// `#[wire_enum]` — its allowed wire values.
pub struct ResourceDoc {
    pub fields: Vec<FieldDoc>,
    pub providers: Vec<ProviderDoc>,
    pub nested: Vec<NestedRef>,
    pub enum_values: Vec<&'static str>,
}

/// A nested type referenced by a resource, with a lazy provider for its own
/// docs — lets the renderer BFS the type graph without an instance.
#[derive(Clone, Copy)]
pub struct NestedRef {
    pub type_name: &'static str,
    pub docs: fn() -> ResourceDoc,
}

/// A flattened provider enum: its discriminator key + one block per variant.
pub struct ProviderDoc {
    /// The discriminator config key (e.g. `implementation`).
    pub discriminator: &'static str,
    pub variants: Vec<VariantDoc>,
}

/// One provider implementation: the discriminator value to write + its fields.
pub struct VariantDoc {
    pub name: &'static str,
    pub wire: &'static str,
    pub fields: Vec<FieldDoc>,
}

fn empty_doc() -> ResourceDoc {
    ResourceDoc {
        fields: Vec::new(),
        providers: Vec::new(),
        nested: Vec::new(),
        enum_values: Vec::new(),
    }
}

/// The flat user-writable fields of a resource, descending `#[flatten]` structs
/// and dropping read-only / id fields. Used per provider variant and as the
/// scalar half of [`resource_docs`].
pub fn field_docs<T: Described>() -> Vec<FieldDoc> {
    let mut doc = empty_doc();
    collect_resource_docs(T::empty().descriptor_erased(), &mut doc);
    doc.fields
}

/// Full doc for a resource: flat fields, per-variant provider blocks, referenced
/// nested types, and (for a `#[wire_enum]`) its allowed values. Drives config-doc-gen.
pub fn resource_docs<T: Described>() -> ResourceDoc {
    let mut doc = empty_doc();
    collect_resource_docs(T::empty().descriptor_erased(), &mut doc);
    let d = T::descriptor();
    if d.codec == crate::CodecKind::StringEnum {
        doc.enum_values = d.variants.iter().filter_map(|v| v.wire).collect();
    }
    doc
}

fn collect_resource_docs(d: crate::described::ResourceDescriptorErased<'_>, doc: &mut ResourceDoc) {
    use crate::field::{FieldRef, FieldRole};
    for f in d.fields.iter {
        if f.flatten
            && let FieldRef::Nested(n) = f.value
        {
            let nested = n.descriptor_erased();
            if let Some(discriminator) = nested.discriminator {
                collect_provider(discriminator, nested.variants, doc);
            } else {
                collect_resource_docs(nested, doc);
            }
            continue;
        }
        // `config_only` is read-only *on the wire* but is still config the user
        // writes, so it stays in the docs — dropping it would leave a whole
        // section of the config surface undocumented.
        if (f.read_only && !f.config_only) || f.role == FieldRole::Id {
            continue;
        }
        // A nested-type field records its inner type (for the BFS) and links its
        // label; the doc-provider comes from the descriptor (works even absent).
        let nested_type = nested_type_name(f.kind);
        if let (Some(tn), Some(docs)) = (nested_type, f.nested_docs)
            && !doc.nested.iter().any(|n| n.type_name == tn)
        {
            doc.nested.push(NestedRef {
                type_name: tn,
                docs,
            });
        }
        doc.fields.push(FieldDoc {
            name: f.name,
            doc: f.doc,
            type_label: type_label(f.kind, f.secret),
            required: is_required(f.kind, f.default),
            default: f.default,
            reference: f.reference,
            secret: f.secret,
            nested_type,
        });
    }
}

/// The nested type name behind a `Nested` / `Option<Nested>` / `Vec<Nested>`
/// kind, else `None`.
fn nested_type_name(kind: &crate::field::FieldKind) -> Option<&'static str> {
    use crate::field::FieldKind as K;
    match *kind {
        K::Nested { type_name } => Some(type_name),
        K::Optional(inner) | K::Vec(inner) => nested_type_name(inner),
        _ => None,
    }
}

/// Record a flattened provider enum: the discriminator key + one [`VariantDoc`]
/// per typed variant (skipping the `#[fallback]`, whose `wire` is `None`). The
/// renderer turns this into the discriminator selector + per-variant sections;
/// engine stays markdown-agnostic.
fn collect_provider(
    discriminator: &'static str,
    variants: &'static [crate::descriptor::VariantDescriptor],
    doc: &mut ResourceDoc,
) {
    let mut out = Vec::new();
    for v in variants {
        if let Some(wire) = v.wire {
            out.push(VariantDoc {
                name: v.name,
                wire,
                fields: (v.field_docs)(),
            });
        }
    }
    doc.providers.push(ProviderDoc {
        discriminator,
        variants: out,
    });
}

/// A field is required when the user must write it: not optional, not a vec
/// (defaults to empty), and no declared `#[default]`.
fn is_required(
    kind: &crate::field::FieldKind,
    default: Option<crate::descriptor::DefaultLit>,
) -> bool {
    use crate::field::FieldKind as K;
    !matches!(kind, K::Optional(_) | K::Vec(_)) && default.is_none()
}

/// Human-readable type label for a [`FieldKind`].
fn type_label(kind: &crate::field::FieldKind, secret: bool) -> String {
    use crate::field::FieldKind as K;
    match kind {
        K::Bool => "boolean".into(),
        K::Int32 | K::Int64 => "integer".into(),
        K::Float64 => "number".into(),
        K::String if secret => "secret string".into(),
        K::String => "string".into(),
        K::Optional(inner) => type_label(inner, secret),
        K::Vec(inner) => format!("array of {}", type_label(inner, secret)),
        K::Nested { type_name } => format!("`{type_name}`"),
        K::Json => "any".into(),
    }
}

fn collect_secret_keys(d: crate::described::ResourceDescriptorErased<'_>, out: &mut Vec<String>) {
    use crate::field::FieldRef;
    for f in d.fields.iter {
        if f.secret {
            out.push(codec::standard::wire_key(f.name, f.wire_name, d.case));
        }
        if f.flatten
            && let FieldRef::Nested(n) = f.value
        {
            collect_secret_keys(n.descriptor_erased(), out);
        }
    }
}

fn find_key(d: crate::described::ResourceDescriptorErased<'_>) -> Option<String> {
    use crate::field::{FieldRef, FieldRole};
    for f in d.fields.iter {
        if f.role == FieldRole::Key {
            return Some(codec::standard::wire_key(f.name, f.wire_name, d.case));
        }
        if f.flatten
            && let FieldRef::Nested(n) = f.value
            && let Some(k) = find_key(n.descriptor_erased())
        {
            return Some(k);
        }
    }
    None
}

// Interpolation resolves on the Value tree before decode — see [`crate::resolve`].
