//! The Cleanuparr v1 service. One `#[service]` struct carries the connection
//! fields plus every managed resource.
//!
//! Auth: Cleanuparr accepts the user's API key in the `X-Api-Key` header (an
//! `apikey` query parameter also works, but the header is the documented path).
//! The key is minted per user and readable from `GET /api/account/api-key`.
//!
//! Health is checked against `/api/configuration/general` rather than `/health`.
//! `/health` is unauthenticated and returns a bare `text/plain` word, so it
//! proves neither that credentials work nor that first-run setup finished — and
//! `/health/detailed` answers **503 whenever a download client is merely
//! degraded**, which would hang `--wait-for-healthy` on a healthy Cleanuparr.
//! `/api/configuration/general` is authenticated JSON and 403s until setup
//! completes, so waiting on it gates on up + set-up + credentials-valid.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::arr::lidarr::{LidarrConfig, LidarrInstance};
use crate::resources::arr::radarr::{RadarrConfig, RadarrInstance};
use crate::resources::arr::readarr::{ReadarrConfig, ReadarrInstance};
use crate::resources::arr::sonarr::{SonarrConfig, SonarrInstance};
use crate::resources::arr::whisparr::{WhisparrConfig, WhisparrInstance};
use crate::resources::blacklist_sync::BlacklistSync;
use crate::resources::download_cleaner::DownloadCleaner;
use crate::resources::download_client::DownloadClient;
use crate::resources::general::General;
use crate::resources::malware_blocker::MalwareBlocker;
use crate::resources::notifications::apprise::AppriseProvider;
use crate::resources::notifications::discord::DiscordProvider;
use crate::resources::notifications::gotify::GotifyProvider;
use crate::resources::notifications::notifiarr::NotifiarrProvider;
use crate::resources::notifications::ntfy::NtfyProvider;
use crate::resources::notifications::pushover::PushoverProvider;
use crate::resources::notifications::telegram::TelegramProvider;
use crate::resources::oidc::Oidc;
use crate::resources::queue_cleaner::QueueCleaner;
use crate::resources::seeker::Seeker;
use crate::resources::slow_rule::SlowRule;
use crate::resources::stall_rule::StallRule;

/// Cleanuparr v1 — desired-state config for one instance.
#[service(
    name = "cleanuparr_v1",
    health = "/api/configuration/general",
    auth = api_key(header = "X-Api-Key"),
)]
pub struct CleanuparrV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- singletons ---
    /// Global application behaviour, connectivity checks, logging and auth settings.
    pub general: Option<General>,
    /// OpenID Connect single sign-on configuration.
    pub oidc: Option<Oidc>,
    /// Periodic cleanup of stalled, slow and failed-import download-queue items.
    pub queue_cleaner: Option<QueueCleaner>,
    /// Blocks or strikes downloads whose content matches a per-*arr malware blocklist.
    pub malware_blocker: Option<MalwareBlocker>,
    /// Periodically publishes a blacklist file that seeds download-client blocklists.
    pub blacklist_sync: Option<BlacklistSync>,
    /// Periodic cleanup of downloads that finished seeding or became orphaned/unlinked.
    pub download_cleaner: Option<DownloadCleaner>,

    /// Sonarr-wide behaviour (failed-import strike threshold).
    pub sonarr_config: Option<SonarrConfig>,
    /// Radarr-wide behaviour (failed-import strike threshold).
    pub radarr_config: Option<RadarrConfig>,
    /// Lidarr-wide behaviour (failed-import strike threshold).
    pub lidarr_config: Option<LidarrConfig>,
    /// Readarr-wide behaviour (failed-import strike threshold).
    pub readarr_config: Option<ReadarrConfig>,
    /// Whisparr-wide behaviour (failed-import strike threshold).
    pub whisparr_config: Option<WhisparrConfig>,

    // --- collections ---
    /// Rules that strike and remove torrents whose download has stalled.
    pub stall_rules: Vec<StallRule>,
    /// Rules that strike and remove torrents whose download speed dropped too low.
    pub slow_rules: Vec<SlowRule>,

    /// Connected Sonarr instances. Addressable as `${ref.sonarr_instance.<name>}`.
    pub sonarr_instances: Vec<SonarrInstance>,
    /// Connected Radarr instances. Addressable as `${ref.radarr_instance.<name>}`.
    pub radarr_instances: Vec<RadarrInstance>,
    /// Connected Lidarr instances. Addressable as `${ref.lidarr_instance.<name>}`.
    pub lidarr_instances: Vec<LidarrInstance>,
    /// Connected Readarr instances. Addressable as `${ref.readarr_instance.<name>}`.
    pub readarr_instances: Vec<ReadarrInstance>,
    /// Connected Whisparr instances. Addressable as `${ref.whisparr_instance.<name>}`.
    pub whisparr_instances: Vec<WhisparrInstance>,

    // --- notification providers ---
    // Provider names are unique across *all* provider types, server-side, even
    // though each type reconciles independently here.
    /// Notifiarr notification providers.
    pub notifiarr_providers: Vec<NotifiarrProvider>,
    /// Apprise notification providers.
    pub apprise_providers: Vec<AppriseProvider>,
    /// ntfy.sh notification providers.
    pub ntfy_providers: Vec<NtfyProvider>,
    /// Pushover notification providers.
    pub pushover_providers: Vec<PushoverProvider>,
    /// Telegram notification providers.
    pub telegram_providers: Vec<TelegramProvider>,
    /// Discord webhook notification providers.
    pub discord_providers: Vec<DiscordProvider>,
    /// Gotify notification providers.
    pub gotify_providers: Vec<GotifyProvider>,

    /// Download clients, each owning its seeding rules and its unlinked /
    /// dead-torrent / orphaned-files handling. Addressable as
    /// `${ref.download_client.<name>}`.
    pub download_clients: Vec<DownloadClient>,

    /// Proactive search engine: global search behaviour plus per-*arr-instance settings.
    pub seeker: Option<Seeker>,
}
