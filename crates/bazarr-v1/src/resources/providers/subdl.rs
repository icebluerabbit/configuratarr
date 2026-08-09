use core_macros::nested;

/// SubDL subtitle-provider credentials (`settings-subdl-*`). Enable by adding `subdl` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct SubDl {
    /// API key.
    pub api_key: Option<String>,
}
