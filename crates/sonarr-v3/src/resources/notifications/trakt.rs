use core_lib::SecretValue;
use core_macros::fields_blob;

/// Trakt notification provider configuration.
///
/// Scrobbles episode watch events to the Trakt.tv service.
#[fields_blob(implementation = "Trakt", config_contract = "TraktSettings")]
pub struct TraktConfig {
    /// Trakt OAuth access token.
    pub access_token: SecretValue,
    /// Trakt OAuth refresh token used to obtain a new access token.
    pub refresh_token: SecretValue,
    /// ISO 8601 timestamp at which the access token expires.
    pub expires: Option<String>,
    /// Trakt username associated with the authenticated account.
    pub auth_user: Option<String>,
}
