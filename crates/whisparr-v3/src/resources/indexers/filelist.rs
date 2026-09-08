use core_lib::SecretValue;
use core_macros::fields_blob;

/// FileList private torrent tracker indexer.
#[fields_blob(
    implementation = "FileList",
    config_contract = "FileListSettings",
    protocol = "torrent"
)]
pub struct FileListConfig {
    /// FileList account username.
    pub username: String,
    /// FileList account passkey used for API authentication.
    pub passkey: Option<SecretValue>,
    /// Base URL of the FileList tracker.
    #[wire(name = "baseUrl")]
    pub base_url: Option<String>,
    /// FileList category IDs to include in searches.
    pub categories: Vec<i32>,
    /// Minimum number of seeders a torrent must have to be grabbed.
    #[wire(name = "minimumSeeders")]
    pub minimum_seeders: i32,
    /// Tracker-specific flag IDs that a release must carry to be grabbed.
    #[wire(name = "requiredFlags")]
    pub required_flags: Vec<i32>,
    /// Minimum seed ratio Whisparr must reach before stopping seeding.
    #[wire(name = "seedCriteria.seedRatio")]
    pub seed_ratio: Option<f64>,
    /// Minimum seeding time in minutes Whisparr must seed after download.
    #[wire(name = "seedCriteria.seedTime")]
    pub seed_time: Option<i32>,
    /// Reject grabs whose torrent hash is on the blocklist.
    #[wire(name = "rejectBlocklistedTorrentHashesWhileGrabbing")]
    pub reject_blocklisted_torrent_hashes_while_grabbing: bool,
    /// Language IDs to treat as multi-language releases.
    #[wire(name = "multiLanguages")]
    pub multi_languages: Vec<i32>,
    /// Download outcomes (e.g. executables, potentially dangerous files) that
    /// should be treated as a failed grab.
    #[wire(name = "failDownloads")]
    pub fail_downloads: Vec<i32>,
}
