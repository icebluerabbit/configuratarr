use core_macros::nested;

/// Jimaku subtitle-provider credentials (`settings-jimaku-*`). Enable by adding `jimaku` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Jimaku {
    /// API key.
    pub api_key: Option<String>,
    /// Fall back to name search.
    pub enable_name_search_fallback: Option<bool>,
    /// Allow downloading subtitle archives.
    pub enable_archives_download: Option<bool>,
    /// Enable AI-generated subtitles.
    pub enable_ai_subs: Option<bool>,
}
