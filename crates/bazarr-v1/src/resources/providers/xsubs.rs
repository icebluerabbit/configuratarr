use core_macros::nested;

/// XSubs subtitle-provider credentials (`settings-xsubs-*`). Enable by adding `xsubs` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct XSubs {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
}
