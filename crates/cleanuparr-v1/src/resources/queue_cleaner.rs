use crate::resources::failed_import_config::FailedImportConfig;
use core_macros::resource;

/// `/api/configuration/queue_cleaner` — periodic cleanup of stalled, slow, and
/// failed-import download-queue items.
#[resource(
    sync = singleton,
    read = get("/api/configuration/queue_cleaner"),
    update = put("/api/configuration/queue_cleaner"),
)]
pub struct QueueCleaner {
    /// Enables the queue cleaner job.
    pub enabled: Option<bool>,
    /// Cron expression controlling how often the queue cleaner job runs.
    #[default("0 0/5 * * * ?")]
    pub cron_expression: String,
    /// Uses `cron_expression` instead of the built-in interval scheduling.
    pub use_advanced_scheduling: Option<bool>,
    /// Failed-import striking settings.
    pub failed_import: Option<FailedImportConfig>,
    /// Number of strikes for a download stuck downloading metadata before it
    /// is removed.
    pub downloading_metadata_max_strikes: Option<i32>,
    /// Strikes/removes downloads that have no content id (e.g. private-tracker
    /// downloads without size/hash info) instead of leaving them queued.
    pub process_no_content_id: Option<bool>,
    /// Download names or category names excluded from queue cleaning.
    pub ignored_downloads: Vec<String>,
}
