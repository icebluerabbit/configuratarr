//! End-to-end tests against a live Komga.
//!
//! Drive the real apply engine (connect → GET live → diff → write), exactly as
//! the CLI does. Guarded by `KOMGA_URL` + `KOMGA_API_KEY`; `#[ignore]` by
//! default so a normal `cargo nextest run` skips them.
//!
//! Run inside the e2e dev shell (starts Komga, claims it, mints a key, exports
//! the env vars):
//!   nix develop .#e2e-komga --command \
//!     cargo nextest run -p komga-v1 --test e2e --run-ignored all -j1
//!
//! What only a live run can prove for this crate:
//!   1. auth — the minted key is accepted in the `X-API-Key` header;
//!   2. string ids — a created library's opaque string id round-trips through
//!      the `RefStore` and into `${ref.library.<name>}`;
//!   3. the `PATCH` update path and the crud diff converge (a second apply is
//!      a no-op).

use std::time::Duration;

use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use komga_v1::KomgaV1;
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("KOMGA_URL", "KOMGA_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<KomgaV1>(url, key, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("komga healthy");
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
async fn reset(url: &str, key: &str, collections: &[&str]) {
    let resources = collections
        .iter()
        .map(|c| ((*c).to_string(), json!([])))
        .collect::<serde_json::Map<_, _>>();
    run(
        url,
        key,
        Value::Object(resources),
        ApplyOptions { prune: true },
    )
    .await;
}

/// `wait_healthy` against the live API: `/api/v2/users/me` answers OK, which
/// also proves the API key is accepted.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<KomgaV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("komga should report healthy");
}

/// No managed resources: connect + auth reach the live API, nothing changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = env() else { return };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// Create a library, re-apply (must be a no-op), then prune it away. The
/// second apply is the real assertion: it proves the crud diff converges
/// against whatever Komga echoes back, including the fields the config never
/// declared.
#[tokio::test]
#[ignore]
async fn library_create_idempotent_prune() {
    let Some((url, key)) = env() else { return };
    reset(&url, &key, &["libraries"]).await;

    let root = "/tmp/configuratarr-e2e-library";
    std::fs::create_dir_all(root).expect("library root is creatable");

    let desired = json!({
        "libraries": [{ "name": "configuratarr-e2e", "root": root }]
    });

    let created = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        created.created > 0,
        "first apply should create the library, got {created:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — the crud diff did not converge",
    );

    let pruned = run(
        &url,
        &key,
        json!({ "libraries": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(
        pruned.deleted > 0,
        "prune should delete the library, got {pruned:?}"
    );
}

/// A user's `shared_libraries.library_ids` written as
/// `${ref.library.<name>}`, with the library created in the same apply.
///
/// This is the crate's headline risk. Komga ids are opaque **strings**, so the
/// ref has to land in the `RefStore` as a `RefId::Str` and substitute into a
/// `Vec<String>` — conformance only ever exercises that through `GuidRefs`, a
/// double. It also proves the user hook's read/write asymmetry converges: the
/// live `GET` reports flat `sharedAllLibraries` + `sharedLibrariesIds`, while
/// the write takes a nested `sharedLibraries` object.
///
/// **Users are never pruned here.** The connecting API key belongs to the admin
/// account, so a prune that doesn't declare that account tries to delete it and
/// Komga answers 403. Only `libraries` is reset — which also makes the re-run
/// path more interesting than a clean create, since the library comes back with
/// a *new* id and the user's stored ref has to follow it.
#[tokio::test]
#[ignore]
async fn user_resolves_a_ref_to_a_library_created_in_the_same_apply() {
    let Some((url, key)) = env() else { return };
    reset(&url, &key, &["libraries"]).await;

    let root = "/tmp/configuratarr-e2e-shared";
    std::fs::create_dir_all(root).expect("library root is creatable");

    let desired = json!({
        "libraries": [{ "name": "configuratarr-e2e-shared", "root": root }],
        "users": [{
            "email": "e2e-reader@configuratarr.test",
            "password": "configuratarr-e2e-pw",
            "roles": ["PAGE_STREAMING", "FILE_DOWNLOAD"],
            "shared_libraries": {
                "all": false,
                "library_ids": ["${ref.library.configuratarr-e2e-shared}"],
            },
        }],
    });

    let applied = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        applied.created + applied.updated >= 2,
        "library + user should both be written, got {applied:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — a churning user means either `password` \
         leaked into the diff or the flat/nested shared-libraries shapes were \
         compared directly",
    );
}

/// The `settings` singleton — a `sync = custom` hook, because three fields
/// (`server_port`, `server_context_path`, `kepubify_path`) read back as
/// `{configurationSource, databaseSource, effectiveValue}` triples but are
/// written as plain scalars. A plain `sync = singleton` would compare the
/// scalar against the triple and report an update forever, so the convergence
/// assertion here is the whole point.
///
/// `server_port` and `server_context_path` are deliberately not declared:
/// changing either moves the API out from under the running test.
#[tokio::test]
#[ignore]
async fn settings_singleton_converges() {
    let Some((url, key)) = env() else { return };

    let desired = json!({
        "settings": {
            "delete_empty_collections": false,
            "delete_empty_read_lists": false,
            "remember_me_duration_days": 30,
            "task_pool_size": 4,
            "thumbnail_size": "LARGE",
        }
    });

    run(&url, &key, desired.clone(), ApplyOptions::default()).await;

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — the hook did not unwrap a source triple",
    );
}

/// Client settings are dotted-key maps, not arrays: the list endpoint returns
/// a JSON *object* and writes are a whole-map `PATCH` that upsert-merges. The
/// hook folds the map key into each entry as `key` so `reconcile::upsert` can
/// match on it — this proves that folding round-trips.
///
/// No prune assertion: `DELETE /api/v1/client-settings/global` needs a JSON
/// array body, which `HttpClient` has no verb for, so the hook cannot delete
/// (documented in `client_settings.rs`).
#[tokio::test]
#[ignore]
async fn client_setting_global_upserts_and_converges() {
    let Some((url, key)) = env() else { return };

    let desired = json!({
        "client_settings_global": [{
            "key": "configuratarr.e2e",
            "value": "true",
            "allow_unauthorized": false,
        }]
    });

    let applied = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert_eq!(
        applied.created + applied.updated + applied.unchanged,
        1,
        "the declared setting should be accounted for, got {applied:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — the object-map fold did not round-trip",
    );
}

/// API keys are create-only: Komga exposes no update verb and returns the
/// secret exactly once, on create. So a re-apply must leave an existing key
/// alone rather than minting a second one — and the secret must never reach
/// the plan, which is why nothing here inspects it.
///
/// **This test never prunes.** `api_keys` belong to the authenticated user, so
/// a prune revokes every key the config doesn't declare — including the one
/// configuratarr is holding. Doing that here 401s the rest of the suite. The
/// hazard is real and documented on the resource; it is not something to
/// demonstrate from inside a test that needs the credential to keep working.
#[tokio::test]
#[ignore]
async fn api_key_is_created_once_then_left_alone() {
    let Some((url, key)) = env() else { return };

    let desired = json!({
        "api_keys": [{ "comment": "configuratarr-e2e-managed" }]
    });

    let applied = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert_eq!(
        applied.created + applied.unchanged,
        1,
        "the declared key should be accounted for, got {applied:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "a create-only resource must never re-create an existing key",
    );
}
