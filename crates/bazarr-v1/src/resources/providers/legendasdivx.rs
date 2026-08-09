use core_macros::nested;

/// LegendasDivx subtitle-provider credentials (`settings-legendasdivx-*`). Enable by adding `legendasdivx` to `general.enabled_providers`. Stored plaintext, so it diffs idempotently.
#[nested(case = snake)]
pub struct LegendasDivx {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
    /// Skip subtitles with a mismatched framerate.
    pub skip_wrong_fps: Option<bool>,
}
