//! End-to-end tests against a live Lidarr.
//!
//! These drive the *real* apply engine (`core_lib::apply::apply`) — connect →
//! GET live → plan → merge → POST/PUT/DELETE — exactly as the CLI does. Guarded
//! by `LIDARR_URL` + `LIDARR_API_KEY`; `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts Lidarr, exports the env vars):
//!   nix develop .#e2e-lidarr --command \
//!     cargo test -p lidarr-v1 --test e2e -- --ignored

use std::time::Duration;

use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use lidarr_v1::LidarrV1;
use serde_json::{Value, json};

/// `(url, api_key)` from the environment, or `None` to skip when not in the shell.
fn env() -> Option<(String, String)> {
    env_pair("LIDARR_URL", "LIDARR_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<LidarrV1>(url, key, resources);
    // Every test goes through the health gate first — exercises wait_healthy and
    // tolerates a still-starting Lidarr.
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("lidarr healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// `wait_healthy` against the live API: Lidarr's `/system/status` answers OK.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<LidarrV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("lidarr should report healthy");
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

/// An empty singleton config is a no-op — presence-masking must not clobber the
/// server's media-management settings with type defaults.
#[tokio::test]
#[ignore]
async fn singleton_empty_config_noop() {
    let Some((url, key)) = env() else { return };
    let report = run(
        &url,
        &key,
        json!({ "media_management": {} }),
        ApplyOptions::default(),
    )
    .await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "empty singleton must no-op: {report:?}"
    );
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

/// A download client referencing a tag by name. The apply succeeding against the
/// real API *is* the proof the ref resolved — an unresolved `${ref}` would send
/// `tags: ["${ref...}"]`, which Lidarr rejects (not an integer).
#[tokio::test]
#[ignore]
async fn download_client_resolves_tag_ref() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "tags": [ { "label": "e2e-configuratarr-reftag" } ],
        "download_clients": [ {
            "name": "e2e-configuratarr-refdc",
            "implementation": "UsenetBlackhole",
            "nzb_folder": "/tmp",
            "watch_folder": "/tmp",
            "enable": true,
            "protocol": "usenet",
            "tags": [ "${ref.tag.e2e-configuratarr-reftag}" ]
        } ]
    });

    // Tag is created first (topo order), its id fed to the client's ref.
    let r = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert!(r.created + r.unchanged >= 2, "tag + client applied: {r:?}");

    // Cleanup, client before tag (a referenced tag can't be deleted first).
    let prune = ApplyOptions { prune: true };
    let _ = run(&url, &key, json!({ "download_clients": [] }), prune).await;
    let _ = run(&url, &key, json!({ "tags": [] }), prune).await;
}
