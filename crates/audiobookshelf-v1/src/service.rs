//! The Audiobookshelf service. One `#[service]` struct carries the connection
//! fields and every managed resource.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::email_settings::EmailSettings;
use crate::resources::ereader_device::EreaderDevice;
use crate::resources::library::Library;
use crate::resources::notification::Notification;
use crate::resources::notification_settings::NotificationSettings;
use crate::resources::server_settings::ServerSettings;
use crate::resources::user::User;

/// Audiobookshelf — desired-state config for one instance.
///
/// The credential is a **bearer token**, not an API key: log in at
/// `POST /login` and read `user.accessToken`, or mint one under
/// `/api/api-keys`. It is named `token` rather than `api_key` because that is
/// what it is — and because doc-gen documents the connection from these very
/// field names.
#[service(
    name = "audiobookshelf_v1",
    health = "/api/libraries",
    auth = bearer,
)]
pub struct AudiobookshelfV1 {
    // --- connection ---
    pub url: String,
    #[credential(bearer)]
    pub token: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- collections ---
    /// Libraries. `GET /api/libraries` returns a `{libraries: […]}` envelope,
    /// and the folders PATCH is a destructive replace-by-id — see
    /// [`crate::resources::library`].
    pub libraries: Vec<Library>,
    /// Notification rules, identified by `(event_name, library_id)`.
    pub notifications: Vec<Notification>,
    /// Accounts. `GET /api/users` returns a `{users: […]}` envelope, and
    /// `password` never reads back — declaring one rewrites it on every apply
    /// until the key is removed.
    pub users: Vec<User>,
    /// Registered e-reader devices. The write replaces the whole array, so the
    /// config is authoritative and `--prune` has no effect.
    pub ereader_devices: Vec<EreaderDevice>,

    // --- singletons ---
    /// Apprise delivery settings.
    pub notification_settings: Option<NotificationSettings>,
    /// SMTP configuration for sending e-books to e-readers.
    pub email_settings: Option<EmailSettings>,
    /// Server-wide configuration. Write-only: ABS exposes no REST read for it,
    /// so every declared key is PATCHed and reported as an update.
    pub server_settings: Option<ServerSettings>,
}
