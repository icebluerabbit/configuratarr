//! End-to-end tests against a live Audiobookshelf.
//!
//! Drive the real apply engine (connect → GET live → diff → write), exactly as
//! the CLI does. Guarded by `AUDIOBOOKSHELF_URL` + `AUDIOBOOKSHELF_TOKEN`;
//! `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts ABS, creates the root user, logs in,
//! exports the env vars):
//!   nix develop .#e2e-audiobookshelf --command \
//!     cargo nextest run -p audiobookshelf-v1 --test e2e --run-ignored all -j1
//!
//! This crate is the repo's first **bearer** service, so the live run is the
//! only place two things get proven:
//!   1. auth — the engine sends `Authorization: Bearer <token>` and ABS accepts
//!      it (no other crate exercises `Auth::Bearer`);
//!   2. the envelope hooks — `GET /api/libraries` and `GET /api/users` return
//!      `{libraries: […]}` / `{users: […]}`, so each resource unwraps its own
//!      payload and registers its ids into the `RefStore` by hand.

use std::time::Duration;

use audiobookshelf_v1::AudiobookshelfV1;
use core_lib::Service;
use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::env_pair;
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("AUDIOBOOKSHELF_URL", "AUDIOBOOKSHELF_TOKEN")
}

/// Build the instance `Value` the executor reads desired state from.
///
/// [`core_testkit::instance`] can't serve here: it hardcodes an `api_key`
/// connection key, and this service's credential field is `token` (a bearer
/// JWT, not an API key), so `from_config` would fail with
/// `service config missing 'token'`.
fn build(url: &str, token: &str, resources: Value) -> (AudiobookshelfV1, Value) {
    let mut value = json!({ "url": url, "token": token });
    if let Value::Object(extra) = resources {
        value.as_object_mut().unwrap().extend(extra);
    }
    let svc = AudiobookshelfV1::from_config(&value).expect("connection decodes");
    (svc, value)
}

async fn run(url: &str, token: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = build(url, token, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("audiobookshelf healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// Assert an apply performed no writes.
///
/// Not `assert_eq!(report, Report::default())` — a converged apply still counts
/// every managed resource it found already correct under `unchanged`, so the
/// all-zero report only ever happens when nothing is managed at all.
#[track_caller]
fn assert_no_writes(r: &Report, why: &str) {
    assert_eq!((r.created, r.updated, r.deleted), (0, 0, 0), "{why}: {r:?}");
}

/// Prune the named collections empty so the test that follows starts from a
/// known state — these tests share one live instance, and a previous run's
/// leftovers would turn a `Created` into an `Unchanged`.
async fn reset(url: &str, token: &str, collections: &[&str]) {
    let resources = collections
        .iter()
        .map(|c| ((*c).to_string(), json!([])))
        .collect::<serde_json::Map<_, _>>();
    run(
        url,
        token,
        Value::Object(resources),
        ApplyOptions { prune: true },
    )
    .await;
}

/// `wait_healthy` against the live API: `/api/libraries` answers OK, which also
/// proves the bearer token is accepted.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, token)) = env() else { return };
    let (svc, _) = build(&url, &token, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("audiobookshelf should report healthy");
}

/// No managed resources: connect + bearer auth reach the live API, nothing
/// changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, token)) = env() else { return };
    let report = run(&url, &token, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// Create a library, re-apply (must be a no-op), then prune it. The second
/// apply is what proves the envelope hook's `in_sync` ignores every enriched
/// read-only field ABS adds to `folders[]`.
#[tokio::test]
#[ignore]
async fn library_create_idempotent_prune() {
    let Some((url, token)) = env() else { return };
    reset(&url, &token, &["libraries"]).await;

    let folder = "/tmp/configuratarr-e2e-audiobooks";
    std::fs::create_dir_all(folder).expect("library folder is creatable");

    let desired = json!({
        "libraries": [{
            "name": "configuratarr-e2e",
            "media_type": "book",
            "folders": [{ "full_path": folder }],
        }]
    });

    let created = run(&url, &token, desired.clone(), ApplyOptions::default()).await;
    assert!(
        created.created > 0,
        "first apply should create the library, got {created:?}"
    );

    let again = run(&url, &token, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — the envelope hook's in_sync did not \
         converge against the server-enriched `folders[]`",
    );

    let pruned = run(
        &url,
        &token,
        json!({ "libraries": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(
        pruned.deleted > 0,
        "prune should delete the library, got {pruned:?}"
    );
}

/// A notification's `library_id` written as `${ref.library.<name>}`, with the
/// library created in the same apply.
///
/// This is the crate's headline risk, and nothing offline can reach it.
/// `GET /api/libraries` answers `{libraries: […]}`, an envelope — the engine's
/// own ref-registration only understands a bare array, so the library hook has
/// to `refs.insert` each id by hand. If it stops doing that, this `${ref}` has
/// nothing to resolve against and the apply fails outright.
#[tokio::test]
#[ignore]
async fn notification_resolves_a_ref_to_a_library_created_in_the_same_apply() {
    let Some((url, token)) = env() else { return };
    reset(&url, &token, &["notifications", "libraries"]).await;

    let folder = "/tmp/configuratarr-e2e-notify";
    std::fs::create_dir_all(folder).expect("library folder is creatable");

    let desired = json!({
        "libraries": [{
            "name": "configuratarr-e2e-notify",
            "media_type": "book",
            "folders": [{ "full_path": folder }],
        }],
        "notifications": [{
            "event_name": "onBackupFailed",
            "library_id": "${ref.library.configuratarr-e2e-notify}",
            "urls": ["http://localhost:9/configuratarr-e2e"],
            "title_template": "Backup failed",
            "body_template": "{{message}}",
            "enabled": true,
            "notification_type": "warning",
        }],
    });

    let created = run(&url, &token, desired.clone(), ApplyOptions::default()).await;
    assert!(
        created.created >= 2,
        "library + notification should both be created, got {created:?}"
    );

    let again = run(&url, &token, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — the notification's composite \
         (eventName, libraryId) identity did not match what the server stored",
    );

    let pruned = run(
        &url,
        &token,
        json!({ "notifications": [], "libraries": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(
        pruned.deleted >= 2,
        "prune should delete both, got {pruned:?}"
    );
}

/// `EmailSettings` is the crate's one plain singleton (`GET` + `PATCH`, same
/// shape). The convergence check is what proves the presence mask works: the
/// config declares a handful of keys, and the fields it leaves out — including
/// the server-owned `ereaderDevices` list — must not be sent or diffed.
#[tokio::test]
#[ignore]
async fn email_settings_singleton_converges() {
    let Some((url, token)) = env() else { return };

    let desired = json!({
        "email_settings": {
            "host": "smtp.configuratarr.test",
            "port": 587,
            "secure": false,
            "from_address": "audiobookshelf@configuratarr.test",
        }
    });

    run(&url, &token, desired.clone(), ApplyOptions::default()).await;

    let again = run(&url, &token, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — an undeclared key leaked into the diff",
    );
}

/// E-reader devices have no server id: `name` is the whole identity and the
/// write is a bulk POST of the entire array (`reconcile::replace`). So this
/// proves the order-insensitive structural identity holds — anything
/// server-added leaking into it would re-POST the list every apply.
#[tokio::test]
#[ignore]
async fn ereader_device_replace_converges() {
    let Some((url, token)) = env() else { return };

    let desired = json!({
        "ereader_devices": [{
            "name": "configuratarr-e2e-reader",
            "email": "reader@configuratarr.test",
            "availability_option": "adminOrUp",
        }]
    });

    run(&url, &token, desired.clone(), ApplyOptions::default()).await;

    let again = run(&url, &token, desired, ApplyOptions::default()).await;
    assert_no_writes(&again, "a whole-array replace must still be idempotent");

    run(
        &url,
        &token,
        json!({ "ereader_devices": [] }),
        ApplyOptions { prune: true },
    )
    .await;
}

/// Users come back inside a `{users: […]}` envelope too, and `password` never
/// reads back — so the hook has to unwrap the envelope itself and keep the
/// credential out of the diff. The root account is deliberately left alone:
/// the bearer token belongs to it, and pruning it would end the run.
#[tokio::test]
#[ignore]
async fn user_create_idempotent() {
    let Some((url, token)) = env() else { return };

    let desired = json!({
        "users": [{
            "username": "configuratarr-e2e-user",
            "user_type": "user",
            "password": "configuratarr-e2e-pw",
            "is_active": true,
        }]
    });

    let applied = run(&url, &token, desired.clone(), ApplyOptions::default()).await;
    assert_eq!(
        applied.created + applied.updated + applied.unchanged,
        1,
        "the declared user should be accounted for, got {applied:?}"
    );

    let again = run(&url, &token, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — `password` never reads back, so anything \
         that diffs it would churn forever",
    );
}
