use core_macros::fields_blob;

/// Synology Indexer notification provider configuration.
/// This provider has no configuration fields — it uses the Synology DS File notification system.
#[fields_blob(
    implementation = "SynologyIndexer",
    config_contract = "SynologyIndexerSettings"
)]
pub struct SynologyIndexerConfig {
    /// Trigger a Synology media library update after a movie is imported.
    pub update_library: Option<bool>,
}
