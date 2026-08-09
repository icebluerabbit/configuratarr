//! Shape conformance: our encoded wire payloads must validate against the
//! Cleanuparr OpenAPI request schemas.
//!
//! Cleanuparr publishes no OpenAPI document; `spec/cleanuparr-v1.json` is
//! reconstructed from the upstream source (v2.10.3, commit `9edc685`) and its
//! request schemas were made `additionalProperties: false` precisely so this
//! suite has teeth — a hallucinated, mis-cased, or mis-typed field fails with a
//! field-level error.
//!
//! Cleanuparr serialises **camelCase** JSON (ASP.NET Core web defaults), which
//! is the engine's default casing, so no resource sets `case = ...`.
//!
//! `${ref}` / `${env}` in fixtures are dummy-resolved (real resolution needs a
//! live server); we only care about the resulting shape, not the values.

use core_testkit::{GuidRefs, check, check_with_refs};
use serde_json::Value;

#[allow(unused_imports)]
use cleanuparr_v1::resources;

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/cleanuparr-v1.json")).expect("spec parses")
}

/// One conformance test per resource: `check` its `config.yaml` fixture against
/// the named OpenAPI schema. The harness lives in [`core_testkit::check`].
#[allow(unused_macros)]
macro_rules! conformance {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check::<$ty>(&spec(), $schema, include_str!($fixture));
        }
    };
}

/// Same, for a resource whose fixture carries a `${ref}` to a GUID-id resource.
/// Cleanuparr's ids are GUIDs, so those references must resolve to a *string*;
/// the default [`core_testkit::DummyRefs`] yields the integer `1` (correct for
/// the *arr crates) and would fail decode with `expected string, got 1`.
#[allow(unused_macros)]
macro_rules! conformance_guid_refs {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check_with_refs::<$ty>(&spec(), $schema, include_str!($fixture), &GuidRefs);
        }
    };
}

/// The spec is hand-reconstructed, so guard the property this whole suite leans
/// on: every schema we validate against must reject unknown members. Without it
/// conformance would pass a hallucinated field silently.
#[test]
fn request_schemas_are_strict() {
    let spec = spec();
    let schemas = &spec["components"]["schemas"];
    let targets = [
        "UpdateGeneralConfigRequest",
        "LoggingConfig",
        "AuthConfig",
        "UpdateQueueCleanerConfigRequest",
        "FailedImportConfig",
        "UpdateMalwareBlockerConfigRequest",
        "BlocklistSettings",
        "UpdateBlacklistSyncConfigRequest",
        "UpdateDownloadCleanerConfigRequest",
        "UpdateOidcConfigRequest",
        "UpdateSeekerConfigRequest",
        "UpdateSeekerInstanceConfigRequest",
        "UpdateArrConfigRequest",
        "ArrInstanceRequest",
        "CreateDownloadClientRequest",
        "SeedingRuleRequest",
        "UnlinkedConfigRequest",
        "DeadTorrentConfigRequest",
        "OrphanedFilesConfigRequest",
        "StallRuleDto",
        "SlowRuleDto",
    ];
    for name in targets {
        let schema = &schemas[name];
        assert!(!schema.is_null(), "schema `{name}` missing from spec");
        assert_eq!(
            schema["additionalProperties"],
            Value::Bool(false),
            "schema `{name}` must be additionalProperties:false or conformance has no teeth",
        );
    }
}

// ── singletons ───────────────────────────────────────────────────────────────
conformance!(
    general,
    resources::general::General,
    "UpdateGeneralConfigRequest",
    "testdata/general/config.yaml"
);
conformance!(
    oidc,
    resources::oidc::Oidc,
    "UpdateOidcConfigRequest",
    "testdata/oidc/config.yaml"
);
conformance!(
    queue_cleaner,
    resources::queue_cleaner::QueueCleaner,
    "UpdateQueueCleanerConfigRequest",
    "testdata/queue_cleaner/config.yaml"
);
conformance!(
    malware_blocker,
    resources::malware_blocker::MalwareBlocker,
    "UpdateMalwareBlockerConfigRequest",
    "testdata/malware_blocker/config.yaml"
);
conformance!(
    blacklist_sync,
    resources::blacklist_sync::BlacklistSync,
    "UpdateBlacklistSyncConfigRequest",
    "testdata/blacklist_sync/config.yaml"
);
conformance!(
    download_cleaner,
    resources::download_cleaner::DownloadCleaner,
    "UpdateDownloadCleanerConfigRequest",
    "testdata/download_cleaner/config.yaml"
);

// ── *arr app configs ─────────────────────────────────────────────────────────
// One per app: the five structs are hand-written per file, so each gets its own
// check rather than trusting sonarr's to stand in for the rest.
conformance!(
    sonarr_config,
    resources::arr::sonarr::SonarrConfig,
    "UpdateArrConfigRequest",
    "testdata/sonarr_config/config.yaml"
);
conformance!(
    radarr_config,
    resources::arr::radarr::RadarrConfig,
    "UpdateArrConfigRequest",
    "testdata/radarr_config/config.yaml"
);
conformance!(
    lidarr_config,
    resources::arr::lidarr::LidarrConfig,
    "UpdateArrConfigRequest",
    "testdata/lidarr_config/config.yaml"
);
conformance!(
    readarr_config,
    resources::arr::readarr::ReadarrConfig,
    "UpdateArrConfigRequest",
    "testdata/readarr_config/config.yaml"
);
conformance!(
    whisparr_config,
    resources::arr::whisparr::WhisparrConfig,
    "UpdateArrConfigRequest",
    "testdata/whisparr_config/config.yaml"
);

// ── collections ──────────────────────────────────────────────────────────────
conformance!(
    sonarr_instance,
    resources::arr::sonarr::SonarrInstance,
    "ArrInstanceRequest",
    "testdata/sonarr_instance/config.yaml"
);
conformance!(
    radarr_instance,
    resources::arr::radarr::RadarrInstance,
    "ArrInstanceRequest",
    "testdata/radarr_instance/config.yaml"
);
conformance!(
    lidarr_instance,
    resources::arr::lidarr::LidarrInstance,
    "ArrInstanceRequest",
    "testdata/lidarr_instance/config.yaml"
);
conformance!(
    readarr_instance,
    resources::arr::readarr::ReadarrInstance,
    "ArrInstanceRequest",
    "testdata/readarr_instance/config.yaml"
);
conformance!(
    whisparr_instance,
    resources::arr::whisparr::WhisparrInstance,
    "ArrInstanceRequest",
    "testdata/whisparr_instance/config.yaml"
);

// ── notification providers ───────────────────────────────────────────────────
// The write body is flat; the read shape nests `events` + `configuration`. These
// validate the flat *write* shape, which is what the hooks send.
conformance!(
    notifiarr_provider,
    resources::notifications::notifiarr::NotifiarrProvider,
    "CreateNotifiarrProviderRequest",
    "testdata/notifiarr_provider/config.yaml"
);
conformance!(
    apprise_provider,
    resources::notifications::apprise::AppriseProvider,
    "CreateAppriseProviderRequest",
    "testdata/apprise_provider/config.yaml"
);
conformance!(
    ntfy_provider,
    resources::notifications::ntfy::NtfyProvider,
    "CreateNtfyProviderRequest",
    "testdata/ntfy_provider/config.yaml"
);
conformance!(
    pushover_provider,
    resources::notifications::pushover::PushoverProvider,
    "CreatePushoverProviderRequest",
    "testdata/pushover_provider/config.yaml"
);
conformance!(
    telegram_provider,
    resources::notifications::telegram::TelegramProvider,
    "CreateTelegramProviderRequest",
    "testdata/telegram_provider/config.yaml"
);
conformance!(
    discord_provider,
    resources::notifications::discord::DiscordProvider,
    "CreateDiscordProviderRequest",
    "testdata/discord_provider/config.yaml"
);
conformance!(
    gotify_provider,
    resources::notifications::gotify::GotifyProvider,
    "CreateGotifyProviderRequest",
    "testdata/gotify_provider/config.yaml"
);
conformance!(
    stall_rule,
    resources::stall_rule::StallRule,
    "StallRuleDto",
    "testdata/stall_rule/config.yaml"
);
conformance!(
    slow_rule,
    resources::slow_rule::SlowRule,
    "SlowRuleDto",
    "testdata/slow_rule/config.yaml"
);

conformance!(
    download_client,
    resources::download_client::DownloadClient,
    "CreateDownloadClientRequest",
    "testdata/download_client/config.yaml"
);
// The four per-download-client sub-concerns. They are `#[wire(config_only)]` on
// `DownloadClient` — deliberately absent from *its* wire body — so the
// `download_client` test above cannot cover them, yet each is a body the hook
// really does PUT/POST to its own endpoint.
conformance!(
    seeding_rule,
    resources::download_client::SeedingRule,
    "SeedingRuleRequest",
    "testdata/seeding_rule/config.yaml"
);
conformance!(
    unlinked_config,
    resources::download_client::unlinked::UnlinkedConfig,
    "UnlinkedConfigRequest",
    "testdata/unlinked_config/config.yaml"
);
conformance!(
    dead_torrent_config,
    resources::download_client::dead_torrent::DeadTorrentConfig,
    "DeadTorrentConfigRequest",
    "testdata/dead_torrent_config/config.yaml"
);
conformance!(
    orphaned_files_config,
    resources::download_client::orphaned_files::OrphanedFilesConfig,
    "OrphanedFilesConfigRequest",
    "testdata/orphaned_files_config/config.yaml"
);

conformance_guid_refs!(
    seeker,
    resources::seeker::Seeker,
    "UpdateSeekerConfigRequest",
    "testdata/seeker/config.yaml"
);
