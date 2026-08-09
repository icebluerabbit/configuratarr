use core_lib::SecretValue;
use core_macros::fields_blob;

/// TMDb User import list — imports movies from a TMDb user's list.
#[fields_blob(
    implementation = "TMDbUserImport",
    config_contract = "TMDbUserSettings"
)]
pub struct TmdbUserConfig {
    /// TMDb account identifier for the target user.
    #[wire(name = "accountId")]
    pub account_id: Option<String>,
    /// TMDb v4 read access token for the user's account.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
    /// Type of user list to import (integer enum; e.g. favorites, watchlist, rated).
    #[wire(name = "userListType")]
    pub user_list_type: Option<i32>,
}
