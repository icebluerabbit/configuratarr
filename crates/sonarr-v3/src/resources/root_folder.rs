//! Root folder resource — a filesystem path Sonarr watches for series.
//!
//! **No update endpoint**: the Sonarr v3 API exposes only `POST` (create) and
//! `DELETE` for root folders; there is no `PUT /api/v3/rootfolder/{id}`. The
//! `update` endpoint is therefore omitted from the resource declaration. The
//! engine will create and delete, but never issue a PUT for this resource.

use core_macros::{nested, resource};

/// Read-only metadata about a subdirectory Sonarr found that has no series
/// mapped to it.
#[nested]
pub struct UnmappedFolder {
    /// Folder name as reported by Sonarr.
    pub name: Option<String>,
    /// Absolute filesystem path of the unmapped folder.
    pub path: Option<String>,
    /// Path relative to the root folder.
    pub relative_path: Option<String>,
}

/// A root folder Sonarr watches for series.
#[resource(
    sync = crud,
    list = get("/api/v3/rootfolder"),
    create = post("/api/v3/rootfolder"),
    delete = delete("/api/v3/rootfolder/${self.id}"),
)]
pub struct RootFolder {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the absolute filesystem path Sonarr watches.
    #[key]
    pub path: String,
    /// Whether Sonarr can currently access the folder, reported by the API.
    #[wire(read_only)]
    pub accessible: Option<bool>,
    /// Available disk space in bytes, reported by the API.
    #[wire(read_only)]
    pub free_space: Option<i64>,
    /// Subdirectories Sonarr found that have no series mapped to them.
    #[wire(read_only)]
    pub unmapped_folders: Vec<UnmappedFolder>,
}
