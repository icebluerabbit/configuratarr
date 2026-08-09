//! Provider-resource template (outer). Composes the shared `Provider` envelope
//! (identity + tags + read-only metadata) with the typed `DownloadClientProvider`
//! variant, then adds the fields specific to download clients. Both composed
//! pieces use `#[flatten]`, so their keys sit top-level on the wire.

use core_macros::resource;

use crate::resources::download_clients::DownloadClientProvider;
use crate::resources::download_protocol::DownloadProtocol;
use crate::resources::provider::Provider;

// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list = get("/api/v3/downloadclient"),
    create = post("/api/v3/downloadclient?forceSave=true"),
    update = put("/api/v3/downloadclient/${self.id}?forceSave=true"),
    delete = delete("/api/v3/downloadclient/${self.id}"),
)]
pub struct DownloadClient {
    /// Identity (id + name), tag refs, read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: DownloadClientProvider,
    /// Whether this download client is active.
    pub enable: bool,
    /// Download protocol used by this client (torrent or usenet).
    pub protocol: DownloadProtocol,
    /// Client priority relative to other configured download clients.
    #[default(1)]
    pub priority: i32,
    /// Remove downloads from the client once Radarr has imported them.
    #[default(true)]
    pub remove_completed_downloads: bool,
    /// Remove downloads from the client if they fail to complete.
    pub remove_failed_downloads: bool,
}
