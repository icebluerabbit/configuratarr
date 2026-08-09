use core_lib::SecretValue;
use core_macros::fields_blob;

/// Trakt List import list — imports movies from a specific Trakt user list.
#[fields_blob(
    implementation = "TraktListImport",
    config_contract = "TraktListSettings"
)]
pub struct TraktListConfig {
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
    /// Slug name of the Trakt list to import.
    pub listname: Option<String>,
    /// Maximum number of movies to import from the list.
    pub limit: Option<i32>,
}
