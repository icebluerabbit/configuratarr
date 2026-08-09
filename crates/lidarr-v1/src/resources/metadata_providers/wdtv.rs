use core_macros::fields_blob;

/// WDTV metadata plugin — writes track-level metadata alongside media.
#[fields_blob(
    implementation = "WdtvMetadata",
    config_contract = "WdtvMetadataSettings"
)]
pub struct WdtvConfig {
    /// Write track-level metadata files.
    #[wire(name = "trackMetadata")]
    pub track_metadata: Option<bool>,
}
