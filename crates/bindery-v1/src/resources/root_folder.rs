//! Root folder — a filesystem path Bindery watches for content.
//!
//! `sync = crud`, keyed by `path`. **No update endpoint**: the API exposes only
//! `POST /api/v1/rootfolder` (create) and `DELETE /api/v1/rootfolder/{id}` — no
//! `PUT`. That is only legal because `path` is both the natural key *and* the
//! only field `CreateRootFolderRequest` accepts; `id`/`freeSpace`/`createdAt`
//! are `#[wire(read_only)]` so a merge can never see a difference on any other
//! field and try to reach for a nonexistent update endpoint.

use core_macros::resource;

/// A root folder Bindery watches for content.
#[resource(
    sync = crud,
    list = get("/api/v1/rootfolder"),
    create = post("/api/v1/rootfolder"),
    delete = delete("/api/v1/rootfolder/${self.id}"),
)]
pub struct RootFolder {
    /// Server-assigned id. Read-only.
    #[id]
    pub id: Option<i64>,
    /// Natural key — the absolute path on the Bindery server (or container)
    /// filesystem. Must already exist, be accessible, and be a directory.
    #[key]
    pub path: String,
    /// Available bytes on the filesystem hosting `path`, reported by the API.
    /// Read-only.
    #[wire(read_only)]
    pub free_space: Option<i64>,
    /// Row creation time, reported by the API. Read-only.
    #[wire(read_only)]
    pub created_at: Option<String>,
}
