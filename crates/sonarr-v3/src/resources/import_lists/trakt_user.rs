use core_lib::SecretValue;
use core_macros::fields_blob;

/// Trakt User import list — imports series from a Trakt user's watch list, watched list, or collection.
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
    pub auth_user: Option<String>,
    /// Trakt username whose list is being imported.
    pub username: Option<String>,
    /// Which Trakt user list to import (integer enum):
    /// `0` = WatchList, `1` = WatchedList, `2` = CollectionList.
    #[wire(name = "traktListType")]
    pub trakt_list_type: Option<i32>,
    /// Maximum number of series to import.
    pub limit: Option<i32>,
    /// Extra query parameters appended to the Trakt API request.
    #[wire(name = "traktAdditionalParameters")]
    pub trakt_additional_parameters: Option<String>,
}
