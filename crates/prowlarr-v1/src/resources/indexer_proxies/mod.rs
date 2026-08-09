//! IndexerProxyProvider — Prowlarr provider enum. Each variant binds an `implementation`
//! string from the wire to a typed fields-blob variant. `#[fallback]` catches any
//! implementation we don't model and preserves it via `RawProvider`.

pub mod flaresolverr;
pub mod http;
pub mod socks4;
pub mod socks5;

pub use flaresolverr::FlaresolverrConfig;
pub use http::HttpConfig;
pub use socks4::Socks4Config;
pub use socks5::Socks5Config;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all supported Prowlarr indexer proxy implementations.
#[tagged(by = "implementation")]
pub enum IndexerProxyProvider {
    #[variant("Flaresolverr")]
    FlareSolverr(FlaresolverrConfig),
    #[variant("HTTP")]
    Http(HttpConfig),
    #[variant("Socks4")]
    Socks4(Socks4Config),
    #[variant("Socks5")]
    Socks5(Socks5Config),
    #[fallback]
    Unknown(RawProvider),
}
