use core_macros::nested;

/// Assrt subtitle-provider credentials (`settings-assrt-*`). Enable by adding `assrt` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Assrt {
    /// API token.
    pub token: Option<String>,
}
