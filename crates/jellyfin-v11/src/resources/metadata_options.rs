use core_macros::nested;

/// Per-item-type metadata fetcher/saver ordering.
#[nested(case = pascal)]
pub struct MetadataOptions {
    /// Item Type
    pub item_type: Option<String>,
    /// Disabled Metadata Savers
    pub disabled_metadata_savers: Vec<String>,
    /// Local Metadata Reader Order
    pub local_metadata_reader_order: Vec<String>,
    /// Disabled Metadata Fetchers
    pub disabled_metadata_fetchers: Vec<String>,
    /// Metadata Fetcher Order
    pub metadata_fetcher_order: Vec<String>,
    /// Disabled Image Fetchers
    pub disabled_image_fetchers: Vec<String>,
    /// Image Fetcher Order
    pub image_fetcher_order: Vec<String>,
}
