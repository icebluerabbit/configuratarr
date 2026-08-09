use core_macros::nested;

/// Web-UI authentication settings (`settings-auth-*`).
///
/// `password` is stored md5-hashed server-side; the reconcile hook hashes the
/// declared plaintext before diffing, so it stays idempotent. The `auth.apikey`
/// is intentionally **not** modelled — rotating the key mid-apply would break the
/// very connection configuratarr is authenticating with.
#[nested(case = snake)]
pub struct Auth {
    /// Authentication method: unset = none, `basic`, or `form`. Declare it as
    /// `kind:` under `auth:` — it maps to bazarr's `type` key (`type` is a Rust
    /// keyword, so the field is named `kind`).
    #[wire(name = "type")]
    pub kind: Option<String>,
    /// Login username.
    pub username: Option<String>,
    /// Login password (sent in plaintext; bazarr stores it md5-hashed).
    pub password: Option<String>,
}
