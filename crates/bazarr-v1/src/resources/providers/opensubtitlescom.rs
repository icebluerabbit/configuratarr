use core_macros::nested;

/// OpenSubtitles.com subtitle-provider credentials (`settings-opensubtitlescom-*`). Enable by adding `opensubtitlescom` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct OpenSubtitlesCom {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
    /// Match by file hash.
    pub use_hash: Option<bool>,
    /// Include AI-translated subtitles.
    pub include_ai_translated: Option<bool>,
    /// Include machine-translated subtitles.
    pub include_machine_translated: Option<bool>,
}
