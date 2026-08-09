use core_macros::nested;

/// Plex integration settings (`settings-plex-*`) — connection + library refresh.
///
/// Curated to the declarative connection/library keys; bazarr's OAuth/migration
/// bookkeeping (`encryption_key`, `client_identifier`, `migration_*`, `server_*`)
/// is runtime state and is not modelled.
#[nested(case = snake)]
pub struct Plex {
    /// Plex host/IP.
    pub ip: Option<String>,
    /// Plex port.
    pub port: Option<i32>,
    /// Use HTTPS to reach Plex.
    pub ssl: Option<bool>,
    /// Plex API key.
    pub apikey: Option<String>,
    /// Plex auth token.
    pub token: Option<String>,
    /// How to authenticate (`apikey` / `oauth`).
    pub auth_method: Option<String>,
    /// Refresh the movie library after a subtitle change.
    pub update_movie_library: Option<bool>,
    /// Refresh the series library after a subtitle change.
    pub update_series_library: Option<bool>,
    /// Movie library ids to refresh.
    pub movie_library_ids: Vec<String>,
    /// Series library ids to refresh.
    pub series_library_ids: Vec<String>,
    /// Set the "added" date on movies from the subtitle date.
    pub set_movie_added: Option<bool>,
    /// Set the "added" date on episodes from the subtitle date.
    pub set_episode_added: Option<bool>,
}
