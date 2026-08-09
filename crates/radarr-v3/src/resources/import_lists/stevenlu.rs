use core_macros::fields_blob;

/// StevenLu import list — imports from the StevenLu popular movies list.
#[fields_blob(
    implementation = "StevenLuImport",
    config_contract = "StevenLuSettings"
)]
pub struct StevenLuConfig {
    /// URL of the StevenLu popular movies list feed.
    pub link: Option<String>,
}
