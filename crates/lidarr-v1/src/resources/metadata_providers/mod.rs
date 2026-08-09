//! Tagged-enum for metadata providers (`#[tagged]`). Each `#[variant("...")]`
//! binds an `implementation` string to a typed fields-blob variant. `#[fallback]`
//! catches any implementation we don't model and preserves it via `RawProvider`.

pub mod kodi;
pub mod roksbox;
pub mod wdtv;

pub use kodi::KodiConfig;
pub use roksbox::RoksboxConfig;
pub use wdtv::WdtvConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all modelled Lidarr metadata implementations.
/// The `implementation` field on the wire selects the variant; `#[fallback]`
/// preserves any implementation not listed here as a passthrough blob.
#[tagged(by = "implementation")]
pub enum MetadataProvider {
    #[variant("XbmcMetadata")]
    Kodi(KodiConfig),
    #[variant("RoksboxMetadata")]
    Roksbox(RoksboxConfig),
    #[variant("WdtvMetadata")]
    Wdtv(WdtvConfig),
    #[fallback]
    Unknown(RawProvider),
}
