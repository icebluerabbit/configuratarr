use core_macros::nested;

/// BetaSeries subtitle-provider credentials (`settings-betaseries-*`). Enable by adding `betaseries` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct BetaSeries {
    /// API token.
    pub token: Option<String>,
}
