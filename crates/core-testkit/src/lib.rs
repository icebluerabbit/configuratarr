//! Shared test harness for service crates — the service-agnostic mechanics that
//! every `<service>/tests/` would otherwise copy verbatim.
//!
//! * [`check`] — spec-conformance: run a config fixture through the real encode
//!   path and validate the wire payload against the service's OpenAPI schema.
//! * [`DummyEnv`] / [`DummyRefs`] — resolver doubles for conformance (values
//!   don't matter, only the resulting shape).
//! * [`env_pair`] / [`instance`] — e2e wiring: read credentials from the
//!   environment and build a connection-only service from them.
//!
//! Each service keeps only the declarative part — its spec path, the
//! resource→schema list, and its env-var names.

use core_lib::resolver::{RefSource, StaticEnv};
use core_lib::{Described, Service, engine};
use serde_json::Value;

// ── spec-conformance ─────────────────────────────────────────────────────────

/// A [`StaticEnv`] that resolves every `${env}` / `${file}` to `"dummy"`. The
/// conformance harness only cares about the resulting *shape*, not the values.
pub struct DummyEnv;

impl StaticEnv for DummyEnv {
    fn env(&self, _: &str) -> Option<&str> {
        Some("dummy")
    }
    fn file(&self, _: &str) -> anyhow::Result<String> {
        Ok("dummy".into())
    }
}

/// A [`RefSource`] that resolves every `${ref}` to the integer id `1` — the
/// right shape for the *arr APIs, whose ids are `i32`.
///
/// A service with **GUID ids** needs [`GuidRefs`] instead, via
/// [`check_with_refs`]: an `Int` substituted into a `String`-typed FK fails
/// decode with `expected string, got 1`.
pub struct DummyRefs;

impl RefSource for DummyRefs {
    fn lookup(&self, _: &str, _: &str) -> Option<core_lib::RefId> {
        Some(core_lib::RefId::Int(1))
    }
}

/// A [`RefSource`] for GUID-id services: resolves every `${ref}` to a fixed,
/// obviously-fake UUID string. Pair it with [`check_with_refs`].
pub struct GuidRefs;

/// The placeholder [`GuidRefs`] substitutes. Shaped like a real UUID so any
/// format validation in a spec still passes, but recognisable in a failure.
pub const DUMMY_GUID: &str = "00000000-0000-0000-0000-000000000001";

impl RefSource for GuidRefs {
    fn lookup(&self, _: &str, _: &str) -> Option<core_lib::RefId> {
        Some(core_lib::RefId::Str(DUMMY_GUID.to_string()))
    }
}

/// Parse a config fixture, dummy-resolve its interpolations, encode it through
/// the real path (`decode_config` → `encode`), and validate the wire payload
/// against the named component schema from `spec`. Panics with a field-level
/// message on any mismatch — the radarr schemas are `additionalProperties:
/// false`, so a hallucinated, mis-cased, or mis-typed field fails.
///
/// `${ref}` resolves through [`DummyRefs`] (integer ids). For a GUID-id service,
/// use [`check_with_refs`] with [`GuidRefs`].
pub fn check<T: Described>(spec: &Value, schema_name: &str, yaml: &str) {
    check_with_refs::<T>(spec, schema_name, yaml, &DummyRefs);
}

/// [`check`], with the `${ref}` resolver supplied by the caller.
///
/// The id *type* a reference resolves to has to match the FK field's Rust type,
/// and that varies by service: the *arr APIs use `i32`, Jellyfin and Cleanuparr
/// use GUID strings. One fixed double can't serve both, so the service picks.
pub fn check_with_refs<T: Described>(
    spec: &Value,
    schema_name: &str,
    yaml: &str,
    refs: &impl RefSource,
) {
    let mut cfg: Value =
        serde_saphyr::from_str(yaml).unwrap_or_else(|e| panic!("{schema_name}: yaml parse: {e}"));
    core_lib::resolve::resolve_static(&mut cfg, &DummyEnv)
        .unwrap_or_else(|e| panic!("{schema_name}: static resolve: {e}"));
    core_lib::resolve::resolve_refs(&mut cfg, refs)
        .unwrap_or_else(|e| panic!("{schema_name}: ref resolve: {e}"));

    let decoded = engine::decode_config::<T>(&cfg)
        .unwrap_or_else(|e| panic!("{schema_name}: config decode failed: {e}"));
    let wire =
        engine::encode(&decoded).unwrap_or_else(|e| panic!("{schema_name}: encode failed: {e}"));

    // Codec round-trip invariant: decode the wire back and re-encode — it must be
    // byte-stable. Catches asymmetric codecs (a field that decodes but doesn't
    // re-encode, or whose value silently changes shape) — which pure schema
    // validation misses, since e.g. a dropped array still validates as an array.
    let redecoded = engine::decode::<T>(&wire)
        .unwrap_or_else(|e| panic!("{schema_name}: wire re-decode failed: {e}"));
    let wire2 = engine::encode(&redecoded)
        .unwrap_or_else(|e| panic!("{schema_name}: re-encode failed: {e}"));
    if wire != wire2 {
        panic!(
            "{schema_name}: codec is not idempotent (decode→encode unstable):\n  first:  {}\n  second: {}",
            serde_json::to_string_pretty(&wire).unwrap(),
            serde_json::to_string_pretty(&wire2).unwrap(),
        );
    }

    // Self-contained schema: component + the whole components map injected so
    // internal `$ref: #/components/schemas/...` resolve against it.
    let mut schema = spec["components"]["schemas"][schema_name].clone();
    assert!(
        !schema.is_null(),
        "schema `{schema_name}` not found in spec"
    );
    schema["components"] = spec["components"].clone();

    let validator = jsonschema::validator_for(&schema).expect("schema compiles");
    let errors: Vec<String> = validator
        .iter_errors(&wire)
        .map(|e| format!("  at /{}: {e}", e.instance_path()))
        .collect();
    if !errors.is_empty() {
        panic!(
            "{schema_name}: encoded payload does not conform:\n{}\n\npayload: {}",
            errors.join("\n"),
            serde_json::to_string_pretty(&wire).unwrap(),
        );
    }
}

// ── e2e wiring ───────────────────────────────────────────────────────────────

/// `(url, api_key)` read from the named environment variables, or `None` when
/// either is unset — the signal to skip an `#[ignore]`d e2e test outside the
/// dev shell.
pub fn env_pair(url_var: &str, key_var: &str) -> Option<(String, String)> {
    Some((std::env::var(url_var).ok()?, std::env::var(key_var).ok()?))
}

/// Build a connection-only service `S` plus the full instance `Value` the
/// executor reads desired state from: the `url` + `api_key` connection merged
/// with `resources` (a JSON object of resource-type → desired list/singleton).
///
/// Assumes the *arr-style `api_key` connection shared by the supported services.
pub fn instance<S: Service>(url: &str, key: &str, resources: Value) -> (S, Value) {
    let mut value = serde_json::json!({ "url": url, "api_key": key });
    if let Value::Object(extra) = resources {
        value.as_object_mut().unwrap().extend(extra);
    }
    let svc = S::from_config(&value).expect("connection decodes");
    (svc, value)
}
