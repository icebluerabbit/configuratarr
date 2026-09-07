use core_lib::SecretValue;
use core_macros::fields_blob;

/// Notifiarr notification provider configuration.
#[fields_blob(implementation = "Notifiarr", config_contract = "NotifiarrSettings")]
pub struct NotifiarrConfig {
    /// Notifiarr API key for authentication.
    ///
    /// The eros C# property is `APIKey`, which the *arr field-name derivation
    /// lower-cases only in the first position — the wire key is `aPIKey`, not
    /// `apiKey`.
    #[wire(name = "aPIKey")]
    pub api_key: SecretValue,
}
