use core_macros::nested;

/// Titlovi subtitle-provider credentials (`settings-titlovi-*`). Enable by adding `titlovi` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Titlovi {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
}
