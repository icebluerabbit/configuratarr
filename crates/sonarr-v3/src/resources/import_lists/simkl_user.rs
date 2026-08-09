use core_lib::SecretValue;
use core_macros::fields_blob;

/// Simkl user import list — imports series from the authenticated Simkl user's lists.
#[fields_blob(
    implementation = "SimklUserImport",
    config_contract = "SimklUserSettings"
)]
pub struct SimklUserConfig {
    /// Integer identifying which Simkl list to import:
    /// `0` = Watching, `1` = Plan to Watch, `2` = Hold, `3` = Completed, `4` = Dropped.
    #[wire(name = "listType")]
    pub list_type: Option<i32>,
    /// OAuth access token for the Simkl account.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
    /// OAuth refresh token used to renew the access token.
    #[wire(name = "refreshToken")]
    pub refresh_token: Option<SecretValue>,
    /// Simkl username linked to the OAuth credentials.
    #[wire(name = "authUser")]
    pub auth_user: Option<String>,
    /// ISO 8601 expiry timestamp for the access token.
    pub expires: Option<String>,
}
