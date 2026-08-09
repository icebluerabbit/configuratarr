use core_macros::fields_blob;

/// IMDb import list — imports series from a public IMDb list.
#[fields_blob(
    implementation = "ImdbListImport",
    config_contract = "ImdbListSettings"
)]
pub struct ImdbConfig {
    /// IMDb list identifier (e.g. `"ls012345678"`).
    #[wire(name = "listId")]
    pub list_id: Option<String>,
}
