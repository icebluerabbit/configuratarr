//! Resolver traits passed to the two resolve phases.
//!
//! The split into [`StaticEnv`] and [`RefSource`] mirrors the two-phase apply
//! pipeline: static resolution (env vars + file reads + literal templates)
//! happens once up front, ref resolution happens per resource in topological
//! order as the engine learns ids from POST/PUT responses.

/// Read-only access to the static side of resolution.
///
/// `StaticEnv` is consulted during the static resolve phase, before any HTTP
/// I/O. It does not see resource ids — those are produced during apply and
/// flow through [`RefSource`] instead.
pub trait StaticEnv {
    /// Read an environment variable. `None` means unset; callers decide
    /// whether that's an error (e.g. template references it) or fine
    /// (defaulted).
    fn env(&self, name: &str) -> Option<&str>;

    /// Read a file's contents. Used for `${file./path}` templates. Errors
    /// propagate (e.g. permission denied, not found) so the user sees the
    /// real cause.
    fn file(&self, path: &str) -> anyhow::Result<String>;
}

/// The default [`StaticEnv`]: process environment + filesystem.
///
/// Environment is snapshotted at construction (the trait hands back `&str`, so
/// the values must outlive the call); files are read fresh on each lookup.
pub struct SystemEnv {
    vars: std::collections::HashMap<String, String>,
}

impl SystemEnv {
    /// Snapshot the current process environment.
    pub fn new() -> Self {
        Self {
            vars: std::env::vars().collect(),
        }
    }
}

impl Default for SystemEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl StaticEnv for SystemEnv {
    fn env(&self, name: &str) -> Option<&str> {
        self.vars.get(name).map(String::as_str)
    }

    fn file(&self, path: &str) -> anyhow::Result<String> {
        std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("reading `${{file.{path}}}`: {e}"))
    }
}

/// A server-assigned resource id: a `Number` for the *arr integer ids, a
/// `String` for GUID/string-id APIs (Jellyfin). This bounded type is what the
/// engine stores and substitutes into `${ref.*}` — an id is only ever an int or
/// a string, never a bool/array/object, so garbage is rejected at the boundary.
///
/// [`Pending`](RefId::Pending) is not a server id: it marks a create that hasn't
/// happened yet (a `plan` preview, or an `apply` create whose response carried no
/// id). It substitutes into `${ref.*}` as the placeholder matching its
/// [`IdShape`] so preview encoding still succeeds; a real `apply` never leaves a
/// ref `Pending` once the dependency has actually been created.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefId {
    Int(i64),
    Str(String),
    /// A not-yet-created id (preview, or a create with no id in its response),
    /// carrying the shape the real id will have so the placeholder still
    /// decodes into the field that consumes it.
    Pending(IdShape),
}

/// Whether a resource's server ids are integers or strings.
///
/// A [`Pending`](RefId::Pending) has no value to substitute, but it still has
/// to substitute *something* of the right JSON type: a `String` FK given an
/// integer placeholder fails decode with `expected string, got -1`, which
/// aborts the whole plan. Derived from the resource's `#[id]` field by
/// [`crate::engine::id_shape`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IdShape {
    /// Integer ids — every *arr, and the default when a resource declares no
    /// `#[id]` field.
    #[default]
    Int,
    /// String/GUID ids (Jellyfin, Komga, Audiobookshelf, Cleanuparr).
    Str,
}

impl RefId {
    /// The integer stand-in a [`Pending`](RefId::Pending) ref resolves to in a
    /// preview — the historical `-1` placeholder, kept in one place so no call
    /// site has to spell the sentinel.
    pub const PENDING: i64 = -1;

    /// The string stand-in, for a resource whose ids are strings. Spelled the
    /// same as [`PENDING`](Self::PENDING) so a plan reads identically either
    /// way; it is a placeholder, never a value any server would return.
    pub const PENDING_STR: &'static str = "-1";

    /// Read an id out of a live/create response value (`Number` or `String`);
    /// `None` for anything else (or a null/absent field). Never yields
    /// [`Pending`](RefId::Pending) — that is engine-internal, not a wire value.
    pub fn from_value(v: &serde_json::Value) -> Option<Self> {
        match v {
            serde_json::Value::Number(n) => n.as_i64().map(RefId::Int),
            serde_json::Value::String(s) => Some(RefId::Str(s.clone())),
            _ => None,
        }
    }

    /// The id as the JSON value substituted into a `${ref.*}` position, keeping
    /// its native wire type. [`Pending`](RefId::Pending) renders as the
    /// placeholder matching its [`IdShape`].
    pub fn to_value(&self) -> serde_json::Value {
        match self {
            RefId::Int(i) => serde_json::Value::from(*i),
            RefId::Str(s) => serde_json::Value::String(s.clone()),
            RefId::Pending(IdShape::Int) => serde_json::Value::from(Self::PENDING),
            RefId::Pending(IdShape::Str) => {
                serde_json::Value::String(Self::PENDING_STR.to_string())
            }
        }
    }
}

/// Read-only access to the live id map built during apply.
///
/// `RefSource` is consulted during the ref-resolve phase, just before
/// encoding each resource. The engine populates it from each create/update
/// response: after `Tag { label: "4k" }` is POSTed and the server returns
/// `id: 3`, the engine inserts `("tag", "4k") -> RefId::Int(3)` so downstream
/// resources referencing `${ref.tag.4k}` can resolve.
pub trait RefSource {
    /// Look up an id by (type, key). It is substituted into `${ref.*}` positions
    /// as its native wire value, so the FK keeps its type. `None` means the
    /// target hasn't been applied yet — a bug if topo ordering is correct, since
    /// the dep graph guarantees deps come first.
    fn lookup(&self, type_name: &str, key: &str) -> Option<RefId>;
}
