//! Provider-resource: indexer. Composes the shared `Provider` envelope with the
//! typed `IndexerProvider` variant, plus indexer-specific fields. Both composed
//! pieces use `#[flatten]`, so their keys sit top-level on the wire.

use core_macros::resource;

use crate::resources::download_protocol::DownloadProtocol;
use crate::resources::indexers::IndexerProvider;
use crate::resources::provider::Provider;

/// Indexer definition — connects Radarr to a usenet or torrent search source.
// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list = get("/api/v3/indexer"),
    create = post("/api/v3/indexer?forceSave=true"),
    update = put("/api/v3/indexer/${self.id}?forceSave=true"),
    delete = delete("/api/v3/indexer/${self.id}"),
)]
pub struct Indexer {
    /// Identity (id + name), tag refs, read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: IndexerProvider,
    /// Whether to include this indexer in RSS sync feeds.
    pub enable_rss: bool,
    /// Whether to use this indexer for automatic (monitored) searches.
    pub enable_automatic_search: bool,
    /// Whether to use this indexer for interactive (manual) searches.
    pub enable_interactive_search: bool,
    /// Whether the implementation supports RSS (API-reported).
    #[wire(read_only)]
    pub supports_rss: bool,
    /// Whether the implementation supports search (API-reported).
    #[wire(read_only)]
    pub supports_search: bool,
    /// Transport protocol used by this indexer (usenet or torrent).
    pub protocol: DownloadProtocol,
    /// Indexer priority; lower values are preferred when multiple indexers match a grab.
    #[default(25)]
    pub priority: i32,
    /// Download client to use exclusively for grabs from this indexer; absent
    /// means use the default.
    #[reference(download_client)]
    pub download_client_id: Option<i32>,
}
