use core_lib::SecretValue;
use core_macros::fields_blob;

/// TMDb User import list — imports titles from a TMDb user's own lists.
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
    /// Type of user list to import. Integer enum:
    /// 1 = Watchlist, 2 = Recommendations, 3 = Rated, 4 = Favorite.
    #[wire(name = "listType")]
    pub list_type: Option<i32>,
    /// OAuth handshake slot the UI's "Authenticate with TMDB" button writes.
    /// Normally left unset in config — the account id and access token above
    /// are what the list actually authenticates with.
    #[wire(name = "signIn")]
    pub sign_in: Option<String>,
}
