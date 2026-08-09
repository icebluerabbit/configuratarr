use core_macros::wire_enum;

/// TLS/SSL certificate validation mode for outbound Prowlarr connections.
#[wire_enum(rename_all = "camelCase")]
pub enum CertificateValidationType {
    /// Full certificate validation is enforced for all connections.
    Enabled,
    /// Certificate validation is skipped for connections to local/private addresses.
    DisabledForLocalAddresses,
    /// Certificate validation is disabled for all connections.
    Disabled,
    /// Unknown or future validation mode.
    #[fallback]
    Unknown,
}
