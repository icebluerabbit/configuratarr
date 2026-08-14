use core_macros::nested;

use core_lib::SecretValue;

/// Auth for a download client's web endpoint. Supersedes the deprecated
/// [`DownloadClientBasic`](crate::resources::download_client_basic::DownloadClientBasic);
/// autobrr translates a set `basic` block into this on write.
#[nested(case = snake)]
pub struct DownloadClientAuth {
    /// Whether auth is required.
    pub enabled: Option<bool>,
    /// Auth scheme: `NONE`, `BASIC_AUTH`, or `DIGEST_AUTH`.
    #[wire(name = "type")]
    pub auth_type: Option<String>,
    /// Auth username.
    pub username: Option<String>,
    /// Auth password.
    pub password: Option<SecretValue>,
}
