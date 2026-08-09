//! Service descriptors emitted by the `#[service]` macro. A service is a bag of
//! resources (`Vec<R>` collections, `Option<R>` singletons) plus a connection
//! bundle; the engine reads both to drive plan/apply without the concrete types.

use crate::described::ResourceErased;
use crate::secret::SecretValue;

/// User-facing trait, implemented by the `#[service]` macro.
pub trait Service: Sized + 'static {
    fn descriptor() -> &'static ServiceDescriptor<Self>;
    fn connection(&self) -> Connection<'_>;

    /// Build an instance from one resolved config-file entry. Connection scalars
    /// are read directly; resource fields decode through the config codec
    /// ([`crate::engine::decode_config`]).
    fn from_config(value: &serde_json::Value) -> anyhow::Result<Self>;
}

/// Static description of one managed service.
pub struct ServiceDescriptor<S: 'static> {
    /// Snake-case service identifier, e.g. `"radarr_v3"`.
    pub name: &'static str,

    /// The struct's `///` comment, for doc-gen; unused by the runtime engine.
    pub doc: Option<&'static str>,

    /// Path GETed after connecting to validate it (e.g. `/api/v3/system/status`).
    pub health_check: Option<&'static str>,

    /// Resource-holding fields in declaration order.
    pub fields: &'static [ServiceField<S>],
}

/// One resource-bearing field on a service struct. The `fn()` accessors read the
/// resource type's descriptor without naming the concrete type here.
pub struct ServiceField<S: 'static> {
    pub name: &'static str,
    pub doc: Option<&'static str>,

    /// The resource's snake-case type name (`${ref}` target, dep-graph node).
    pub type_name: &'static str,

    /// The resource's declared sync strategy (a `Custom` carries its hook). The
    /// executor dispatches on this, *not* on the field's `Vec`/`Option` shape.
    pub sync: fn() -> crate::SyncKind,

    /// Type-erased iterator over the resource(s) — each `Vec<R>` element, or
    /// zero/one for `Option<R>`.
    pub iter: for<'a> fn(&'a S) -> Box<dyn Iterator<Item = &'a dyn ResourceErased> + 'a>,

    /// The types this resource references (`#[reference(t)]`), read statically so
    /// apply ordering works even for empty collections.
    pub ref_targets: fn() -> Vec<&'static str>,

    /// The resource's HTTP operations.
    pub endpoints: fn() -> crate::Endpoints,

    /// Wire name of the natural key, to match desired vs live. `None` for singletons.
    pub key_wire: fn() -> Option<String>,

    /// One resource's config `Value` (`${ref}` resolved) → wire `Value`
    /// (`decode_config` then `encode`).
    pub config_to_wire: fn(&serde_json::Value) -> anyhow::Result<serde_json::Value>,

    /// Like [`Self::config_to_wire`] but keeps only the keys the user wrote, so a
    /// singleton's default-filled fields don't clobber live server values.
    pub config_to_wire_present: fn(&serde_json::Value) -> anyhow::Result<serde_json::Value>,

    /// Wire-key names of secret fields, to redact credentials in plan output.
    pub secret_keys: fn() -> Vec<String>,

    /// The resource's documentation (fields + provider variants), for config-doc-gen.
    pub resource_docs: fn() -> crate::engine::ResourceDoc,

    /// The resource struct's `///` comment, for the doc section intro.
    pub type_doc: fn() -> Option<&'static str>,
}

/// Instance-level connection bundle. Borrows from the service instance (any
/// interpolations were resolved earlier); the engine reads it to build the client.
pub struct Connection<'a> {
    pub url: &'a String,
    pub auth: Auth<'a>,

    /// Skip TLS verification. `None` = verify (default); `Some(true)` = skip.
    pub insecure: Option<bool>,

    /// Request timeout in seconds. `None` = engine default.
    pub timeout_secs: Option<u64>,
}

/// Auth scheme + credential borrows for one instance. Each variant carries its
/// static metadata + runtime credentials; `connect` matches on it.
pub enum Auth<'a> {
    None,

    /// API key in a custom header. Covers almost every supported service.
    ApiKey {
        header: &'static str,
        key: &'a SecretValue,
    },

    /// API key carried as a URL query parameter on every request (e.g.
    /// LazyLibrarian's `?apikey=`). [`connect`](crate::apply::connect) registers
    /// it as a default query pair on the client so it rides every request,
    /// including the authenticated health check.
    ApiKeyQuery {
        param: &'static str,
        key: &'a SecretValue,
    },

    /// Bearer token in the `Authorization` header.
    Bearer {
        token: &'a SecretValue,
    },

    /// HTTP Basic auth.
    Basic {
        user: &'a String,
        pass: &'a SecretValue,
    },

    /// Form/cookie auth: [`connect`](crate::apply::connect) POSTs the
    /// credentials as an `username`/`password` form to `login_path`, and the
    /// client's cookie store reuses the returned session for every request.
    FormCookie {
        login_path: &'static str,
        user: &'a String,
        pass: &'a SecretValue,
    },
    // OAuth2 + Custom(Box<dyn AuthScheme>) land with the async auth work.
}
