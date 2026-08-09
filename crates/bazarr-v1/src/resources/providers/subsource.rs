use core_macros::nested;

/// SubSource subtitle-provider credentials (`settings-subsource-*`). Enable by adding `subsource` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct SubSource {
    /// API key.
    pub apikey: Option<String>,
}
