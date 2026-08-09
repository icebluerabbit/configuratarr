//! End-to-end tests against a live Cleanuparr.
//!
//! Drive the real apply engine (connect → GET live → plan/reconcile → write),
//! exactly as the CLI does. Guarded by `CLEANUPARR_URL` + `CLEANUPARR_API_KEY`;
//! `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts Cleanuparr, runs first-run setup, reads
//! the minted API key, exports the env vars):
//!   nix develop .#e2e-cleanuparr --command \
//!     cargo nextest run -p cleanuparr-v1 --test e2e --run-ignored all -j1
//!
//! The live instance is shared and persists across runs, so every test begins by
//! wiping it to a clean slate ([`setup`] → [`reset`], API-driven). That makes
//! tests order-independent and lets a prune test delete to empty without harming
//! a sibling.
//!
//! Runtime assumptions validated here — every one of them a place where the
//! static spec alone can't prove we read the API correctly:
//!   1. auth — the API key is accepted in the `X-Api-Key` header, and the
//!      authenticated health path answers only once setup is complete;
//!   2. casing — Cleanuparr serialises/accepts camelCase (ASP.NET web defaults),
//!      not the PascalCase of the other .NET service in this repo;
//!   3. the masked-secret contract — a secret reads back as `••••••••`, and a
//!      second apply must NOT report an update for it (that is what
//!      [`cleanuparr_v1::diff`] exists for);
//!   4. GUID ids — the ref store carries `RefId::Str`, so `${ref.*}` resolves to
//!      a string and a downstream resource can address an upstream one;
//!   5. apply order — the seeker's polymorphic `#[reference(sonarr_instance,
//!      radarr_instance)]` puts both instance collections ahead of it;
//!   6. the wrapper-object list shapes (`{clients:[…]}`, `{providers:[…]}`,
//!      `{instances:[…]}`) are plucked correctly by the custom hooks.

use std::time::Duration;

use cleanuparr_v1::CleanuparrV1;
use core_lib::Service;
use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("CLEANUPARR_URL", "CLEANUPARR_API_KEY")
}

/// Wipe every managed collection so each test starts from a clean instance.
///
/// Purely API-driven (GET-list → DELETE each) and so **independent of the
/// engine's own prune path** — a prune bug can't hide by breaking the reset.
///
/// Singletons (general, queue cleaner, seeker, …) have no DELETE and are left
/// as-is; tests that touch them assert on their own fields rather than on a
/// pristine starting value.
async fn reset(client: &core_lib::HttpClient) {
    // Notification providers: one list, one delete path, id is a GUID.
    let providers: Value = client
        .get("/api/configuration/notification_providers")
        .await
        .expect("reset: list notification providers");
    for p in providers
        .get("providers")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if let Some(id) = p.get("id").and_then(Value::as_str) {
            client
                .delete(&format!("/api/configuration/notification_providers/{id}"))
                .await
                .expect("reset: delete notification provider");
        }
    }

    // Download clients — deleting one cascades its seeding rules and its
    // unlinked / dead-torrent / orphaned-files configs.
    let clients: Value = client
        .get("/api/configuration/download_client")
        .await
        .expect("reset: list download clients");
    for c in clients
        .get("clients")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if let Some(id) = c.get("id").and_then(Value::as_str) {
            client
                .delete(&format!("/api/configuration/download_client/{id}"))
                .await
                .expect("reset: delete download client");
        }
    }

    // Queue rules: two sibling collections, plain crud.
    for kind in ["stall", "slow"] {
        let rules: Vec<Value> = client
            .get(&format!("/api/queue-rules/{kind}"))
            .await
            .expect("reset: list queue rules");
        for r in &rules {
            if let Some(id) = r.get("id").and_then(Value::as_str) {
                client
                    .delete(&format!("/api/queue-rules/{kind}/{id}"))
                    .await
                    .expect("reset: delete queue rule");
            }
        }
    }

    // *arr instances, per app. The list is nested inside the app-config object.
    for app in ["sonarr", "radarr", "lidarr", "readarr", "whisparr"] {
        let cfg: Value = client
            .get(&format!("/api/configuration/{app}"))
            .await
            .expect("reset: list arr instances");
        for i in cfg
            .get("instances")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(id) = i.get("id").and_then(Value::as_str) {
                client
                    .delete(&format!("/api/configuration/{app}/instances/{id}"))
                    .await
                    .expect("reset: delete arr instance");
            }
        }
    }
}

/// Env gate + health wait + [`reset`] to a clean slate. Returns `(url, key)`, or
/// `None` when the env vars are absent (test skipped outside the e2e shell).
async fn setup() -> Option<(String, String)> {
    let (url, key) = env()?;
    let (svc, _) = instance::<CleanuparrV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("cleanuparr healthy");
    let client = core_lib::apply::connect(&svc.connection())
        .await
        .expect("connect");
    reset(&client).await;
    Some((url, key))
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<CleanuparrV1>(url, key, resources);
    apply(&svc, &value, opts).await.expect("apply")
}

/// Apply the same desired state twice; the second run must report no writes.
/// This is the single most valuable e2e assertion for this service, because
/// every custom hook decides idempotency itself via `diff::subset`, and every
/// secret reads back masked.
async fn assert_idempotent(url: &str, key: &str, resources: Value) -> Report {
    let first = run(url, key, resources.clone(), ApplyOptions { prune: false }).await;
    let second = run(url, key, resources, ApplyOptions { prune: false }).await;
    assert_eq!(
        (second.created, second.updated, second.deleted),
        (0, 0, 0),
        "second apply was not a no-op: {second:?} (first: {first:?})",
    );
    first
}

#[tokio::test]
#[ignore]
async fn general_singleton_round_trips() {
    let Some((url, key)) = setup().await else {
        return;
    };
    // A singleton with a nested object (`log`) and an array — the shapes most
    // likely to churn if the codec or merge is wrong.
    assert_idempotent(
        &url,
        &key,
        json!({
            "general": {
                "dry_run": true,
                "http_timeout": 120,
                "ignored_downloads": ["ignore-me"],
                "log": { "level": "Debug", "archive_enabled": true },
            }
        }),
    )
    .await;
}

#[tokio::test]
#[ignore]
async fn arr_instance_lifecycle() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let desired = json!({
        "sonarr_instances": [{
            "name": "main",
            "url": "http://localhost:8989",
            "api_key": "0123456789abcdef0123456789abcdef",
            "version": 4.0,
        }]
    });

    // Create, then prove the masked `apiKey` on read doesn't cause a phantom
    // update on the next run.
    let first = assert_idempotent(&url, &key, desired.clone()).await;
    assert_eq!(first.created, 1, "expected one instance created: {first:?}");

    // Drift a non-secret field → exactly one update.
    let mut drifted = desired.clone();
    drifted["sonarr_instances"][0]["url"] = json!("http://localhost:9999");
    let updated = run(&url, &key, drifted, ApplyOptions { prune: false }).await;
    assert_eq!(updated.updated, 1, "expected one update: {updated:?}");

    // Prune to empty. An **explicit empty list** is required: an absent key
    // means "unmanaged", so `json!({})` would prune nothing.
    let pruned = run(
        &url,
        &key,
        json!({ "sonarr_instances": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert_eq!(pruned.deleted, 1, "expected one delete: {pruned:?}");
}

#[tokio::test]
#[ignore]
async fn queue_rules_crud_and_prune() {
    let Some((url, key)) = setup().await else {
        return;
    };
    let desired = json!({
        "stall_rules": [{
            "name": "stalled-public",
            "max_strikes": 3,
            "privacy_type": "Public",
            "min_completion_percentage": 0,
            "max_completion_percentage": 100,
        }]
    });
    let first = assert_idempotent(&url, &key, desired).await;
    assert_eq!(first.created, 1, "expected one rule created: {first:?}");

    let pruned = run(
        &url,
        &key,
        json!({ "stall_rules": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert_eq!(pruned.deleted, 1, "expected one delete: {pruned:?}");
}

#[tokio::test]
#[ignore]
async fn notification_provider_survives_id_rotation() {
    let Some((url, key)) = setup().await else {
        return;
    };
    // An update deletes and recreates the row server-side, so the provider's id
    // changes underneath us — matching on `name` is what keeps this stable. The
    // password also reads back masked.
    let desired = json!({
        "ntfy_providers": [{
            "name": "ntfy-main",
            "is_enabled": true,
            "server_url": "https://ntfy.sh",
            "topics": ["cleanuparr"],
            "priority": "Default",
            "on_queue_item_deleted": true,
        }]
    });
    let first = assert_idempotent(&url, &key, desired.clone()).await;
    assert_eq!(first.created, 1, "expected one provider created: {first:?}");

    let mut drifted = desired;
    drifted["ntfy_providers"][0]["on_download_cleaned"] = json!(true);
    let updated = run(&url, &key, drifted.clone(), ApplyOptions { prune: false }).await;
    assert_eq!(updated.updated, 1, "expected one update: {updated:?}");

    // …and the post-rotation id is still matched by name on the next run.
    let again = run(&url, &key, drifted, ApplyOptions { prune: false }).await;
    assert_eq!(
        (again.created, again.updated),
        (0, 0),
        "provider churned after its id rotated: {again:?}",
    );
}

#[tokio::test]
#[ignore]
async fn download_client_with_nested_concerns() {
    let Some((url, key)) = setup().await else {
        return;
    };
    // The client plus all four sub-endpoint concerns in one hook: the seeding
    // rules and the three per-client configs are addressed by the client's GUID,
    // which only exists after the client itself is created.
    let desired = json!({
        "download_clients": [{
            "name": "qbit",
            "enabled": true,
            "type_name": "qBittorrent",
            "protocol": "Torrent",
            "host": "http://localhost:8080",
            "username": "admin",
            "password": "adminadmin",
            "seeding_rules": [{
                "name": "movies",
                "categories": ["radarr"],
                "max_ratio": 2.0,
                "min_seed_time": 0.0,
                "max_seed_time": -1.0,
            }],
            "unlinked_config": {
                "enabled": true,
                "target_category": "cleanuparr-unlinked",
                "categories": ["radarr"],
            },
        }]
    });
    let first = assert_idempotent(&url, &key, desired).await;
    assert_eq!(first.created, 1, "expected one client created: {first:?}");
}

#[tokio::test]
#[ignore]
async fn seeker_resolves_a_guid_reference_to_an_instance() {
    let Some((url, key)) = setup().await else {
        return;
    };
    // The payoff test for GUID refs + apply order: the seeker's instance entry
    // addresses a Sonarr instance created in the *same* apply. It only resolves
    // if (a) the instance hook registered its GUID as `RefId::Str`, and (b) the
    // polymorphic `#[reference(sonarr_instance, radarr_instance)]` ordered the
    // seeker after the instance collection.
    let desired = json!({
        "sonarr_instances": [{
            "name": "main",
            "url": "http://localhost:8989",
            "api_key": "0123456789abcdef0123456789abcdef",
            "version": 4.0,
        }],
        "seeker": {
            "search_enabled": true,
            "search_interval": 5,
            "instances": [{
                "arr_instance_id": "${ref.sonarr_instance.main}",
                "enabled": true,
                "monitored_only": true,
            }],
        }
    });
    assert_idempotent(&url, &key, desired).await;
}
