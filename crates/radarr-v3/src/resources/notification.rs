use core_macros::resource;

use crate::resources::notifications::NotificationProvider;
use crate::resources::provider::Provider;

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
    /// Fire notification when a movie file is imported after download.
    pub on_download: bool,
    /// Fire notification when a file is upgraded to a higher-quality version.
    pub on_upgrade: bool,
    /// Fire notification when a movie file is renamed.
    pub on_rename: bool,
    /// Fire notification when a movie is added to the Radarr library.
    pub on_movie_added: bool,
    /// Fire notification when a movie is deleted from the library.
    pub on_movie_delete: bool,
    /// Fire notification when a movie file is deleted.
    pub on_movie_file_delete: bool,
    /// Fire notification when a file is deleted to make room for an upgrade.
    pub on_movie_file_delete_for_upgrade: bool,
    /// Fire notification when a health-check issue is detected.
    pub on_health_issue: bool,
    /// Fire notification when a previously detected health-check issue is resolved.
    pub on_health_restored: bool,
    /// Fire notification when a Radarr application update is available.
    pub on_application_update: bool,
    /// Fire notification when a download requires manual interaction.
    pub on_manual_interaction_required: bool,
    /// Include health warnings (not just errors) in health-issue notifications.
    pub include_health_warnings: bool,
    /// API flag — indicates this provider implementation supports on-grab events.
    #[wire(read_only)]
    pub supports_on_grab: bool,
    /// API flag — indicates this provider implementation supports on-download events.
    #[wire(read_only)]
    pub supports_on_download: bool,
    /// API flag — indicates this provider implementation supports on-upgrade events.
    #[wire(read_only)]
    pub supports_on_upgrade: bool,
    /// API flag — indicates this provider implementation supports on-rename events.
    #[wire(read_only)]
    pub supports_on_rename: bool,
    /// API flag — indicates this provider implementation supports on-movie-added events.
    #[wire(read_only)]
    pub supports_on_movie_added: bool,
    /// API flag — indicates this provider implementation supports on-movie-delete events.
    #[wire(read_only)]
    pub supports_on_movie_delete: bool,
    /// API flag — indicates this provider implementation supports on-movie-file-delete events.
    #[wire(read_only)]
    pub supports_on_movie_file_delete: bool,
    /// API flag — indicates this provider implementation supports on-movie-file-delete-for-upgrade events.
    #[wire(read_only)]
    pub supports_on_movie_file_delete_for_upgrade: bool,
    /// API flag — indicates this provider implementation supports on-health-issue events.
    #[wire(read_only)]
    pub supports_on_health_issue: bool,
    /// API flag — indicates this provider implementation supports on-health-restored events.
    #[wire(read_only)]
    pub supports_on_health_restored: bool,
    /// API flag — indicates this provider implementation supports on-application-update events.
    #[wire(read_only)]
    pub supports_on_application_update: bool,
    /// API flag — indicates this provider implementation supports on-manual-interaction-required events.
    #[wire(read_only)]
    pub supports_on_manual_interaction_required: bool,
    /// Command identifier used internally to trigger a test notification via the API.
    #[wire(read_only)]
    pub test_command: Option<String>,
}
