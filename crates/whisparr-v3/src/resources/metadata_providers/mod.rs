//! Tagged-enum for metadata providers (`#[tagged]`). Each `#[variant("...")]`
//! binds an `implementation` string to a typed fields-blob variant. `#[fallback]`
//! catches any implementation we don't model and preserves it via `RawProvider`.

pub mod kometa;
pub mod media_browser;
pub mod roksbox;
pub mod wdtv;
pub mod xbmc;

pub use kometa::KometaConfig;
pub use media_browser::MediaBrowserConfig;
pub use roksbox::RoksboxConfig;
pub use wdtv::WdtvConfig;
pub use xbmc::XbmcConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all modelled Whisparr metadata implementations.
/// The `implementation` field on the wire selects the variant; `#[fallback]`
/// preserves any implementation not listed here as a passthrough blob.
#[tagged(by = "implementation")]
pub enum MetadataProvider {
    #[variant("XbmcMetadata")]
    Xbmc(XbmcConfig),
    #[variant("RoksboxMetadata")]
    Roksbox(RoksboxConfig),
    #[variant("WdtvMetadata")]
    Wdtv(WdtvConfig),
    #[variant("KometaMetadata")]
    Kometa(KometaConfig),
    #[variant("MediaBrowserMetadata")]
    MediaBrowser(MediaBrowserConfig),
    #[fallback]
    Unknown(RawProvider),
}
