use core_macros::nested;

/// Subsarr subtitle-provider credentials (`settings-subsarr-*`). Enable by adding `subsarr` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Subsarr {
    /// Provider base URL.
    pub base_url: Option<String>,
}
