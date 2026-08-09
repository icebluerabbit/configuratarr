use core_macros::nested;

/// Authentication bypass and reverse-proxy trust settings.
///
/// Embedded in [`crate::resources::general::General`] under the `auth` key.
#[nested]
pub struct AuthConfig {
    /// Authenticates callers from local or trusted networks as the admin
    /// user without requiring credentials.
    pub disable_auth_for_local_addresses: Option<bool>,
    /// Honours `X-Forwarded-For` / `-Proto` / `-Host` from trusted hops when
    /// resolving the client address. Only enable this when Cleanuparr sits
    /// behind a trusted reverse proxy.
    pub trust_forwarded_headers: Option<bool>,
    /// Plain IPs or CIDR ranges treated as trusted networks. Loopback and
    /// private ranges are always trusted regardless of this list.
    pub trusted_networks: Vec<String>,
}
