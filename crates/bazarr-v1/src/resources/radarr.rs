use core_macros::nested;

/// Radarr connection settings (`settings-radarr-*`).
#[nested(case = snake)]
pub struct Radarr {
    /// Radarr host/IP.
    pub ip: Option<String>,
    /// Radarr port.
    pub port: Option<i32>,
    /// Radarr base URL (reverse-proxy subpath).
    pub base_url: Option<String>,
    /// Use HTTPS to reach Radarr.
    pub ssl: Option<bool>,
    /// HTTP request timeout, in seconds.
    pub http_timeout: Option<i32>,
    /// Radarr API key.
    pub apikey: Option<String>,
    /// Full-sync cadence (`Manually` / `Daily` / `Weekly`).
    pub full_update: Option<String>,
    /// Day of week for a weekly full sync (0–6).
    pub full_update_day: Option<i32>,
    /// Hour of day for the full sync (0–23).
    pub full_update_hour: Option<i32>,
    /// Only manage monitored movies.
    pub only_monitored: Option<bool>,
    /// Keep the movie list in sync via SignalR live updates.
    pub movies_sync_on_live: Option<bool>,
    /// Movie-sync frequency, in minutes.
    pub movies_sync: Option<i32>,
    /// Radarr tags to exclude.
    pub excluded_tags: Vec<String>,
    /// Cache ffprobe results.
    pub use_ffprobe_cache: Option<bool>,
    /// Defer searches triggered by SignalR events.
    pub defer_search_signalr: Option<bool>,
    /// Only sync monitored movies.
    pub sync_only_monitored_movies: Option<bool>,
}
