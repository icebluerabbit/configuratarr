use core_macros::resource;

use crate::resources::notifications::NotificationProvider;
use crate::resources::provider::Provider;

/// A Prowlarr notification connection — routes indexer/health/update events to external services.
// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list = get("/api/v1/notification"),
    create = post("/api/v1/notification?forceSave=true"),
    update = put("/api/v1/notification/${self.id}?forceSave=true"),
    delete = delete("/api/v1/notification/${self.id}"),
)]
pub struct Notification {
    /// Identity (id + name), tag refs, read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: NotificationProvider,
    /// Fire notification when a release is grabbed for download.
    pub on_grab: bool,
    /// Fire notification when a health-check issue is detected.
    pub on_health_issue: bool,
    /// Fire notification when a previously detected health-check issue is resolved.
    pub on_health_restored: bool,
    /// Fire notification when a Prowlarr application update is available.
    pub on_application_update: bool,
    /// Include grabs triggered manually (not via an automated search).
    pub include_manual_grabs: bool,
    /// Include health warnings (not just errors) in health-issue notifications.
    pub include_health_warnings: bool,
    /// API flag — indicates this provider implementation supports on-grab events.
    #[wire(read_only)]
    pub supports_on_grab: bool,
    /// API flag — indicates this provider implementation supports on-health-issue events.
    #[wire(read_only)]
    pub supports_on_health_issue: bool,
    /// API flag — indicates this provider implementation supports on-health-restored events.
    #[wire(read_only)]
    pub supports_on_health_restored: bool,
    /// API flag — indicates this provider implementation supports on-application-update events.
    #[wire(read_only)]
    pub supports_on_application_update: bool,
    /// Documentation link for this notification provider (read-only).
    #[wire(read_only)]
    pub link: Option<String>,
    /// Command identifier used internally to trigger a test notification via the API.
    #[wire(read_only)]
    pub test_command: Option<String>,
}
