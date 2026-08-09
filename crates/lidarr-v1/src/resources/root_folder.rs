//! Root folder resource — a filesystem path Lidarr watches for artists.
//!
//! Lidarr's root folder differs substantially from Sonarr's: it carries default
//! metadata profile, quality profile, monitor option, new-item monitor option,
//! and tag references that apply to artists added under this root folder. It also
//! supports a `PUT` update endpoint (Sonarr's root folder does not). The
//! read-only fields `accessible`, `free_space`, and `total_space` are
//! server-computed and never written.

use core_macros::{resource, wire_enum};

/// Monitor strategy applied when adding an artist to Lidarr.
#[wire_enum(rename_all = "lowercase")]
pub enum MonitorTypes {
    /// Monitor all albums.
    All,
    /// Monitor future albums only.
    Future,
    /// Monitor missing albums.
    Missing,
    /// Monitor existing albums only.
    Existing,
    /// Monitor the latest album.
    Latest,
    /// Monitor the first album.
    First,
    /// Do not monitor any albums.
    None,
    /// Unknown monitor strategy.
    #[fallback]
    Unknown,
}

/// Monitor strategy applied when a new album is added to an already-monitored artist.
#[wire_enum(rename_all = "lowercase")]
pub enum NewItemMonitorTypes {
    /// Monitor all new albums.
    All,
    /// Do not monitor new albums.
    None,
    /// Monitor only albums marked as "new".
    New,
    /// Unknown new-item monitor strategy.
    #[fallback]
    Unknown,
}

/// A root folder Lidarr watches for artists.
///
/// Root folders carry default profile and monitoring settings applied to artists
/// added under this path. The `path` field is the natural key used to match
/// desired entries against live ones.
#[resource(
    sync = crud,
    list = get("/api/v1/rootfolder"),
    create = post("/api/v1/rootfolder"),
    update = put("/api/v1/rootfolder/${self.id}"),
    delete = delete("/api/v1/rootfolder/${self.id}"),
)]
pub struct RootFolder {
    #[id]
    pub id: Option<i32>,
    /// Optional display name for the root folder.
    pub name: Option<String>,
    /// Natural key — the absolute filesystem path Lidarr watches.
    #[key]
    pub path: String,
    /// Id of the metadata profile applied by default to new artists; resolved
    /// from `${ref.metadata_profile.<name>}` at apply.
    #[reference(metadata_profile)]
    pub default_metadata_profile_id: i32,
    /// Id of the quality profile applied by default to new artists; resolved
    /// from `${ref.quality_profile.<name>}` at apply.
    #[reference(quality_profile)]
    pub default_quality_profile_id: i32,
    /// Monitor strategy applied when adding a new artist under this root folder.
    pub default_monitor_option: MonitorTypes,
    /// Monitor strategy applied when a new album is added to a monitored artist.
    pub default_new_item_monitor_option: NewItemMonitorTypes,
    /// Tag ids applied by default to new artists added under this root folder;
    /// resolved from `${ref.tag.<label>}` at apply.
    #[reference(tag)]
    pub default_tags: Vec<i32>,
    /// Whether Lidarr can currently access the folder; server-computed.
    #[wire(read_only)]
    pub accessible: Option<bool>,
    /// Available disk space in bytes; server-computed.
    #[wire(read_only)]
    pub free_space: Option<i64>,
    /// Total disk space in bytes; server-computed.
    #[wire(read_only)]
    pub total_space: Option<i64>,
}
