use core_macros::nested;

/// Death by Captcha CAPTCHA-solver credentials (`settings-deathbycaptcha-*`), used by providers that hit CAPTCHAs.
#[nested(case = snake)]
pub struct DeathByCaptcha {
    /// Account username.
    pub username: Option<String>,
    /// Account password.
    pub password: Option<String>,
}
