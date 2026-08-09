use core_macros::wire_enum;

/// Authentication method for the Prowlarr web UI.
#[wire_enum(rename_all = "lowercase")]
pub enum AuthenticationType {
    /// No authentication required.
    None,
    /// HTTP Basic authentication.
    Basic,
    /// Forms-based (login page) authentication.
    Forms,
    /// Authentication is handled by an external reverse proxy.
    External,
    /// Unknown or future authentication type.
    #[fallback]
    Unknown,
}
