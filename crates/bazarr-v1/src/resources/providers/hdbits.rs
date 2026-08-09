use core_macros::nested;

/// HDBits subtitle-provider credentials (`settings-hdbits-*`). Enable by adding `hdbits` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct HdBits {
    /// Account username.
    pub username: Option<String>,
    /// Account passkey.
    pub passkey: Option<String>,
}
