//! Quality profile — ordered quality ladder and custom-format score gates.

use core_macros::{nested, resource};

/// Quality metadata returned by the Radarr catalogue.
#[nested]
pub struct Quality {
    pub id: Option<i32>,
    /// Quality tier name, e.g. `"Bluray-2160p"`.
    pub name: Option<String>,
    /// Source medium string, e.g. `"bluray"`, `"webdl"`.
    pub source: Option<String>,
    /// Vertical pixel resolution for this quality tier, e.g. `2160`.
    pub resolution: Option<i32>,
    /// Modifier string, e.g. `"none"`, `"remux"`.
    pub modifier: Option<String>,
}

/// One rung in the quality ladder.
///
/// Leaf items carry a `quality` sub-object and an empty `items` list. Group
/// items have a non-empty `items` list and a null `quality`. Radarr walks the
/// ladder top-to-bottom when selecting an upgrade candidate.
#[nested]
pub struct QualityProfileItem {
    pub id: Option<i32>,
    /// Group or quality tier label displayed in the UI, e.g. `"HD Bluray"`.
    pub name: Option<String>,
    /// Quality definition for leaf items; `None` for group items.
    pub quality: Option<Quality>,
    /// Nested group members — empty for leaf items.
    pub items: Vec<QualityProfileItem>,
    /// When `true`, Radarr will accept releases at this quality tier.
    pub allowed: bool,
}

/// Custom-format score contribution inside a quality profile.
#[nested]
pub struct ProfileFormatItem {
    pub id: Option<i32>,
    /// Custom-format id — resolved from `${ref.custom_format.<name>}` at apply.
    #[reference(custom_format)]
    pub format: i32,
    /// Custom-format name, mirrored from the format definition.
    pub name: Option<String>,
    /// Points awarded to a release matching this format; negative values penalise.
    pub score: Option<i32>,
}

/// Language constraint applied to the profile.
#[nested]
pub struct Language {
    pub id: i32,
    /// Language name, e.g. `"English"`.
    pub name: Option<String>,
}

/// Named quality profile — ordered quality ladder with format-score gates.
#[resource(
    sync = crud,
    list = get("/api/v3/qualityprofile"),
    create = post("/api/v3/qualityprofile"),
    update = put("/api/v3/qualityprofile/${self.id}"),
    delete = delete("/api/v3/qualityprofile/${self.id}"),
)]
pub struct QualityProfile {
    #[id]
    pub id: Option<i32>,
    /// Natural key — referenced in `${ref.quality_profile.<name>}`.
    #[key]
    pub name: String,
    /// When `true`, Radarr will seek a better-quality release after the initial download.
    pub upgrade_allowed: bool,
    /// Id of the cutoff quality; Radarr will not seek upgrades past this point.
    pub cutoff: i32,
    /// Ordered quality ladder — all quality tiers and groups this profile considers.
    pub items: Vec<QualityProfileItem>,
    /// Minimum aggregate custom-format score a release must reach to be grabbed.
    pub min_format_score: i32,
    /// Minimum format score that satisfies the upgrade cutoff.
    pub cutoff_format_score: i32,
    /// Minimum improvement in custom-format score required to trigger an upgrade.
    pub min_upgrade_format_score: i32,
    /// Custom-format score contributions attached to this profile.
    pub format_items: Vec<ProfileFormatItem>,
    /// Language requirement for grabbed releases.
    pub language: Option<Language>,
}
