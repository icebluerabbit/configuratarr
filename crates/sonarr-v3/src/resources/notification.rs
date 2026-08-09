use core_macros::resource;

use crate::resources::notifications::NotificationProvider;
use crate::resources::provider::Provider;

/// A Sonarr notification connection — routes series/episode events to external services.
// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list = get("/api/v3/notification"),
    create = post("/api/v3/notification?forceSave=true"),
    update = put("/api/v3/notification/${self.id}?forceSave=true"),
    delete = delete("/api/v3/notification/${self.id}"),
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
    /// Fire notification when an episode file is imported after download.
    pub on_download: bool,
    /// Fire notification when a file is upgraded to a higher-quality version.
    pub on_upgrade: bool,
    /// Fire notification when an import is completed (post-download processing).
    pub on_import_complete: bool,
    /// Fire notification when episode files are renamed.
    pub on_rename: bool,
    /// Fire notification when a series is added to the Sonarr library.
    pub on_series_add: bool,
    /// Fire notification when a series is deleted from the library.
    pub on_series_delete: bool,
    /// Fire notification when an episode file is deleted.
    pub on_episode_file_delete: bool,
    /// Fire notification when an episode file is deleted to make room for an upgrade.
    pub on_episode_file_delete_for_upgrade: bool,
    /// Fire notification when a health-check issue is detected.
    pub on_health_issue: bool,
    /// Include health warnings (not just errors) in health-issue notifications.
    pub include_health_warnings: bool,
    /// Fire notification when a previously detected health-check issue is resolved.
    pub on_health_restored: bool,
    /// Fire notification when a Sonarr application update is available.
    pub on_application_update: bool,
    /// Fire notification when a download requires manual interaction.
    pub on_manual_interaction_required: bool,
    /// API flag — indicates this provider implementation supports on-grab events.
    #[wire(read_only)]
    pub supports_on_grab: bool,
    /// API flag — indicates this provider implementation supports on-download events.
    #[wire(read_only)]
    pub supports_on_download: bool,
    /// API flag — indicates this provider implementation supports on-upgrade events.
    #[wire(read_only)]
    pub supports_on_upgrade: bool,
    /// API flag — indicates this provider implementation supports on-import-complete events.
    #[wire(read_only)]
    pub supports_on_import_complete: bool,
    /// API flag — indicates this provider implementation supports on-rename events.
    #[wire(read_only)]
    pub supports_on_rename: bool,
    /// API flag — indicates this provider implementation supports on-series-add events.
    #[wire(read_only)]
    pub supports_on_series_add: bool,
    /// API flag — indicates this provider implementation supports on-series-delete events.
    #[wire(read_only)]
    pub supports_on_series_delete: bool,
    /// API flag — indicates this provider implementation supports on-episode-file-delete events.
    #[wire(read_only)]
    pub supports_on_episode_file_delete: bool,
    /// API flag — indicates this provider supports on-episode-file-delete-for-upgrade events.
    #[wire(read_only)]
    pub supports_on_episode_file_delete_for_upgrade: bool,
    /// API flag — indicates this provider implementation supports on-health-issue events.
    #[wire(read_only)]
    pub supports_on_health_issue: bool,
    /// API flag — indicates this provider implementation supports on-health-restored events.
    #[wire(read_only)]
    pub supports_on_health_restored: bool,
    /// API flag — indicates this provider implementation supports on-application-update events.
    #[wire(read_only)]
    pub supports_on_application_update: bool,
    /// API flag — indicates this provider supports on-manual-interaction-required events.
    #[wire(read_only)]
    pub supports_on_manual_interaction_required: bool,
    /// Documentation link for this notification provider (read-only).
    #[wire(read_only)]
    pub link: Option<String>,
    /// Command identifier used internally to trigger a test notification via the API.
    #[wire(read_only)]
    pub test_command: Option<String>,
}
