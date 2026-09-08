use core_lib::SecretValue;
use core_macros::fields_blob;

/// Webhook notification provider configuration.
//
// `headers` (a `KeyValueList` of `{key, value}` objects) is deliberately not
// modelled — same as radarr's variant. It has no scalar carrier in the fields
// blob and no declarative use yet; an existing definition's headers survive a
// sparse update because `merge` keeps live-only `fields[]` entries by name.
#[fields_blob(implementation = "Webhook", config_contract = "WebhookSettings")]
pub struct WebhookConfig {
    /// Webhook endpoint URL that receives the HTTP request.
    pub url: String,
    /// HTTP method to use: 1 = POST, 2 = PUT.
    pub method: i32,
    /// HTTP basic-auth username sent with the request.
    pub username: Option<String>,
    /// HTTP basic-auth password sent with the request.
    pub password: Option<SecretValue>,
}
