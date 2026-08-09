use core_macros::nested;

/// A plugin repository entry.
#[nested(case = pascal)]
pub struct RepositoryInfo {
    /// Name
    pub name: Option<String>,
    /// Url
    pub url: Option<String>,
    /// Enabled
    pub enabled: bool,
}
