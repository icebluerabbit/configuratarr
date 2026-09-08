//! Tagged-enum for notification providers (`#[tagged]`). The discriminator is
//! the `implementation` string in the wire object; each `#[variant("...")]`
//! binds that value to a typed fields-blob variant. `#[fallback]` catches any
//! implementation we don't model and preserves it via `RawProvider`.
//!
//! Variant set + fields sourced from the eros C# tree
//! (`NzbDrone.Core/Notifications/`), not radarr and not the (V2-targeted,
//! stale) `terraform-provider-whisparr`. Four are eros-only — `Pushcut`,
//! `Pushsafer`, `Signal`, `Stash` — and two differ from radarr in the
//! discriminator's *casing* only: eros' provider class is `MailGun` (settings
//! contract stays `MailgunSettings`), and `SendGrid`/`SendGridSettings` replace
//! radarr's `Sendgrid`/`SendgridSettings`.

pub mod apprise;
pub mod custom_script;
pub mod discord;
pub mod email;
pub mod emby;
pub mod gotify;
pub mod join;
pub mod kodi;
pub mod mailgun;
pub mod notifiarr;
pub mod ntfy;
pub mod plex;
pub mod prowl;
pub mod pushbullet;
pub mod pushcut;
pub mod pushover;
pub mod pushsafer;
pub mod sendgrid;
pub mod signal;
pub mod simplepush;
pub mod slack;
pub mod stash;
pub mod synology_indexer;
pub mod telegram;
pub mod trakt;
pub mod twitter;
pub mod webhook;

pub use apprise::AppriseConfig;
pub use custom_script::CustomScriptConfig;
pub use discord::DiscordConfig;
pub use email::EmailConfig;
pub use emby::EmbyConfig;
pub use gotify::GotifyConfig;
pub use join::JoinConfig;
pub use kodi::KodiConfig;
pub use mailgun::MailgunConfig;
pub use notifiarr::NotifiarrConfig;
pub use ntfy::NtfyConfig;
pub use plex::PlexConfig;
pub use prowl::ProwlConfig;
pub use pushbullet::PushbulletConfig;
pub use pushcut::PushcutConfig;
pub use pushover::PushoverConfig;
pub use pushsafer::PushsaferConfig;
pub use sendgrid::SendgridConfig;
pub use signal::SignalConfig;
pub use simplepush::SimplepushConfig;
pub use slack::SlackConfig;
pub use stash::StashConfig;
pub use synology_indexer::SynologyIndexerConfig;
pub use telegram::TelegramConfig;
pub use trakt::TraktConfig;
pub use twitter::TwitterConfig;
pub use webhook::WebhookConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all modelled Whisparr notification implementations.
/// The `implementation` field on the wire selects the variant; `#[fallback]`
/// preserves any implementation not listed here as a passthrough blob.
#[tagged(by = "implementation")]
pub enum NotificationProvider {
    #[variant("Apprise")]
    Apprise(AppriseConfig),
    #[variant("CustomScript")]
    CustomScript(CustomScriptConfig),
    #[variant("Discord")]
    Discord(DiscordConfig),
    #[variant("Email")]
    Email(EmailConfig),
    #[variant("MediaBrowser")]
    Emby(EmbyConfig),
    #[variant("Gotify")]
    Gotify(GotifyConfig),
    #[variant("Join")]
    Join(JoinConfig),
    #[variant("Xbmc")]
    Kodi(KodiConfig),
    #[variant("MailGun")]
    Mailgun(MailgunConfig),
    #[variant("Notifiarr")]
    Notifiarr(NotifiarrConfig),
    #[variant("Ntfy")]
    Ntfy(NtfyConfig),
    #[variant("PlexServer")]
    Plex(PlexConfig),
    #[variant("Prowl")]
    Prowl(ProwlConfig),
    #[variant("PushBullet")]
    Pushbullet(PushbulletConfig),
    #[variant("Pushcut")]
    Pushcut(PushcutConfig),
    #[variant("Pushover")]
    Pushover(PushoverConfig),
    #[variant("Pushsafer")]
    Pushsafer(PushsaferConfig),
    #[variant("SendGrid")]
    Sendgrid(SendgridConfig),
    #[variant("Signal")]
    Signal(SignalConfig),
    #[variant("Simplepush")]
    Simplepush(SimplepushConfig),
    #[variant("Slack")]
    Slack(SlackConfig),
    #[variant("Stash")]
    Stash(StashConfig),
    #[variant("SynologyIndexer")]
    SynologyIndexer(SynologyIndexerConfig),
    #[variant("Telegram")]
    Telegram(TelegramConfig),
    #[variant("Trakt")]
    Trakt(TraktConfig),
    #[variant("Twitter")]
    Twitter(TwitterConfig),
    #[variant("Webhook")]
    Webhook(WebhookConfig),
    #[fallback]
    Unknown(RawProvider),
}
