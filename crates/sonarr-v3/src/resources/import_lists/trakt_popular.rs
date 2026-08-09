use core_lib::SecretValue;
use core_macros::fields_blob;

/// Trakt Popular import list — imports popular/trending series from Trakt with optional filters.
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
    pub auth_user: Option<String>,
    /// Category of popular series to import (integer enum):
    /// `0` = Trending, `1` = Popular, `2` = Anticipated, `3`–`6` = TopWatched*, `7`–`10` = Recommended*.
    #[wire(name = "traktListType")]
    pub trakt_list_type: Option<i32>,
    /// Maximum number of series to import.
    pub limit: Option<i32>,
    /// Comma-separated list of genre slugs to include (e.g. `"drama,comedy"`).
    pub genres: Option<String>,
    /// Year or year range filter (e.g. `"2020"` or `"2015-2020"`).
    pub years: Option<String>,
    /// Score range filter (e.g. `"70-100"`).
    pub rating: Option<String>,
    /// Extra query parameters appended to the Trakt API request.
    #[wire(name = "traktAdditionalParameters")]
    pub trakt_additional_parameters: Option<String>,
}
