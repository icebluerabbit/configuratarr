//! Tagged-enum for Lidarr import list provider implementations.
//!
//! The discriminator is the `implementation` string on the wire object.
//! Each `#[variant("...")]` binds that value to a typed fields-blob variant.
//! `#[fallback]` catches any implementation we don't model and preserves it
//! round-trip via `RawProvider`.

pub mod headphones;
pub mod lastfm_tag;
pub mod lastfm_user;
pub mod lidarr_import;
pub mod lidarr_lists;
pub mod music_brainz;
pub mod spotify_albums;
pub mod spotify_artists;
pub mod spotify_playlists;

pub use headphones::HeadphonesConfig;
pub use lastfm_tag::LastFmTagConfig;
pub use lastfm_user::LastFmUserConfig;
pub use lidarr_import::LidarrImportConfig;
pub use lidarr_lists::LidarrListsConfig;
pub use music_brainz::MusicBrainzConfig;
pub use spotify_albums::SpotifyAlbumsConfig;
pub use spotify_artists::SpotifyArtistsConfig;
pub use spotify_playlists::SpotifyPlaylistsConfig;

use core_macros::tagged;

use crate::resources::raw_provider::RawProvider;

/// Discriminator-dispatched provider config for Lidarr import lists.
///
/// Each variant corresponds to one implementation string the API returns in the
/// `implementation` field. `Unknown` catches everything else and round-trips it
/// losslessly via the raw fields map.
#[tagged(by = "implementation")]
pub enum ImportListProvider {
    #[variant("HeadphonesImport")]
    Headphones(HeadphonesConfig),
    #[variant("LastFmTag")]
    LastFmTag(LastFmTagConfig),
    #[variant("LastFMUser")]
    LastFmUser(LastFmUserConfig),
    #[variant("LidarrImport")]
    LidarrImport(LidarrImportConfig),
    #[variant("LidarrLists")]
    LidarrLists(LidarrListsConfig),
    #[variant("MusicBrainzSeries")]
    MusicBrainz(MusicBrainzConfig),
    #[variant("SpotifySavedAlbums")]
    SpotifyAlbums(SpotifyAlbumsConfig),
    #[variant("SpotifyFollowedArtists")]
    SpotifyArtists(SpotifyArtistsConfig),
    #[variant("SpotifyPlaylist")]
    SpotifyPlaylists(SpotifyPlaylistsConfig),
    #[fallback]
    Unknown(RawProvider),
}
