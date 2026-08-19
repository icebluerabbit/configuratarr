//! `/api/v2/users/me/api-keys` — API keys for the authenticated user, one per
//! `comment` (Komga's free-text label — there is no separate `name` field).
//!
//! Komga mints the secret `key` server-side and returns it exactly once, in the
//! `POST` response body — every later `GET` (including the live list this hook
//! reads) omits it. There is no update verb (`PUT`/`PATCH`), so this doesn't fit
//! crud/singleton: `sync = custom`, reconciled via
//! [`core_lib::reconcile::create_only_prune`] — create any declared `comment`
//! that isn't already live, and delete any live key the config no longer
//! declares.
//!
//! **DANGER — `--prune` here revokes configuratarr's own credential.** The keys
//! this resource manages are *the authenticated user's own*, and `api_key` is
//! how this crate connects, so the connecting key is necessarily inside the
//! managed set. A `--prune` apply that doesn't declare it revokes it, and every
//! subsequent request 401s. Verified the hard way, live: it killed the rest of
//! the e2e suite. Either declare a `comment` matching the connecting key, or
//! don't manage `api_keys` at all. (Komga does also accept HTTP basic auth, so
//! a Basic-authenticated connection could prune freely — but this crate's
//! `#[service]` only wires the `X-API-Key` scheme.)
//!
//! SECURITY: the `key` value in a create response is never read past the `POST`
//! call below — the closure discards it (`let _: Value = ...`) and
//! `create_only_prune` reports a create as a bare [`core_lib::Change::created`]
//! with no detail rows, so the secret can never reach a plan `Report` /
//! `Plan::render` output.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// An API key issued to the authenticated user under a free-text comment/label.
#[resource(sync = custom, list = get("/api/v2/users/me/api-keys"))]
pub struct ApiKey {
    /// Free-text label identifying the key — its identity (Komga calls this
    /// field `comment`, not `name`).
    #[key]
    pub comment: String,
}

impl CustomSync for ApiKey {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/v2/users/me/api-keys").await?;

            reconcile::create_only_prune(
                desired,
                &live,
                "comment",
                prune,
                execute,
                |_comment, cfg| {
                    let client = client.clone();
                    async move {
                        // ApiKeyRequestDto is just `{ comment }` — same shape as
                        // our config, so the config->wire encode is exact.
                        let body = core_lib::engine::encode_config::<Self>(&cfg)?;
                        // The response carries the once-only secret `key`; it is
                        // deliberately dropped here (see module docs) rather than
                        // threaded into a `Change` detail row.
                        let _: Value = client.post("/api/v2/users/me/api-keys", &body).await?;
                        Ok(())
                    }
                },
                |live| {
                    let client = client.clone();
                    let id = live.get("id").and_then(Value::as_str).map(str::to_string);
                    async move {
                        let id =
                            id.ok_or_else(|| anyhow::anyhow!("live api key is missing `id`"))?;
                        client
                            .delete(&format!("/api/v2/users/me/api-keys/{id}"))
                            .await
                    }
                },
            )
            .await
        })
    }
}
