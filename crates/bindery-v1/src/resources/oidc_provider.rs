//! `/api/v1/auth/oidc/providers` — OIDC identity providers, managed as a
//! whole set.
//!
//! `GET` returns `OIDCProviderWithStatus`: the public config plus a runtime
//! `status`, with `client_secret` always stripped (write-only, never
//! returned). `PUT` takes the **entire** array and does a server-side secret
//! merge: sending `client_secret: ""` for an `id` that already exists
//! preserves its stored secret, while omitting the key for an `id` that
//! doesn't exist yet is a 400. Which secret to send therefore depends on
//! whether the id is already live — a plain collection (`sync = crud`) has no
//! way to express that per-item conditional, hence `sync = custom`.
//!
//! Identity for the diff is `(id, name, issuer, client_id, sorted scopes,
//! sorted allowed_groups)` — deliberately dropping the read-only `status`
//! (live side only) and the write-only `client_secret` (never round-trips, so
//! comparing it would churn on every apply).
//!
//! The endpoint owns the whole set in one PUT, so there's no separate delete
//! path: `--prune` is meaningless here — a provider absent from config is
//! simply absent from the array this hook sends, which already removes it
//! server-side.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine};
use core_macros::resource;
use serde_json::Value;
use std::collections::HashSet;

/// `/api/v1/auth/oidc/providers` — one OIDC identity provider. `case = snake`
/// — the API's JSON keys (`client_id`, `allowed_groups`, …) are the snake
/// field names verbatim, not camelCase.
#[resource(sync = custom, case = snake, list = get("/api/v1/auth/oidc/providers"))]
pub struct OidcProvider {
    /// Stable provider id used in the login/callback paths
    /// (`^[a-z0-9_-]{1,32}$`).
    #[key]
    pub id: String,
    /// Display name rendered on the login page.
    pub name: Option<String>,
    /// OIDC issuer URL — discovery is fetched from
    /// `<issuer>/.well-known/openid-configuration`.
    pub issuer: String,
    /// OAuth2 client id.
    pub client_id: String,
    /// OAuth2 client secret. Write-only, never read back. The reconcile hook
    /// sends this verbatim only for a provider id that isn't live yet; for
    /// one that already exists it always sends `""` instead, to preserve the
    /// stored secret regardless of what's declared here.
    pub client_secret: Option<SecretValue>,
    /// Requested scopes. Empty falls back to the server's `openid profile
    /// email` default.
    pub scopes: Vec<String>,
    /// When non-empty, a login is admitted only if the user's group claim
    /// intersects this list.
    pub allowed_groups: Vec<String>,
}

fn str_field(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

fn sorted_str_array(v: &Value, key: &str) -> Vec<String> {
    let mut a: Vec<String> = v
        .get(key)
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    a.sort();
    a
}

/// Order-insensitive identity for the set diff: `(id, name, issuer,
/// client_id, sorted scopes, sorted allowed_groups)`. Deliberately excludes
/// `status` (read-only, live side only) and `client_secret` (write-only,
/// never round-trips).
fn provider_identity(v: &Value) -> (String, String, String, String, Vec<String>, Vec<String>) {
    (
        str_field(v, "id"),
        str_field(v, "name"),
        str_field(v, "issuer"),
        str_field(v, "client_id"),
        sorted_str_array(v, "scopes"),
        sorted_str_array(v, "allowed_groups"),
    )
}

impl CustomSync for OidcProvider {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/v1/auth/oidc/providers").await?;
            let live_ids: HashSet<String> = live
                .iter()
                .filter_map(|v| v.get("id").and_then(Value::as_str).map(String::from))
                .collect();

            // Encode each declared provider to its wire (snake) shape, then fix
            // up `client_secret` per the server's merge contract: preserve the
            // stored secret ("") for an id already live, send the declared
            // secret verbatim for a new one.
            let mut wire: Vec<Value> = Vec::with_capacity(desired.len());
            for cfg in desired {
                let mut w = engine::encode_config::<Self>(cfg)?;
                if let Some(obj) = w.as_object_mut() {
                    let id = obj
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    let secret = if live_ids.contains(&id) {
                        Value::String(String::new())
                    } else {
                        obj.get("client_secret")
                            .cloned()
                            .unwrap_or_else(|| Value::String(String::new()))
                    };
                    obj.insert("client_secret".to_string(), secret);
                }
                wire.push(w);
            }

            let to_put = wire.clone();
            let http = client.clone();
            reconcile::replace(
                &wire,
                &live,
                "oidc_providers",
                execute,
                provider_identity,
                move || async move {
                    let _: Value = http
                        .put("/api/v1/auth/oidc/providers", &Value::Array(to_put))
                        .await?;
                    Ok(())
                },
            )
            .await
        })
    }
}
