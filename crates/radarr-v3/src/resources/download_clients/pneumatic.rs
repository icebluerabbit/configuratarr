use core_macros::fields_blob;

/// Pneumatic usenet blackhole client settings.
///
/// Pneumatic watches a folder for NZB files and passes them to an external
/// downloader via a `.strm` sidecar.
#[fields_blob(
    implementation = "Pneumatic",
    config_contract = "PneumaticSettings",
    protocol = "usenet"
)]
pub struct PneumaticConfig {
    /// Folder Radarr drops NZB files into for Pneumatic to pick up.
    #[wire(name = "nzbFolder")]
    pub nzb_folder: Option<String>,
    /// Folder Pneumatic writes `.strm` stream files to.
    #[wire(name = "strmFolder")]
    pub strm_folder: Option<String>,
}
