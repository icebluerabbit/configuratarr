//! End-to-end tests against a live Bazarr.
//!
//! Drive the real apply engine (connect → GET live settings → diff → form POST),
//! exactly as the CLI does. Guarded by `BAZARR_URL` + `BAZARR_API_KEY`;
//! `#[ignore]` by default.
//!
//! Run inside the e2e dev shell (starts Bazarr, exports the env vars):
//!   nix develop .#e2e-bazarr --command \
//!     cargo nextest run -p bazarr-v1 --test e2e --run-ignored all -j1
//!
//! This is where the two bazarr runtime assumptions are validated:
//!   1. auth — the API key is accepted in the `X-API-KEY` header;
//!   2. write path — the settings blob round-trips through the form POST and the
//!      diff makes a repeated apply idempotent.

use std::time::Duration;

use bazarr_v1::BazarrV1;
use core_lib::apply::{ApplyOptions, Report, apply, wait_healthy};
use core_testkit::{env_pair, instance};
use serde_json::{Value, json};

fn env() -> Option<(String, String)> {
    env_pair("BAZARR_URL", "BAZARR_API_KEY")
}

async fn run(url: &str, key: &str, resources: Value, opts: ApplyOptions) -> Report {
    let (svc, value) = instance::<BazarrV1>(url, key, resources);
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("bazarr healthy");
    apply(&svc, &value, opts).await.expect("apply succeeds")
}

/// `wait_healthy` against the live API: `/api/system/status` answers OK.
#[tokio::test]
#[ignore]
async fn waits_for_healthy() {
    let Some((url, key)) = env() else { return };
    let (svc, _) = instance::<BazarrV1>(&url, &key, json!({}));
    wait_healthy(&svc, Duration::from_secs(60))
        .await
        .expect("bazarr should report healthy");
}

/// No managed resources: connect + auth reach the live API, nothing changes.
#[tokio::test]
#[ignore]
async fn connects_with_no_resources() {
    let Some((url, key)) = env() else { return };
    let report = run(&url, &key, json!({}), ApplyOptions::default()).await;
    assert_eq!(report, Report::default());
}

/// An empty settings config declares no keys → no diff, no write.
#[tokio::test]
#[ignore]
async fn empty_settings_noop() {
    let Some((url, key)) = env() else { return };
    let report = run(
        &url,
        &key,
        json!({ "settings": {} }),
        ApplyOptions::default(),
    )
    .await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Report::default()
        }
    );
}

/// Applying a settings value and then applying it again is idempotent: the
/// second run declares the same keys, finds them already set, and writes nothing.
#[tokio::test]
#[ignore]
async fn settings_apply_is_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "settings": {
            "general": {
                "single_language": false,
                "minimum_score": 90,
                "use_scenename": true,
            }
        }
    });

    // First apply converges the declared keys (changed or already-unchanged).
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // Second apply must be a pure no-op.
    let report = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Report::default()
        },
        "re-applying identical settings must be idempotent"
    );
}

/// Enabling languages and creating a language profile round-trips: after the
/// first apply, re-applying the same profile + enabled set writes nothing.
#[tokio::test]
#[ignore]
async fn language_profiles_apply_is_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "languages": {
            "enabled_languages": ["en"],
            "language_profiles": [{
                "profile_id": 1,
                "name": "English",
                "cutoff": null,
                "items": [{ "id": 1, "language": "en" }],
                "must_contain": [],
                "must_not_contain": [],
                "original_format": false,
                "tag": null,
            }],
        }
    });

    // First apply creates the profile + enables the language.
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // Second apply must be a pure no-op (diff sees the profile already stored).
    let report = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Report::default()
        },
        "re-applying an identical language profile must be idempotent"
    );
}

/// Auth is idempotent despite `password` being md5-hashed server-side: the hook
/// hashes the declared plaintext to compare, so re-applying the same password
/// writes nothing.
#[tokio::test]
#[ignore]
async fn auth_password_apply_is_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "settings": {
            "auth": {
                "kind": "form",
                "username": "admin",
                "password": "configuratarre2e",
            }
        }
    });

    // First apply hashes + stores the password.
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // Second apply must be a no-op — the declared plaintext hashes to the stored value.
    let report = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Report::default()
        },
        "re-applying an identical auth password must be idempotent"
    );
}

/// The newly-modelled sections round-trip: the `general` default-profile toggles,
/// and — the novel wire shape — `subsync.checker`, a nested object *inside* a
/// section (flattened to `settings-subsync-checker-*` form keys). Applying then
/// re-applying is a pure no-op, proving both encode/read shapes match bazarr's
/// contract.
#[tokio::test]
#[ignore]
async fn extended_sections_apply_is_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "settings": {
            "general": {
                "serie_default_enabled": false,
                "movie_default_enabled": false,
            },
            "subsync": {
                "checker": {
                    "blacklisted_languages": [],
                    "blacklisted_providers": ["podnapisi"],
                }
            },
        }
    });

    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    let report = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Report::default()
        },
        "re-applying the extended sections must be idempotent"
    );
}

/// Notification providers round-trip idempotently — the `notifications-providers`
/// side-channel field (one JSON string per provider, sparse-update-by-`name`)
/// writes on the first apply and is a pure no-op on the second.
#[tokio::test]
#[ignore]
async fn notifications_apply_is_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "notifications": {
            "providers": [{
                "name": "Discord",
                "enabled": true,
                "url": "discord://configuratarr_e2e/token",
            }],
        }
    });

    // First apply enables + sets the provider's URL.
    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    // Second apply must be a pure no-op (diff sees the provider already stored).
    let report = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Report::default()
        },
        "re-applying an identical notification provider must be idempotent"
    );
}

/// Subtitle-provider credentials round-trip idempotently — they're stored
/// plaintext (unlike `auth.password`), so the generic section diff suffices.
#[tokio::test]
#[ignore]
async fn provider_credentials_apply_is_idempotent() {
    let Some((url, key)) = env() else { return };
    let cfg = json!({
        "settings": {
            "general": { "enabled_providers": ["opensubtitlescom"] },
            "opensubtitlescom": {
                "username": "cfg-e2e",
                "password": "cfg-e2e-pw",
                "use_hash": true,
            }
        }
    });

    run(&url, &key, cfg.clone(), ApplyOptions::default()).await;

    let report = run(&url, &key, cfg, ApplyOptions::default()).await;
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Report::default()
        },
        "re-applying identical provider credentials must be idempotent"
    );
}
