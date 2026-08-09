use core_macros::nested;

/// Machine-translation settings (`settings-translator-*`) — the engine bazarr
/// uses to translate subtitles when no native subtitle is found.
#[nested(case = snake)]
pub struct Translator {
    /// Translation engine (`google_translate`, `gemini`, `lingarr`, …).
    pub translator_type: Option<String>,
    /// Minimum score a translated subtitle must reach to be kept.
    pub default_score: Option<i32>,
    /// Show translation info/attribution in the UI.
    pub translator_info: Option<bool>,
    /// Google Gemini API key (used when `translator_type` is `gemini`).
    pub gemini_key: Option<String>,
    /// Gemini model id (e.g. `gemini-2.0-flash`).
    pub gemini_model: Option<String>,
    /// Lingarr base URL (used when `translator_type` is `lingarr`).
    pub lingarr_url: Option<String>,
    /// Lingarr API token.
    pub lingarr_token: Option<String>,
}
