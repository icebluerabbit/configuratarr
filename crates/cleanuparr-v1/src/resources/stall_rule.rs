use core_macros::{resource, wire_enum};

/// Privacy classification a torrent must belong to for a queue-cleaner rule
/// to apply to it. Shared by [`StallRule`] and
/// [`crate::resources::slow_rule::SlowRule`].
#[wire_enum]
pub enum TorrentPrivacyType {
    /// Applies only to torrents on public trackers.
    Public,
    /// Applies only to torrents on private trackers.
    Private,
    /// Applies to torrents on both public and private trackers.
    Both,
    /// Unknown or future privacy classification not yet modelled by this
    /// version.
    #[fallback]
    Unknown,
}

/// `/api/queue-rules/stall` — a rule that strikes, and eventually removes,
/// torrents whose download has made no progress for too long.
#[resource(
    sync = crud,
    list   = get("/api/queue-rules/stall"),
    create = post("/api/queue-rules/stall"),
    update = put("/api/queue-rules/stall/${self.id}"),
    delete = delete("/api/queue-rules/stall/${self.id}"),
)]
pub struct StallRule {
    /// Server-assigned id (GUID). Ignored on write — the path parameter
    /// identifies the rule being created or updated.
    #[id]
    pub id: Option<String>,
    /// Rule name. Must be unique among stall rules.
    #[key]
    pub name: String,
    /// Whether this rule is active.
    #[default(true)]
    pub enabled: bool,
    /// Number of times a torrent may be struck for stalling before it is
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
    /// Reset a torrent's strike count whenever it makes fresh download
    /// progress.
    #[default(true)]
    pub reset_strikes_on_progress: bool,
    /// Minimum progress delta (e.g. `"10MB"`) that counts as "made progress"
    /// for `reset_strikes_on_progress` purposes. `None` uses the server's
    /// own threshold.
    pub minimum_progress: Option<String>,
}
