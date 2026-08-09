use core_macros::nested;

/// Turkcealtyazi.org subtitle-provider credentials (`settings-turkcealtyaziorg-*`). Enable by adding `turkcealtyaziorg` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct TurkceAltyaziOrg {
    /// Session cookies.
    pub cookies: Option<String>,
    /// User-agent to send with cookie auth.
    pub user_agent: Option<String>,
}
