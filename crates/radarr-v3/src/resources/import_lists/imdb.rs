use core_macros::fields_blob;

/// IMDb import list — imports movies from an IMDb list.
#[fields_blob(
    implementation = "IMDbListImport",
    config_contract = "IMDbListSettings"
)]
pub struct ImdbConfig {
    /// IMDb list identifier (e.g. `"ls012345678"`).
    #[wire(name = "listId")]
    pub list_id: Option<String>,
}
