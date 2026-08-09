//! `dead_torrent_config` — a download client's dead-torrent handling:
//! torrents reporting zero seeders for `max_strikes` consecutive runs are
//! moved to `target_category`. Unsupported on rTorrent, which exposes no
//! seeder count.

use core_macros::nested;

/// A download client's `/api/dead-torrent-config/{downloadClientId}`.
///
/// Nested under [`super::DownloadClient::dead_torrent_config`]. `None` leaves
/// this endpoint unmanaged entirely (no `GET`/`PUT`); `Some` reconciles it
/// against the declared value — `PUT` upserts (the row is created on first
/// write), so a first-time declaration and a later edit look the same to
/// this hook.
#[nested]
pub struct DeadTorrentConfig {
    /// Enables dead-torrent handling for this client. Cannot be enabled for
    /// rTorrent.
    pub enabled: Option<bool>,
    /// Category (or, with `use_tag`, tag) applied to dead torrents. Must not
    /// also appear in `categories`.
    #[default("cleanuparr-dead")]
    pub target_category: String,
    /// Add a tag instead of changing the category.
    pub use_tag: Option<bool>,
    /// Consecutive runs reporting zero seeders before a torrent is moved.
    /// Minimum `3`.
    pub max_strikes: Option<i32>,
    /// Categories this rule watches. At least one is required when `enabled`.
    pub categories: Vec<String>,
}
