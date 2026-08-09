use core_macros::resource;

/// `/api/v1/config/development` — developer and debugging configuration.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/development"),
    update = put("/api/v1/config/development/${self.id}"),
)]
pub struct DevelopmentConfig {
    #[id]
    pub id: Option<i32>,
    /// Log verbosity level for console output (e.g. `info`, `debug`, `trace`).
    pub console_log_level: Option<String>,
    /// Logs SQL queries executed against the database when enabled.
    pub log_sql: bool,
    /// Logs raw HTTP responses from indexers for debugging.
    pub log_indexer_response: bool,
    /// Number of log files to retain before rotation discards the oldest.
    pub log_rotate: i32,
    /// Filters events sent to Sentry error tracking; reduces noise in reports.
    pub filter_sentry_events: bool,
}
