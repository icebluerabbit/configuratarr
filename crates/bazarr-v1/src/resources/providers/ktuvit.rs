use core_macros::nested;

/// Ktuvit subtitle-provider credentials (`settings-ktuvit-*`). Enable by adding `ktuvit` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Ktuvit {
    /// Account email.
    pub email: Option<String>,
    /// Pre-hashed account password (provide the hash bazarr expects).
    pub hashed_password: Option<String>,
}
