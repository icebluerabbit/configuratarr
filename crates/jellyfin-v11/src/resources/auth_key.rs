//! `/Auth/Keys` — API keys, one per application name.
//!
//! Jellyfin mints the key server-side (you can't declare its value) and offers
//! no update, so this doesn't fit crud/singleton/bulk_replace. It's a `sync =
//! custom` resource: the reconcile hook GETs the live keys and POSTs any
//! declared `app` that isn't present yet (create-or-leave; no prune, mirroring
//! the *arr "don't delete what you didn't make" caution).

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// `/Auth/Keys` — an API key issued to an application name.
#[resource(sync = custom, list = get("/Auth/Keys"))]
pub struct AuthKey {
    /// Application name the key is issued to — its identity.
    #[key]
    pub app: String,
}

impl CustomSync for AuthKey {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            // Live keys come back as `{ Items: [{ AppName, AccessToken, ... }] }`.
            let live: Value = client.get("/Auth/Keys").await?;
            let present = reconcile::present_keys(
                live.get("Items")
                    .and_then(Value::as_array)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
                "AppName",
            );
            let client = client.clone();
            reconcile::create_only(desired, "app", &present, execute, move |app, _cfg| {
                let client = client.clone();
                // The key is issued via a query param (core-http encodes it); the
                // body is ignored.
                async move {
                    let _: Value = client
                        .post_query("/Auth/Keys", &[("app", app.as_str())], &Value::Null)
                        .await?;
                    Ok(())
                }
            })
            .await
        })
    }
}
