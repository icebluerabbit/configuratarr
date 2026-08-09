//! A single notification provider (Apprise-backed).
//!
//! Bazarr ships a fixed catalogue of ~100 notifier providers (Discord, Telegram,
//! Pushover, …), each identified by its display `name`. A provider is configured
//! by giving it an Apprise `url` and toggling `enabled`; there is no create or
//! delete — you only ever *update* a provider the catalogue already contains.
//!
//! These are written through the **same** `/api/system/settings` POST as the rest
//! of the config, but as a top-level `notifications-providers` form field (one
//! repeated field per provider, each a **JSON string** — **not** a `settings-*`
//! key) with **sparse-update-by-`name`** semantics: bazarr updates only the
//! providers you submit and leaves every other provider untouched. Read back from
//! the `notifications.providers` list on `/api/system/settings`.
//!
//! The reconcile hook ([`crate::resources::notifications`]) translates these
//! config structs into bazarr's exact stored JSON shape before diffing/POSTing.

use core_macros::nested;

/// One notification provider. `name` is the natural key (a value from bazarr's
/// fixed provider catalogue, e.g. `"Discord"`); `url` is its Apprise URL.
#[nested]
pub struct Notifier {
    /// Provider name — must match one of bazarr's built-in providers exactly
    /// (e.g. `"Discord"`, `"Telegram"`, `"Pushover"`).
    pub name: String,
    /// Whether this provider is active. Omitted = disabled.
    #[default(false)]
    pub enabled: bool,
    /// Apprise notification URL (e.g. `discord://webhook_id/webhook_token`).
    /// Omitted = cleared (encoded as an explicit `null`, matching bazarr's store).
    #[wire(null)]
    pub url: Option<String>,
}
