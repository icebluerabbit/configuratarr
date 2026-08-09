use core_macros::fields_blob;

/// Synology Indexer notification provider configuration.
///
/// Triggers a Synology NAS media indexer refresh after track imports or renames.
#[fields_blob(
    implementation = "SynologyIndexer",
    config_contract = "SynologyIndexerSettings"
)]
pub struct SynologyIndexerConfig {
    /// Trigger a Synology library update after a track is imported.
    pub update_library: Option<bool>,
}
