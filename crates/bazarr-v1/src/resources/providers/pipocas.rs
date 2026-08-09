use core_macros::nested;

/// Pipocas subtitle-provider credentials (`settings-pipocas-*`). Enable by adding `pipocas` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Pipocas {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
}
