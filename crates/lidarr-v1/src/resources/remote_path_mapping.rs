//! Remote path mapping resource — maps a path as seen by a remote download
//! client host to the corresponding local path visible to Lidarr.

use core_macros::resource;

/// A remote-to-local path mapping for a download client host.
///
/// Lidarr uses these to translate paths returned by download clients that run
/// on a different host (or container) where filesystem paths differ.
#[resource(
    sync = crud,
    list = get("/api/v1/remotepathmapping"),
    create = post("/api/v1/remotepathmapping"),
    update = put("/api/v1/remotepathmapping/${self.id}"),
    delete = delete("/api/v1/remotepathmapping/${self.id}"),
)]
pub struct RemotePathMapping {
    #[id]
    pub id: Option<i32>,
    /// Hostname or IP of the download client that uses the remote path.
    pub host: String,
    /// Natural key — the path as the remote download client reports it.
    #[key]
    pub remote_path: String,
    /// The local filesystem path that corresponds to `remote_path`.
    pub local_path: String,
}
