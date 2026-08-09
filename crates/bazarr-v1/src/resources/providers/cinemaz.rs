use core_macros::nested;

/// CinemaZ subtitle-provider credentials (`settings-cinemaz-*`). Enable by adding `cinemaz` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct CinemaZ {
    /// Session cookies.
    pub cookies: Option<String>,
    /// User-agent to send with cookie auth.
    pub user_agent: Option<String>,
}
