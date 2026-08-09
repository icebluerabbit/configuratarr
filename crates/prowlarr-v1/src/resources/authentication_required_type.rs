use core_macros::wire_enum;

/// Controls whether Prowlarr requires authentication for local addresses.
#[wire_enum(rename_all = "camelCase")]
pub enum AuthenticationRequiredType {
    /// Authentication is required for all requests.
    Enabled,
    /// Authentication is disabled for requests from local network addresses.
    DisabledForLocalAddresses,
    /// Unknown or future requirement type.
    #[fallback]
    Unknown,
}
