//! Tagged-enum for application providers (`#[tagged]`). Each `#[variant("...")]`
//! binds an `implementation` string to a typed fields-blob variant. `#[fallback]`
//! catches any implementation we don't model and preserves it via `RawProvider`.

pub mod lazy_librarian;
pub mod lidarr;
pub mod mylar;
pub mod radarr;
pub mod readarr;
pub mod sonarr;
pub mod whisparr;

pub use lazy_librarian::LazyLibrarianConfig;
pub use lidarr::LidarrConfig;
pub use mylar::MylarConfig;
pub use radarr::RadarrConfig;
pub use readarr::ReadarrConfig;
pub use sonarr::SonarrConfig;
pub use whisparr::WhisparrConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all modelled Prowlarr application implementations.
/// The `implementation` field on the wire selects the variant; `#[fallback]`
/// preserves any implementation not listed here as a passthrough blob.
#[tagged(by = "implementation")]
pub enum ApplicationProvider {
    #[variant("LazyLibrarian")]
    LazyLibrarian(LazyLibrarianConfig),
    #[variant("Lidarr")]
    Lidarr(LidarrConfig),
    #[variant("Mylar")]
    Mylar(MylarConfig),
    #[variant("Radarr")]
    Radarr(RadarrConfig),
    #[variant("Readarr")]
    Readarr(ReadarrConfig),
    #[variant("Sonarr")]
    Sonarr(SonarrConfig),
    #[variant("Whisparr")]
    Whisparr(WhisparrConfig),
    #[fallback]
    Unknown(RawProvider),
}
