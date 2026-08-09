use core_macros::nested;

/// Outbound proxy settings (`settings-proxy-*`).
#[nested(case = snake)]
pub struct Proxy {
    /// Proxy type: `socks5` / `http`, or unset to disable. Declare it as `kind:`
    /// under `proxy:` — it maps to bazarr's `type` key (`type` is a Rust keyword,
    /// so the field is named `kind`).
    #[wire(name = "type")]
    pub kind: Option<String>,
    /// Proxy host.
    pub url: Option<String>,
    /// Proxy port.
    pub port: Option<String>,
    /// Proxy username.
    pub username: Option<String>,
    /// Proxy password.
    pub password: Option<String>,
    /// Hosts to reach directly, bypassing the proxy.
    pub exclude: Vec<String>,
}
