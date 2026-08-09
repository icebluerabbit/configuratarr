use core_lib::SecretValue;
use core_macros::fields_blob;

/// Spotify Playlist import list — imports artists from one or more Spotify playlists.
#[fields_blob(
    implementation = "SpotifyPlaylist",
    config_contract = "SpotifyPlaylistSettings"
)]
pub struct SpotifyPlaylistsConfig {
    /// OAuth access token for the Spotify user session.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
    /// OAuth refresh token used to renew the access token.
    #[wire(name = "refreshToken")]
    pub refresh_token: Option<SecretValue>,
    /// Expiry timestamp of the current access token.
    pub expires: Option<String>,
    /// Spotify playlist IDs to import artists from.
    #[wire(name = "playlistIds")]
    pub playlist_ids: Vec<String>,
}
