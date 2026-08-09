use core_macros::nested;

/// Throughput/queue rules autobrr enforces before pushing to this client.
#[nested(case = snake)]
pub struct DownloadClientRules {
    /// Whether the rules are enforced.
    pub enabled: Option<bool>,
    /// Cap on simultaneously active downloads (0 = unlimited).
    pub max_active_downloads: Option<i32>,
    /// Skip pushing when existing torrents are slow.
    pub ignore_slow_torrents: Option<bool>,
    /// When the slow-torrent check applies (`ALWAYS` / `MAX_DOWNLOADS_REACHED`).
    pub ignore_slow_torrents_condition: Option<String>,
    /// Download-speed threshold (KB/s) for the slow check.
    pub download_speed_threshold: Option<i32>,
    /// Upload-speed threshold (KB/s) for the slow check.
    pub upload_speed_threshold: Option<i32>,
}
