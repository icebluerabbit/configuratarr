use core_macros::resource;

use crate::resources::stall_rule::TorrentPrivacyType;

/// `/api/queue-rules/slow` — a rule that strikes, and eventually removes,
/// torrents whose download speed drops too low for too long.
#[resource(
    sync = crud,
    list   = get("/api/queue-rules/slow"),
    create = post("/api/queue-rules/slow"),
    update = put("/api/queue-rules/slow/${self.id}"),
    delete = delete("/api/queue-rules/slow/${self.id}"),
)]
pub struct SlowRule {
    /// Server-assigned id (GUID). Ignored on write — the path parameter
    /// identifies the rule being created or updated.
    #[id]
    pub id: Option<String>,
    /// Rule name. Must be unique among slow rules.
    #[key]
    pub name: String,
    /// Whether this rule is active.
    #[default(true)]
    pub enabled: bool,
    /// Number of times a torrent may be struck for being slow before it is
    /// removed.
    #[default(3)]
    pub max_strikes: i32,
    /// Restrict this rule to torrents of a given privacy classification.
    /// Omitted, the server treats it as `Public`.
    pub privacy_type: Option<TorrentPrivacyType>,
    /// Minimum download completion percentage a torrent must reach before
    /// this rule starts evaluating it.
    #[default(0)]
    pub min_completion_percentage: i32,
    /// Maximum download completion percentage this rule applies to; a
    /// torrent past this point is left alone. The API has no server default
    /// for this field — omitting it binds `0`, which then fails the server's
    /// own `[Range(1,100)]` validation, so it is modelled non-optional with a
    /// working default rather than `Option`.
    #[default(100)]
    pub max_completion_percentage: i32,
    /// Delete a private torrent from the download client (not just the
    /// queue) once this rule strikes it out.
    #[default(false)]
    pub delete_private_torrents_from_client: bool,
    /// Move the torrent to a different category once this rule strikes it
    /// out.
    #[default(false)]
    pub change_category: bool,
    /// Reset a torrent's strike count whenever its speed recovers.
    #[default(true)]
    pub reset_strikes_on_progress: bool,
    /// Minimum download speed (e.g. `"10MB"`) below which a torrent is
    /// considered slow.
    #[default("")]
    pub min_speed: String,
    /// Number of hours a torrent may run below `min_speed` before it is
    /// struck.
    #[default(0.0)]
    pub max_time_hours: f64,
    /// Torrents above this size (e.g. `"5GB"`) are exempt from this rule.
    /// `None` applies the rule regardless of torrent size.
    pub ignore_above_size: Option<String>,
    /// Skip slow-speed evaluation while the download client's alternative
    /// speed limits are active.
    #[default(true)]
    pub ignore_while_alt_speed_active: bool,
}
