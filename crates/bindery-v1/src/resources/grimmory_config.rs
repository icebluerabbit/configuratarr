//! `/api/v1/grimmory/config` — the Grimmory integration configuration.
//!
//! Same `*Configured` boolean read/write split as
//! [`crate::resources::abs_config`], but with **two** independently-tracked
//! secrets: `apiKeyConfigured` and `passwordConfigured`. `sync = custom`; see
//! [`crate::resources::config_secret`] for the shared decision this hook
//! delegates to.
//!
//! `serverVersion` is a read-only response field (reserved for a probed
//! server version; never populated by the config endpoints) this resource
//! never writes.
//!
//! Caveat: an in-place rotation of either secret with an otherwise-identical
//! config is not detected — its `*Configured` flag stays `true` across a
//! rotation, so nothing here signals that the stored value is stale.

use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine};
use core_macros::resource;
use serde_json::Value;

use crate::resources::config_secret::{self, SecretConfigChange};

const PATH: &str = "/api/v1/grimmory/config";
const SECRET_KEYS: &[&str] = &["apiKey", "password"];
const CONFIGURED_KEYS: &[(&str, &str)] = &[
    ("apiKey", "apiKeyConfigured"),
    ("password", "passwordConfigured"),
];

/// `/api/v1/grimmory/config` — the Grimmory integration configuration.
/// Singleton: exactly one entry, no natural key.
#[resource(sync = custom, list = get("/api/v1/grimmory/config"))]
pub struct GrimmoryConfig {
    /// Master switch for the integration.
    pub enabled: Option<bool>,
    /// Grimmory server URL; validated by a secure-URL check and stored
    /// normalized.
    pub base_url: Option<String>,
    /// API token. An absent/empty value leaves the stored key untouched.
    pub api_key: Option<SecretValue>,
    /// Login username. Echoed verbatim by the API — trimmed before storage,
    /// not treated as a secret there, so it's kept as a plain string here.
    pub username: Option<String>,
    /// Login password. An absent/empty value leaves the stored password
    /// untouched.
    pub password: Option<SecretValue>,
}

impl CustomSync for GrimmoryConfig {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(Vec::new());
            };
            let wire = engine::config_present_to_wire::<Self>(cfg)?;
            let live: Value = client.get(PATH).await?;

            match config_secret::plan_secret_config(&wire, &live, SECRET_KEYS, CONFIGURED_KEYS) {
                SecretConfigChange::Unchanged => Ok(vec![Change::unchanged("grimmory_config")]),
                SecretConfigChange::Full(body) => {
                    if execute {
                        let _: Value = client.put(PATH, &body).await?;
                    }
                    Ok(vec![Change::updated("grimmory_config")])
                }
                SecretConfigChange::SecretOnly(body) => {
                    let rotated = body
                        .as_object()
                        .map(|o| o.keys().cloned().collect::<Vec<_>>().join(", "))
                        .unwrap_or_default();
                    if execute {
                        let _: Value = client.put(PATH, &body).await?;
                    }
                    Ok(vec![
                        Change::updated("grimmory_config").with("rotated", rotated),
                    ])
                }
            }
        })
    }
}
