//! `/api/v1/client-settings/user` — per-user client settings, scoped to the
//! authenticated account (unlike [`crate::resources::client_setting_global`],
//! there is no `allowUnauthorized` — a user setting is never exposed to
//! anonymous requests).
//!
//! Keys are dotted namespace strings (`"application.domain.key"`, per the Komga
//! API docs). The list/write/diff mechanics — object-map GET, upsert-merge
//! PATCH, no prune — are shared with the global settings and live in
//! [`crate::resources::client_settings`]; read that module's docs first.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

use crate::resources::client_settings;

/// One per-user client setting.
#[resource(sync = custom, list = get("/api/v1/client-settings/user/list"))]
pub struct ClientSettingUser {
    /// Dotted setting key, e.g. `"webui.locale"` — its identity.
    #[key]
    pub key: String,
    /// The setting's value (always a plain string on the wire — the API
    /// documents a JSON-object value as a JSON-*encoded string*, not a nested
    /// object).
    pub value: String,
}

impl CustomSync for ClientSettingUser {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        // No body-carrying DELETE on `HttpClient` — see client_settings.rs.
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let map: Value = client.get("/api/v1/client-settings/user/list").await?;
            let live = client_settings::live_list(&map);

            let wire: Vec<Value> = desired
                .iter()
                .map(core_lib::engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert(
                &wire,
                &live,
                "key",
                |desired, live| desired.get("value") == live.get("value"),
                execute,
                |wire| {
                    let client = client.clone();
                    async move { client_settings::patch_one(&client, "user", wire).await }
                },
                |_live, wire| {
                    let client = client.clone();
                    async move { client_settings::patch_one(&client, "user", wire).await }
                },
            )
            .await
        })
    }
}
