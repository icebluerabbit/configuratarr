use core_macros::nested;

/// Anti-Captcha CAPTCHA-solver credentials (`settings-anticaptcha-*`), used by providers that hit CAPTCHAs.
#[nested(case = snake)]
pub struct AntiCaptcha {
    /// Anti-Captcha API key.
    pub anti_captcha_key: Option<String>,
}
