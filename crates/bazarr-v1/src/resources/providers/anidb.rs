use core_macros::nested;

/// AniDB client used for anime hash matching.
#[nested(case = snake)]
pub struct AniDb {
    /// API client id.
    pub api_client: Option<String>,
    /// API client version.
    pub api_client_ver: Option<i32>,
}
