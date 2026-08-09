use core_macros::nested;

/// Log-filtering settings (`settings-log-*`) — which log lines the UI keeps.
#[nested(case = snake)]
pub struct Log {
    /// Only keep log lines matching this filter.
    pub include_filter: Option<String>,
    /// Drop log lines matching this filter.
    pub exclude_filter: Option<String>,
    /// Treat the filters as regular expressions.
    pub use_regex: Option<bool>,
    /// Match the filters case-insensitively.
    pub ignore_case: Option<bool>,
}
