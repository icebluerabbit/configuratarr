use core_macros::nested;

/// NapiProjekt subtitle-provider credentials (`settings-napiprojekt-*`). Enable by adding `napiprojekt` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct NapiProjekt {
    /// Only match subtitles by known authors.
    pub only_authors: Option<bool>,
    /// Only match subtitles with real author names.
    pub only_real_names: Option<bool>,
}
