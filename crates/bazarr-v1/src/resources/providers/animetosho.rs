use core_macros::nested;

/// AnimeTosho anime subtitle provider.
#[nested(case = snake)]
pub struct AnimeTosho {
    /// Search score threshold.
    pub search_threshold: Option<i32>,
    /// AniDB API client id.
    pub anidb_api_client: Option<String>,
    /// AniDB API client version.
    pub anidb_api_client_ver: Option<i32>,
}
