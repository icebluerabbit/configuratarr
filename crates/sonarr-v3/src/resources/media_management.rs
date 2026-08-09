use core_macros::resource;

/// `/api/v3/config/mediamanagement` — file handling and media management settings.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/mediamanagement"),
    update = put("/api/v3/config/mediamanagement/${self.id}"),
)]
pub struct MediaManagement {
    #[id]
    pub id: Option<i32>,
    /// Automatically unmonitors an episode after its file has been downloaded.
    pub auto_unmonitor_previously_downloaded_episodes: bool,
    /// Path to the recycle bin folder for deleted episode files; empty disables the recycle bin.
    pub recycle_bin: Option<String>,
    /// Number of days before files in the recycle bin are permanently deleted; 0 disables automatic cleanup.
    #[default(7)]
    pub recycle_bin_cleanup_days: i32,
    /// Whether to download proper/repack releases: `preferAndUpgrade`, `doNotUpgrade`, or `doNotPrefer`.
    #[default("doNotPrefer")]
    pub download_propers_and_repacks: String,
    /// Creates a folder for a series even before its file has been downloaded.
    pub create_empty_series_folders: bool,
    /// Removes empty series/season folders after a file is deleted or moved.
    pub delete_empty_folders: bool,
    /// Sets the file modification date to the episode air date: `none`, `localAirDate`, or `utcAirDate`.
    #[default("none")]
    pub file_date: String,
    /// When to rescan the series folder after a library refresh: `always`, `afterManual`, or `never`.
    #[default("always")]
    pub rescan_after_refresh: String,
    /// Sets file and folder permissions on imported files (Linux/macOS only).
    pub set_permissions_linux: bool,
    /// Octal permission bits applied to imported episode folders (e.g. `755`); requires `set_permissions_linux`.
    pub chmod_folder: Option<String>,
    /// Group name or GID to chown imported files and folders to; requires `set_permissions_linux`.
    pub chown_group: Option<String>,
    /// When an episode title is required to import: `always`, `bulkSeasonReleases`, or `never`.
    #[default("always")]
    pub episode_title_required: String,
    /// Skips the available disk space check before importing an episode file.
    pub skip_free_space_check_when_importing: bool,
    /// Minimum free disk space in MB required on the destination before Sonarr will import.
    #[default(100)]
    pub minimum_free_space_when_importing: i32,
    /// Uses hardlinks instead of copying when source and destination are on the same filesystem.
    #[default(true)]
    pub copy_using_hardlinks: bool,
    /// Delegates file import handling to an external script instead of the built-in importer.
    pub use_script_import: bool,
    /// Absolute path to the script used for custom imports; required when `use_script_import` is true.
    pub script_import_path: Option<String>,
    /// Imports extra files (subtitles, NFO, etc.) alongside the episode file.
    pub import_extra_files: bool,
    /// Comma-separated list of file extensions to import alongside the episode file (e.g. `srt,nfo`).
    pub extra_file_extensions: Option<String>,
    /// Reads and stores media info (codec, resolution, audio channels) for imported files.
    #[default(true)]
    pub enable_media_info: bool,
}
