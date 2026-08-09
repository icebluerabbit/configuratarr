//! Idempotency predicate shared by Cleanuparr's `sync = custom` resources.
//!
//! Cleanuparr's collections don't round-trip a write. Two reasons:
//!
//! 1. **Secrets read back masked.** Every `[SensitiveData]` property serialises
//!    as the placeholder [`MASK`] (Apprise service URLs keep their scheme:
//!    `discord://••••••••`). Sending the placeholder back is how the API says
//!    "keep the stored value", so the mask is a protocol feature, not noise.
//! 2. **Reads are enriched.** A GET carries server-owned keys the write contract
//!    has no field for — `id`, `queueCleanerConfigId`, `instanceName`,
//!    `lastProcessedAt`, and so on.
//!
//! Plain merge-equality (what `sync = crud` uses) would therefore report a
//! perpetual "update". These resources converge on a **structural subset** test
//! instead: the desired wire is in sync when every declared key/value is already
//! present in live, ignoring extra server-added keys, and treating a masked live
//! value as satisfied.
//!
//! This is Cleanuparr *policy*, not engine mechanism: the recurring create/update
//! skeleton lives in [`core_lib::reconcile::upsert`]; this decides what "already
//! matches" means here. One home, imported by every custom resource, so it isn't
//! reinvented per file.
//!
//! **Known limitation** (shared with `autobrr-v1`): because a live secret is
//! always masked, *rotating* a secret in config is invisible to this predicate —
//! the update won't fire on the secret alone. Change a non-secret field
//! alongside it, or delete and recreate the resource.

use serde_json::Value;

/// The placeholder Cleanuparr substitutes for every `[SensitiveData]` property
/// on read (`SensitiveDataHelper.Placeholder`). Sending it back in an update
/// preserves the stored value.
pub const MASK: &str = "••••••••";

/// True when a live value is (or contains) the redaction placeholder — e.g. a
/// masked api key, or an Apprise URL rendered as `discord://••••••••`.
pub(crate) fn is_masked(v: &Value) -> bool {
    v.as_str().is_some_and(|s| s.contains(MASK))
}

/// An empty declared value that can't meaningfully differ from a server default.
///
/// Only `null` and `[]` qualify. A `Vec` field can't distinguish "declared
/// empty" from "omitted" — both decode to `vec![]` — so an empty list has to be
/// read as "unmanaged", or removing every entry from a list would be impossible
/// to express *and* every omitted list would report drift.
///
/// An empty **string** deliberately does not qualify: `""` is a value the user
/// typed, and swallowing it would make clearing a field impossible. That is safe
/// here because no `sync = custom` resource — the only kind that uses this
/// module — has a non-`Option` string field defaulting to `""`; the two that do
/// (`oidc`, `slow_rule`) are a presence-masked singleton and a merge-based crud
/// resource respectively, neither of which routes through [`subset`].
pub(crate) fn is_empty(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::Array(a) => a.is_empty(),
        _ => false,
    }
}

/// True when every value in `want` is already present (structurally) in `have`:
/// an empty declared value is always satisfied; a masked live value is always
/// satisfied; objects match key-by-key on `want`'s keys (extra `have` keys —
/// server ids, enrichment — ignored); arrays match element-wise; scalars compare
/// numeric-insensitively so `3` and `3.0` agree.
pub(crate) fn subset(want: &Value, have: &Value) -> bool {
    if is_empty(want) || is_masked(have) {
        return true;
    }
    match (want, have) {
        // A key absent from `have` is compared as null, so an empty declared
        // value still counts as in sync.
        (Value::Object(w), Value::Object(h)) => w
            .iter()
            .all(|(k, wv)| subset(wv, h.get(k).unwrap_or(&Value::Null))),
        (Value::Array(w), Value::Array(h)) => {
            w.len() == h.len() && w.iter().zip(h).all(|(wv, hv)| subset(wv, hv))
        }
        _ => match (want.as_f64(), have.as_f64()) {
            (Some(a), Some(b)) => a == b,
            _ => match (want.as_str(), have.as_str()) {
                (Some(w), Some(h)) => same_url_or_string(w, h),
                _ => want == have,
            },
        },
    }
}

/// [`subset`], but for an array that is a **keyed sub-collection** rather than a
/// positional list: each declared element must match *some* live element with
/// the same `key` value, and live elements the config doesn't declare are
/// ignored.
///
/// [`subset`]'s array rule is length-equality plus element-wise comparison,
/// which is right for a positional list (`topics`, `categories`) and wrong for a
/// keyed one. Cleanuparr's seeker `instances` are keyed by `arrInstanceId` and
/// upserted server-side — the API returns an entry per *arr instance whether the
/// config mentions it or not, and omitted entries keep their stored settings. So
/// the positional rule would see a length mismatch the moment a single unmanaged
/// instance exists and report drift on every apply, forever.
///
/// Returns `false` if any declared element is missing its `key`, or has no live
/// counterpart.
pub(crate) fn subset_keyed(want: &[Value], have: &[Value], key: &str) -> bool {
    want.iter().all(|w| {
        let Some(k) = w.get(key) else {
            return false;
        };
        have.iter()
            .find(|h| h.get(key) == Some(k))
            .is_some_and(|h| subset(w, h))
    })
}

/// Scalar string equality, tolerating the trailing slash .NET adds to a URL.
///
/// Cleanuparr stores these fields as `System.Uri` and serialises them with
/// `ToString()`, which normalises an authority-only URL by appending `/`: a
/// declared `http://sonarr:8989` reads back as `http://sonarr:8989/`. Comparing
/// those verbatim would report an update on every single apply, forever.
fn same_url_or_string(want: &str, have: &str) -> bool {
    want == have || want.trim_end_matches('/') == have.trim_end_matches('/')
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn masked_live_secret_is_in_sync() {
        assert!(subset(&json!("s3cret"), &json!(MASK)));
        assert!(subset(
            &json!("discord://tok"),
            &json!(format!("discord://{MASK}"))
        ));
    }

    #[test]
    fn extra_live_keys_are_ignored() {
        let want = json!({ "name": "qbit", "enabled": true });
        let have = json!({ "id": "3f2b…", "name": "qbit", "enabled": true, "type": "Torrent" });
        assert!(subset(&want, &have));
    }

    #[test]
    fn declared_drift_is_detected() {
        let want = json!({ "name": "qbit", "enabled": true });
        let have = json!({ "name": "qbit", "enabled": false });
        assert!(!subset(&want, &have));
    }

    #[test]
    fn dotnet_trailing_slash_on_urls_is_not_drift() {
        // `new Uri("http://sonarr:8989").ToString()` == "http://sonarr:8989/"
        assert!(subset(
            &json!("http://sonarr:8989"),
            &json!("http://sonarr:8989/")
        ));
        assert!(subset(
            &json!("http://sonarr:8989/"),
            &json!("http://sonarr:8989")
        ));
        // A genuinely different host still diffs.
        assert!(!subset(
            &json!("http://sonarr:8989"),
            &json!("http://sonarr:9999")
        ));
    }

    #[test]
    fn declared_empty_string_is_a_clear_not_a_no_op() {
        // `""` is a value the user typed; swallowing it would make clearing a
        // field impossible.
        assert!(!subset(&json!(""), &json!("something")));
        assert!(subset(&json!(""), &json!("")));
    }

    #[test]
    fn declared_empty_list_is_unmanaged() {
        // A `Vec` can't distinguish "declared empty" from "omitted", so an empty
        // list stays permissive.
        assert!(subset(&json!([]), &json!(["a", "b"])));
    }

    #[test]
    fn keyed_arrays_ignore_undeclared_live_entries() {
        let want = json!([{ "arrInstanceId": "a", "enabled": true }]);
        let have = json!([
            { "arrInstanceId": "b", "enabled": false, "instanceName": "other" },
            { "arrInstanceId": "a", "enabled": true, "instanceName": "mine" },
        ]);
        // Positional comparison would fail on the length mismatch alone.
        assert!(!subset(&want, &have));
        assert!(subset_keyed(
            want.as_array().unwrap(),
            have.as_array().unwrap(),
            "arrInstanceId"
        ));
    }

    #[test]
    fn keyed_arrays_still_catch_drift_and_missing_entries() {
        let want = json!([{ "arrInstanceId": "a", "enabled": true }]);
        let drifted = json!([{ "arrInstanceId": "a", "enabled": false }]);
        let missing = json!([{ "arrInstanceId": "z", "enabled": true }]);
        for have in [&drifted, &missing] {
            assert!(!subset_keyed(
                want.as_array().unwrap(),
                have.as_array().unwrap(),
                "arrInstanceId"
            ));
        }
    }

    #[test]
    fn numeric_forms_agree() {
        assert!(subset(&json!(3), &json!(3.0)));
    }

    #[test]
    fn arrays_match_elementwise() {
        assert!(subset(&json!(["a", "b"]), &json!(["a", "b"])));
        assert!(!subset(&json!(["a", "b"]), &json!(["a", "c"])));
        assert!(!subset(&json!(["a"]), &json!(["a", "b"])));
    }
}
