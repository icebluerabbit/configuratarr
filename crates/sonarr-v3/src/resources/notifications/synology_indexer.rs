use core_macros::fields_blob;

/// Synology Indexer notification provider configuration.
///
/// Triggers Synology DSM media indexing after episode files are imported or renamed.
#[fields_blob(
    implementation = "SynologyIndexer",
    config_contract = "SynologyIndexerSettings"
)]
pub struct SynologyIndexerConfig {
    /// Trigger a Synology media library update after an episode is imported.
    pub update_library: Option<bool>,
}
