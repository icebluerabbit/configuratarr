use core_macros::{nested, resource};

/// Read-only metadata about a folder that Radarr knows about but has no movie mapped to.
#[nested]
pub struct UnmappedFolder {
    /// Folder name as reported by Radarr.
    pub name: Option<String>,
    /// Absolute filesystem path of the unmapped folder.
    pub path: Option<String>,
    /// Path relative to the root folder.
    pub relative_path: Option<String>,
}

/// A root folder Radarr watches for movies.
#[resource(
    sync = crud,
    list = get("/api/v3/rootfolder"),
    create = post("/api/v3/rootfolder"),
    update = put("/api/v3/rootfolder/${self.id}"),
    delete = delete("/api/v3/rootfolder/${self.id}"),
)]
pub struct RootFolder {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the absolute filesystem path.
    #[key]
    pub path: String,
    /// Whether Radarr can currently access the folder, reported by the API.
    #[wire(read_only)]
    pub accessible: Option<bool>,
    /// Available disk space in bytes, reported by the API.
    #[wire(read_only)]
    pub free_space: Option<i64>,
    /// Subdirectories Radarr found that have no movie mapped to them.
    #[wire(read_only)]
    pub unmapped_folders: Vec<UnmappedFolder>,
}
