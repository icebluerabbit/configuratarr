use core_macros::fields_blob;

/// Usenet Black Hole (folder-drop) download client settings.
#[fields_blob(
    implementation = "UsenetBlackhole",
    config_contract = "UsenetBlackholeSettings",
    protocol = "usenet"
)]
pub struct UsenetBlackholeConfig {
    /// Directory to drop NZB files into for processing.
    #[wire(name = "nzbFolder")]
    pub nzb_folder: Option<String>,
}
