use core_macros::fields_blob;

/// TMDb List import list — imports movies from a specific TMDb list.
#[fields_blob(
    implementation = "TMDbListImport",
    config_contract = "TMDbListSettings"
)]
pub struct TmdbListConfig {
    /// TMDb list identifier to import movies from.
    #[wire(name = "listId")]
    pub list_id: Option<String>,
}
