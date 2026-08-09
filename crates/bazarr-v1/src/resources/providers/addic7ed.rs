use core_macros::nested;

/// Addic7ed subtitle-provider credentials (`settings-addic7ed-*`). Enable by adding `addic7ed` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Addic7ed {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
    /// Session cookies.
    pub cookies: Option<String>,
    /// User-agent to send with cookie auth.
    pub user_agent: Option<String>,
    /// Account has VIP access.
    pub vip: Option<bool>,
}
