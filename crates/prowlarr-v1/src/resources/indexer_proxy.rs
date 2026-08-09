use core_macros::resource;

use crate::resources::indexer_proxies::IndexerProxyProvider;
use crate::resources::provider::Provider;

/// A Prowlarr indexer proxy — routes requests for specific indexers through an
/// intermediate proxy (FlareSolverr, HTTP CONNECT, SOCKS4, or SOCKS5) to
/// bypass bot-protection or access geo-restricted indexers.
// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list = get("/api/v1/indexerproxy"),
    create = post("/api/v1/indexerproxy?forceSave=true"),
    update = put("/api/v1/indexerproxy/${self.id}?forceSave=true"),
    delete = delete("/api/v1/indexerproxy/${self.id}"),
)]
pub struct IndexerProxy {
    /// Identity (id + name), tag refs, and read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: IndexerProxyProvider,
    /// Fire notification/action when a health-check issue is detected.
    pub on_health_issue: bool,
    /// Include health warnings (not just errors) in health notifications.
    pub include_health_warnings: bool,
    /// API flag — indicates whether this proxy supports on-health-issue events (read-only).
    #[wire(read_only)]
    pub supports_on_health_issue: bool,
    /// Documentation link for this proxy implementation (read-only).
    #[wire(read_only)]
    pub link: Option<String>,
    /// Command identifier used internally to trigger a proxy test via the API (read-only).
    #[wire(read_only)]
    pub test_command: Option<String>,
}
