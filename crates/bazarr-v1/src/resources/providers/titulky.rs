use core_macros::nested;

/// Titulky subtitle-provider credentials (`settings-titulky-*`). Enable by adding `titulky` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct Titulky {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
    /// Only use approved subtitles.
    pub approved_only: Option<bool>,
    /// Skip subtitles with a mismatched framerate.
    pub skip_wrong_fps: Option<bool>,
}
