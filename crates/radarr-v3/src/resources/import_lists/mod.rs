//! Tagged-enum template (`#[tagged]`). The discriminator is the `implementation`
//! string in the wire object; each `#[variant("...")]` binds that value to a
//! typed fields-blob variant. `#[fallback]` catches any implementation we don't
//! model and preserves it via `RawProvider`.
//!
//! Subagents: add `pub mod <name>;` + a `#[variant("<Impl>")] <Name>(<Name>Config),`
//! line per import-list implementation in the spec. One file per variant,
//! following `couch_potato.rs`.

pub mod couch_potato;
pub mod custom;
pub mod imdb;
pub mod plex;
pub mod radarr;
pub mod rss;
pub mod stevenlu;
pub mod stevenlu2;
pub mod tmdb_company;
pub mod tmdb_keyword;
pub mod tmdb_list;
pub mod tmdb_person;
pub mod tmdb_popular;
pub mod tmdb_user;
pub mod trakt_list;
pub mod trakt_popular;
pub mod trakt_user;

pub use couch_potato::CouchPotatoConfig;
pub use custom::CustomConfig;
pub use imdb::ImdbConfig;
pub use plex::PlexConfig;
pub use radarr::RadarrConfig;
pub use rss::RssConfig;
pub use stevenlu::StevenLuConfig;
pub use stevenlu2::StevenLu2Config;
pub use tmdb_company::TmdbCompanyConfig;
pub use tmdb_keyword::TmdbKeywordConfig;
pub use tmdb_list::TmdbListConfig;
pub use tmdb_person::TmdbPersonConfig;
pub use tmdb_popular::TmdbPopularConfig;
pub use tmdb_user::TmdbUserConfig;
pub use trakt_list::TraktListConfig;
pub use trakt_popular::TraktPopularConfig;
pub use trakt_user::TraktUserConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

#[tagged(by = "implementation")]
pub enum ImportListProvider {
    #[variant("CouchPotatoImport")]
    CouchPotato(CouchPotatoConfig),
    #[variant("RadarrListImport")]
    Custom(CustomConfig),
    #[variant("IMDbListImport")]
    Imdb(ImdbConfig),
    #[variant("PlexImport")]
    Plex(PlexConfig),
    #[variant("RadarrImport")]
    Radarr(RadarrConfig),
    #[variant("RSSImport")]
    Rss(RssConfig),
    #[variant("StevenLuImport")]
    StevenLu(StevenLuConfig),
    #[variant("Stevenlu2Import")]
    StevenLu2(StevenLu2Config),
    #[variant("TMDbCompanyImport")]
    TmdbCompany(TmdbCompanyConfig),
    #[variant("TMDbKeywordImport")]
    TmdbKeyword(TmdbKeywordConfig),
    #[variant("TMDbListImport")]
    TmdbList(TmdbListConfig),
    #[variant("TMDbPersonImport")]
    TmdbPerson(TmdbPersonConfig),
    #[variant("TMDbPopularImport")]
    TmdbPopular(TmdbPopularConfig),
    #[variant("TMDbUserImport")]
    TmdbUser(TmdbUserConfig),
    #[variant("TraktListImport")]
    TraktList(TraktListConfig),
    #[variant("TraktPopularImport")]
    TraktPopular(TraktPopularConfig),
    #[variant("TraktUserImport")]
    TraktUser(TraktUserConfig),
    #[fallback]
    Unknown(RawProvider),
}
