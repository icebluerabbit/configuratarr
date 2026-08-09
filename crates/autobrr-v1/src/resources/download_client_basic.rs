use core_macros::nested;

use core_lib::SecretValue;

/// HTTP basic-auth credentials for a download client's web endpoint.
#[nested(case = snake)]
pub struct DownloadClientBasic {
    /// Whether basic auth is required.
    pub auth: Option<bool>,
    /// Basic-auth username.
    pub username: Option<String>,
    /// Basic-auth password.
    pub password: Option<SecretValue>,
}
