//! `/api/v1/auth/config` (read) / `/api/v1/auth/mode` (write) — the server's
//! authentication mode.
//!
//! The read and write paths are split and shaped differently: `GET
//! /api/v1/auth/config` returns `{mode, apiKey, username}` — the current admin
//! API key and username ride along — while `PUT /api/v1/auth/mode` takes only
//! `{mode}`. A plain singleton (`sync = singleton`) merges live ⊕ desired and
//! PUTs the merged body; here that would ship the live `apiKey` into
//! `Op.body` on every apply, which is unacceptable even though this resource
//! never declares an `api_key` field of its own — the merge doesn't know that,
//! it merges whatever the live GET returned. So this is `sync = custom`: GET,
//! compare only `.mode`, and PUT `{mode}` when it differs — `apiKey` and
//! `username` never enter the write path at all.

use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::{resource, wire_enum};
use serde_json::{Value, json};

/// The server's authentication mode.
#[wire_enum(rename_all = "kebab-case")]
pub enum AuthModeValue {
    /// Normal authentication: local accounts (plus OIDC, if configured).
    Enabled,
    /// Local accounts only — OIDC login is disabled even if providers are
    /// configured.
    LocalOnly,
    /// Authentication is fully disabled; every request is treated as admin.
    Disabled,
    /// Authentication is delegated to an upstream reverse proxy.
    Proxy,
    /// Unrecognised value. The server silently coerces this to `enabled`.
    #[fallback]
    Unknown,
}

/// `/api/v1/auth/config` / `/api/v1/auth/mode` — the server's authentication
/// mode. Singleton: exactly one entry, no natural key.
#[resource(sync = custom, list = get("/api/v1/auth/config"))]
pub struct AuthMode {
    /// The desired authentication mode.
    pub mode: AuthModeValue,
}

impl CustomSync for AuthMode {
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
            let wire = engine::encode_config::<Self>(cfg)?;
            let want = wire.get("mode").and_then(Value::as_str).unwrap_or_default();

            let live: Value = client.get("/api/v1/auth/config").await?;
            let have = live.get("mode").and_then(Value::as_str).unwrap_or_default();

            if want == have {
                return Ok(vec![Change::unchanged("auth_mode")]);
            }
            if execute {
                let _: Value = client
                    .put("/api/v1/auth/mode", &json!({ "mode": want }))
                    .await?;
            }
            Ok(vec![Change::updated("auth_mode").with("mode", want)])
        })
    }
}
