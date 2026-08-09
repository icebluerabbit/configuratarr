//! The [`Described`] trait ties a resource type to its static descriptor. Engine
//! functions are generic over `T: Described` and dispatch on the descriptor's
//! `codec`; they never call methods on `T` directly beyond the accessor.
//!
//! [`ResourceErased`] is the trait-object shape used for nested-struct recursion
//! (hit when a [`crate::FieldRef`] is `Nested`/`VecNested`).

use crate::descriptor::ResourceDescriptor;
use crate::field::FieldRef;

/// User-facing trait, implemented by the `#[resource]` macro. The only required
/// method is the descriptor accessor; everything else is free functions in
/// [`crate::engine`].
pub trait Described: Sized + 'static {
    fn descriptor() -> &'static ResourceDescriptor<Self>;

    /// An all-empty instance (type defaults). Decode constructs this, then
    /// populates each field via the descriptor's `set` closures. Enum resources
    /// build their active variant directly instead, so their `empty()` is
    /// `unimplemented!`.
    fn empty() -> Self;

    /// Enum support: encode the active variant. Tagged enums delegate to the
    /// inner variant's codec; wire-enums produce a bare string. Structs return
    /// `None`.
    fn encode_variant(&self) -> anyhow::Result<Option<serde_json::Value>> {
        Ok(None)
    }

    /// Enum support: build the variant matching `discriminator` from a wire
    /// `value`. `Ok(None)` means no match and no `#[fallback]`.
    fn decode_variant(
        _discriminator: &str,
        _value: &serde_json::Value,
    ) -> anyhow::Result<Option<Self>> {
        Ok(None)
    }

    /// Like [`decode_variant`](Self::decode_variant) but the inner variant
    /// decodes through the *config* codec (snake_case) rather than the wire codec.
    fn decode_config_variant(
        _discriminator: &str,
        _value: &serde_json::Value,
    ) -> anyhow::Result<Option<Self>> {
        Ok(None)
    }

    /// `CodecKind::Custom` hand-written encode. The default bails so a missing
    /// override is a clear error.
    fn custom_encode(&self) -> anyhow::Result<serde_json::Value> {
        anyhow::bail!("codec = custom selected but custom_encode is not implemented")
    }

    /// `CodecKind::Custom` hand-written decode. See [`custom_encode`](Self::custom_encode).
    fn custom_decode(_value: &serde_json::Value) -> anyhow::Result<Self> {
        anyhow::bail!("codec = custom selected but custom_decode is not implemented")
    }
}

/// Trait-object shape for nested-resource recursion inside [`FieldRef`].
/// Auto-implemented for every `T: Described` via the blanket impl below; codecs
/// call it to walk a nested value's fields without knowing `T` statically.
pub trait ResourceErased {
    fn descriptor_erased(&self) -> ResourceDescriptorErased<'_>;

    /// Encode this value via its own codec (struct field-walk, enum variant
    /// dispatch, …) — lets the standard codec encode a nested field without
    /// knowing whether it's a struct or an enum.
    fn encode_self(&self) -> anyhow::Result<serde_json::Value>;
}

/// Type-erased descriptor view passed to engine recursion: the codec
/// discriminator plus a way to enumerate field descriptors without their `T`.
pub struct ResourceDescriptorErased<'a> {
    pub type_name: &'static str,
    pub codec: crate::codec::CodecKind,
    /// Default wire-key casing (camel|pascal), for engine walks that compute
    /// wire keys off the erased view (secret keys, natural key).
    pub case: crate::descriptor::Case,
    /// The tagged-enum discriminator key, if this is a `TaggedByImpl` resource.
    pub discriminator: Option<&'static str>,
    /// Enum variants (empty for structs).
    pub variants: &'static [crate::descriptor::VariantDescriptor],
    pub fields: ErasedFields<'a>,
}

/// Erased field iteration: each entry pairs the field's metadata with a borrowed
/// [`FieldRef`] on the underlying instance.
pub struct ErasedFields<'a> {
    pub iter: Box<dyn Iterator<Item = ErasedField<'a>> + 'a>,
}

pub struct ErasedField<'a> {
    pub name: &'static str,
    pub doc: Option<&'static str>,
    pub role: crate::field::FieldRole,
    pub kind: &'static crate::field::FieldKind,
    pub wire_name: Option<&'static str>,
    pub read_only: bool,
    pub config_only: bool,
    pub flatten: bool,
    pub reference: &'static [&'static str],
    pub secret: bool,
    pub default: Option<crate::descriptor::DefaultLit>,
    pub nested_docs: Option<fn() -> crate::engine::ResourceDoc>,
    pub value: FieldRef<'a>,
}

impl<T: Described> ResourceErased for T {
    fn descriptor_erased(&self) -> ResourceDescriptorErased<'_> {
        let desc = T::descriptor();
        let iter = desc.fields.iter().map(move |f| ErasedField {
            name: f.name,
            doc: f.doc,
            role: f.role,
            kind: f.kind,
            wire_name: f.wire_name,
            read_only: f.read_only,
            config_only: f.config_only,
            flatten: f.flatten,
            reference: f.reference,
            secret: f.secret,
            default: f.default,
            nested_docs: f.nested_docs,
            value: (f.get)(self),
        });
        let discriminator = match &desc.codec_meta {
            crate::descriptor::CodecMeta::TaggedByImpl { discriminator } => Some(*discriminator),
            _ => None,
        };
        ResourceDescriptorErased {
            type_name: desc.type_name,
            codec: desc.codec,
            case: desc.case,
            discriminator,
            variants: desc.variants,
            fields: ErasedFields {
                iter: Box::new(iter),
            },
        }
    }

    fn encode_self(&self) -> anyhow::Result<serde_json::Value> {
        crate::engine::encode(self)
    }
}
