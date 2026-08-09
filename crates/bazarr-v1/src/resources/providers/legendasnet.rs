use core_macros::nested;

/// Legendas.net subtitle-provider credentials (`settings-legendasnet-*`). Enable by adding `legendasnet` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct LegendasNet {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
}
