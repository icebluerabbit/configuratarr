use core_lib::SecretValue;
use core_macros::resource;

/// `/api/account/oidc` — OpenID Connect single sign-on configuration.
///
/// The GET response (`OidcConfig`) additionally carries a read-only
/// `authorizedSubject` field (the subject claim of the last account that
/// completed the OIDC flow); the PUT contract does not accept it, so it is
/// not modelled here and is left unmanaged.
#[resource(
    sync = singleton,
    read = get("/api/account/oidc"),
    update = put("/api/account/oidc"),
)]
pub struct Oidc {
    /// Enables OIDC single sign-on for the Cleanuparr UI.
    #[default(false)]
    pub enabled: bool,
    /// Base URL of the OIDC identity provider (issuer).
    #[default("")]
    pub issuer_url: String,
    /// OAuth2 client id registered with the identity provider.
    #[default("")]
    pub client_id: String,
    /// OAuth2 client secret registered with the identity provider.
    pub client_secret: Option<SecretValue>,
    /// Space-separated OAuth2 scopes requested during authentication.
    #[default("openid profile email")]
    pub scopes: String,
    /// Display name for the OIDC provider shown on the Cleanuparr login page.
    #[default("OIDC")]
    pub provider_name: String,
    /// URL the identity provider redirects back to after authentication.
    #[default("")]
    pub redirect_url: String,
    /// When enabled, OIDC is the only allowed sign-in method and Cleanuparr's
    /// built-in username/password login is disabled.
    #[default(false)]
    pub exclusive_mode: bool,
}
