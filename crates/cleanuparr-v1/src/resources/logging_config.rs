use core_macros::{nested, wire_enum};

/// Serilog minimum log event level.
///
/// Controls the verbosity of Cleanuparr's application logs, from the most
/// detailed (`Verbose`) to only critical failures (`Fatal`).
#[wire_enum]
pub enum LogEventLevel {
    /// Highly detailed tracing, typically only useful for diagnosing a
    /// specific problem.
    Verbose,
    /// Detailed information useful during development and debugging.
    Debug,
    /// Routine informational messages tracking application flow.
    Information,
    /// An unexpected or unusual event that is not necessarily an error.
    Warning,
    /// A failure within the application or an external dependency.
    Error,
    /// A critical error that causes the application to stop working.
    Fatal,
    /// Unknown or future log level not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// File logging and archiving settings.
///
/// A `0` means "disabled" for `rolling_size_mb` and "unlimited" for the
/// retention fields. Embedded in [`crate::resources::general::General`]
/// under the `log` key.
#[nested]
pub struct LoggingConfig {
    /// Minimum Serilog level that will be written to the log.
    pub level: Option<LogEventLevel>,
    /// Maximum size, in megabytes, a single log file may reach before it
    /// rolls over into a new file. `0` disables rolling-file logging.
    #[wire(name = "rollingSizeMB")]
    #[default(10)]
    pub rolling_size_mb: i32,
    /// Number of rolled-over log files retained on disk before the oldest is
    /// deleted. `0` retains files indefinitely.
    #[default(5)]
    pub retained_file_count: i32,
    /// Maximum age, in hours, a log file is kept before it is eligible for
    /// deletion. `0` retains files indefinitely.
    #[default(24)]
    pub time_limit_hours: i32,
    /// Whether old log files are moved into a compressed archive instead of
    /// being deleted outright.
    #[default(true)]
    pub archive_enabled: bool,
    /// Number of archived log files retained on disk. `0` retains files
    /// indefinitely. Cannot be `0` at the same time as
    /// `archive_time_limit_hours` when archiving is enabled.
    #[default(60)]
    pub archive_retained_count: i32,
    /// Maximum age, in hours, an archived log file is kept before deletion.
    /// `0` retains files indefinitely. Cannot be `0` at the same time as
    /// `archive_retained_count` when archiving is enabled.
    #[default(720)]
    pub archive_time_limit_hours: i32,
}
