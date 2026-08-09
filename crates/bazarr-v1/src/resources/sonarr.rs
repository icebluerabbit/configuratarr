use core_macros::nested;

/// Sonarr connection settings (`settings-sonarr-*`).
#[nested(case = snake)]
pub struct Sonarr {
    /// Sonarr host/IP.
    pub ip: Option<String>,
    /// Sonarr port.
    pub port: Option<i32>,
    /// Sonarr base URL (reverse-proxy subpath).
    pub base_url: Option<String>,
    /// Use HTTPS to reach Sonarr.
    pub ssl: Option<bool>,
    /// HTTP request timeout, in seconds.
    pub http_timeout: Option<i32>,
    /// Sonarr API key.
    pub apikey: Option<String>,
    /// Full-sync cadence (`Manually` / `Daily` / `Weekly`).
    pub full_update: Option<String>,
    /// Day of week for a weekly full sync (0–6).
    pub full_update_day: Option<i32>,
    /// Hour of day for the full sync (0–23).
    pub full_update_hour: Option<i32>,
    /// Only manage monitored series.
    pub only_monitored: Option<bool>,
    /// Keep the series list in sync via SignalR live updates.
    pub series_sync_on_live: Option<bool>,
    /// Series-sync frequency, in minutes.
    pub series_sync: Option<i32>,
    /// Sonarr tags to exclude.
    pub excluded_tags: Vec<String>,
    /// Sonarr series types to exclude.
    pub excluded_series_types: Vec<String>,
    /// Cache ffprobe results.
    pub use_ffprobe_cache: Option<bool>,
    /// Exclude season 0 (specials).
    pub exclude_season_zero: Option<bool>,
    /// Defer searches triggered by SignalR events.
    pub defer_search_signalr: Option<bool>,
    /// Only sync monitored series.
    pub sync_only_monitored_series: Option<bool>,
    /// Only sync monitored episodes.
    pub sync_only_monitored_episodes: Option<bool>,
}
