use core_macros::resource;

/// `/api/configuration/download_cleaner` — periodic cleanup of downloads that
/// have finished seeding or been orphaned/unlinked from their *arr instance.
#[resource(
    sync = singleton,
    read = get("/api/configuration/download_cleaner"),
    update = put("/api/configuration/download_cleaner"),
)]
pub struct DownloadCleaner {
    /// Enables the download cleaner job.
    pub enabled: Option<bool>,
    /// Cron expression controlling how often the download cleaner job runs.
    #[default("0 0 * * * ?")]
    pub cron_expression: String,
    /// Uses `cron_expression` instead of the built-in interval scheduling.
    pub use_advanced_scheduling: Option<bool>,
    /// Download names or category names excluded from download cleaning.
    pub ignored_downloads: Vec<String>,
}
