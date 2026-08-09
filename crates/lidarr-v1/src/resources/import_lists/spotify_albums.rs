use core_lib::SecretValue;
use core_macros::fields_blob;

/// Spotify Saved Albums import list — imports artists from a Spotify user's saved albums.
#[fields_blob(
    implementation = "SpotifySavedAlbums",
    config_contract = "SpotifySavedAlbumsSettings"
)]
pub struct SpotifyAlbumsConfig {
    /// OAuth access token for the Spotify user session.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
    /// OAuth refresh token used to renew the access token.
    #[wire(name = "refreshToken")]
    pub refresh_token: Option<SecretValue>,
    /// Expiry timestamp of the current access token.
    pub expires: Option<String>,
}
