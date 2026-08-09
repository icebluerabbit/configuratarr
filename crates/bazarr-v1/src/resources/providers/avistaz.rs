use core_macros::nested;

/// AvistaZ subtitle-provider credentials (`settings-avistaz-*`). Enable by adding `avistaz` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct AvistaZ {
    /// Session cookies.
    pub cookies: Option<String>,
    /// User-agent to send with cookie auth.
    pub user_agent: Option<String>,
}
