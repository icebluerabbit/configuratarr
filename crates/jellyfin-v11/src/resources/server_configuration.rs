use crate::resources::cast_receiver_application::CastReceiverApplication;
use crate::resources::metadata_options::MetadataOptions;
use crate::resources::name_value_pair::NameValuePair;
use crate::resources::path_substitution::PathSubstitution;
use crate::resources::repository_info::RepositoryInfo;
use crate::resources::trickplay_options::TrickplayOptions;
use core_macros::resource;

/// `/System/Configuration` — core server settings.
#[resource(
    sync = singleton,
    case = pascal,
    read = get("/System/Configuration"),
    update = post("/System/Configuration"),
)]
pub struct ServerConfiguration {
    /// Log File Retention Days
    pub log_file_retention_days: i32,
    /// Is Startup Wizard Completed
    pub is_startup_wizard_completed: bool,
    /// Cache Path
    pub cache_path: Option<String>,
    /// Previous Version
    pub previous_version: Option<String>,
    /// Previous Version Str
    pub previous_version_str: Option<String>,
    /// Enable Metrics
    pub enable_metrics: bool,
    /// Enable Normalized Item By Name Ids
    pub enable_normalized_item_by_name_ids: bool,
    /// Is Port Authorized
    pub is_port_authorized: bool,
    /// Quick Connect Available
    pub quick_connect_available: bool,
    /// Enable Case Sensitive Item Ids
    pub enable_case_sensitive_item_ids: bool,
    /// Disable Live Tv Channel User Data Name
    pub disable_live_tv_channel_user_data_name: bool,
    /// Metadata Path
    pub metadata_path: Option<String>,
    /// Preferred Metadata Language
    pub preferred_metadata_language: Option<String>,
    /// Metadata Country Code
    pub metadata_country_code: Option<String>,
    /// Sort Replace Characters
    pub sort_replace_characters: Vec<String>,
    /// Sort Remove Characters
    pub sort_remove_characters: Vec<String>,
    /// Sort Remove Words
    pub sort_remove_words: Vec<String>,
    /// Min Resume Pct
    pub min_resume_pct: i32,
    /// Max Resume Pct
    pub max_resume_pct: i32,
    /// Min Resume Duration Seconds
    pub min_resume_duration_seconds: i32,
    /// Min Audiobook Resume
    pub min_audiobook_resume: i32,
    /// Max Audiobook Resume
    pub max_audiobook_resume: i32,
    /// Inactive Session Threshold
    pub inactive_session_threshold: i32,
    /// Library Monitor Delay
    pub library_monitor_delay: i32,
    /// Library Update Duration
    pub library_update_duration: i32,
    /// Cache Size
    pub cache_size: i32,
    /// Image Saving Convention. One of: `Legacy`, `Compatible`.
    pub image_saving_convention: Option<String>,
    /// Metadata Options
    pub metadata_options: Vec<MetadataOptions>,
    /// Skip Deserialization For Basic Types
    pub skip_deserialization_for_basic_types: bool,
    /// Server Name
    pub server_name: Option<String>,
    /// UI Culture
    pub u_i_culture: Option<String>,
    /// Save Metadata Hidden
    pub save_metadata_hidden: bool,
    /// Content Types
    pub content_types: Vec<NameValuePair>,
    /// Remote Client Bitrate Limit
    pub remote_client_bitrate_limit: i32,
    /// Enable Folder View
    pub enable_folder_view: bool,
    /// Enable Grouping Movies Into Collections
    pub enable_grouping_movies_into_collections: bool,
    /// Enable Grouping Shows Into Collections
    pub enable_grouping_shows_into_collections: bool,
    /// Display Specials Within Seasons
    pub display_specials_within_seasons: bool,
    /// Codecs Used
    pub codecs_used: Vec<String>,
    /// Plugin Repositories
    pub plugin_repositories: Vec<RepositoryInfo>,
    /// Enable External Content In Suggestions
    pub enable_external_content_in_suggestions: bool,
    /// Image Extraction Timeout Ms
    pub image_extraction_timeout_ms: i32,
    /// Path Substitutions
    pub path_substitutions: Vec<PathSubstitution>,
    /// Enable Slow Response Warning
    pub enable_slow_response_warning: bool,
    /// Slow Response Threshold Ms
    pub slow_response_threshold_ms: i64,
    /// Cors Hosts
    pub cors_hosts: Vec<String>,
    /// Activity Log Retention Days
    pub activity_log_retention_days: i32,
    /// Library Scan Fanout Concurrency
    pub library_scan_fanout_concurrency: i32,
    /// Library Metadata Refresh Concurrency
    pub library_metadata_refresh_concurrency: i32,
    /// Allow Client Log Upload
    pub allow_client_log_upload: bool,
    /// Dummy Chapter Duration
    pub dummy_chapter_duration: i32,
    /// Chapter Image Resolution. One of: `MatchSource`, `P144`, `P240`, `P360`, `P480`, `P720`, `P1080`, `P1440`, `P2160`.
    pub chapter_image_resolution: Option<String>,
    /// Parallel Image Encoding Limit
    pub parallel_image_encoding_limit: i32,
    /// Cast Receiver Applications
    pub cast_receiver_applications: Vec<CastReceiverApplication>,
    /// Trickplay Options
    pub trickplay_options: Option<TrickplayOptions>,
    /// Enable Legacy Authorization
    pub enable_legacy_authorization: bool,
}
