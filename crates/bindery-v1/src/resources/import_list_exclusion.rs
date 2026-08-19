//! `/api/v1/importlistexclusion` — a work blocked from being (re-)added by
//! import-list and author-catalogue syncs, keyed by its provider foreign id.
//!
//! The API exposes only `GET`/`POST /api/v1/importlistexclusion` and
//! `DELETE /api/v1/importlistexclusion/{id}` — no update route. `sync = crud`
//! would hard-bail the moment a user edited `title`/`author_name` (it has
//! nowhere to send the update), so this is `sync = custom`, create-or-leave
//! plus `--prune` ([`core_lib::reconcile::create_only_prune`]), keyed by
//! `foreignId`: a live exclusion with the same `foreign_id` is left alone even
//! if its display fields differ, and (under `--prune`) an exclusion the config
//! no longer declares is deleted.
//!
//! `id`/`created_at` are `#[wire(read_only)]`/`#[id]` — server-assigned, never
//! sent. **Known gap:** the OpenAPI schema has no dedicated create-request
//! shape for this resource (unlike `RootFolder`'s `CreateRootFolderRequest`) —
//! it reuses the full `ImportListExclusion` response schema for the `POST`
//! body too, and that schema marks `id`/`title`/`authorName`/`createdAt` all
//! `required`. Because `#[id]` and `#[wire(read_only)]` fields are *never*
//! emitted by the encoder (by design — see `core-architecture`), no config
//! fixture can make the encoded payload satisfy `required: [id, ...,
//! createdAt]`; `cargo nextest run -p bindery-v1 --test spec_conformance` is
//! expected to report `"id" is a required property` /
//! `"createdAt" is a required property` for this resource until the spec gets
//! a slim create-only schema. `id` is documented as "ignored in a create
//! request body" so sending it would be harmless, but marking it read-only —
//! consistent with every other server-assigned id in this codebase — was
//! judged more valuable than gaming this one conformance check.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::resource;
use serde_json::Value;

/// `/api/v1/importlistexclusion` — a blocked work.
#[resource(sync = custom, list = get("/api/v1/importlistexclusion"))]
pub struct ImportListExclusion {
    /// Server-assigned id; ignored in a create request body. Read-only.
    #[id]
    pub id: Option<i64>,
    /// Natural key — the provider id of the excluded work. Required on
    /// create.
    #[key]
    pub foreign_id: String,
    /// Display-only title of the excluded work.
    pub title: Option<String>,
    /// Display-only author name of the excluded work.
    pub author_name: Option<String>,
    /// Row creation time, reported by the API. Read-only.
    #[wire(read_only)]
    pub created_at: Option<String>,
}

impl CustomSync for ImportListExclusion {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/v1/importlistexclusion").await?;
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::create_only_prune(
                &wire,
                &live,
                "foreignId",
                prune,
                execute,
                |_key, cfg| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/v1/importlistexclusion", &cfg).await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        client
                            .delete(&format!("/api/v1/importlistexclusion/{id}"))
                            .await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
