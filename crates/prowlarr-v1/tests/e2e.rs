//! End-to-end tests against a live Prowlarr.
//!
//! These drive the *real* apply engine (`core_lib::apply::apply`) — connect →
//! GET live → plan → merge → POST/PUT/DELETE — exactly as the CLI does. Guarded
//! by `PROWLARR_URL` + `PROWLARR_API_KEY`; `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts Prowlarr, exports the env vars):
//!   nix develop .#e2e-prowlarr --command \
//!     cargo test -p prowlarr-v1 --test e2e -- --ignored

use std::time::Duration;

use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use prowlarr_v1::ProwlarrV1;
use serde_json::{Value, json};

/// `(url, api_key)` from the environment, or `None` to skip when not in the shell.
fn env() -> Option<(String, String)> {
    env_pair("PROWLARR_URL", "PROWLARR_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<ProwlarrV1>(url, key, resources);
    // Every test goes through the health gate first — exercises wait_healthy and
    // tolerates a still-starting Prowlarr.
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("prowlarr healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// `wait_healthy` against the live API: Prowlarr's `/system/status` answers OK.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<ProwlarrV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("prowlarr should report healthy");
}

/// A config with no managed resource types still connects and does nothing —
/// proves connect + auth (X-Api-Key) reach the live API.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = env() else { return };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// Full tag lifecycle through the engine: create → idempotent re-apply → prune.
#[tokio::test]
#[ignore]
async fn tag_create_idempotent_prune() {
    let Some((url, key)) = env() else { return };
    let desired = json!({ "tags": [ { "label": "e2e-configuratarr" } ] });

    let r1 = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 1,
        "expected the tag present after apply: {r1:?}"
    );

    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second apply must be a no-op"
    );

    let r3 = run(
        &url,
        &key,
        json!({ "tags": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(r3.deleted >= 1, "prune should delete the e2e tag: {r3:?}");
}

/// Singleton UPDATE — PUT to `/config/ui/${self.id}`. Two distinct values: after
/// step 1 the server holds the first, so step 2 is guaranteed to change
/// (regardless of the server's starting value). Exercises the id-in-path PUT.
#[tokio::test]
#[ignore]
async fn singleton_update_via_id_path() {
    let Some((url, key)) = env() else { return };
    let _ = run(
        &url,
        &key,
        json!({ "ui_config": { "first_day_of_week": 1 } }),
        ApplyOptions::default(),
    )
    .await;
    let r = run(
        &url,
        &key,
        json!({ "ui_config": { "first_day_of_week": 2 } }),
        ApplyOptions::default(),
    )
    .await;
    assert_eq!(
        r,
        Report {
            updated: 1,
            ..Default::default()
        },
        "singleton PUT to /{{id}} must update: {r:?}"
    );
    // restore
    let _ = run(
        &url,
        &key,
        json!({ "ui_config": { "first_day_of_week": 1 } }),
        ApplyOptions::default(),
    )
    .await;
}

/// Prowlarr-specific collection create + idempotent re-apply. App profiles are
/// NOT prune-tested: Prowlarr ships an undeletable default "Standard" profile,
/// so pruning the collection to empty hits `ProfileInUseException` (HTTP 500) on
/// the default. (Tag/download-client cover the create→prune path cleanly.) The
/// e2e instance is ephemeral, so the created profile is left behind.
#[tokio::test]
#[ignore]
async fn app_profile_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let desired = json!({ "app_profiles": [ {
        "name": "e2e-configuratarr-profile",
        "enable_rss": true,
        "enable_automatic_search": true,
        "enable_interactive_search": true,
        "minimum_seeders": 1
    } ] });

    let r1 = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 1,
        "app profile present after apply: {r1:?}"
    );

    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(r2.created, 0, "second apply must not recreate: {r2:?}");
}

/// Raw `#[fields_map]` provider lifecycle against the real API: a Cardigann
/// indexer whose open settings are authored as a `fields` *map* must reach
/// Prowlarr as the `fields: [{name, value}]` blob. Proves the map→array splay
/// end-to-end (the typed `download_client` test can't — it has no `RawProvider`).
/// Uses the bundled `0magnet` public definition; `?forceSave=true` on the
/// resource's create skips the live connectivity test against the fake baseUrl.
/// The indexer refs an app profile created in the same apply (topo-ordered).
#[tokio::test]
#[ignore]
async fn indexer_raw_fields_map_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "app_profiles": [ {
            "name": "e2e-configuratarr-idx-profile",
            "enable_rss": true,
            "enable_automatic_search": true,
            "enable_interactive_search": true,
            "minimum_seeders": 1
        } ],
        "indexers": [ {
            "name": "e2e-configuratarr-0magnet",
            "implementation": "Cardigann",
            "config_contract": "CardigannSettings",
            // The open Cardigann settings as a plain map — the #[fields_map] path.
            "fields": {
                "definitionFile": "0magnet",
                "baseUrl": "https://0magnet.test"
            },
            "enable": false,
            "redirect": false,
            "priority": 25,
            "protocol": "torrent",
            "privacy": "public",
            "app_profile_id": "${ref.app_profile.e2e-configuratarr-idx-profile}",
            "tags": []
        } ]
    });

    // Profile + indexer both created; the ref resolved (an unresolved
    // app_profile_id would send a string and Prowlarr would reject it).
    let r1 = run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 2,
        "app profile + indexer applied: {r1:?}"
    );

    // Re-apply must not recreate the indexer — proves the map round-trips
    // (decode live blob → map → merge → no-op), idempotent.
    let r2 = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(r2.created, 0, "second apply must not recreate: {r2:?}");

    // Cleanup: prune the indexer (deletable). The app profile is left behind —
    // pruning it hits the undeletable-default trap; the e2e instance is ephemeral.
    let _ = run(
        &url,
        &key,
        json!({ "indexers": [] }),
        ApplyOptions { prune: true },
    )
    .await;
}

/// Provider (fields-blob) lifecycle against the real API: the typed flat config
/// (`implementation` + flat `nzb_folder`) becomes the *arr `{implementation,
/// configContract, fields:[...]}` envelope. UsenetBlackhole needs no
/// connectivity check, so it's the safe choice.
#[tokio::test]
#[ignore]
async fn download_client_create_idempotent() {
    let Some((url, key)) = env() else { return };
    let dc = json!({ "download_clients": [ {
        "name": "e2e-configuratarr-dc",
        "implementation": "UsenetBlackhole",
        "nzb_folder": "/tmp",
        "watch_folder": "/tmp",
        "enable": true,
        "protocol": "usenet",
        "categories": [],
        "tags": []
    } ] });

    let r1 = run(&url, &key, dc.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 1,
        "client present after apply: {r1:?}"
    );

    // Re-apply must not recreate (idempotent); merge keeps the server's extra
    // provider fields, so at worst it's a no-op.
    let r2 = run(&url, &key, dc, ApplyOptions::default()).await;
    assert_eq!(r2.created, 0, "second apply must not recreate: {r2:?}");

    // cleanup
    let _ = run(
        &url,
        &key,
        json!({ "download_clients": [] }),
        ApplyOptions { prune: true },
    )
    .await;
}
