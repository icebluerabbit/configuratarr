//! Static descriptors emitted by the `#[resource]` macro — the macro's entire
//! output. The engine reads them at runtime to drive encode/decode/resolve.

use crate::apply::CustomSyncFn;
use crate::codec::CodecKind;
use crate::field::{FieldRef, FieldRole, FieldValue};

/// Default wire-key casing for fields without an explicit `#[wire(name)]`.
/// `Camel` is snake→camelCase (the *arr shape); `Pascal` upper-cases the first
/// character too, for .NET-style APIs (Jellyfin) whose JSON is PascalCase;
/// `Snake` is identity — the wire key *is* the snake field name (bazarr, whose
/// settings JSON is snake_case).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    Camel,
    Pascal,
    Snake,
}

/// An HTTP verb, always declared explicitly at the `#[resource]` site (no
/// inferred defaults).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// One HTTP operation. `path` may carry `${self.*}` interpolation, resolved
/// against the resource value at apply time (e.g. `/api/v3/tag/${self.id}`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endpoint {
    pub method: HttpMethod,
    pub path: &'static str,
}

/// The HTTP operations a resource exposes. Pure data; the sync strategy reads
/// the slots it needs. `Embedded` resources carry [`Endpoints::NONE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endpoints {
    pub list: Option<Endpoint>,
    /// GET-one (singletons, fanned reads).
    pub read: Option<Endpoint>,
    pub create: Option<Endpoint>,
    pub update: Option<Endpoint>,
    pub delete: Option<Endpoint>,
}

impl Endpoints {
    /// No endpoints — for embedded sub-resources (`#[nested]` / `#[fields_blob]`
    /// / `#[tagged]` / `#[wire_enum]`).
    pub const NONE: Endpoints = Endpoints {
        list: None,
        read: None,
        create: None,
        update: None,
        delete: None,
    };
}

/// Static description of one managed resource (struct or enum). Generic over `T`
/// so the field accessors are concretely typed; each macro invocation produces a
/// `&'static ResourceDescriptor<Self>`.
pub struct ResourceDescriptor<T: 'static> {
    /// HTTP operations; [`Endpoints::NONE`] for embedded sub-resources.
    pub endpoints: Endpoints,

    /// Snake-case type id, used in `${ref.<type_name>.<key>}` and dep-graph nodes.
    pub type_name: &'static str,

    /// The struct's `///` comment, for doc-gen; unused by the runtime engine.
    pub doc: Option<&'static str>,

    /// Which wire-format codec the engine dispatches to.
    pub codec: CodecKind,

    /// How a field's wire key is cased when it has no explicit `#[wire(name)]`.
    /// The Standard/config codecs read this; `FieldsBlob` variants are always
    /// camelCase (*arr). Data, not behaviour — the codec applies it.
    pub case: Case,

    /// Write strategy. Orthogonal to `codec` and field roles; always explicit —
    /// the write contract varies per API, never inferred from struct shape.
    /// `Custom` carries its reconcile hook inline.
    pub sync: SyncKind,

    /// Codec-specific metadata; interpretation depends on `codec`. For
    /// `FieldsBlob` it carries the *arr `implementation` / `configContract` /
    /// `protocol` discriminators; for `Standard` it's empty.
    pub codec_meta: CodecMeta,

    /// Fields in declaration order; the engine walks this for every operation.
    /// Empty for enum descriptors (they use `variants`).
    pub fields: &'static [FieldDescriptor<T>],

    /// Variants, for enum resources (`#[tagged]` / `#[wire_enum]`); empty for
    /// structs.
    pub variants: &'static [VariantDescriptor],
}

/// One variant of an enum resource. Reused by both enum codecs: `TaggedByImpl`
/// reads `wire` (the matched discriminator value) + `inner_type`; `StringEnum`
/// reads `wire` (the rendered string) and ignores `inner_type`.
pub struct VariantDescriptor {
    /// Rust variant identifier, e.g. `QBittorrent`.
    pub name: &'static str,

    /// Discriminator value (tagged) or wire string (wire-enum) this variant
    /// matches/renders. `None` only for the fallback variant.
    pub wire: Option<&'static str>,

    /// Inner nested type name for tagged variants; `None` for unit variants.
    pub inner_type: Option<&'static str>,

    /// The catch-all `#[fallback]` variant.
    pub fallback: bool,

    /// The variant's inner fields, for config-doc generation. Empty for unit /
    /// fallback variants.
    pub field_docs: fn() -> Vec<crate::engine::FieldDoc>,
}

/// A literal `#[default(expr)]` value. Scalars only (const-friendly so it lives
/// directly in the static descriptor).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefaultLit {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(&'static str),
}

/// Recurses reference-target collection into a nested field's inner type:
/// `fn(out, seen)`, appending targets to `out` and using `seen` (type names) to
/// terminate cyclic nesting. See [`FieldDescriptor::nested_reference_targets`].
pub type NestedRefCollector = fn(&mut Vec<&'static str>, &mut Vec<&'static str>);

/// Static description of one field on a resource.
pub struct FieldDescriptor<T: 'static> {
    /// Snake-case Rust field name; the codec maps it to the wire name.
    pub name: &'static str,

    /// The field's `///` comment, for doc-gen; unused by the runtime engine.
    pub doc: Option<&'static str>,

    /// Whether the field is a key, an id, or normal data. Drives ref key-lookup
    /// and identifies the id field for the `RefStore`.
    pub role: FieldRole,

    /// Structural shape, sufficient for the codec to render JSON without
    /// re-inspecting the Rust type.
    pub kind: &'static crate::field::FieldKind,

    /// Wire-name override; when `None` the codec derives it from `name`.
    pub wire_name: Option<&'static str>,

    /// Present on read, skipped on write (API-computed metadata: `presets`,
    /// `implementationName`, `message`). Id fields are implicitly read-only.
    pub read_only: bool,

    /// User-authored, but not part of *this* resource's wire body — a
    /// `sync = custom` hook writes it to a different endpoint (Cleanuparr's
    /// per-download-client seeding rules and sub-configs). Skipped on encode
    /// exactly like [`Self::read_only`], but **kept in the generated config
    /// docs**: unlike server-owned metadata, it is part of the config surface a
    /// user has to be told about.
    pub config_only: bool,

    /// `#[wire(null)]` — emit a `None` optional as JSON `null` rather than
    /// omitting the key. For APIs whose diff/replace compares the *whole* object
    /// (bazarr's form-encoded settings round-trip nulls verbatim), so an omitted
    /// key would make the desired object structurally differ from the live one.
    /// Standard codec only.
    pub emit_none: bool,

    /// `#[wire(int)]` — encode a `bool` (or `Option<bool>`) as the integer `0`/`1`
    /// on the wire, and decode `0`/`1` back to a bool. For APIs that store
    /// booleans as ints (bazarr `originalFormat`). Standard codec only.
    pub as_int: bool,

    /// Non-zero `#[default(expr)]`. `None` means type default. Applied by the
    /// config decode for absent keys, and read by doc-gen.
    pub default: Option<DefaultLit>,

    /// Credential field — value is a [`crate::SecretValue`]. Inferred from the
    /// field's type, not an attribute.
    pub secret: bool,

    /// `#[flatten]` — a nested struct whose fields are hoisted to the parent
    /// object on the wire. Only meaningful on `Nested` fields.
    pub flatten: bool,

    /// `#[fields_map]` — a `Json` *object* field rendered on the wire as a
    /// `[{name, value}]` array (the *arr provider fields blob) and collected
    /// back into a map on decode. Lets users author the open provider settings
    /// as a plain YAML map (`fields: { baseUrl: ... }`) instead of a list of
    /// `{name, value}` entries. Only the standard wire codec acts on this flag.
    pub fields_map: bool,

    /// Reference target types from `#[reference(<type>, …)]`: this FK field (an
    /// `i32`/`Vec<i32>` for *arr integer ids, or a `String` for GUID APIs) is
    /// resolved from a `${ref.<type>.<key>}` expression to the target's [`RefId`].
    /// Drives the dependency graph — every listed target is an edge.
    ///
    /// Usually one target; more than one for a *polymorphic* FK, where the same
    /// field can address any of several resource types. Empty when the field is
    /// not a reference.
    pub reference: &'static [&'static str],

    /// For a nested-type field (`Nested` / `Option<Nested>` / `Vec<Nested>`), a
    /// provider for the inner type's docs — so doc-gen can render its section
    /// even when the value is absent in an `empty()` instance. `None` otherwise.
    pub nested_docs: Option<fn() -> crate::engine::ResourceDoc>,

    /// For a nested-type field, recurse [`crate::engine::collect_reference_targets`]
    /// into the inner type — collecting its `#[reference]` targets into `out`,
    /// with `seen` guarding self-/mutually-recursive nested types (e.g. a quality
    /// group whose items nest the same type). Lets `reference_targets` reach a
    /// `Vec<Nested>`/`Option<Nested>` FK without an instance (an `empty()`
    /// list/option is empty). `None` for non-nested fields.
    pub nested_reference_targets: Option<NestedRefCollector>,

    /// For a nested single-object field (`Nested` / `Option<Nested>`), the inner
    /// type's presence-masked config→wire, so `present_to_wire` masks the nested
    /// object to the keys the user actually wrote (not the whole struct with type
    /// defaults). Without it, presence masking would only apply one level deep.
    /// `None` for scalars and `Vec<Nested>`.
    pub nested_present: Option<fn(&serde_json::Value) -> anyhow::Result<serde_json::Value>>,

    /// Read the field's current value as a borrowed [`FieldRef`].
    pub get: fn(&T) -> FieldRef<'_>,

    /// Write a decoded [`FieldValue`] into the field, coercing to its concrete
    /// type; errors if the kinds disagree.
    pub set: fn(&mut T, FieldValue) -> anyhow::Result<()>,
}

/// How the engine pushes a resource's desired state upstream. Dispatched on at
/// apply time like [`CodecKind`] is at encode time; new API contracts add a
/// variant plus a hand-written arm in the apply engine. Independent of identity
/// (`#[key]`) and wire format (`codec`).
///
/// Not `PartialEq`: the `Custom` variant carries a fn-pointer, whose equality is
/// unpredictable (codegen may merge/split addresses). Match on it instead.
#[derive(Debug, Clone, Copy)]
pub enum SyncKind {
    /// Per-element CRUD against a collection endpoint: list, POST new, PUT
    /// changed, DELETE pruned. The *arr default.
    Crud,

    /// Single object behind one endpoint: GET current, PUT merged. No key.
    Singleton,

    /// Never synced directly — only flattened or nested inside a parent
    /// (variant structs, tagged/unit enums, shared envelopes). Carries
    /// [`Endpoints::NONE`].
    Embedded,

    /// Escape hatch for a write contract no reusable strategy fits: carries a
    /// hand-written [`CustomSync`](crate::CustomSync) hook that owns the whole
    /// reconcile (its own HTTP, ordering, idempotency).
    Custom(CustomSyncFn),
}

/// Codec-specific metadata attached to a [`ResourceDescriptor`]. Variants are
/// named after the codecs that consume them.
pub enum CodecMeta {
    /// Standard JSON codec needs no extra metadata.
    Standard,

    /// *arr fields-blob codec. `implementation` / `config_contract` are stamped
    /// onto every wire payload as discriminators.
    FieldsBlob {
        implementation: &'static str,
        config_contract: Option<&'static str>,
        protocol: Option<&'static str>,
    },

    /// Discriminator-dispatched enum; `discriminator` is the JSON key the codec
    /// reads to pick a variant (e.g. `"implementation"` for *arr).
    TaggedByImpl { discriminator: &'static str },
}
