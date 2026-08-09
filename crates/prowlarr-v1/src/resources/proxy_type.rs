use core_macros::wire_enum;

/// Proxy protocol used for Prowlarr's outbound HTTP traffic.
#[wire_enum(rename_all = "lowercase")]
pub enum ProxyType {
    /// HTTP/HTTPS proxy.
    Http,
    /// SOCKS4 proxy.
    Socks4,
    /// SOCKS5 proxy.
    Socks5,
    /// Unknown or future proxy type.
    #[fallback]
    Unknown,
}
