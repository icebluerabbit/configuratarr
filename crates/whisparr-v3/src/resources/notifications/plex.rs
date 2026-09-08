use core_lib::SecretValue;
use core_macros::fields_blob;

/// Plex Media Server notification provider configuration.
#[fields_blob(implementation = "PlexServer", config_contract = "PlexServerSettings")]
pub struct PlexConfig {
    /// Plex server selected from the plex.tv account, populated by the UI's
    /// `servers` action. Not persisted with the settings blob.
    pub server: Option<String>,
    /// Plex Media Server hostname or IP address.
    pub host: String,
    /// Plex Media Server HTTP port (default 32400).
    pub port: i32,
    /// Connect to Plex over HTTPS.
    #[wire(name = "useSsl")]
    pub use_ssl: Option<bool>,
    /// URL base path when Plex is hosted behind a reverse proxy.
    #[wire(name = "urlBase")]
    pub url_base: Option<String>,
    /// Plex authentication token (X-Plex-Token).
    #[wire(name = "authToken")]
    pub auth_token: SecretValue,
    /// OAuth sign-in marker used by the UI to start the plex.tv flow
    /// (default `startOAuth`).
    #[wire(name = "signIn")]
    pub sign_in: Option<String>,
    /// Trigger a Plex library section refresh after a scene is imported.
    #[wire(name = "updateLibrary")]
    pub update_library: Option<bool>,
    /// Whisparr-side path prefix to rewrite when Plex mounts the library at a
    /// different location.
    #[wire(name = "mapFrom")]
    pub map_from: Option<String>,
    /// Plex-side path prefix that `map_from` is rewritten to.
    #[wire(name = "mapTo")]
    pub map_to: Option<String>,
}
