use core_lib::SecretValue;
use core_macros::fields_blob;

/// Headphones import list — imports artists from a Headphones instance.
#[fields_blob(
    implementation = "HeadphonesImport",
    config_contract = "HeadphonesImportSettings"
)]
pub struct HeadphonesConfig {
    /// Base URL of the Headphones instance (e.g. `"http://headphones:8181"`).
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// API key for authenticating with the Headphones instance.
    #[wire(name = "apiKey")]
    pub api_key: Option<SecretValue>,
}
