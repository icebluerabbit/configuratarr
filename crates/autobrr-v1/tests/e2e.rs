//! End-to-end tests against a live autobrr.
//!
//! Drive the real apply engine (connect → GET live → plan/reconcile → write),
//! exactly as the CLI does. Guarded by `AUTOBRR_URL` + `AUTOBRR_API_KEY`;
//! `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts autobrr, onboards a user, mints a key,
//! exports the env vars):
//!   nix develop .#e2e-autobrr --command \
//!     cargo nextest run -p autobrr-v1 --test e2e --run-ignored all -j1
//!
//! The live instance is shared and persists across runs, so every test begins by
//! wiping it to a clean slate ([`setup`] → [`reset`], API-driven, api keys
//! excluded). That makes tests order-independent and lets a prune test delete to
//! empty without harming a sibling.
//!
//! Runtime assumptions validated here:
//!   1. auth — the API key is accepted in the `X-API-Token` header;
//!   2. casing — autobrr serialises/accepts snake_case JSON;
//!   3. the custom seams are idempotent — `create_only` (api keys), the
//!      `notification` upsert+prune (create → update on drift → DELETE), the
//!      `filter` two-step create (POST subset → PATCH full), the `indexer`
//!      create/update (base id on create, stored id on update), and the
//!      `irc_network` structural-subset diff;
//!   4. `proxy` crud round-trips and prunes.

use std::time::Duration;

use autobrr_v1::AutobrrV1;
use core_lib::Service;
use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("AUTOBRR_URL", "AUTOBRR_API_KEY")
}

/// Every collection autobrr exposes a DELETE for, as `(list endpoint, delete
/// prefix)`, ordered **dependents-first** so a purge never trips an FK: a list
/// references filters + a download client; a filter's actions reference download
/// clients and a dedup profile; a feed references an indexer; an indexer
/// references a proxy. **api keys are excluded** — deleting the key configuratarr
/// authenticates with would lock the run out.
const PURGE: &[(&str, &str)] = &[
    ("/api/lists", "/api/lists"),
    ("/api/feeds", "/api/feeds"),
    ("/api/filters", "/api/filters"),
    ("/api/indexer", "/api/indexer"),
    (
        "/api/release/profiles/duplicate",
        "/api/release/profiles/duplicate",
    ),
    ("/api/download_clients", "/api/download_clients"),
    ("/api/irc", "/api/irc/network"),
    ("/api/notification", "/api/notification"),
    ("/api/proxy", "/api/proxy"),
];

/// Wipe every managed collection so each test starts from a clean instance. The
/// live e2e instance is **shared and persists across runs**, so without this a
/// prune-to-empty test would delete another test's resources and a create test
/// would race stale state. Purely API-driven (GET-list → DELETE each) and so
/// **independent of the engine's own prune path** — a prune bug can't hide by
/// breaking the reset. Runs at the start of every test via [`setup`].
async fn reset(client: &core_lib::HttpClient) {
    for (list_path, del_prefix) in PURGE {
        let items: Vec<Value> = client.get(list_path).await.expect("reset: list collection");
        for item in &items {
            if let Some(id) = item.get("id").and_then(Value::as_i64) {
                client
                    .delete(&format!("{del_prefix}/{id}"))
                    .await
                    .expect("reset: delete item");
            }
        }
    }
}

/// Env gate + health wait + [`reset`] to a clean slate. Returns `(url, key)`, or
/// `None` when the env vars are absent (test skipped outside the e2e shell). Each
/// test starts from this, so tests are order-independent and free to prune to
/// empty without harming a sibling.
async fn setup() -> Option<(String, String)> {
    let (url, key) = env()?;
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("autobrr healthy");
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    reset(&client).await;
    Some((url, key))
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<AutobrrV1>(url, key, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("autobrr healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// Live `(action name, id)` pairs for a filter, sorted by name — for asserting
/// action-set membership and id stability across applies.
async fn filter_action_ids(client: &core_lib::HttpClient, filter: &str) -> Vec<(String, i64)> {
    let filters: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let fid = filters
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some(filter))
        .and_then(|f| f.get("id").and_then(Value::as_i64))
        .expect("filter present");
    let full: Value = client
        .get(&format!("/api/filters/{fid}"))
        .await
        .expect("get filter");
    let mut out: Vec<(String, i64)> = full["actions"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| {
                    Some((x.get("name")?.as_str()?.to_string(), x.get("id")?.as_i64()?))
                })
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

/// `wait_healthy` against the live API: `/api/config` answers OK (and confirms
/// the token).
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("autobrr should report healthy");
}

/// No managed resources: connect + auth reach the live API, nothing changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// Create-only api key: created on the first apply, unchanged on the second.
/// api keys are the one no-prune exception (pruning could delete the credential
/// configuratarr authenticates with), so `--prune` must **not** delete this key:
/// the run passes `prune: true` and asserts the key is left untouched.
#[tokio::test]
#[ignore]
async fn api_key_create_idempotent_no_prune() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "api_keys": [{ "name": "cfg-e2e", "scopes": [] }] });
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.created, 0,
        "second api-key apply should create nothing"
    );
    assert_eq!(second.unchanged, 1);

    // Even with --prune and an empty api_keys list, api keys are never deleted.
    let pruned = run(
        &url,
        &key,
        json!({ "api_keys": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert_eq!(
        pruned.deleted, 0,
        "api keys are never pruned (self-lockout guard): {pruned:?}"
    );
}

/// Notification upsert + prune: created/settled, idempotent, then a changed
/// `webhook` drives an **update** (the bug that started this — a create-only
/// notification never pushed field drift; `webhook` is not redacted on read, so
/// the drift is detectable and settles), then `--prune` deletes it via DELETE.
#[tokio::test]
#[ignore]
async fn notification_upsert_and_prune() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let mk = |webhook: &str| {
        json!({ "notifications": [{
            "name": "cfg-e2e-discord",
            "notification_type": "DISCORD",
            "enabled": true,
            "events": ["PUSH_APPROVED"],
            "webhook": webhook,
        }] })
    };

    // Settle (create or reconcile pre-existing), then a repeated apply is a no-op.
    run(
        &url,
        &key,
        mk("https://discord.com/api/webhooks/1/abc"),
        ApplyOptions::default(),
    )
    .await;
    let second = run(
        &url,
        &key,
        mk("https://discord.com/api/webhooks/1/abc"),
        ApplyOptions::default(),
    )
    .await;
    assert_eq!(second.created, 0);
    assert_eq!(
        second.unchanged, 1,
        "second notification apply is a no-op: {second:?}"
    );

    // Change the webhook → drift must be pushed as an update (the headline fix).
    let updated = run(
        &url,
        &key,
        mk("https://discord.com/api/webhooks/2/xyz"),
        ApplyOptions::default(),
    )
    .await;
    assert_eq!(
        updated.updated, 1,
        "notification webhook drift pushed: {updated:?}"
    );
    let settled = run(
        &url,
        &key,
        mk("https://discord.com/api/webhooks/2/xyz"),
        ApplyOptions::default(),
    )
    .await;
    assert_eq!(
        settled.unchanged, 1,
        "update settles to unchanged: {settled:?}"
    );

    // Prune: declaring no notifications with --prune deletes it.
    let pruned = run(
        &url,
        &key,
        json!({ "notifications": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(pruned.deleted >= 1, "notification pruned: {pruned:?}");
}

/// Download client: created on the first apply, unchanged on the second (crud
/// diff makes a repeated apply a no-op).
#[tokio::test]
#[ignore]
async fn download_client_create_idempotent() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "download_clients": [{
        "name": "cfg-e2e-qbit",
        "client_type": "QBITTORRENT",
        "enabled": true,
        "host": "http://localhost",
        "port": 8080,
        "username": "admin",
        "password": "adminadmin",
    }] });
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.created, 0,
        "second dl-client apply should create nothing"
    );
}

/// Filter two-step create (POST subset → PATCH full), then a repeated apply is a
/// no-op — validates both the two-step create and the structural subset diff.
#[tokio::test]
#[ignore]
async fn filter_two_step_create_idempotent() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "filters": [{
        "name": "cfg-e2e-1080p",
        "enabled": true,
        "priority": 5,
        "resolutions": ["1080p"],
        "codecs": ["x265"],
        "match_release_types": ["MOVIE"],
    }] });
    // First apply may create or reconcile pre-existing state (the e2e instance
    // is reused across runs); the invariant is that a *repeated* apply is a no-op.
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.created, 0);
    assert_eq!(
        second.updated, 0,
        "second filter apply should be a no-op: {second:?}"
    );
    assert_eq!(second.unchanged, 1);
}

/// Full-coverage guard: a filter carrying the newly-modelled scalar/array fields
/// and a fully-specified action, plus a dedup profile attached by
/// `${ref.release_profile_duplicate.<name>}`. Conformance can't catch a field
/// autobrr rejects at runtime (the `base_url` lesson), so this asserts the live
/// API accepts every new field, that the dedup id lands, and that it's idempotent.
#[tokio::test]
#[ignore]
async fn filter_full_fields_and_dedup_ref() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({
        "download_clients": [{
            "name": "cfg-e2e-qbit2", "client_type": "QBITTORRENT", "enabled": true,
            "host": "http://localhost", "port": 8080,
        }],
        "release_profile_duplicates": [{
            "name": "cfg-e2e-dedup",
            "title": true, "year": true, "resolution": true, "group": true, "proper": true,
        }],
        "filters": [{
            "name": "cfg-e2e-full",
            "enabled": true,
            "priority": 5,
            "resolutions": ["1080p"],
            "announce_types": ["NEW"],
            "scene": false,
            "freeleech": true,
            "freeleech_percent": "100",
            "max_downloads": 5,
            "max_downloads_unit": "DAY",
            "min_seeders": 2,
            "max_seeders": 500,
            "match_description": "internal",
            "log": false,
            "release_profile_duplicate_id": "${ref.release_profile_duplicate.cfg-e2e-dedup}",
            "actions": [{
                "name": "qb", "action_type": "QBITTORRENT", "enabled": true,
                "client_id": "${ref.download_client.cfg-e2e-qbit2}",
                "category": "movies", "content_layout": "ORIGINAL", "priority": "max",
                "limit_ratio": 2.0, "limit_seed_time": 1440, "limit_upload_speed": 1000,
                "skip_hash_check": true, "paused": false,
            }],
        }],
    });

    // Live must accept every new field (an unknown/mistyped one 500s in apply).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The dedup ref resolved and landed as the profile's numeric id.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let dups: Vec<Value> = client
        .get("/api/release/profiles/duplicate")
        .await
        .expect("list dedup");
    let dup_id = dups
        .iter()
        .find(|d| d.get("name").and_then(Value::as_str) == Some("cfg-e2e-dedup"))
        .and_then(|d| d.get("id").and_then(Value::as_i64))
        .expect("dedup present");
    let filters: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let fid = filters
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-full"))
        .and_then(|f| f.get("id").and_then(Value::as_i64))
        .expect("filter present");
    let full: Value = client
        .get(&format!("/api/filters/{fid}"))
        .await
        .expect("get filter");
    assert_eq!(
        full.get("release_profile_duplicate_id")
            .and_then(Value::as_i64),
        Some(dup_id),
        "dedup id attached: {full:?}"
    );

    // Idempotent — the subset diff over the new fields must settle.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.updated, 0, "second apply is a no-op: {second:?}");
}

/// Cross-reference a custom-sync producer: a filter attaches an `indexer` by
/// `${ref.indexer.<name>}`. Proves the engine now exports custom-sync ids into
/// the RefStore (an unresolved ref would fail `apply`), that autobrr stores the
/// resolved numeric id (`indexers[0].id`), and that it settles idempotently (a
/// dropped attachment would perpetually re-update).
#[tokio::test]
#[ignore]
async fn filter_attaches_indexer_by_ref() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({
        "indexers": [{
            "name": "cfg-e2e-refidx",
            "identifier": "torznab",
            "implementation": "torznab",
            "settings": { "url": "https://tracker.example.org/t", "api_key": "K" },
        }],
        "filters": [{
            "name": "cfg-e2e-reffilter",
            "enabled": true,
            "indexers": [{ "id": "${ref.indexer.cfg-e2e-refidx}" }],
        }],
    });

    // A successful apply already proves the ref resolved (an unresolved
    // `${ref.indexer.*}` errors out in `run`).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The attachment must land server-side as the indexer's numeric id.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let indexers: Vec<Value> = client.get("/api/indexer").await.expect("list indexers");
    let idx_id = indexers
        .iter()
        .find(|i| i.get("name").and_then(Value::as_str) == Some("cfg-e2e-refidx"))
        .and_then(|i| i.get("id").and_then(Value::as_i64))
        .expect("indexer present");
    let filters: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let fid = filters
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-reffilter"))
        .and_then(|f| f.get("id").and_then(Value::as_i64))
        .expect("filter present");
    let full: Value = client
        .get(&format!("/api/filters/{fid}"))
        .await
        .expect("get filter");
    let attached: Vec<i64> = full["indexers"]
        .as_array()
        .map(|a| a.iter().filter_map(|i| i["id"].as_i64()).collect())
        .unwrap_or_default();
    assert!(
        attached.contains(&idx_id),
        "filter should have indexer {idx_id} attached, got {attached:?}"
    );

    // Idempotent: a dropped attachment would re-update forever.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.updated, 0, "second apply is a no-op: {second:?}");
}

/// Feed create referencing its indexer by `${ref.indexer.<name>}`: proves the
/// feed→indexer FK resolves out of the RefStore, autobrr stores the resolved
/// numeric `indexer_id`, and a repeated apply is a no-op (the redacted `api_key`
/// must not perpetually re-update via the structural-subset diff).
#[tokio::test]
#[ignore]
async fn feed_references_indexer_idempotent() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({
        "indexers": [{
            "name": "cfg-e2e-feedidx",
            "identifier": "torznab",
            "implementation": "torznab",
            "settings": { "url": "https://tracker.example.org/t", "api_key": "K" },
        }],
        "feeds": [{
            "name": "cfg-e2e-feed",
            "indexer_id": "${ref.indexer.cfg-e2e-feedidx}",
            "feed_type": "TORZNAB",
            "enabled": true,
            "url": "https://tracker.example.org/rss",
            "interval": 15,
        }],
    });

    // Successful apply proves the ref resolved (an unresolved one errors in `run`).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The feed's indexer_id landed as the indexer's numeric id.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let indexers: Vec<Value> = client.get("/api/indexer").await.expect("list indexers");
    let idx_id = indexers
        .iter()
        .find(|i| i.get("name").and_then(Value::as_str) == Some("cfg-e2e-feedidx"))
        .and_then(|i| i.get("id").and_then(Value::as_i64))
        .expect("indexer present");
    let feeds: Vec<Value> = client.get("/api/feeds").await.expect("list feeds");
    let feed = feeds
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-feed"))
        .expect("feed present");
    assert_eq!(
        feed.get("indexer")
            .and_then(|i| i.get("id"))
            .and_then(Value::as_i64),
        Some(idx_id),
        "feed indexer attached: {feed:?}"
    );

    // Idempotent — the subset diff over the redacted api_key must settle.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.updated, 0,
        "second feed apply is a no-op: {second:?}"
    );
}

/// List create referencing a filter by `${ref.filter.<name>}`: an external
/// (`PLAINTEXT`) list needs a `url` + at least one filter. Proves the
/// list→filter FK resolves, the attachment lands, and a repeated apply is a
/// no-op (the redacted `api_key` must not perpetually re-update).
#[tokio::test]
#[ignore]
async fn list_references_filter_idempotent() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({
        "filters": [{
            "name": "cfg-e2e-listfilter",
            "enabled": true,
            "resolutions": ["1080p"],
        }],
        // `enabled: false` so autobrr doesn't fetch titles from `url` on
        // create/update — the e2e VM is hermetic (no DNS), and the fetch is the
        // only network dependency; the filter attachment + diff don't need it.
        "lists": [{
            "name": "cfg-e2e-list",
            "list_type": "PLAINTEXT",
            "enabled": false,
            "url": "https://example.org/list.txt",
            "filters": [{ "id": "${ref.filter.cfg-e2e-listfilter}" }],
        }],
    });

    // Successful apply proves the `${ref.filter.…}` resolved and autobrr accepted
    // the list with its filter attachment (an invalid filter id would 400/500).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The list landed. NB: autobrr's `GET /api/lists` does **not** echo the filter
    // attachment (`List()` hardcodes `filters: []`, and there is no
    // `GET /api/lists/{id}`), so the attachment is write-only and can't be
    // asserted via the API — the successful create above is the evidence it took.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let lists: Vec<Value> = client.get("/api/lists").await.expect("list lists");
    assert!(
        lists
            .iter()
            .any(|l| l.get("name").and_then(Value::as_str) == Some("cfg-e2e-list")),
        "list should be present after apply"
    );

    // Idempotent — `filters` is write-only (never read back), so the readable
    // subset diff must settle to a no-op rather than re-`PUT` forever (a list
    // update 500s on a never-refreshed list — autobrr's NULL last_refresh scan).
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.updated, 0,
        "second list apply is a no-op: {second:?}"
    );
    // Both the filter and the list read back unchanged.
    assert_eq!(
        second.unchanged, 2,
        "second apply leaves the filter + list unchanged: {second:?}"
    );
}

/// Proxy crud: created, then a repeated apply is a no-op (no secret sent, so the
/// merge is stable), then pruned via DELETE.
#[tokio::test]
#[ignore]
async fn proxy_crud_idempotent_prune() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "proxies": [{
        "name": "cfg-e2e-proxy",
        "enabled": true,
        "proxy_type": "SOCKS5",
        "addr": "socks5://127.0.0.1:1080",
    }] });
    // First apply may create or reconcile pre-existing state; the invariant is
    // that a *repeated* apply is a no-op (no secret sent → stable merge).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.created, 0, "second proxy apply creates nothing");
    assert_eq!(
        second.updated, 0,
        "second proxy apply is a no-op: {second:?}"
    );

    let pruned = run(
        &url,
        &key,
        json!({ "proxies": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(pruned.deleted >= 1, "proxy pruned: {pruned:?}");
}

/// `release_profile_duplicate` uses the `create_only_prune` seam: created on the
/// first apply, left untouched on a repeat (no update path), then `--prune` with
/// an empty list deletes it. Prune-to-empty is safe because [`setup`] hands each
/// test a clean instance. The live counterpart to the `create_only_prune` unit
/// test.
#[tokio::test]
#[ignore]
async fn release_profile_duplicate_create_and_prune() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "release_profile_duplicates": [{
        "name": "cfg-e2e-dedup-prune",
        "title": true, "year": true, "resolution": true,
    }] });
    let first = run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    assert_eq!(first.created, 1, "dedup profile created: {first:?}");
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.created, 0);
    assert_eq!(
        second.unchanged, 1,
        "create_only: a repeat apply is a no-op: {second:?}"
    );

    // --prune with an empty list deletes it (create_only_prune's prune tail).
    let pruned = run(
        &url,
        &key,
        json!({ "release_profile_duplicates": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert_eq!(
        pruned.deleted, 1,
        "dedup profile pruned to empty: {pruned:?}"
    );
}

/// A whole filter is pruned via `filter.rs`'s direct `reconcile::prune_absent`
/// tail (distinct from the *action-level* prune in
/// `filter_prunes_undeclared_action`): create two filters, then re-apply
/// declaring only one with `--prune` → the undeclared filter is deleted, the
/// declared one kept. Safe to prune here because [`setup`] starts clean.
#[tokio::test]
#[ignore]
async fn filter_prune_to_empty() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let two = json!({ "filters": [
        { "name": "cfg-e2e-keep", "enabled": true, "resolutions": ["1080p"] },
        { "name": "cfg-e2e-drop", "enabled": true, "resolutions": ["720p"] },
    ] });
    let created = run(&url, &key, two, ApplyOptions::default()).await;
    assert_eq!(created.created, 2, "two filters created: {created:?}");

    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let before: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    assert_eq!(before.len(), 2, "two filters live before prune");

    // Declare only "keep" with --prune → "drop" is deleted.
    let one = json!({ "filters": [
        { "name": "cfg-e2e-keep", "enabled": true, "resolutions": ["1080p"] },
    ] });
    let pruned = run(&url, &key, one, ApplyOptions { prune: true }).await;
    assert_eq!(pruned.deleted, 1, "undeclared filter pruned: {pruned:?}");

    let after: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let names: Vec<&str> = after
        .iter()
        .filter_map(|f| f.get("name").and_then(Value::as_str))
        .collect();
    assert_eq!(
        names,
        vec!["cfg-e2e-keep"],
        "only 'keep' remains: {names:?}"
    );
}

/// Indexer custom sync: created, idempotent, then a toggled `enabled` drives an
/// update — which must echo the *server-stored* identifier (`torznab-<name>`);
/// sending the base id 500s. Regression guard for that update path.
#[tokio::test]
#[ignore]
async fn indexer_create_update_idempotent() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let mk = |enabled: bool| {
        json!({ "indexers": [{
        "name": "cfg-e2e-tz",
        "identifier": "torznab",
        "implementation": "torznab",
        "enabled": enabled,
        "settings": { "url": "https://tracker.example.org/t", "api_key": "K" },
    }] })
    };

    // Settle to enabled=true (create or reconcile pre-existing), then a repeated
    // apply must be a no-op.
    run(&url, &key, mk(true), ApplyOptions::default()).await;
    let second = run(&url, &key, mk(true), ApplyOptions::default()).await;
    assert_eq!(second.created, 0);
    assert_eq!(
        second.unchanged, 1,
        "second indexer apply is a no-op: {second:?}"
    );

    // Toggle enabled → update via the stored identifier (the 500-prone path).
    let updated = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(updated.updated, 1, "indexer updated: {updated:?}");
    let settled = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(
        settled.unchanged, 1,
        "update settles to unchanged: {settled:?}"
    );
}

/// An `irc`-implementation indexer round-trips with a top-level `base_url` and
/// its IRC login in `settings`. autobrr rejects an IRC indexer whose `base_url`
/// is empty (`indexer baseURL must not be empty`), so a clean create+idempotent
/// apply proves configuratarr can produce a valid IRC indexer.
#[tokio::test]
#[ignore]
async fn irc_indexer_with_base_url() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "indexers": [{
        "name": "cfg-e2e-tl",
        "identifier": "torrentleech",
        "implementation": "irc",
        "base_url": "https://www.torrentleech.org",
        "settings": { "nick": "cfg_e2e_bot", "auth.account": "cfg", "auth.password": "pw" },
    }] });

    // A missing base_url would 500 inside apply; a clean apply proves it's sent.
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // base_url persisted server-side.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let indexers: Vec<Value> = client.get("/api/indexer").await.expect("list indexers");
    let idx = indexers
        .iter()
        .find(|i| i.get("name").and_then(Value::as_str) == Some("cfg-e2e-tl"))
        .expect("indexer present");
    assert_eq!(
        idx.get("base_url").and_then(Value::as_str),
        Some("https://www.torrentleech.org"),
        "base_url persisted: {idx:?}"
    );

    // Idempotent (base_url is now in the readable diff).
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.updated, 0, "second apply is a no-op: {second:?}");
}

/// IRC network custom sync: the structural-subset diff makes a repeated apply a
/// no-op despite server-enriched `channels[]` + runtime fields; a toggled
/// `enabled` drives a `PUT /api/irc/network/{id}` update.
#[tokio::test]
#[ignore]
async fn irc_network_create_update_idempotent() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let mk = |enabled: bool| {
        json!({ "irc_networks": [{
        "name": "cfg-e2e-net",
        "enabled": enabled,
        "server": "irc.example.org",
        "port": 6697,
        "tls": true,
        "nick": "cfg-e2e-bot",
        "channels": [{ "name": "#announce" }],
    }] })
    };
    // Settle (create or reconcile pre-existing), then a repeated apply is a no-op.
    run(&url, &key, mk(true), ApplyOptions::default()).await;
    let second = run(&url, &key, mk(true), ApplyOptions::default()).await;
    assert_eq!(second.created, 0, "second irc apply creates nothing");
    assert_eq!(
        second.unchanged, 1,
        "second irc apply is a no-op: {second:?}"
    );

    // Toggle enabled → update via /api/irc/network/{id}.
    let updated = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(updated.updated, 1, "irc network updated: {updated:?}");
    let settled = run(&url, &key, mk(false), ApplyOptions::default()).await;
    assert_eq!(
        settled.unchanged, 1,
        "update settles to unchanged: {settled:?}"
    );

    // The channel must be stored *enabled* even though the config didn't say so:
    // autobrr's channel struct decodes an absent `enabled` as `false`, and since
    // 1.82 a disabled channel is skipped by the join workflow (no announces).
    // Read the stored network — the `GET /api/irc` list reports handler state,
    // not the rows.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let networks: Vec<Value> = client.get("/api/irc").await.expect("list networks");
    let id = networks
        .iter()
        .find(|n| n.get("name").and_then(Value::as_str) == Some("cfg-e2e-net"))
        .and_then(|n| n.get("id"))
        .expect("network present")
        .clone();
    let stored: Value = client
        .get(&format!("/api/irc/network/{id}"))
        .await
        .expect("get network");
    let channel = stored
        .get("channels")
        .and_then(Value::as_array)
        .and_then(|c| c.first())
        .expect("channel stored");
    assert_eq!(
        channel.get("enabled"),
        Some(&Value::Bool(true)),
        "channel stored enabled: {channel:?}"
    );
}

/// Create-only notification of the newly-added `WEBHOOK` type, subscribed to the
/// newly-added `RELEASE_NEW` event. Live must accept both (an unknown `type` or
/// `events[]` value errors in apply — the enum additions can't be proven by
/// conformance alone). Created once, then unchanged.
#[tokio::test]
#[ignore]
async fn notification_webhook_release_new_idempotent() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "notifications": [{
        "name": "cfg-e2e-webhook",
        "notification_type": "WEBHOOK",
        "enabled": true,
        "events": ["PUSH_APPROVED", "PUSH_REJECTED", "RELEASE_NEW"],
        "webhook": "http://localhost:9999/hook",
    }] });
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(second.created, 0);
    assert_eq!(
        second.unchanged, 1,
        "second webhook-notification apply is a no-op: {second:?}"
    );
}

/// A download client carrying the modern `auth` block (`type` = `BASIC_AUTH`)
/// and `rules` with the corrected `ignore_slow_torrents_condition`
/// (`MAX_DOWNLOADS_REACHED`, not the old bogus `MAX_DOWNLOAD_SPEED`). Proves the
/// live API accepts both, stores the condition verbatim, and that a repeated
/// apply creates nothing.
#[tokio::test]
#[ignore]
async fn download_client_auth_block_and_rules() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "download_clients": [{
        "name": "cfg-e2e-auth",
        "client_type": "QBITTORRENT",
        "enabled": true,
        "host": "http://localhost",
        "port": 8080,
        "settings": {
            "auth": { "enabled": true, "auth_type": "BASIC_AUTH", "username": "u", "password": "p" },
            "rules": {
                "enabled": true,
                "ignore_slow_torrents": true,
                "ignore_slow_torrents_condition": "MAX_DOWNLOADS_REACHED",
                "download_speed_threshold": 5000,
            },
        },
    }] });

    // Live must accept the auth block + corrected condition (an unknown/mistyped
    // field 500s in apply).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // The corrected condition value round-tripped and persisted.
    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let clients: Vec<Value> = client
        .get("/api/download_clients")
        .await
        .expect("list clients");
    let dc = clients
        .iter()
        .find(|c| c.get("name").and_then(Value::as_str) == Some("cfg-e2e-auth"))
        .expect("client present");
    assert_eq!(
        dc["settings"]["rules"]["ignore_slow_torrents_condition"].as_str(),
        Some("MAX_DOWNLOADS_REACHED"),
        "rules condition stored verbatim: {dc:?}"
    );

    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.created, 0,
        "second auth-client apply creates nothing: {second:?}"
    );
}

/// A filter with an external `WEBHOOK` check carrying every newly-modelled field
/// (`webhook_headers`, `webhook_retry_*`, `on_error`). Live must accept them all
/// (conformance can't catch a field autobrr rejects at runtime — the `base_url`
/// lesson), store `on_error`/`webhook_headers` verbatim, and settle idempotently.
#[tokio::test]
#[ignore]
async fn filter_external_check_new_fields() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "filters": [{
        "name": "cfg-e2e-extcheck",
        "enabled": true,
        "resolutions": ["1080p"],
        "external": [{
            "name": "size-check",
            "external_type": "WEBHOOK",
            "enabled": true,
            "index": 0,
            "webhook_host": "http://localhost:9000/check",
            "webhook_method": "POST",
            "webhook_data": "{\"n\":\"x\"}",
            "webhook_headers": "X-Api-Key=abc,Accept=application/json",
            "webhook_expect_status": 200,
            "webhook_retry_status": "500,502,503",
            "webhook_retry_attempts": 3,
            "webhook_retry_delay_seconds": 5,
            "on_error": "REJECT",
        }],
    }] });

    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let filters: Vec<Value> = client.get("/api/filters").await.expect("list filters");
    let fid = filters
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("cfg-e2e-extcheck"))
        .and_then(|f| f.get("id").and_then(Value::as_i64))
        .expect("filter present");
    let full: Value = client
        .get(&format!("/api/filters/{fid}"))
        .await
        .expect("get filter");
    let ext = full["external"]
        .as_array()
        .and_then(|a| a.first())
        .expect("external check present");
    assert_eq!(
        ext["on_error"].as_str(),
        Some("REJECT"),
        "on_error stored: {ext:?}"
    );
    assert_eq!(
        ext["webhook_headers"].as_str(),
        Some("X-Api-Key=abc,Accept=application/json"),
        "webhook_headers stored: {ext:?}"
    );
    assert_eq!(
        ext["webhook_retry_attempts"].as_i64(),
        Some(3),
        "webhook_retry_attempts stored: {ext:?}"
    );

    // Idempotent — the subset diff over the new external fields must settle.
    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.updated, 0,
        "second external-check apply is a no-op: {second:?}"
    );
}

/// The usenet download-client types added to the enum (`SABNZBD`, `NZBGET`).
/// autobrr doesn't test-connect on create, so a clean apply + a read-back of the
/// stored `type` proves the values are real and accepted.
#[tokio::test]
#[ignore]
async fn download_client_usenet_types() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let cfg = json!({ "download_clients": [
        {
            "name": "cfg-e2e-sab", "client_type": "SABNZBD", "enabled": true,
            "host": "http://localhost:8085", "settings": { "apikey": "sabkey" },
        },
        {
            "name": "cfg-e2e-nzbget", "client_type": "NZBGET", "enabled": true,
            "host": "http://localhost:6789", "username": "nzbget", "password": "tegbzn6789",
        },
    ] });

    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    let clients: Vec<Value> = client
        .get("/api/download_clients")
        .await
        .expect("list clients");
    for (name, ty) in [("cfg-e2e-sab", "SABNZBD"), ("cfg-e2e-nzbget", "NZBGET")] {
        let dc = clients
            .iter()
            .find(|c| c.get("name").and_then(Value::as_str) == Some(name))
            .unwrap_or_else(|| panic!("{name} present in {clients:?}"));
        assert_eq!(
            dc["type"].as_str(),
            Some(ty),
            "{name} stored as {ty}: {dc:?}"
        );
    }

    let second = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        second.created, 0,
        "second usenet apply creates nothing: {second:?}"
    );
}

/// Filter actions are reconciled **by identity**: an update (here a priority
/// change) must update the existing actions in place, keeping their ids and
/// count stable — not re-insert them. Regression guard for the duplication bug
/// (autobrr's action store is upsert-by-id with no delete, so sending id-less
/// actions on every apply used to append a fresh copy each time).
#[tokio::test]
#[ignore]
async fn filter_actions_stable_on_update() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let mk = |priority: i64| {
        json!({ "filters": [{
            "name": "cfg-e2e-actionids",
            "enabled": true,
            "priority": priority,
            "resolutions": ["1080p"],
            "actions": [
                { "name": "a", "action_type": "WEBHOOK", "enabled": true,
                  "webhook_host": "http://localhost:9001/a", "webhook_method": "POST" },
                { "name": "b", "action_type": "WEBHOOK", "enabled": true,
                  "webhook_host": "http://localhost:9002/b", "webhook_method": "POST" },
            ],
        }] })
    };

    run(&url, &key, mk(5), ApplyOptions::default()).await;

    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");

    let before = filter_action_ids(&client, "cfg-e2e-actionids").await;
    assert_eq!(before.len(), 2, "two actions after create: {before:?}");

    // Force an update; actions must be updated in place, not duplicated.
    let updated = run(&url, &key, mk(10), ApplyOptions::default()).await;
    assert_eq!(
        updated.updated, 1,
        "priority change drives an update: {updated:?}"
    );

    let after = filter_action_ids(&client, "cfg-e2e-actionids").await;
    assert_eq!(
        after, before,
        "action set + ids stable across update (no duplication): {after:?} vs {before:?}"
    );

    let settled = run(&url, &key, mk(10), ApplyOptions::default()).await;
    assert_eq!(settled.updated, 0, "re-apply is a no-op: {settled:?}");
}

/// Removing an action from the desired config **prunes** it live. Regression
/// guard for the non-pruning bug (undeclared actions used to linger forever and,
/// via the length-mismatch diff, drive perpetual re-inserts).
#[tokio::test]
#[ignore]
async fn filter_prunes_undeclared_action() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let base = |actions: Value| {
        json!({ "filters": [{
            "name": "cfg-e2e-prune",
            "enabled": true,
            "resolutions": ["1080p"],
            "actions": actions,
        }] })
    };
    let two = base(json!([
        { "name": "keep", "action_type": "WEBHOOK", "enabled": true,
          "webhook_host": "http://localhost:9001/keep", "webhook_method": "POST" },
        { "name": "drop", "action_type": "WEBHOOK", "enabled": true,
          "webhook_host": "http://localhost:9002/drop", "webhook_method": "POST" },
    ]));
    let one = base(json!([
        { "name": "keep", "action_type": "WEBHOOK", "enabled": true,
          "webhook_host": "http://localhost:9001/keep", "webhook_method": "POST" },
    ]));

    run(&url, &key, two, ApplyOptions::default()).await;

    let (svc, _) = instance::<AutobrrV1>(&url, &key, json!({}));
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    assert_eq!(
        filter_action_ids(&client, "cfg-e2e-prune").await.len(),
        2,
        "two actions after the two-action apply"
    );

    // Drop "drop" from config → it must be pruned, "keep" untouched.
    run(&url, &key, one.clone(), ApplyOptions::default()).await;
    let after = filter_action_ids(&client, "cfg-e2e-prune").await;
    assert_eq!(
        after.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(),
        vec!["keep"],
        "undeclared action pruned, exactly one left: {after:?}"
    );

    let settled = run(&url, &key, one, ApplyOptions::default()).await;
    assert_eq!(settled.updated, 0, "re-apply is a no-op: {settled:?}");
}
