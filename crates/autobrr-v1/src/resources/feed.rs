//! `/api/feeds` — RSS/Torznab/Newznab feeds. A feed is the polling half of a
//! feed-type indexer: autobrr's UI creates the indexer and the feed as two calls,
//! and the feed carries an `indexer_id` back to its parent indexer.
//!
//! `sync = custom`, keyed by `name`, create + update by id (`POST /api/feeds`,
//! `PUT /api/feeds/{id}`), and `--prune` deletes feeds the config no longer
//! declares via `DELETE /api/feeds/{id}`. `api_key`/`cookie` are redacted on
//! read, so idempotency is the structural-subset test on the readable fields
//! ([`crate::diff::subset`]); the create/update/prune skeleton is
//! [`core_lib::reconcile::upsert_prune`].
//!
//! FK: `indexer_id` → the managed feed-type indexer this feed polls
//! (`${ref.indexer.<name>}`). A feed is meaningless without its indexer, so
//! declare the indexer too and point the feed at it.

use core_lib::{
    CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine, reconcile,
};
use core_macros::resource;
use serde_json::Value;

use crate::diff;
use crate::resources::feed_settings::FeedSettings;

/// `/api/feeds` — a configured feed.
#[resource(sync = custom, case = snake, list = get("/api/feeds"))]
pub struct Feed {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.feed.<name>}`).
    #[key]
    pub name: String,
    /// The feed-type indexer this feed polls (`${ref.indexer.<name>}`).
    #[reference(indexer)]
    pub indexer_id: Option<i32>,
    /// Feed protocol: `TORZNAB`, `NEWZNAB`, or `RSS`.
    #[wire(name = "type")]
    pub feed_type: String,
    /// Whether the feed is active.
    pub enabled: Option<bool>,
    /// Feed URL.
    pub url: Option<String>,
    /// Poll interval, minutes.
    pub interval: Option<i32>,
    /// Per-request timeout, seconds.
    pub timeout: Option<i32>,
    /// Ignore items older than this many minutes (`0` = no limit).
    pub max_age: Option<i32>,
    /// Torznab/Newznab category ids to fetch.
    pub categories: Vec<i32>,
    /// API key for the feed source, where required.
    pub api_key: Option<SecretValue>,
    /// Cookie to send with feed requests (private trackers).
    pub cookie: Option<SecretValue>,
    /// Skip TLS certificate verification.
    pub tls_skip_verify: Option<bool>,
    /// Feed settings blob (currently just `download_type`).
    pub settings: Option<FeedSettings>,
}

/// autobrr echoes the FK only nested (`indexer.id`), never as a top-level
/// `indexer_id`, so lift it up before the subset diff — otherwise the resolved
/// `indexer_id` would re-update forever.
fn lift_indexer_id(live: &Value) -> Value {
    let mut norm = live.clone();
    if let (Some(obj), Some(iid)) = (
        norm.as_object_mut(),
        live.get("indexer").and_then(|i| i.get("id")).cloned(),
    ) {
        obj.insert("indexer_id".to_string(), iid);
    }
    norm
}

impl CustomSync for Feed {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/feeds").await?;
            // Full desired wire (refs already resolved to ids; id is read-only).
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert_prune(
                &wire,
                &live,
                "name",
                |w, l| diff::subset(w, &lift_indexer_id(l)),
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/feeds", &w).await?;
                        Ok(())
                    }
                },
                |l, w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        let _: Value = client.put(&format!("/api/feeds/{id}"), &w).await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        client.delete(&format!("/api/feeds/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
