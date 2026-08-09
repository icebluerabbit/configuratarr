use core_macros::nested;

/// Napisy24 subtitle-provider credentials (`settings-napisy24-*`). Enable by adding `napisy24` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Napisy24 {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
}
