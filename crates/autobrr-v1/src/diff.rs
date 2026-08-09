//! Idempotency predicate shared by autobrr's `sync = custom` resources.
//!
//! autobrr's collections don't round-trip a write: unset lists read back as
//! `null`/absent, unset scalars as `""`, nilable `*bool`s as `null`, secrets are
//! redacted, and the server adds ids to nested items. Plain merge-equality (what
//! `sync = crud` uses) would therefore report a perpetual "update". So these
//! resources converge on a **structural subset** test instead — the desired wire
//! is in sync when every declared key/value is already present in live, ignoring
//! extra server-added keys.
//!
//! This is autobrr *policy*, not engine mechanism: the recurring create/update
//! skeleton lives in [`core_lib::reconcile::upsert`]; this decides what "already
//! matches" means for autobrr. One home, imported by every custom resource, so it
//! isn't reinvented per file.

use serde_json::Value;

/// An empty declared value — one that can't meaningfully differ from an autobrr
/// server default (unset lists come back `null`/absent, unset scalars `""`).
/// Treated as always in sync, since the typed `Vec`/`Option` fields can't
/// distinguish "declared empty" from "omitted".
pub(crate) fn is_empty(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        _ => false,
    }
}

/// True when every value in `want` is already present (structurally) in `have`:
/// an empty declared value is always satisfied; objects match key-by-key on
/// `want`'s keys (extra `have` keys — e.g. server ids — ignored); arrays match
/// element-wise; scalars compare numeric-insensitively.
pub(crate) fn subset(want: &Value, have: &Value) -> bool {
    if is_empty(want) {
        return true;
    }
    // autobrr stores an unset nilable `*bool` as null, so a declared `false`
    // reads back as null (or false). Treat those as equal. Non-nullable bools
    // (e.g. irc `enabled`) always read back as a real boolean, never null, so a
    // genuine `true → false` toggle still diffs and triggers an update.
    if want == &Value::Bool(false) && (have.is_null() || have == &Value::Bool(false)) {
        return true;
    }
    match (want, have) {
        // A key absent from `have` is compared as null, so an empty/`false`
        // declared value (which autobrr drops on write) still counts as in sync.
        (Value::Object(w), Value::Object(h)) => w
            .iter()
            .all(|(k, wv)| subset(wv, h.get(k).unwrap_or(&Value::Null))),
        (Value::Array(w), Value::Array(h)) => {
            w.len() == h.len() && w.iter().zip(h).all(|(wv, hv)| subset(wv, hv))
        }
        _ => match (want.as_f64(), have.as_f64()) {
            (Some(a), Some(b)) => a == b,
            _ => want == have,
        },
    }
}
