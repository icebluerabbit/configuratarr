//! Release profile — term-based filter applied to grabbed releases.
//!
//! Sonarr-only concept (Radarr does not have release profiles). A release
//! profile lets you whitelist or blacklist releases by requiring or ignoring
//! specific terms (e.g. `"PROPER"`, `"REPACK"`, codec tags) anywhere in the
//! release title.
//!
//! The `required` and `ignored` fields are typed as `nullable` with no
//! element-type constraint in the OpenAPI spec. In practice Sonarr uses them
//! as arrays of strings; `Option<Vec<String>>` covers both the populated and
//! `null` (empty) states the API may return.

use core_lib::Json;
use core_macros::resource;

/// Release profile — term-based acceptance and rejection filter for grabbed releases.
///
/// When `enabled`, Sonarr checks every candidate release title against the
/// `required` and `ignored` term lists before deciding to grab it:
/// - `required`: at least one term must appear in the release title.
/// - `ignored`: none of the terms may appear in the release title.
///
/// Setting `indexer_id` to `0` (the default) applies the profile to releases
/// from all indexers; a non-zero value restricts it to a specific indexer.
/// Tag-scoped profiles (via `tags`) apply only to series that carry one of
/// the listed tags; an empty `tags` list means the profile applies globally.
#[resource(
    sync = crud,
    list = get("/api/v3/releaseprofile"),
    create = post("/api/v3/releaseprofile"),
    update = put("/api/v3/releaseprofile/${self.id}"),
    delete = delete("/api/v3/releaseprofile/${self.id}"),
)]
pub struct ReleaseProfile {
    /// Server-assigned profile id.
    #[id]
    pub id: Option<i32>,
    /// Natural key — referenced in `${ref.release_profile.<name>}`.
    #[key]
    pub name: String,
    /// When `false`, this profile is saved but not applied to any grabs.
    pub enabled: bool,
    /// Terms that must appear in a release title for it to be accepted.
    /// Untyped in the spec; in practice an array of term strings. `None` means
    /// no required-term constraint.
    pub required: Option<Json>,
    /// Terms that must **not** appear in a release title; releases containing
    /// any of these terms are rejected. Untyped in the spec; in practice an
    /// array of term strings. `None` means no ignored-term constraint.
    pub ignored: Option<Json>,
    /// Id of the indexer this profile is restricted to; `0` means all indexers.
    #[default(0)]
    pub indexer_id: i32,
    /// Series tag ids this profile applies to; resolved from
    /// `${ref.tag.<label>}` at apply. An empty list means the profile
    /// is applied globally to all series.
    #[reference(tag)]
    pub tags: Vec<i32>,
}
