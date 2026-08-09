use core_macros::fields_blob;

/// Lidarr Lists import list — imports artists from the Lidarr community lists service.
#[fields_blob(
    implementation = "LidarrLists",
    config_contract = "LidarrListsSettings"
)]
pub struct LidarrListsConfig {
    /// Identifier of the Lidarr Lists list to import from.
    #[wire(name = "listId")]
    pub list_id: Option<String>,
}
