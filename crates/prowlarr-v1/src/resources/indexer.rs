//! Provider-shaped resource: indexer. Composes the shared `Provider` envelope
//! with `RawProvider` for the dynamic Cardigann config blob, plus
//! indexer-specific fields.
//!
//! Unlike *arr indexers (Sonarr/Radarr), Prowlarr indexers use dynamic
//! Cardigann definitions with no fixed set of typed implementations. The config
//! is always stored as the raw `fields` array via `RawProvider` — there is no
//! typed variant enum.

use core_lib::Json;
use core_macros::resource;

use crate::resources::download_protocol::DownloadProtocol;
use crate::resources::indexer_privacy::IndexerPrivacy;
use crate::resources::provider::Provider;
use crate::resources::raw_provider::RawProvider;

/// Indexer definition — connects Prowlarr to a usenet or torrent search source.
///
/// Create/update use `?forceSave=true`: Prowlarr otherwise runs a live
/// connectivity test against the indexer on save and rejects (HTTP 400) when the
/// site is unreachable *from the host running configuratarr* — but the tracker
/// may only be reachable through Prowlarr's own proxy/FlareSolverr, or be
/// transiently down. A declarative sync must converge to the desired config
/// regardless; Prowlarr still surfaces the failing health check afterward.
#[resource(
    sync = crud,
    list = get("/api/v1/indexer"),
    create = post("/api/v1/indexer?forceSave=true"),
    update = put("/api/v1/indexer/${self.id}?forceSave=true"),
    delete = delete("/api/v1/indexer/${self.id}"),
)]
pub struct Indexer {
    /// Identity (id + name), tag refs, and read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// Raw Cardigann config: implementation discriminators and the `fields` array.
    #[flatten]
    pub config: RawProvider,

    // --- writable fields ---
    /// Whether this indexer is active and included in searches.
    pub enable: bool,
    /// Whether Prowlarr should follow redirects when querying this indexer.
    pub redirect: bool,
    /// Indexer priority; lower values are preferred when multiple indexers match.
    #[default(25)]
    pub priority: i32,
    /// Transport protocol used by this indexer.
    pub protocol: DownloadProtocol,
    /// Privacy level of this indexer (public, semi-private, or private).
    pub privacy: IndexerPrivacy,
    /// Language of content indexed by this indexer.
    pub language: Option<String>,
    /// Application profile controlling how this indexer is synced to applications.
    #[reference(app_profile)]
    pub app_profile_id: Option<i32>,
    /// Download client to use exclusively for grabs from this indexer; absent
    /// means use the default client.
    #[reference(download_client)]
    pub download_client_id: Option<i32>,

    // --- read-only fields ---
    /// Base URLs for this indexer definition (API-reported).
    #[wire(read_only)]
    pub indexer_urls: Vec<String>,
    /// Legacy/deprecated URLs for this indexer definition (API-reported).
    #[wire(read_only)]
    pub legacy_urls: Vec<String>,
    /// Cardigann definition name for this indexer (API-reported).
    #[wire(read_only)]
    pub definition_name: Option<String>,
    /// Human-readable description of this indexer (API-reported).
    #[wire(read_only)]
    pub description: Option<String>,
    /// Character encoding used by this indexer (API-reported).
    #[wire(read_only)]
    pub encoding: Option<String>,
    /// Whether the implementation supports RSS feeds (API-reported).
    #[wire(read_only)]
    pub supports_rss: bool,
    /// Whether the implementation supports search queries (API-reported).
    #[wire(read_only)]
    pub supports_search: bool,
    /// Whether the implementation supports redirect-based grabs (API-reported).
    #[wire(read_only)]
    pub supports_redirect: bool,
    /// Whether the implementation supports paginated results (API-reported).
    #[wire(read_only)]
    pub supports_pagination: bool,
    /// Indexer capability metadata (categories, search modes). Opaque — API-reported.
    #[wire(read_only)]
    pub capabilities: Option<Json>,
    /// ISO 8601 timestamp when this indexer was added to Prowlarr (API-reported).
    #[wire(read_only)]
    pub added: Option<String>,
    /// Indexer health and status information. Opaque — API-reported.
    #[wire(read_only)]
    pub status: Option<Json>,
    /// Sortable display name derived from the indexer name (API-reported).
    #[wire(read_only)]
    pub sort_name: Option<String>,
}
