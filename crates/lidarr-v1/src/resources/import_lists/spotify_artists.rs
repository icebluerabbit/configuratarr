use core_lib::SecretValue;
use core_macros::fields_blob;

/// Spotify Followed Artists import list — imports artists a Spotify user follows.
#[fields_blob(
    implementation = "SpotifyFollowedArtists",
    config_contract = "SpotifyFollowedArtistsSettings"
)]
pub struct SpotifyArtistsConfig {
    /// OAuth access token for the Spotify user session.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
    /// OAuth refresh token used to renew the access token.
    #[wire(name = "refreshToken")]
    pub refresh_token: Option<SecretValue>,
    /// Expiry timestamp of the current access token.
    pub expires: Option<String>,
}
