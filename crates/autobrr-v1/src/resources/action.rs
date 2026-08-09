use core_macros::nested;

/// An action a filter runs on a matched release — push to a download client,
/// call a webhook, or execute a command.
#[nested(case = snake)]
pub struct Action {
    /// Server-assigned id.
    pub id: Option<i32>,
    /// Display name.
    pub name: Option<String>,
    /// Action kind: `QBITTORRENT`, `DELUGE_V2`, `WEBHOOK`, `EXEC`, `TEST`, …
    #[wire(name = "type")]
    pub action_type: Option<String>,
    /// Whether the action is active.
    pub enabled: Option<bool>,
    /// Category to file the release under (client actions).
    pub category: Option<String>,
    /// Tags to apply (client actions).
    pub tags: Option<String>,
    /// Label to apply (Deluge/qBittorrent).
    pub label: Option<String>,
    /// Save path override (client actions).
    pub save_path: Option<String>,
    /// Download path override (client actions).
    pub download_path: Option<String>,
    /// Add the torrent in a paused state.
    pub paused: Option<bool>,
    /// Ignore the client's own throughput rules for this push.
    pub ignore_rules: Option<bool>,
    /// Prioritise the first and last pieces (streaming).
    pub first_last_piece_prio: Option<bool>,
    /// Skip the client's hash check on add.
    pub skip_hash_check: Option<bool>,
    /// Torrent content layout: `ORIGINAL`, `SUBFOLDER_CREATE`, `SUBFOLDER_NONE`.
    pub content_layout: Option<String>,
    /// Upload speed limit, KiB/s.
    pub limit_upload_speed: Option<i64>,
    /// Download speed limit, KiB/s.
    pub limit_download_speed: Option<i64>,
    /// Seeding ratio limit.
    pub limit_ratio: Option<f64>,
    /// Seeding time limit, minutes.
    pub limit_seed_time: Option<i64>,
    /// Queue priority: `max` / `min` (qBittorrent).
    pub priority: Option<String>,
    /// Skip reannounce handling.
    pub reannounce_skip: Option<bool>,
    /// Delete the torrent if reannounce never succeeds.
    pub reannounce_delete: Option<bool>,
    /// Reannounce interval, seconds.
    pub reannounce_interval: Option<i32>,
    /// Max reannounce attempts.
    pub reannounce_max_attempts: Option<i32>,
    /// Download client this action pushes to (`${ref.download_client.<name>}`).
    #[reference(download_client)]
    pub client_id: Option<i32>,
    /// Secondary client to delegate the push to (`${ref.download_client.<name>}`).
    #[reference(download_client)]
    pub external_download_client_id: Option<i32>,
    /// Secondary client name (free-form, where not referenced by id).
    pub external_download_client: Option<String>,
    /// Webhook URL (`WEBHOOK` type).
    pub webhook_host: Option<String>,
    /// HTTP method for the webhook.
    pub webhook_method: Option<String>,
    /// Webhook payload content type.
    pub webhook_type: Option<String>,
    /// Request body sent to the webhook.
    pub webhook_data: Option<String>,
    /// Extra webhook headers (`Key: value`).
    pub webhook_headers: Vec<String>,
    /// Command to run (`EXEC` type).
    pub exec_cmd: Option<String>,
    /// Arguments passed to the command.
    pub exec_args: Option<String>,
    /// Folder to watch for `.torrent` files (`WATCH_FOLDER` type).
    pub watch_folder: Option<String>,
}
