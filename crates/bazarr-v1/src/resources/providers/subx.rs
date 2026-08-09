use core_macros::nested;

/// SubX subtitle-provider credentials (`settings-subx-*`). Enable by adding `subx` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct SubX {
    /// API key.
    pub api_key: Option<String>,
}
