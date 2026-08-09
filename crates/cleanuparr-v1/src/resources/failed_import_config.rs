use core_macros::{nested, wire_enum};

/// Whether `patterns` excludes matching files from striking, or requires a
/// match before striking.
#[wire_enum]
pub enum PatternMode {
    /// Files whose names match a pattern are excluded from being counted as
    /// failed imports.
    Exclude,
    /// Only files whose names match a pattern are counted as failed imports.
    Include,
    /// Unknown or future pattern mode not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// Failed-import striking. `max_strikes` of `0` disables it; any other value
/// must be at least 3.
///
/// Embedded in [`crate::resources::queue_cleaner::QueueCleaner`] under the
/// `failedImport` key.
#[nested]
pub struct FailedImportConfig {
    /// Number of failed-import strikes before the download is removed. `0`
    /// disables striking; any other value must be at least 3.
    pub max_strikes: i32,
    /// Ignores private-tracker downloads when striking for failed imports.
    pub ignore_private: Option<bool>,
    /// Deletes private-tracker downloads instead of striking them. Mutually
    /// exclusive with `change_category`.
    pub delete_private: Option<bool>,
    /// Skips striking a download that is no longer present in the download
    /// client's queue.
    #[default(true)]
    pub skip_if_not_found_in_client: bool,
    /// File-name patterns used to identify failed imports. At least one
    /// pattern is required when striking is on and `pattern_mode` is
    /// `Include`.
    pub patterns: Vec<String>,
    /// Whether `patterns` excludes or requires matches.
    pub pattern_mode: Option<PatternMode>,
    /// Moves the download to a different category instead of striking it.
    /// Mutually exclusive with `delete_private`.
    pub change_category: Option<bool>,
}
