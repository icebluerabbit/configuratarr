use core_macros::nested;

/// One *arr instance's proactive-search settings, embedded in
/// [`crate::resources::seeker::Seeker`] under `instances`. The API upserts
/// each entry by `arr_instance_id`; instances the config omits keep whatever
/// settings are already stored for them.
#[nested]
pub struct SeekerInstance {
    /// The Sonarr or Radarr instance these settings apply to.
    // Both targets are declared because each one is a dependency edge, and that
    // is the only thing ordering the seeker after the instance collections —
    // apply order never reads the `${ref}` text itself.
    #[reference(sonarr_instance, radarr_instance)]
    pub arr_instance_id: Option<String>,
    /// Whether proactive search runs for this instance.
    #[default(true)]
    pub enabled: bool,
    /// *arr tag ids to exclude from search.
    pub skip_tags: Vec<String>,
    /// Skip a proactive search cycle when this many items are already
    /// downloading. `0` disables the limit.
    #[default(3)]
    pub active_download_limit: i32,
    /// Minimum number of days between proactive-search cycles for the same
    /// item.
    #[default(7)]
    pub min_cycle_time_days: i32,
    /// Only proactively search monitored items.
    #[default(true)]
    pub monitored_only: bool,
    /// Search up to the quality cutoff instead of stopping once an item has
    /// any acceptable file.
    #[default(false)]
    pub use_cutoff: bool,
    /// Weigh search candidates by custom format score.
    #[default(false)]
    pub use_custom_format_score: bool,
}
