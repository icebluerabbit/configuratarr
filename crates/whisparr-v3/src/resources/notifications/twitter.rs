use core_lib::SecretValue;
use core_macros::fields_blob;

/// Twitter notification provider configuration.
#[fields_blob(implementation = "Twitter", config_contract = "TwitterSettings")]
pub struct TwitterConfig {
    /// Twitter application consumer key (API key).
    #[wire(name = "consumerKey")]
    pub consumer_key: SecretValue,
    /// Twitter application consumer secret (API secret).
    #[wire(name = "consumerSecret")]
    pub consumer_secret: SecretValue,
    /// Twitter user OAuth access token.
    #[wire(name = "accessToken")]
    pub access_token: SecretValue,
    /// Twitter user OAuth access token secret.
    #[wire(name = "accessTokenSecret")]
    pub access_token_secret: SecretValue,
    /// Twitter username to mention in the notification tweet.
    pub mention: Option<String>,
    /// Send the notification as a direct message rather than a public tweet.
    #[wire(name = "directMessage")]
    pub direct_message: Option<bool>,
    /// OAuth sign-in marker used by the UI to start the Twitter authorisation
    /// flow (default `startOAuth`). Must be empty until the access token pair
    /// is set.
    #[wire(name = "authorizeNotification")]
    pub authorize_notification: Option<String>,
}
