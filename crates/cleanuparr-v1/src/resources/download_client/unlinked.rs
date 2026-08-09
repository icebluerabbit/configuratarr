//! `unlinked_config` — a download client's unlinked-download handling:
//! torrents whose files aren't hardlinked anywhere are moved to
//! `target_category` (or tagged, on clients that support tagging instead of
//! re-categorising).

use core_macros::nested;

/// A download client's `/api/unlinked-config/{downloadClientId}`.
///
/// Nested under [`super::DownloadClient::unlinked_config`]. `None` leaves
/// this endpoint unmanaged entirely (no `GET`/`PUT`); `Some` reconciles it
/// against the declared value — `PUT` upserts (the row is created on first
/// write), so a first-time declaration and a later edit look the same to
/// this hook.
#[nested]
pub struct UnlinkedConfig {
    /// Enables unlinked-download handling for this client.
    pub enabled: Option<bool>,
    /// Category (or, with `use_tag`, tag) applied to unlinked downloads.
    /// Must not also appear in `categories`.
    #[default("cleanuparr-unlinked")]
    pub target_category: String,
    /// Add a tag instead of changing the category (qBittorrent and
    /// Transmission only).
    pub use_tag: Option<bool>,
    /// Root directories excluded from the unlinked-file scan. Each entry
    /// must exist on disk.
    pub ignored_root_dirs: Vec<String>,
    /// Categories this rule watches. At least one is required when `enabled`.
    pub categories: Vec<String>,
}
