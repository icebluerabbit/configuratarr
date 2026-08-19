//! End-to-end tests against a live Bindery.
//!
//! Drive the real apply engine (connect → GET live → diff → write), exactly as
//! the CLI does. Guarded by `BINDERY_URL` + `BINDERY_API_KEY`; `#[ignore]` by
//! default.
//!
//! Run inside the e2e dev shell (starts Bindery with a known instance key and
//! exports the env vars):
//!   nix develop .#e2e-bindery --command \
//!     cargo nextest run -p bindery-v1 --test e2e --run-ignored all -j1
//!
//! What only a live run can prove for this crate:
//!   1. auth — the key is accepted in the `X-Api-Key` header (note the casing);
//!   2. the crud diff converges against what Bindery echoes back — several
//!      fields are rewritten server-side (`host` loses its scheme, quality
//!      names are trimmed and lower-cased), which is exactly what a second
//!      apply catches;
//!   3. `${ref.*}` resolution — the id a create returns lands in the `RefStore`
//!      and substitutes into a later resource's FK. Conformance can't see this:
//!      it resolves every ref through a dummy and never runs the apply order;
//!   4. the `sync = custom` hooks. Nine of this crate's seventeen resources are
//!      custom, and each one owns its own HTTP, ordering and idempotency —
//!      none of which conformance exercises, since it never calls a hook.

use std::time::Duration;

use bindery_v1::BinderyV1;
use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("BINDERY_URL", "BINDERY_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<BinderyV1>(url, key, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("bindery healthy");
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
/// known state.
///
/// These tests share one live instance and a previous run's leftovers would
/// otherwise turn a `Created` into an `Unchanged` — which is exactly the
/// difference several of them assert on. Cheaper and more honest than making
/// every assertion tolerate both outcomes.
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

/// `wait_healthy` against the live API: `/api/v1/system/status` answers OK,
/// which also proves the API key is accepted (it is an authenticated route).
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<BinderyV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("bindery should report healthy");
}

/// No managed resources: connect + auth reach the live API, nothing changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = env() else { return };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// Create a quality profile, re-apply (must be a no-op), then prune it. The
/// second apply is the real assertion: Bindery lower-cases `cutoff` and each
/// `quality` server-side, so a config that does not already match would churn.
#[tokio::test]
#[ignore]
async fn quality_profile_create_idempotent_prune() {
    let Some((url, key)) = env() else { return };
    reset(&url, &key, &["quality_profiles"]).await;

    let desired = json!({
        "quality_profiles": [{
            "name": "configuratarr-e2e",
            "cutoff": "epub",
            "items": [
                { "quality": "epub", "allowed": true },
                { "quality": "mobi", "allowed": false },
            ],
        }]
    });

    let created = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        created.created > 0 || created.updated > 0,
        "first apply should write the profile, got {created:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — the crud diff did not converge",
    );

    let pruned = run(
        &url,
        &key,
        json!({ "quality_profiles": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(
        pruned.deleted > 0,
        "prune should delete the profile, got {pruned:?}"
    );
}

/// A root folder is create-and-delete only (its sole writable field is its
/// key), so this checks the archetype the crate leans on: no update endpoint,
/// yet a repeated apply still converges.
#[tokio::test]
#[ignore]
async fn root_folder_create_idempotent() {
    let Some((url, key)) = env() else { return };

    // Bindery stats the path and rejects a root folder that doesn't exist, so
    // the directory has to be there first. The test binary runs on the same
    // host as the server in both the dev shell and the VM.
    let path = "/tmp/configuratarr-e2e-books";
    std::fs::create_dir_all(path).expect("root folder path is creatable");

    let desired = json!({ "root_folders": [{ "path": path }] });

    run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "a root folder with no update endpoint must never produce a diff",
    );

    run(
        &url,
        &key,
        json!({ "root_folders": [] }),
        ApplyOptions { prune: true },
    )
    .await;
}

/// `Indexer.prowlarr_instance_id` written as `${ref.prowlarr_instance.<name>}`.
///
/// The instance is created in the same apply, so its id exists only in the
/// `RefStore` — the ref resolves to a server-assigned `RefId` that no config
/// file ever spelled out. That is the whole point: it proves `apply_order`
/// really put `prowlarr_instance` first (the alphabetical fallback would not
/// have) and that the create response's id was registered before the indexer
/// encoded.
#[tokio::test]
#[ignore]
async fn indexer_resolves_a_ref_to_a_prowlarr_instance_created_in_the_same_apply() {
    let Some((url, key)) = env() else { return };
    reset(&url, &key, &["indexers", "prowlarr_instances"]).await;

    let desired = json!({
        "prowlarr_instances": [{
            "name": "configuratarr-e2e-prowlarr",
            "url": "http://localhost:9696",
            "api_key": "e2e-prowlarr-key",
        }],
        "indexers": [{
            "name": "configuratarr-e2e-indexer",
            "url": "http://localhost:9117/api",
            "api_key": "e2e-indexer-key",
            "prowlarr_instance_id": "${ref.prowlarr_instance.configuratarr-e2e-prowlarr}",
        }],
    });

    let created = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        created.created >= 2,
        "both resources should be created, got {created:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — a churning `prowlarr_instance_id` means \
         the ref resolved to a different value than the server stored",
    );

    // Deleting a Prowlarr instance cascade-deletes every indexer synced from
    // it, and prune visits `prowlarr_instance` first (apply order, reversed for
    // deletes it is not — prune follows the same order). So this reports one
    // delete, not two: by the time the indexer step lists, the row is gone.
    let empty = json!({ "indexers": [], "prowlarr_instances": [] });
    let pruned = run(&url, &key, empty.clone(), ApplyOptions { prune: true }).await;
    assert!(
        pruned.deleted >= 1,
        "prune should delete the instance, got {pruned:?}"
    );

    let settled = run(&url, &key, empty, ApplyOptions { prune: true }).await;
    assert_no_writes(
        &settled,
        "both collections should be empty after the prune — the indexer went \
         with its instance",
    );
}

/// The `setting` singleton — a `sync = custom` hook over the per-key
/// `PUT /api/v1/setting/{key}` table.
///
/// Only non-secret, non-filesystem keys are touched: validation for the path
/// keys stats the local filesystem as a side effect, and the two writable
/// secrets never read back, so they always report `Updated` and could never be
/// part of a convergence assertion.
#[tokio::test]
#[ignore]
async fn setting_singleton_converges() {
    let Some((url, key)) = env() else { return };

    let desired = json!({
        "setting": {
            "import_mode": "copy",
            "default_media_type": "ebook",
            "default_media_type_strict": true,
            "author_default_monitor_mode": "latest",
            "author_default_monitor_latest_count": 3,
            "metadata_primary_provider": "openlibrary",
        }
    });

    let applied = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert_eq!(
        applied.updated + applied.unchanged,
        6,
        "every declared key should be accounted for, got {applied:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — the hook compares the stringified value \
         against the live row, so a mismatch means `scalar` and the server disagree",
    );
}

/// Users are the crate's one bespoke state machine: create via `POST`, role
/// change via `PUT .../{id}/role`, delete via `prune_absent`. There is no
/// whole-object update, so each transition is driven separately — and the role
/// change is the one an ordinary crud diff would have missed entirely.
///
/// Two users, not one, because Bindery refuses to demote or delete its **last
/// admin**. A single user promoted to `admin` traps itself: the demote and the
/// prune both 400. So `keeper` holds the admin role throughout and `subject`
/// is the one moved around — and `keeper` is deliberately left behind at the
/// end, since nothing can remove it.
#[tokio::test]
#[ignore]
async fn user_role_changes_both_ways_and_prunes() {
    let Some((url, key)) = env() else { return };

    let keeper = json!({
        "username": "configuratarr-e2e-keeper",
        "role": "admin",
        "password": "configuratarr-e2e-pw",
    });
    let subject = |role: &str| {
        json!({
            "username": "configuratarr-e2e-subject",
            "role": role,
            "password": "configuratarr-e2e-pw",
        })
    };
    let both = |role: &str| json!({ "users": [keeper.clone(), subject(role)] });

    // Prune to exactly these two first: a previous run may have left an admin
    // behind, and it has to go before `created + unchanged` means anything.
    // Safe because `keeper` is declared, so an admin always survives.
    let start = run(&url, &key, both("user"), ApplyOptions { prune: true }).await;
    assert_eq!(
        start.created + start.unchanged + start.updated,
        2,
        "both users should exist after the first apply, got {start:?}"
    );

    let again = run(&url, &key, both("user"), ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "second apply must be a no-op — `password` never reads back, so anything          that diffs it would churn forever",
    );

    let promoted = run(&url, &key, both("admin"), ApplyOptions::default()).await;
    assert!(
        promoted.updated > 0,
        "promoting `subject` must drive PUT .../role, got {promoted:?}"
    );

    // Demote only works because `keeper` is still an admin — this is the
    // direction that proves the hook drives the role endpoint rather than
    // relying on a create-time default.
    let demoted = run(&url, &key, both("user"), ApplyOptions::default()).await;
    assert!(
        demoted.updated > 0,
        "demoting `subject` must drive PUT .../role, got {demoted:?}"
    );

    let pruned = run(
        &url,
        &key,
        json!({ "users": [keeper] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert_eq!(
        pruned.deleted, 1,
        "prune should delete `subject` and keep `keeper`, got {pruned:?}"
    );
}

/// Delay profiles have no name field, so no `#[key]` and no crud path: the hook
/// is a whole-list `reconcile::replace` that DELETEs every live profile and
/// re-POSTs the desired ones. The convergence check is what proves the
/// structural identity (`order`, the two delays, the protocol flags) matches
/// what the server echoes — an id or `createdAt` leaking into it would recreate
/// the list on every single apply.
#[tokio::test]
#[ignore]
async fn delay_profile_replace_converges() {
    let Some((url, key)) = env() else { return };

    let desired = json!({
        "delay_profiles": [{
            "usenet_delay": 15,
            "torrent_delay": 45,
            "preferred_protocol": "usenet",
            "enable_usenet": true,
            "enable_torrent": true,
            "order": 0,
        }]
    });

    run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(&again, "a whole-list replace must still be idempotent");
}

/// Import-list exclusions are `create_only_prune`: there is no update verb, so
/// a declared exclusion is created once and then left alone, and only `--prune`
/// removes one.
#[tokio::test]
#[ignore]
async fn import_list_exclusion_create_only_then_prune() {
    let Some((url, key)) = env() else { return };
    reset(&url, &key, &["import_list_exclusions"]).await;

    let desired = json!({
        "import_list_exclusions": [{
            "foreign_id": "configuratarr-e2e:1",
            "title": "Configuratarr E2E",
            "author_name": "Configuratarr",
        }]
    });

    let created = run(&url, &key, desired.clone(), ApplyOptions::default()).await;
    assert!(
        created.created > 0,
        "first apply should create the exclusion, got {created:?}"
    );

    let again = run(&url, &key, desired, ApplyOptions::default()).await;
    assert_no_writes(
        &again,
        "an existing exclusion must be left alone, never re-created",
    );

    let pruned = run(
        &url,
        &key,
        json!({ "import_list_exclusions": [] }),
        ApplyOptions { prune: true },
    )
    .await;
    assert!(
        pruned.deleted > 0,
        "prune should delete the exclusion, got {pruned:?}"
    );
}
