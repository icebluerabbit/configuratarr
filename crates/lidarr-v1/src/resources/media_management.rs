use core_macros::resource;

/// `/api/v1/config/mediamanagement` — file handling and media management settings.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/mediamanagement"),
    update = put("/api/v1/config/mediamanagement/${self.id}"),
)]
pub struct MediaManagement {
    #[id]
    pub id: Option<i32>,
    /// Automatically unmonitors a track after its file has been downloaded.
    pub auto_unmonitor_previously_downloaded_tracks: bool,
    /// Path to the recycle bin folder for deleted track files; empty disables the recycle bin.
    pub recycle_bin: Option<String>,
    /// Number of days before files in the recycle bin are permanently deleted; 0 disables automatic cleanup.
    #[default(7)]
    pub recycle_bin_cleanup_days: i32,
    /// Whether to download proper/repack releases: `preferAndUpgrade`, `doNotUpgrade`, or `doNotPrefer`.
    #[default("doNotPrefer")]
    pub download_propers_and_repacks: String,
    /// Creates a folder for an artist even before any file has been downloaded.
    pub create_empty_artist_folders: bool,
    /// Removes empty artist/album folders after a file is deleted or moved.
    pub delete_empty_folders: bool,
    /// Sets the file modification date to the album release date: `none` or `albumReleaseDate`.
    #[default("none")]
    pub file_date: String,
    /// Re-scans the artist folder for changes on disk: `always`, `afterManual`, or `never`.
    pub watch_library_for_changes: bool,
    /// When to rescan the artist folder after a library refresh: `always`, `afterManual`, or `never`.
    #[default("always")]
    pub rescan_after_refresh: String,
    /// Controls audio fingerprinting for track matching: `never`, `newFiles`, or `allFiles`.
    #[default("never")]
    pub allow_fingerprinting: String,
    /// Sets file and folder permissions on imported files (Linux/macOS only).
    pub set_permissions_linux: bool,
    /// Octal permission bits applied to imported track folders (e.g. `755`); requires `set_permissions_linux`.
    pub chmod_folder: Option<String>,
    /// Group name or GID to chown imported files and folders to; requires `set_permissions_linux`.
    pub chown_group: Option<String>,
    /// Skips the available disk space check before importing a track file.
    pub skip_free_space_check_when_importing: bool,
    /// Minimum free disk space in MB required on the destination before Lidarr will import.
    #[default(100)]
    pub minimum_free_space_when_importing: i32,
    /// Uses hardlinks instead of copying when source and destination are on the same filesystem.
    #[default(true)]
    pub copy_using_hardlinks: bool,
    /// Reads and stores media info (codec, bitrate, audio channels) for imported files.
    #[default(true)]
    pub enable_media_info: bool,
    /// Delegates file import handling to an external script instead of the built-in importer.
    pub use_script_import: bool,
    /// Absolute path to the script used for custom imports; required when `use_script_import` is true.
    pub script_import_path: Option<String>,
    /// Imports extra files (artwork, lyrics, etc.) alongside the track file.
    pub import_extra_files: bool,
    /// Comma-separated list of file extensions to import alongside the track file (e.g. `jpg,png,lrc`).
    pub extra_file_extensions: Option<String>,
}
