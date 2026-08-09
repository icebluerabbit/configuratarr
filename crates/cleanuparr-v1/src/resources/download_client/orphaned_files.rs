//! `orphaned_files_config` — a download client's orphaned-file scanning:
//! entries under `scan_directories` that no torrent claims are moved to
//! `orphaned_directory`.

use core_macros::nested;

/// A download client's `/api/orphaned-files-config/{downloadClientId}`.
///
/// Nested under [`super::DownloadClient::orphaned_files_config`]. `None`
/// leaves this endpoint unmanaged entirely (no `GET`/`PUT`); `Some`
/// reconciles it against the declared value — `PUT` upserts (the row is
/// created on first write), so a first-time declaration and a later edit
/// look the same to this hook.
#[nested]
pub struct OrphanedFilesConfig {
    /// Enables orphaned-file scanning for this client.
    pub enabled: Option<bool>,
    /// Directories scanned for orphaned entries. At least one is required
    /// when `enabled`.
    pub scan_directories: Vec<String>,
    /// Where orphaned entries are moved. Must not overlap any scan
    /// directory, any other client's scan or orphaned directory, or another
    /// client's download directory target.
    pub orphaned_directory: String,
    /// Glob patterns exempt from orphan detection.
    pub exclude_patterns: Vec<String>,
    /// Hours a file must remain untouched before it's eligible for orphan
    /// handling. `0` disables the age check.
    #[default(24)]
    pub min_file_age_hours: i32,
    /// Permanently delete moved entries after this many hours. `None` keeps
    /// them forever.
    pub purge_after_hours: Option<i32>,
}
