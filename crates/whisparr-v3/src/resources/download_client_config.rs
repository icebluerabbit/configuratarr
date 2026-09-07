use core_macros::resource;

/// `/api/v3/config/downloadclient` — download client handling settings.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/downloadclient"),
    update = put("/api/v3/config/downloadclient/${self.id}"),
)]
pub struct DownloadClientConfig {
    #[id]
    pub id: Option<i32>,
    /// Pipe-separated list of category or folder names that download clients use for in-progress downloads (e.g. `_UNPACK_|_FAILED_`).
    pub download_client_working_folders: Option<String>,
    /// Automatically imports completed downloads from the download client.
    #[default(true)]
    pub enable_completed_download_handling: bool,
    /// Interval in minutes between checks for finished downloads when completed download handling is enabled.
    #[default(1)]
    pub check_for_finished_download_interval: i32,
    /// Automatically searches for a replacement release when a download fails.
    #[default(true)]
    pub auto_redownload_failed: bool,
    /// Automatically re-downloads a failed release that was found via interactive search.
    #[default(true)]
    pub auto_redownload_failed_from_interactive_search: bool,
}
