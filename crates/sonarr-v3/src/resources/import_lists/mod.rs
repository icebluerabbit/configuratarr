//! Tagged-enum for Sonarr import list provider implementations.
//!
//! The discriminator is the `implementation` string on the wire object.
//! Each `#[variant("...")]` binds that value to a typed fields-blob variant.
//! `#[fallback]` catches any implementation we don't model and preserves it
//! round-trip via `RawProvider`.

pub mod custom;
pub mod imdb;
pub mod plex;
pub mod plex_rss;
pub mod simkl_user;
pub mod sonarr;
pub mod trakt_list;
pub mod trakt_popular;
pub mod trakt_user;

pub use custom::CustomConfig;
pub use imdb::ImdbConfig;
pub use plex::PlexConfig;
pub use plex_rss::PlexRssConfig;
pub use simkl_user::SimklUserConfig;
pub use sonarr::SonarrConfig;
pub use trakt_list::TraktListConfig;
pub use trakt_popular::TraktPopularConfig;
pub use trakt_user::TraktUserConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

#[tagged(by = "implementation")]
pub enum ImportListProvider {
    #[variant("CustomImport")]
    Custom(CustomConfig),
    #[variant("ImdbListImport")]
    Imdb(ImdbConfig),
    #[variant("PlexImport")]
    Plex(PlexConfig),
    #[variant("PlexRssImport")]
    PlexRss(PlexRssConfig),
    #[variant("SimklUserImport")]
    SimklUser(SimklUserConfig),
    #[variant("SonarrImport")]
    Sonarr(SonarrConfig),
    #[variant("TraktListImport")]
    TraktList(TraktListConfig),
    #[variant("TraktPopularImport")]
    TraktPopular(TraktPopularConfig),
    #[variant("TraktUserImport")]
    TraktUser(TraktUserConfig),
    #[fallback]
    Unknown(RawProvider),
}
