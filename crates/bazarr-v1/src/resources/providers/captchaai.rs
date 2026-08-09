use core_macros::nested;

/// CaptchaAI CAPTCHA-solver credentials (`settings-captchaai-*`), used by providers that hit CAPTCHAs.
#[nested(case = snake)]
pub struct CaptchaAi {
    /// CaptchaAI API key.
    pub captchaai_key: Option<String>,
}
