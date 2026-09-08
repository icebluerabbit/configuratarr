use core_macros::fields_blob;

/// Synology Indexer notification provider configuration — notifies the Synology
/// media indexer so DSM picks up newly imported files.
#[fields_blob(
    implementation = "SynologyIndexer",
    config_contract = "SynologyIndexerSettings"
)]
pub struct SynologyIndexerConfig {
    /// Trigger a Synology media library update after a scene is imported.
    #[wire(name = "updateLibrary")]
    pub update_library: Option<bool>,
}
