//! Release profile — term-based filter applied to grabbed releases.
//!
//! Lidarr's release profile differs from Sonarr's in two key ways:
//! 1. There is **no `name` field** in the Lidarr v1 API — `indexer_id` is used
//!    as the natural key. Users should avoid creating two profiles with the same
//!    `indexer_id` (including the default `0` = all indexers), as they cannot be
//!    distinguished during diff.
//! 2. `required` and `ignored` are typed as `array<string>` in the spec; they
//!    are stored as [`Json`] because the engine does not implement `Described`
//!    for `Vec<String>` — the wire encoding is identical (null or a JSON array
//!    of strings).
//!
//! Setting `indexer_id` to `0` (the default) applies the profile to releases
//! from all indexers; a non-zero value restricts it to a specific indexer.
//! Tag-scoped profiles (via `tags`) apply only to artists that carry one of
//! the listed tags; an empty `tags` list means the profile applies globally.

use core_lib::Json;
use core_macros::resource;

/// Release profile — term-based acceptance and rejection filter for grabbed releases.
///
/// When `enabled`, Lidarr checks every candidate release title against the
/// `required` and `ignored` term lists before deciding to grab it:
/// - `required`: at least one term must appear in the release title.
/// - `ignored`: none of the terms may appear in the release title.
///
/// **Key note:** Lidarr's API does not include a `name` field on release profiles.
/// The `indexer_id` is used as the natural key — keep each value unique across
/// your profiles to enable reliable diff and idempotent apply.
#[resource(
    sync = crud,
    list = get("/api/v1/releaseprofile"),
    create = post("/api/v1/releaseprofile"),
    update = put("/api/v1/releaseprofile/${self.id}"),
    delete = delete("/api/v1/releaseprofile/${self.id}"),
)]
pub struct ReleaseProfile {
    /// Server-assigned profile id.
    #[id]
    pub id: Option<i32>,
    /// When `false`, this profile is saved but not applied to any grabs.
    pub enabled: bool,
    /// Terms that must appear in a release title for it to be accepted.
    /// In practice a JSON array of strings; `null` means no required-term constraint.
    pub required: Option<Json>,
    /// Terms that must **not** appear in a release title; releases containing
    /// any of these terms are rejected. In practice a JSON array of strings;
    /// `null` means no ignored-term constraint.
    pub ignored: Option<Json>,
    /// Natural key — id of the indexer this profile is restricted to; `0` means
    /// all indexers. Keep unique across profiles to ensure reliable diffing.
    #[key]
    #[default(0)]
    pub indexer_id: i32,
    /// Artist tag ids this profile applies to; resolved from
    /// `${ref.tag.<label>}` at apply. An empty list means the profile
    /// is applied globally to all artists.
    #[reference(tag)]
    pub tags: Vec<i32>,
}
