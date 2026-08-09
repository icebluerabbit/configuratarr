use core_macros::nested;

/// Extract subtitles already embedded in the media files.
#[nested(case = snake)]
pub struct EmbeddedSubtitles {
    /// Subtitle codecs to include.
    pub included_codecs: Vec<String>,
    /// Fall back to hearing-impaired subtitles.
    pub hi_fallback: Option<bool>,
    /// Request timeout, in seconds.
    pub timeout: Option<i32>,
    /// Treat unknown-language tracks as a fallback.
    pub unknown_as_fallback: Option<bool>,
    /// Fallback language code.
    pub fallback_lang: Option<String>,
}
