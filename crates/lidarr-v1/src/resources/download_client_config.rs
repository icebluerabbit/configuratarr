use core_macros::resource;

/// `/api/v1/config/downloadclient` — download client handling settings.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/downloadclient"),
    update = put("/api/v1/config/downloadclient/${self.id}"),
)]
pub struct DownloadClientConfig {
    #[id]
    pub id: Option<i32>,
    /// Pipe-separated list of category or folder names that download clients use for in-progress downloads (e.g. `_UNPACK_|_FAILED_`).
    pub download_client_working_folders: Option<String>,
    /// Automatically imports completed downloads from the download client.
    #[default(true)]
    pub enable_completed_download_handling: bool,
    /// Automatically searches for a replacement release when a download fails.
    #[default(true)]
    pub auto_redownload_failed: bool,
    /// Automatically re-downloads a failed release that was found via interactive search.
    #[default(true)]
    pub auto_redownload_failed_from_interactive_search: bool,
}
