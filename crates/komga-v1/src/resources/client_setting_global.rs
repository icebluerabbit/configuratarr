//! `/api/v1/client-settings/global` — server-wide client settings: values every
//! Komga client (web UI, mobile apps) reads, optionally exposed to unauthorized
//! (anonymous) requests via `allowUnauthorized`.
//!
//! Keys are dotted namespace strings (`"application.domain.key"`, per the
//! Komga API docs). The list/write/diff mechanics — object-map GET, upsert-merge
//! PATCH, no prune — are shared with the user-scoped settings and live in
//! [`crate::resources::client_settings`]; read that module's docs first.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

use crate::resources::client_settings;

/// One global (server-wide) client setting.
#[resource(sync = custom, list = get("/api/v1/client-settings/global/list"))]
pub struct ClientSettingGlobal {
    /// Dotted setting key, e.g. `"application.deletion.protection"` — its
    /// identity.
    #[key]
    pub key: String,
    /// The setting's value (always a plain string on the wire — the API
    /// documents a JSON-object value as a JSON-*encoded string*, not a nested
    /// object).
    pub value: String,
    /// Whether an unauthorized (anonymous) client may read this setting.
    /// Required by the API on every write; defaults to `false` when the config
    /// omits it.
    #[default(false)]
    pub allow_unauthorized: bool,
}

impl CustomSync for ClientSettingGlobal {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        // No body-carrying DELETE on `HttpClient` — see client_settings.rs.
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let map: Value = client.get("/api/v1/client-settings/global/list").await?;
            let live = client_settings::live_list(&map);

            let wire: Vec<Value> = desired
                .iter()
                .map(core_lib::engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert(
                &wire,
                &live,
                "key",
                |desired, live| {
                    let value_matches = desired.get("value") == live.get("value");
                    let auth_matches = desired
                        .get("allowUnauthorized")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                        == live
                            .get("allowUnauthorized")
                            .and_then(Value::as_bool)
                            .unwrap_or(false);
                    value_matches && auth_matches
                },
                execute,
                |wire| {
                    let client = client.clone();
                    async move { client_settings::patch_one(&client, "global", wire).await }
                },
                |_live, wire| {
                    let client = client.clone();
                    async move { client_settings::patch_one(&client, "global", wire).await }
                },
            )
            .await
        })
    }
}
