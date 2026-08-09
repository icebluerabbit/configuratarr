use core_macros::nested;

/// Jellyfin connection settings (`settings-jellyfin-*`).
#[nested(case = snake)]
pub struct Jellyfin {
    /// Jellyfin base URL.
    pub url: Option<String>,
    /// Jellyfin API key.
    pub apikey: Option<String>,
    /// Update the movie library after a subtitle change.
    pub update_movie_library: Option<bool>,
    /// Update the series library after a subtitle change.
    pub update_series_library: Option<bool>,
    /// Movie library ids to refresh.
    pub movie_library_ids: Vec<String>,
    /// Series library ids to refresh.
    pub series_library_ids: Vec<String>,
    /// Refresh method (`immediate` / `scan`).
    pub refresh_method: Option<String>,
}
