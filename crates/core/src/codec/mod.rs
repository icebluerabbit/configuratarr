//! Wire-format codecs that the engine dispatches to via [`CodecKind`].
//!
//! Each codec is a module of free functions operating on `&T: Described` plus
//! the type's descriptor. Adding a new wire format = adding a new module + a
//! new [`CodecKind`] variant + a match arm in [`crate::engine`].
//!
//! Codecs never touch `T` directly. They walk `T::descriptor().fields` and
//! call the per-field `get` / `set` closures.

pub mod config;
pub mod fields_blob;
pub mod standard;
pub mod string_enum;
pub mod tagged_by_impl;

/// Which codec a [`crate::ResourceDescriptor`] declares.
///
/// The engine matches on this to pick the encode/decode implementation. The
/// codec-specific metadata for each variant lives on the descriptor in
/// [`crate::descriptor::CodecMeta`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecKind {
    /// Plain JSON object; snake_case Rust fields → camelCase wire keys.
    Standard,

    /// *arr fields-blob: `{implementation, configContract, fields: [{name, value}]}`.
    FieldsBlob,

    /// Discriminator-dispatched enum: reads a string field (default
    /// `"implementation"`) and delegates to the matching variant's codec.
    TaggedByImpl,

    /// Unit enum as a bare JSON string (`#[wire_enum]`); `#[fallback]` absorbs
    /// unknowns.
    StringEnum,

    /// Escape hatch — hand-written `custom_encode` / `custom_decode` hooks in the
    /// contributor's crate.
    Custom,
}
