//! NotificationProvider — Prowlarr provider enum. Each variant binds an `implementation`
//! string from the wire to a typed fields-blob variant. `#[fallback]` catches any
//! implementation we don't model and preserves it via `RawProvider`.

pub mod apprise;
pub mod custom_script;
pub mod discord;
pub mod email;
pub mod gotify;
pub mod join;
pub mod mailgun;
pub mod notifiarr;
pub mod ntfy;
pub mod prowl;
pub mod pushbullet;
pub mod pushover;
pub mod sendgrid;
pub mod signal;
pub mod simplepush;
pub mod slack;
pub mod telegram;
pub mod twitter;
pub mod webhook;

pub use apprise::AppriseConfig;
pub use custom_script::CustomScriptConfig;
pub use discord::DiscordConfig;
pub use email::EmailConfig;
pub use gotify::GotifyConfig;
pub use join::JoinConfig;
pub use mailgun::MailgunConfig;
pub use notifiarr::NotifiarrConfig;
pub use ntfy::NtfyConfig;
pub use prowl::ProwlConfig;
pub use pushbullet::PushbulletConfig;
pub use pushover::PushoverConfig;
pub use sendgrid::SendgridConfig;
pub use signal::SignalConfig;
pub use simplepush::SimplepushConfig;
pub use slack::SlackConfig;
pub use telegram::TelegramConfig;
pub use twitter::TwitterConfig;
pub use webhook::WebhookConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all supported Prowlarr notification provider implementations.
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
    #[variant("Gotify")]
    Gotify(GotifyConfig),
    #[variant("Join")]
    Join(JoinConfig),
    #[variant("Mailgun")]
    Mailgun(MailgunConfig),
    #[variant("Notifiarr")]
    Notifiarr(NotifiarrConfig),
    #[variant("Ntfy")]
    Ntfy(NtfyConfig),
    #[variant("Prowl")]
    Prowl(ProwlConfig),
    #[variant("PushBullet")]
    Pushbullet(PushbulletConfig),
    #[variant("Pushover")]
    Pushover(PushoverConfig),
    #[variant("Sendgrid")]
    Sendgrid(SendgridConfig),
    #[variant("Signal")]
    Signal(SignalConfig),
    #[variant("Simplepush")]
    Simplepush(SimplepushConfig),
    #[variant("Slack")]
    Slack(SlackConfig),
    #[variant("Telegram")]
    Telegram(TelegramConfig),
    #[variant("Twitter")]
    Twitter(TwitterConfig),
    #[variant("Webhook")]
    Webhook(WebhookConfig),
    #[fallback]
    Unknown(RawProvider),
}
