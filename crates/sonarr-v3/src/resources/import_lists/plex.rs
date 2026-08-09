use core_lib::SecretValue;
use core_macros::fields_blob;

/// Plex Watchlist import list — imports series from the authenticated Plex user's watchlist.
#[fields_blob(implementation = "PlexImport", config_contract = "PlexListSettings")]
pub struct PlexConfig {
    /// OAuth access token for the Plex account whose watchlist is imported.
    #[wire(name = "accessToken")]
    pub access_token: Option<SecretValue>,
}
