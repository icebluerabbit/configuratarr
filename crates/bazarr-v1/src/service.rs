//! The Bazarr v1 service. One `#[service]` struct carries the connection fields
//! plus the managed config.
//!
//! Bazarr is **not** an *arr: it has no per-resource CRUD config surface. Its
//! entire configuration lives behind a single endpoint, `/api/system/settings`,
//! whose write contract is a flat `application/x-www-form-urlencoded` POST (the
//! GET is nested JSON). None of the crud/singleton codecs fit, so the whole
//! config is one `sync = custom` singleton — see [`crate::resources::settings`].
//!
//! Auth: `X-API-KEY` header. Health is checked against the authenticated
//! `/api/system/status`.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::languages::Languages;
use crate::resources::notifications::Notifications;
use crate::resources::settings::Settings;

/// Bazarr v1 — desired-state config for one instance.
#[service(
    name = "bazarr_v1",
    health = "/api/system/status",
    auth = api_key(header = "X-API-KEY"),
)]
pub struct BazarrV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- config (settings blob) + the language / notification side-channels ---
    pub settings: Option<Settings>,
    pub languages: Option<Languages>,
    pub notifications: Option<Notifications>,
}
