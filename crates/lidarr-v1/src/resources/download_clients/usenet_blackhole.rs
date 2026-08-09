use core_macros::fields_blob;

/// Usenet blackhole client settings (folder-based, no direct client API).
#[fields_blob(
    implementation = "UsenetBlackhole",
    config_contract = "UsenetBlackholeSettings",
    protocol = "usenet"
)]
pub struct UsenetBlackholeConfig {
    /// Folder Lidarr drops NZB files into for an external client to pick up.
    #[wire(name = "nzbFolder")]
    pub nzb_folder: Option<String>,
    /// Folder Lidarr watches for completed downloads from the external client.
    #[wire(name = "watchFolder")]
    pub watch_folder: Option<String>,
}
