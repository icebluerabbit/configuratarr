//! `engine::config_present_to_wire` — presence masking, including **nested**
//! single objects.
//!
//! A singleton's typed struct fills omitted fields with type defaults; the mask
//! drops everything the user didn't write so "omitted = unmanaged" holds on
//! merge. The nested-recursion case is the interesting one: a declared section
//! must carry only its declared inner keys, not the inner struct's defaults
//! (which would clobber the live values for keys the user never touched).

use core_lib::engine;
use core_macros::{nested, resource, wire_enum};
use serde_json::json;

// Non-`Option` inner fields, so an omitted one still *encodes* to its type
// default — the masking is what must remove it, proving the recursion.
#[nested]
pub struct Section {
    pub a: bool,
    pub b: i32,
    pub c: String,
}

#[resource(sync = singleton, read = get("/s"), update = put("/s"))]
pub struct Cfg {
    pub section: Option<Section>,
    pub top: bool,
}

#[test]
fn masks_nested_object_to_declared_inner_keys() {
    // User declares only `section.a` (and `top`) — not `section.b` / `section.c`.
    let cfg = serde_json::json!({ "section": { "a": true }, "top": false });
    let wire = engine::config_present_to_wire::<Cfg>(&cfg).unwrap();
    // Recursion masks the inner object: only `a` survives. Without it, `b`/`c`
    // would appear as `0` / `""` and clobber the live values on merge.
    assert_eq!(
        wire,
        serde_json::json!({ "section": { "a": true }, "top": false })
    );
}

#[test]
fn absent_nested_section_is_omitted() {
    let cfg = serde_json::json!({ "top": true });
    let wire = engine::config_present_to_wire::<Cfg>(&cfg).unwrap();
    assert_eq!(wire, serde_json::json!({ "top": true }));
}

// ── wire-enum fields ─────────────────────────────────────────────────────────

/// A `#[wire_enum]` is a named type in the AST, exactly like a `#[nested]`
/// struct, so the macro can't tell them apart and emits `nested_present` for
/// both. The config value of a wire-enum field is a bare **string**, though, so
/// recursing into it as if it were an object used to fail with
/// `present_to_wire expected a JSON object`. Guard: only recurse into objects.
#[wire_enum]
pub enum Mode {
    Fast,
    Slow,
    #[fallback]
    Unknown,
}

#[nested]
pub struct Inner {
    pub mode: Option<Mode>,
    pub label: Option<String>,
}

#[resource(
    sync = singleton,
    read = get("/api/v3/enumcfg"),
    update = put("/api/v3/enumcfg"),
)]
pub struct EnumCfg {
    pub mode: Option<Mode>,
    pub inner: Option<Inner>,
    pub flag: Option<bool>,
}

#[test]
fn wire_enum_field_is_not_treated_as_nested() {
    let cfg = json!({ "mode": "Slow" });
    let wire = engine::config_present_to_wire::<EnumCfg>(&cfg).unwrap();
    assert_eq!(wire, json!({ "mode": "Slow" }));
}

#[test]
fn wire_enum_inside_a_nested_struct_still_masks() {
    // The nested struct *is* an object, so it recurses — and the enum inside it
    // must survive that recursion. Only the key the user wrote comes out.
    let cfg = json!({ "inner": { "mode": "Fast" } });
    let wire = engine::config_present_to_wire::<EnumCfg>(&cfg).unwrap();
    assert_eq!(wire, json!({ "inner": { "mode": "Fast" } }));
}
