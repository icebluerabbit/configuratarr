use core_macros::nested;

/// Subs.ro subtitle-provider credentials (`settings-subsro-*`). Enable by adding `subsro` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct SubsRo {
    /// API key.
    pub api_key: Option<String>,
}
