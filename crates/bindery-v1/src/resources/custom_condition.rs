use core_macros::nested;

/// One matching rule inside a [`crate::resources::custom_format::CustomFormat`].
///
/// **Nothing here is validated server-side.** `type` is stored verbatim and
/// is *not* checked against any enum — the Go field comment documents
/// `releaseTitle`, `releaseGroup`, `size`, `indexerFlag` as a convention, not
/// an enforced set. `pattern` is likewise stored verbatim and never compiled
/// or range-checked at write time, so a malformed rule is accepted and only
/// fails (or silently never matches) downstream.
#[nested]
pub struct CustomCondition {
    /// What the condition matches against. Convention only (`releaseTitle`,
    /// `releaseGroup`, `size`, `indexerFlag`) — the API does not validate this
    /// value, so a typo is stored as-is rather than rejected.
    #[wire(name = "type")]
    pub condition_type: String,
    /// The pattern/value matched against the field named by `type`. Stored
    /// verbatim and never compiled or range-checked.
    pub pattern: String,
    /// Invert the match — the condition holds when the pattern does NOT match.
    #[default(false)]
    pub negate: bool,
    /// The condition must match for the format to apply at all, rather than
    /// merely contributing to it.
    #[default(false)]
    pub required: bool,
}
