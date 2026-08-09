use core_lib::SecretValue;
use core_macros::fields_blob;

/// Plex Watchlist import list — imports from a Plex watchlist using an access token.
#[fields_blob(implementation = "PlexImport", config_contract = "PlexListSettings")]
pub struct PlexConfig {
    /// Plex authentication token granting access to the user's watchlist.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
}
