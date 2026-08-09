use core_macros::nested;

/// Subf2m subtitle-provider credentials (`settings-subf2m-*`). Enable by adding `subf2m` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Subf2m {
    /// Verify the provider's TLS certificate.
    pub verify_ssl: Option<bool>,
    /// User-agent to send with cookie auth.
    pub user_agent: Option<String>,
}
