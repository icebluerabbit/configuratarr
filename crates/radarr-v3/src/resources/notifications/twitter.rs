use core_lib::SecretValue;
use core_macros::fields_blob;

/// Twitter notification provider configuration.
#[fields_blob(implementation = "Twitter", config_contract = "TwitterSettings")]
pub struct TwitterConfig {
    /// Twitter application consumer key (API key).
    pub consumer_key: SecretValue,
    /// Twitter application consumer secret (API secret).
    pub consumer_secret: SecretValue,
    /// Twitter user OAuth access token.
    pub access_token: SecretValue,
    /// Twitter user OAuth access token secret.
    pub access_token_secret: SecretValue,
    /// Twitter username to mention in the notification tweet.
    pub mention: Option<String>,
    /// Send the notification as a direct message rather than a public tweet.
    pub direct_message: Option<bool>,
}
