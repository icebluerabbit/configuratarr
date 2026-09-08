use core_lib::SecretValue;
use core_macros::fields_blob;

/// Trakt notification provider configuration.
#[fields_blob(implementation = "Trakt", config_contract = "TraktSettings")]
pub struct TraktConfig {
    /// Trakt OAuth access token.
    #[wire(name = "accessToken")]
    pub access_token: SecretValue,
    /// Trakt OAuth refresh token used to obtain a new access token.
    #[wire(name = "refreshToken")]
    pub refresh_token: SecretValue,
    /// ISO 8601 timestamp at which the access token expires.
    pub expires: Option<String>,
    /// Trakt username associated with the authenticated account.
    #[wire(name = "authUser")]
    pub auth_user: Option<String>,
    /// OAuth sign-in marker used by the UI to start the Trakt flow
    /// (default `startOAuth`).
    #[wire(name = "signIn")]
    pub sign_in: Option<String>,
}
