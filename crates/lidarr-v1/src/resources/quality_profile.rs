//! Quality profile — ordered quality ladder and custom-format score gates.
//!
//! Lidarr's quality profile differs from Sonarr's: items do not carry per-tier
//! size limits, `Quality` is a simpler id+name pair (no source/resolution
//! fields), and there is no `minUpgradeFormatScore` field.

use core_macros::{nested, resource};

/// Quality metadata returned by the Lidarr catalogue.
///
/// Describes a single quality tier by id and name. Used as the `quality`
/// sub-object inside a `QualityProfileQualityItem`.
#[nested]
pub struct Quality {
    /// Server-assigned quality definition id.
    pub id: Option<i32>,
    /// Quality tier label, e.g. `"FLAC"` or `"MP3-320"`.
    pub name: Option<String>,
}

/// One rung in the quality ladder inside a `QualityProfile`.
///
/// Leaf items carry a `quality` sub-object and an empty `items` list. Group
/// items have a non-empty `items` list and a null `quality`. Lidarr walks the
/// ladder top-to-bottom when selecting an upgrade candidate.
#[nested]
pub struct QualityProfileQualityItem {
    /// Server-assigned item id (leaf items only; absent on group items).
    pub id: Option<i32>,
    /// Group or quality tier label displayed in the UI, e.g. `"Lossless"`.
    pub name: Option<String>,
    /// Quality definition for leaf items; `None` for group items.
    pub quality: Option<Quality>,
    /// Nested group members — empty for leaf items.
    pub items: Vec<QualityProfileQualityItem>,
    /// When `true`, Lidarr will accept releases at this quality tier.
    pub allowed: bool,
}

/// Custom-format score contribution inside a quality profile.
///
/// Each entry pairs a custom-format id with the number of points it adds to a
/// release's aggregate score. Negative scores penalise matching releases.
#[nested]
pub struct ProfileFormatItem {
    /// Server-assigned item id.
    pub id: Option<i32>,
    /// Id of the custom format being scored; resolved from
    /// `${ref.custom_format.<name>}` at apply.
    #[reference(custom_format)]
    pub format: i32,
    /// Custom-format name, mirrored from the format definition.
    pub name: Option<String>,
    /// Points awarded to a release matching this format; negative values
    /// penalise.
    pub score: Option<i32>,
}

/// Named quality profile — ordered quality ladder with format-score gates.
///
/// Lidarr evaluates profiles top-to-bottom: the first `allowed` quality (or
/// group) that a release matches determines whether it is grabbed and whether an
/// upgrade is triggered. Custom-format scores further filter grabs and
/// upgrades via `min_format_score` and `cutoff_format_score`.
#[resource(
    sync = crud,
    list = get("/api/v1/qualityprofile"),
    create = post("/api/v1/qualityprofile"),
    update = put("/api/v1/qualityprofile/${self.id}"),
    delete = delete("/api/v1/qualityprofile/${self.id}"),
)]
pub struct QualityProfile {
    /// Server-assigned profile id.
    #[id]
    pub id: Option<i32>,
    /// Natural key — referenced in `${ref.quality_profile.<name>}`.
    #[key]
    pub name: String,
    /// When `true`, Lidarr will seek a better-quality release after the initial
    /// download.
    pub upgrade_allowed: bool,
    /// Id of the cutoff quality tier; Lidarr will not seek upgrades past this
    /// point.
    pub cutoff: i32,
    /// Ordered quality ladder — all quality tiers and groups this profile
    /// considers.
    pub items: Vec<QualityProfileQualityItem>,
    /// Minimum aggregate custom-format score a release must reach to be grabbed.
    pub min_format_score: i32,
    /// Minimum format score that satisfies the upgrade cutoff.
    pub cutoff_format_score: i32,
    /// Custom-format score contributions attached to this profile.
    pub format_items: Vec<ProfileFormatItem>,
}
