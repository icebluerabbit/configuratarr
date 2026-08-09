use core_lib::SecretValue;
use core_macros::fields_blob;

/// Trakt Popular import list — imports popular movies from Trakt with optional filters.
#[fields_blob(
    implementation = "TraktPopularImport",
    config_contract = "TraktPopularSettings"
)]
pub struct TraktPopularConfig {
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
    /// Category of popular movies to import (integer enum; e.g. popular, trending, anticipated).
    #[wire(name = "traktListType")]
    pub trakt_list_type: Option<i32>,
    /// Maximum number of movies to import.
    pub limit: Option<i32>,
    /// Content certification filter (e.g. `"pg-13"`).
    pub certification: Option<String>,
    /// Comma-separated list of genre slugs to include.
    pub genres: Option<String>,
    /// Year or year range filter (e.g. `"2010"` or `"2010-2020"`).
    pub years: Option<String>,
    /// Score range filter (e.g. `"70-100"`).
    pub rating: Option<String>,
    /// Extra query parameters appended to the Trakt API request.
    #[wire(name = "traktAdditionalParameters")]
    pub trakt_additional_parameters: Option<String>,
}
