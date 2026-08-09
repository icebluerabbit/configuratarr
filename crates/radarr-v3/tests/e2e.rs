//! End-to-end tests against a live Radarr.
//!
//! These drive the *real* apply engine (`core_lib::apply::apply`) — connect →
//! GET live → plan → merge → POST/PUT/DELETE — exactly as the CLI does. Guarded
//! by `RADARR_URL` + `RADARR_API_KEY`; `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts Radarr, exports the env vars):
//!   nix develop .#e2e-radarr --command \
//!     cargo test -p radarr-v3 --test e2e -- --ignored

use std::time::Duration;

use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use radarr_v3::RadarrV3;
use serde_json::{Value, json};

/// `(url, api_key)` from the environment, or `None` to skip when not in the shell.
fn env() -> Option<(String, String)> {
    env_pair("RADARR_URL", "RADARR_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<RadarrV3>(url, key, resources);
    // Every test goes through the health gate first — exercises wait_healthy and
    // tolerates a still-starting Radarr.
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("radarr healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// `wait_healthy` against the live API: Radarr's `/system/status` answers OK.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<RadarrV3>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("radarr should report healthy");
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

    // First apply: the tag is created (or already there from a prior run).
    let r1 = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 1,
        "expected the tag present after apply: {r1:?}"
    );

    // Re-apply identical desired state: nothing changes (idempotent merge).
    let r2 = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second apply must be a no-op"
    );

    // Prune with empty desired: the tag is deleted.
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

/// Singleton UPDATE — PUT to `/config/ui/${self.id}`. The no-op test never sent
/// a PUT, so this is what actually exercises the id-in-path fix. Two distinct
/// values: after step 1 the server holds the first, so step 2 is guaranteed to
/// change (regardless of the server's starting value).
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
/// `tags: ["${ref...}"]`, which Radarr rejects (not an integer).
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

/// A MediaBrowser (Emby/Jellyfin) notification whose `api_key` is a `SecretValue`
/// in the fields-blob. Regression guard for issue #31: the secret's plaintext
/// must reach the wire under the field name Radarr expects (`apiKey`, which
/// `api_key` camelCases to — an earlier `#[wire(name = "aPIKey")]` override sent
/// it under a key Radarr ignored, so `ApiKey` arrived empty).
///
/// Radarr validates the provider fields *before* it live-tests the connection,
/// so a missing/empty secret fails with `'Api Key' must not be empty`, while a
/// secret that reached the wire clears field validation and only then fails on
/// the unreachable dummy host. We therefore assert the apply error (if any) is
/// **not** the empty-ApiKey one — a connection failure to `localhost:8096` is
/// the expected, healthy outcome without a real Emby to point at.
#[tokio::test]
#[ignore]
async fn notification_secret_field_reaches_wire() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({ "notifications": [ {
        "name": "e2e-configuratarr-emby",
        "implementation": "MediaBrowser",
        "host": "localhost",
        "port": 8096,
        "api_key": "e2econfiguratarrsecretkey0000",
        "notify": false,
        "update_library": true,
        "on_download": true,
        "on_upgrade": true,
        "on_rename": true,
        "tags": []
    } ] });

    let (svc, value) = instance::<RadarrV3>(&url, &key, cfg);
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("radarr healthy");
    let outcome = apply(&svc, &value, ApplyOptions::default()).await;

    if let Err(e) = &outcome {
        let msg = format!("{e:#}");
        assert!(
            !msg.contains("Api Key") && !msg.contains("ApiKey"),
            "secret did not reach the wire — Radarr rejected the ApiKey: {msg}"
        );
        // Any other error (e.g. connection refused to the dummy Emby host) is
        // fine: field validation passed, proving the secret was on the wire.
    }

    // cleanup (best-effort: the create above may or may not have persisted).
    let _ = run(
        &url,
        &key,
        json!({ "notifications": [] }),
        ApplyOptions { prune: true },
    )
    .await;
}
