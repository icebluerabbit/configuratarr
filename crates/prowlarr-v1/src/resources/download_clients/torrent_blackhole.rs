use core_macros::fields_blob;

/// Torrent Black Hole (folder-drop) download client settings.
#[fields_blob(
    implementation = "TorrentBlackhole",
    config_contract = "TorrentBlackholeSettings",
    protocol = "torrent"
)]
pub struct TorrentBlackholeConfig {
    /// Directory to drop `.torrent` files into for processing.
    #[wire(name = "torrentFolder")]
    pub torrent_folder: Option<String>,
    /// File extension used for saved magnet link files (default: `.magnet`).
    #[wire(name = "magnetFileExtension")]
    pub magnet_file_extension: Option<String>,
    /// Save magnet links to files instead of launching them directly.
    #[wire(name = "saveMagnetFiles")]
    pub save_magnet_files: Option<bool>,
}
