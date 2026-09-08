//! End-to-end tests against a live Whisparr (eros / v3).
//!
//! These drive the *real* apply engine (`core_lib::apply::apply`) — connect →
//! GET live → plan → merge → POST/PUT/DELETE — exactly as the CLI does. Guarded
//! by `WHISPARR_URL` + `WHISPARR_API_KEY`; `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts Whisparr, exports the env vars):
//!   nix develop .#e2e-whisparr --command \
//!     cargo test -p whisparr-v3 --test e2e -- --ignored
//!
//! The instance under test is the **eros** line — a Radarr fork. Whisparr v2 is
//! a *Sonarr* fork and serves a different API on the same `/api/v3` path, so a
//! v2 instance will fail these in confusing ways. `nix/pkgs/whisparr-eros.nix`
//! pins the right one.

use std::time::Duration;

use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use serde_json::{Value, json};
use whisparr_v3::WhisparrV3;

/// `(url, api_key)` from the environment, or `None` to skip when not in the shell.
fn env() -> Option<(String, String)> {
    env_pair("WHISPARR_URL", "WHISPARR_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<WhisparrV3>(url, key, resources);
    // Every test goes through the health gate first — exercises wait_healthy and
    // tolerates a still-starting Whisparr.
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("whisparr healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// `wait_healthy` against the live API: Whisparr's `/system/status` answers OK.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<WhisparrV3>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("whisparr should report healthy");
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
    assert_eq!(
        r1,
        Report {
            created: 1,
            ..Default::default()
        },
        "first apply must create the tag"
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

/// Singleton UPDATE — PUT to `/config/ui/${self.id}`. The no-op test never sends
/// a PUT, so this is what actually exercises the id-in-path form. Two distinct
/// values: after step 1 the server holds the first, so step 2 is guaranteed to
/// change regardless of the server's starting value.
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
/// `tags: ["${ref...}"]`, which Whisparr rejects (not an integer).
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

/// A MediaBrowser (Emby) notification whose `api_key` is a `SecretValue` in the
/// fields-blob: the plaintext must reach the wire under the name eros expects
/// (`apiKey`, which `api_key` camelCases to). Whisparr validates provider fields
/// *before* live-testing the connection, so an empty secret fails with an
/// `'Api Key' must not be empty`-style error, while a secret that reached the
/// wire clears validation and only then fails on the unreachable dummy host. We
/// assert the error (if any) is **not** the empty-key one.
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

    let (svc, value) = instance::<WhisparrV3>(&url, &key, cfg);
    wait_healthy(&svc, Duration::from_secs(30))
        .await
        .expect("whisparr healthy");
    let outcome = apply(&svc, &value, ApplyOptions::default()).await;

    if let Err(e) = &outcome {
        let msg = format!("{e:#}");
        assert!(
            !msg.contains("Api Key") && !msg.contains("ApiKey"),
            "secret did not reach the wire — Whisparr rejected the ApiKey: {msg}"
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

/// Whisparr-specific: `ImportListExclusion` is a `sync = custom` hook because
/// `GET /api/v3/exclusions` is type-filtered (defaults to `scene`) — the hook
/// reads `/exclusions/paged` and unwraps `.records`. This exercises a
/// **`performer`** exclusion specifically: under the old crud list step it was
/// invisible, so the second apply re-created it and the API rejected the
/// duplicate. Create → idempotent re-apply → prune must all hold.
#[tokio::test]
#[ignore]
async fn import_list_exclusion_non_scene_type_round_trips() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({ "import_list_exclusions": [ {
        "foreign_id": "e2e-configuratarr-excl",
        "exclusion_type": "performer",
        "movie_title": "e2e configuratarr exclusion",
        "movie_year": 2020
    } ] });

    let r1 = run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 1,
        "exclusion present after first apply: {r1:?}"
    );

    // The regression that motivated the custom hook: a non-scene exclusion the
    // list step could not see was re-created here and rejected with
    // `ImportListExclusionExistsValidator`.
    // Strict: a hook whose `in_sync` wrongly reports "changed" would churn an
    // `updated: 1` here forever and a `created == 0` assertion would not see it.
    let r2 = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "second apply must be a clean no-op, not an update: {r2:?}"
    );

    let r3 = run(
        &url,
        &key,
        json!({ "import_list_exclusions": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(r3.deleted >= 1, "prune must delete the exclusion: {r3:?}");
}

/// Whisparr-specific: `DelayProfile` is a `sync = custom` hook keyed on its
/// **tag set**, because eros assigns `order` itself (a POST of `"order": 100`
/// returns `"order": 1`) and so no declared key can ever match. Under the old
/// crud step the second apply re-created the profile and eros rejected it with
/// `One or more tags is used in another profile`; this asserts it no longer does.
///
/// Also covers the two live-verified constraints: a non-built-in profile must
/// carry at least one tag (`'Tags' must not be empty`), and prune must leave the
/// undeletable built-in (empty tag set) alone.
#[tokio::test]
#[ignore]
async fn delay_profile_keyed_on_tag_set_round_trips() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "tags": [ { "label": "e2e-configuratarr-delaytag" } ],
        "delay_profiles": [ {
            "enable_usenet": true,
            "enable_torrent": true,
            "preferred_protocol": "usenet",
            "usenet_delay": 0,
            "torrent_delay": 0,
            "bypass_if_highest_quality": true,
            "bypass_if_above_custom_format_score": false,
            "minimum_custom_format_score": 0,
            "tags": [ "${ref.tag.e2e-configuratarr-delaytag}" ]
        } ]
    });

    let r1 = run(&url, &key, cfg.clone(), ApplyOptions::default()).await;
    assert!(
        r1.created + r1.unchanged >= 2,
        "tag + delay profile present after first apply: {r1:?}"
    );

    // The regression: server-assigned `order` made the profile unmatchable.
    // Strict: `unchanged: 2` (tag + profile). A churning `in_sync` would show
    // `updated: 1` here, which a `created == 0` assertion would happily pass.
    let r2 = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        r2,
        Report {
            unchanged: 2,
            ..Default::default()
        },
        "second apply must be a clean no-op, not an update: {r2:?}"
    );

    // Prune the profile but not the built-in; then the tag (profile first, a
    // referenced tag cannot be deleted while in use).
    let prune = ApplyOptions { prune: true };
    let r3 = run(&url, &key, json!({ "delay_profiles": [] }), prune).await;
    assert!(
        r3.deleted >= 1,
        "prune must delete the tagged profile: {r3:?}"
    );
    let _ = run(&url, &key, json!({ "tags": [] }), prune).await;
}

/// The seeded **global** delay profile (`id: 1`, empty tag set) governs every
/// item with no matching tag, so a config must be able to manage it. eros guards
/// only `Delete` on that row (`DelayProfileController.cs:47-55`); `Update` has no
/// id check, and the validator *requires* an empty tag set when `Id == 1`. So
/// `tags: []` addresses it and must produce an UPDATE — not a create attempt,
/// which would fail with `'Tags' must not be empty`.
#[tokio::test]
#[ignore]
async fn global_delay_profile_is_updatable_via_empty_tags() {
    let Some((url, key)) = env() else { return };
    let profile = |usenet_delay: i32| {
        json!({ "delay_profiles": [ {
            "enable_usenet": true,
            "enable_torrent": true,
            "preferred_protocol": "usenet",
            "usenet_delay": usenet_delay,
            "torrent_delay": 0,
            "bypass_if_highest_quality": true,
            "bypass_if_above_custom_format_score": false,
            "minimum_custom_format_score": 0,
            "tags": []
        } ] })
    };

    // Two distinct values so the second apply is guaranteed to be a change
    // whatever the server started from.
    let _ = run(&url, &key, profile(5), ApplyOptions::default()).await;
    let r = run(&url, &key, profile(10), ApplyOptions::default()).await;
    assert_eq!(
        r,
        Report {
            updated: 1,
            ..Default::default()
        },
        "empty tags must UPDATE the global profile, never create: {r:?}"
    );

    // Re-applying the same value is a clean no-op.
    let again = run(&url, &key, profile(10), ApplyOptions::default()).await;
    assert_eq!(
        again,
        Report {
            unchanged: 1,
            ..Default::default()
        },
        "global profile must not churn: {again:?}"
    );

    // A prune that no longer declares it must leave it alone — eros refuses the
    // delete, so an attempt would fail the apply outright.
    let pruned = run(
        &url,
        &key,
        json!({ "delay_profiles": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert_eq!(
        pruned.deleted, 0,
        "the global profile must never be pruned: {pruned:?}"
    );

    let _ = run(&url, &key, profile(0), ApplyOptions::default()).await;
}
