//! The Komga v1 service. One `#[service]` struct carries the connection fields
//! and every managed resource; `Vec<R>` fields are collections, `Option<R>`
//! fields are singletons. The sync strategy comes from each resource's own
//! descriptor, not from the field shape.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::api_key::ApiKey;
use crate::resources::client_setting_global::ClientSettingGlobal;
use crate::resources::client_setting_user::ClientSettingUser;
use crate::resources::library::Library;
use crate::resources::settings::Settings;
use crate::resources::user::User;

/// Komga v1 — desired-state config for one instance.
///
/// Auth is the `X-API-Key` header. Komga also accepts HTTP Basic, but a key is
/// the credential to configure here: keys are minted per user at
/// `POST /api/v2/users/me/api-keys` (the only place their secret is shown).
///
/// The health check is `/api/v2/users/me` — authenticated, always present, and
/// cheap, so a green response proves both reachability and credentials.
#[service(
    name = "komga_v1",
    health = "/api/v2/users/me",
    auth = api_key(header = "X-API-Key"),
)]
pub struct KomgaV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- collections ---
    /// Libraries — scanned root directories and their import/scan behaviour.
    pub libraries: Vec<Library>,
    /// User accounts, keyed by email. Passwords apply on create only.
    pub users: Vec<User>,
    /// API keys for the authenticated user. Create + prune only; the secret is
    /// shown once on create and never echoed into plan output. **Under
    /// `--prune` this revokes any key you don't declare — including the one
    /// `api_key` above connects with.**
    pub api_keys: Vec<ApiKey>,
    /// Server-wide client settings, keyed by dotted setting name.
    pub client_settings_global: Vec<ClientSettingGlobal>,
    /// Per-user client settings, keyed by dotted setting name.
    pub client_settings_user: Vec<ClientSettingUser>,

    // --- singletons ---
    /// Global server settings (`/api/v1/settings`).
    pub settings: Option<Settings>,
}
