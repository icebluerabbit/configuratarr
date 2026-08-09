//! `form::flatten` — form-urlencoded write serialization of a wire object.

use core_lib::form;
use serde_json::json;

#[test]
fn flattens_nested_objects_with_prefix() {
    let mut pairs = form::flatten(
        &json!({
            "general": { "use_sonarr": true, "minimum_score": 90 },
            "sonarr": { "ip": "10.0.0.5" },
        }),
        "settings",
    );
    pairs.sort();
    assert_eq!(
        pairs,
        vec![
            (
                "settings-general-minimum_score".to_string(),
                "90".to_string()
            ),
            (
                "settings-general-use_sonarr".to_string(),
                "true".to_string()
            ),
            ("settings-sonarr-ip".to_string(), "10.0.0.5".to_string()),
        ]
    );
}

#[test]
fn list_emits_one_pair_per_element_empty_clears() {
    assert_eq!(
        form::flatten(&json!({ "g": { "providers": ["a", "b"] } }), "s"),
        vec![
            ("s-g-providers".to_string(), "a".to_string()),
            ("s-g-providers".to_string(), "b".to_string()),
        ]
    );
    // Empty list → one empty value, so the server stores the cleared list.
    assert_eq!(
        form::flatten(&json!({ "g": { "providers": [] } }), "s"),
        vec![("s-g-providers".to_string(), String::new())]
    );
}

#[test]
fn scalars_render_by_convention() {
    // bool → "true"/"false", numbers decimal, strings verbatim.
    let mut v = form::flatten(&json!({ "b": true, "n": 8989, "s": "Daily" }), "x");
    v.sort();
    assert_eq!(
        v,
        vec![
            ("x-b".to_string(), "true".to_string()),
            ("x-n".to_string(), "8989".to_string()),
            ("x-s".to_string(), "Daily".to_string()),
        ]
    );
}

#[test]
fn empty_prefix_roots_at_the_key() {
    assert_eq!(
        form::flatten(&json!({ "a": 1 }), ""),
        vec![("a".to_string(), "1".to_string())]
    );
}
