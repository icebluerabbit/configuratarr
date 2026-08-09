use core_macros::nested;

/// A path remap applied to media paths.
#[nested(case = pascal)]
pub struct PathSubstitution {
    /// From
    pub from: Option<String>,
    /// To
    pub to: Option<String>,
}
