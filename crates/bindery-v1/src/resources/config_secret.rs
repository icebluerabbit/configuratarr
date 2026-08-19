//! Shared helper for the `<field>` / `<field>Configured` singleton config
//! pattern used by [`crate::resources::abs_config`] and
//! [`crate::resources::grimmory_config`]: the read shape reports whether a
//! secret is stored via a boolean (`apiKeyConfigured`, `passwordConfigured`,
//! …), while the write shape takes the secret itself, and an empty/omitted
//! secret means "leave the stored one alone". A plain singleton merge (live ⊕
//! desired, PUT the merge) would compare `apiKeyConfigured: true` against the
//! write side's secret field forever and never converge — it would PUT the
//! declared secret on every single apply. [`plan_secret_config`] is the shared
//! decision both hooks need instead:
//!
//! 1. if any **non-secret** declared field differs from live → PUT everything
//!    the user declared (secrets included, exactly as declared — present or
//!    absent);
//! 2. else, for each secret whose live `*Configured` flag says it isn't
//!    stored yet, if the user declared a value for it → PUT just the
//!    secret(s) that need rotating in;
//! 3. else → unchanged.
//!
//! Caveat inherent to the API shape (documented again at each call site): an
//! in-place secret *rotation* with an otherwise-identical config is invisible
//! to this hook — `*Configured` stays `true` across a rotation, so case 2
//! never fires, and case 1 only fires when a non-secret field also changed.
//! There's no boolean that says "the stored secret is stale"; the API simply
//! doesn't expose one.

use serde_json::{Map, Value};

/// `true` when the live response's `configured_key` boolean says a secret is
/// already stored server-side.
fn is_configured(live: &Value, configured_key: &str) -> bool {
    live.get(configured_key)
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

/// `true` if any key in `wire` other than one of `secret_keys` differs from
/// the live value at the same key. Only compares keys **present** in `wire` —
/// callers pass a presence-masked wire object (see
/// `engine::config_present_to_wire`), so a field the user didn't declare is
/// never treated as a mismatch.
fn non_secret_differs(wire: &Value, live: &Value, secret_keys: &[&str]) -> bool {
    let Some(obj) = wire.as_object() else {
        return false;
    };
    obj.iter().any(|(k, v)| {
        if secret_keys.contains(&k.as_str()) {
            return false;
        }
        live.get(k) != Some(v)
    })
}

/// The write a `*Configured`-shaped singleton needs, decided from `wire`
/// (presence-masked declared config) vs `live` (the GET response).
#[derive(Debug, Clone, PartialEq)]
pub enum SecretConfigChange {
    /// Nothing to write.
    Unchanged,
    /// Non-secret fields differ — PUT the whole declared config, secrets
    /// exactly as the user wrote them (a field they didn't declare stays
    /// absent from the body, which the API treats as "leave it alone").
    Full(Value),
    /// Non-secret fields already match, but at least one declared secret
    /// isn't yet stored — PUT only the secret(s) that need rotating in.
    SecretOnly(Value),
}

/// Decide the write for a `*Configured`-shaped singleton. `secret_keys` names
/// every write-side secret field on the wire; `configured_keys` pairs each
/// with its live-side `*Configured` boolean key, e.g.
/// `[("apiKey", "apiKeyConfigured")]`.
pub fn plan_secret_config(
    wire: &Value,
    live: &Value,
    secret_keys: &[&str],
    configured_keys: &[(&str, &str)],
) -> SecretConfigChange {
    if non_secret_differs(wire, live, secret_keys) {
        return SecretConfigChange::Full(wire.clone());
    }
    let mut only = Map::new();
    for (secret_key, configured_key) in configured_keys {
        if is_configured(live, configured_key) {
            continue;
        }
        if let Some(v) = wire.get(*secret_key) {
            only.insert((*secret_key).to_string(), v.clone());
        }
    }
    if only.is_empty() {
        SecretConfigChange::Unchanged
    } else {
        SecretConfigChange::SecretOnly(Value::Object(only))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn full_when_non_secret_differs() {
        let wire = json!({"baseUrl": "https://a", "apiKey": "secret"});
        let live = json!({"baseUrl": "https://b", "apiKeyConfigured": true});
        let change =
            plan_secret_config(&wire, &live, &["apiKey"], &[("apiKey", "apiKeyConfigured")]);
        assert_eq!(change, SecretConfigChange::Full(wire));
    }

    #[test]
    fn secret_only_when_not_configured() {
        let wire = json!({"baseUrl": "https://a", "apiKey": "secret"});
        let live = json!({"baseUrl": "https://a", "apiKeyConfigured": false});
        let change =
            plan_secret_config(&wire, &live, &["apiKey"], &[("apiKey", "apiKeyConfigured")]);
        assert_eq!(
            change,
            SecretConfigChange::SecretOnly(json!({"apiKey": "secret"}))
        );
    }

    #[test]
    fn unchanged_when_configured_and_matching() {
        let wire = json!({"baseUrl": "https://a"});
        let live = json!({"baseUrl": "https://a", "apiKeyConfigured": true});
        let change =
            plan_secret_config(&wire, &live, &["apiKey"], &[("apiKey", "apiKeyConfigured")]);
        assert_eq!(change, SecretConfigChange::Unchanged);
    }

    #[test]
    fn unchanged_when_secret_not_configured_but_not_declared() {
        // The user didn't declare `apiKey` at all — never PUT one they didn't write.
        let wire = json!({"baseUrl": "https://a"});
        let live = json!({"baseUrl": "https://a", "apiKeyConfigured": false});
        let change =
            plan_secret_config(&wire, &live, &["apiKey"], &[("apiKey", "apiKeyConfigured")]);
        assert_eq!(change, SecretConfigChange::Unchanged);
    }

    #[test]
    fn two_secrets_only_rotates_the_unconfigured_one() {
        let wire = json!({"apiKey": "key", "password": "pw"});
        let live = json!({"apiKeyConfigured": true, "passwordConfigured": false});
        let change = plan_secret_config(
            &wire,
            &live,
            &["apiKey", "password"],
            &[
                ("apiKey", "apiKeyConfigured"),
                ("password", "passwordConfigured"),
            ],
        );
        assert_eq!(
            change,
            SecretConfigChange::SecretOnly(json!({"password": "pw"}))
        );
    }
}
