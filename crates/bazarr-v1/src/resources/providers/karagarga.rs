use core_macros::nested;

/// KaraGarga subtitle-provider credentials (`settings-karagarga-*`). Enable by adding `karagarga` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct KaraGarga {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
    /// Forum username.
    pub f_username: Option<String>,
    /// Forum password.
    pub f_password: Option<String>,
}
