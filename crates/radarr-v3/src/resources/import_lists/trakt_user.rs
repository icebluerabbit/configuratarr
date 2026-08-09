use core_lib::SecretValue;
use core_macros::fields_blob;

/// Trakt User import list — imports movies from a Trakt user's watched/watchlist.
#[fields_blob(
    implementation = "TraktUserImport",
    config_contract = "TraktUserSettings"
)]
pub struct TraktUserConfig {
    /// OAuth access token for authenticating with the Trakt API.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
    /// OAuth refresh token used to renew the access token.
    #[wire(name = "refreshToken")]
    pub refresh_token: Option<SecretValue>,
    /// ISO 8601 expiry timestamp for the access token.
    pub expires: Option<String>,
    /// Trakt username associated with the OAuth credentials.
    #[wire(name = "authUser")]
    pub auth_user: Option<SecretValue>,
    /// Trakt username whose list is being imported.
    pub username: Option<String>,
    /// User list type to import (integer enum; e.g. watchlist, watched, collection).
    #[wire(name = "traktListType")]
    pub trakt_list_type: Option<i32>,
    /// Maximum number of movies to import.
    pub limit: Option<i32>,
}
