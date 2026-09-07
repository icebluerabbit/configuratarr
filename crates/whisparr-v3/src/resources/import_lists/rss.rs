use core_macros::fields_blob;

/// RSS import list — imports scenes from an RSS feed.
#[fields_blob(implementation = "RSSImport", config_contract = "RSSImportSettings")]
pub struct RssConfig {
    /// URL of the RSS feed to import from.
    pub link: Option<String>,
}
