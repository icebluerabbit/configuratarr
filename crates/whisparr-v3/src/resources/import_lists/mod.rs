//! Tagged-enum for import lists (`#[tagged]`). The discriminator is the
//! `implementation` string in the wire object; each `#[variant("...")]` binds
//! that value to a typed fields-blob variant. `#[fallback]` catches any
//! implementation we don't model and preserves it via `RawProvider`.
//!
//! This is the family that diverges most from Radarr. Whisparr (eros) drops
//! CouchPotato, IMDb, Plex, Radarr/RadarrList, StevenLu/Stevenlu2 and all three
//! Trakt lists, and adds the four StashDB lists plus a Whisparr-to-Whisparr
//! list. The variant set below comes from
//! `src/NzbDrone.Core/ImportLists/` in the eros tree — **not** from
//! `terraform-provider-whisparr`, which is pinned to the unrelated Whisparr V2
//! fork and lists implementations that do not exist here.

pub mod rss;
pub mod stashdb_favorite;
pub mod stashdb_performer;
pub mod stashdb_studio;
pub mod stashdb_tags;
pub mod tmdb_company;
pub mod tmdb_keyword;
pub mod tmdb_list;
pub mod tmdb_person;
pub mod tmdb_popular;
pub mod tmdb_user;
pub mod whisparr;

pub use rss::RssConfig;
pub use stashdb_favorite::StashDbFavoriteConfig;
pub use stashdb_performer::StashDbPerformerConfig;
pub use stashdb_studio::StashDbStudioConfig;
pub use stashdb_tags::StashDbTagsConfig;
pub use tmdb_company::TmdbCompanyConfig;
pub use tmdb_keyword::TmdbKeywordConfig;
pub use tmdb_list::TmdbListConfig;
pub use tmdb_person::TmdbPersonConfig;
pub use tmdb_popular::TmdbPopularConfig;
pub use tmdb_user::TmdbUserConfig;
pub use whisparr::WhisparrConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminated union of all modelled Whisparr import list implementations.
/// The `implementation` field on the wire selects the variant; `#[fallback]`
/// preserves any implementation not listed here as a passthrough blob.
#[tagged(by = "implementation")]
pub enum ImportListProvider {
    #[variant("RSSImport")]
    Rss(RssConfig),
    #[variant("StashDBFavoriteImport")]
    StashDbFavorite(StashDbFavoriteConfig),
    #[variant("StashDBPerformerImport")]
    StashDbPerformer(StashDbPerformerConfig),
    #[variant("StashDBStudioImport")]
    StashDbStudio(StashDbStudioConfig),
    #[variant("StashDBTagsImport")]
    StashDbTags(StashDbTagsConfig),
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
    #[variant("WhisparrImport")]
    Whisparr(WhisparrConfig),
    #[fallback]
    Unknown(RawProvider),
}
