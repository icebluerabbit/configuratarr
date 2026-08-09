use core_macros::fields_blob;

/// Torrent blackhole client settings (folder-based, no direct client API).
#[fields_blob(
    implementation = "TorrentBlackhole",
    config_contract = "TorrentBlackholeSettings",
    protocol = "torrent"
)]
pub struct TorrentBlackholeConfig {
    /// Folder Lidarr drops `.torrent` files into for an external client to pick up.
    #[wire(name = "torrentFolder")]
    pub torrent_folder: Option<String>,
    /// Folder Lidarr watches for completed downloads from the external client.
    #[wire(name = "watchFolder")]
    pub watch_folder: Option<String>,
    /// File extension used when saving magnet links as files (e.g. `.magnet`).
    #[wire(name = "magnetFileExtension")]
    pub magnet_file_extension: Option<String>,
    /// Save magnet links as files in the torrent folder instead of ignoring them.
    #[wire(name = "saveMagnetFiles")]
    pub save_magnet_files: Option<bool>,
    /// Do not move or delete files from the watch folder after import (read-only mode).
    #[wire(name = "readOnly")]
    pub read_only: Option<bool>,
}
