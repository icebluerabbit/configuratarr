//! Field-level shapes used by descriptors and codecs.
//!
//! There are no resolution wrapper types: interpolation is resolved on the JSON
//! `Value` tree before decode, so a typed struct only ever holds resolved
//! values. Ref-ness and secret-ness are descriptor metadata
//! ([`crate::FieldDescriptor::reference`] / `secret`), not field types — a ref
//! is a plain `i32`, a credential a [`SecretValue`].

use crate::secret::SecretValue;

/// Why this field exists on the wire. `Id` fields are server-populated and never
/// sent on POST/PUT; `Key` is the natural identifier (`${ref.*.<key>}` + primary
/// diff key); `Normal` is everything else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldRole {
    Id,
    Key,
    Normal,
}

/// Structural shape of a field. Purely structural — a ref is `Int32` with
/// `reference` metadata, a credential is `String` with `secret` metadata.
/// Nesting is unbounded: variants hold `&'static FieldKind`.
#[derive(Debug)]
pub enum FieldKind {
    Bool,
    Int32,
    Int64,
    Float64,
    String,

    /// `Option<T>` — codec treats `None` as JSON null or omit-on-write.
    Optional(&'static FieldKind),

    /// `Vec<T>` — codec emits a JSON array.
    Vec(&'static FieldKind),

    /// A nested [`crate::Described`] struct, encoded by recursive descent.
    Nested {
        type_name: &'static str,
    },

    /// Opaque JSON ([`crate::Json`]), passed through verbatim — API blobs we
    /// don't model (raw provider `fields`, preset templates).
    Json,
}

/// Borrowed view of one field's value, from the descriptor's `get` accessor.
/// The engine matches on the variant to serialise. `Secret` is the one
/// non-plain carrier: a [`SecretValue`], redacted in memory.
pub enum FieldRef<'a> {
    Bool(&'a bool),
    Int32(&'a i32),
    Int64(&'a i64),
    Float64(&'a f64),
    String(&'a String),

    OptBool(&'a Option<bool>),
    OptInt32(&'a Option<i32>),
    OptInt64(&'a Option<i64>),
    OptFloat64(&'a Option<f64>),
    OptString(&'a Option<String>),

    VecBool(&'a [bool]),
    VecInt32(&'a [i32]),
    VecInt64(&'a [i64]),
    VecString(&'a [String]),

    /// Credential value, exposed only at the encode/HTTP-send boundary.
    Secret(&'a SecretValue),

    /// Optional credential; `None` means not provided.
    OptSecret(&'a Option<SecretValue>),

    /// Nested resource — the engine descends via its erased view.
    Nested(&'a dyn crate::described::ResourceErased),

    /// `Vec<Nested>` — a boxed iterator over the elements' erased views (a
    /// `fn(&T) -> FieldRef` accessor can't hand back a `&[&dyn …]` slice without
    /// a temporary, so the encoder consumes this iterator instead).
    VecNested(Box<dyn Iterator<Item = &'a dyn crate::described::ResourceErased> + 'a>),

    /// Opaque JSON, passed through unchanged.
    Json(&'a serde_json::Value),

    /// Optional opaque JSON; `None` ⇒ absent/null on the wire.
    OptJson(&'a Option<serde_json::Value>),

    /// Array of opaque JSON.
    VecJson(&'a [serde_json::Value]),
}

/// Owned value handed to the descriptor's `set` accessor during decode. The
/// codec produces these from the resolved JSON; the descriptor coerces them into
/// the field's concrete type and errors if the kinds disagree.
#[derive(Debug)]
pub enum FieldValue {
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float64(f64),
    String(String),

    OptBool(Option<bool>),
    OptInt32(Option<i32>),
    OptInt64(Option<i64>),
    OptFloat64(Option<f64>),
    OptString(Option<String>),

    VecBool(Vec<bool>),
    VecInt32(Vec<i32>),
    VecInt64(Vec<i64>),
    VecString(Vec<String>),

    /// Resolved credential string; the `set` closure wraps it in a [`SecretValue`].
    Secret(String),

    /// Nested value from the wire codec — decoded via [`crate::engine::decode`]
    /// (camelCase keys).
    Nested(serde_json::Value),
    /// Nested value from the config codec — decoded via
    /// [`crate::engine::decode_config`] (snake_case `field.name` keys). A
    /// distinct variant so the `set` closure picks the right inner decode.
    NestedConfig(serde_json::Value),
    /// `Vec<Nested>` from the wire codec — each element decoded via `decode`.
    VecNested(Vec<serde_json::Value>),
    /// `Vec<Nested>` from the config codec — each element decoded via `decode_config`.
    VecNestedConfig(Vec<serde_json::Value>),

    /// Opaque JSON, stored verbatim into a [`crate::Json`] field.
    Json(serde_json::Value),
    /// Array of opaque JSON, for `Vec<Json>` fields.
    VecJson(Vec<serde_json::Value>),

    Null,
}
