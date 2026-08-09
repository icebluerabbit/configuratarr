//! Quality profile — ordered quality ladder and custom-format score gates.
//!
//! Sonarr's quality profile differs from Radarr's: items carry optional
//! per-tier size limits (`min_size` / `max_size` / `preferred_size`), quality
//! source is a proper enum instead of a free string, there is no `modifier`
//! field on `Quality`, and the profile itself has no embedded `language` field
//! (language preference lives in a separate `LanguageProfile`).

use core_macros::{nested, resource, wire_enum};

/// Source medium for a quality tier.
///
/// Determines the acquisition method, e.g. web stream vs. Blu-ray disc vs. TV
/// capture. Sonarr uses this to classify qualities like HDTV-1080p, WEBDL-720p,
/// and Bluray-1080p into their respective groups.
#[wire_enum(rename_all = "camelCase")]
pub enum QualitySource {
    /// Television broadcast capture.
    Television,
    /// Raw (unprocessed) television broadcast capture.
    TelevisionRaw,
    /// Web stream download.
    Web,
    /// Web stream rip (re-encoded from a web source).
    WebRip,
    /// DVD source.
    Dvd,
    /// Blu-ray disc source.
    Bluray,
    /// Raw (remux) Blu-ray disc source.
    BlurayRaw,
    /// Unknown source, or a source not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// Quality metadata returned by the Sonarr catalogue.
///
/// Describes a single quality tier: its vertical resolution and the source
/// medium that produced it. Used as the `quality` sub-object inside a
/// `QualityProfileItem`.
#[nested]
pub struct Quality {
    /// Server-assigned quality definition id.
    pub id: Option<i32>,
    /// Quality tier label, e.g. `"HDTV-1080p"`.
    pub name: Option<String>,
    /// Acquisition source medium.
    pub source: Option<QualitySource>,
    /// Vertical pixel resolution, e.g. `1080`.
    pub resolution: Option<i32>,
}

/// One rung in the quality ladder inside a `QualityProfile`.
///
/// Leaf items carry a `quality` sub-object and an empty `items` list. Group
/// items have a non-empty `items` list and a null `quality`. Sonarr walks the
/// ladder top-to-bottom when selecting an upgrade candidate.
///
/// The optional `min_size`, `max_size`, and `preferred_size` fields constrain
/// the acceptable file-size range (in megabytes) for a given tier. When `null`
/// Sonarr applies its global defaults.
#[nested]
pub struct QualityProfileItem {
    /// Server-assigned item id (leaf items only; absent on group items).
    pub id: Option<i32>,
    /// Group or quality tier label displayed in the UI, e.g. `"WEB 1080p"`.
    pub name: Option<String>,
    /// Quality definition for leaf items; `None` for group items.
    pub quality: Option<Quality>,
    /// Nested group members — empty for leaf items.
    pub items: Vec<QualityProfileItem>,
    /// When `true`, Sonarr will accept releases at this quality tier.
    pub allowed: bool,
    /// Minimum acceptable file size in megabytes; `None` uses the global default.
    pub min_size: Option<f64>,
    /// Maximum acceptable file size in megabytes; `None` uses the global default.
    pub max_size: Option<f64>,
    /// Preferred file size in megabytes; `None` uses the global default.
    pub preferred_size: Option<f64>,
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
/// Sonarr evaluates profiles top-to-bottom: the first `allowed` quality (or
/// group) that a release matches determines whether it is grabbed and whether an
/// upgrade is triggered. Custom-format scores further filter grabs and
/// upgrades via `min_format_score` and `cutoff_format_score`.
///
/// Unlike Radarr, Sonarr quality profiles do **not** embed a language
/// constraint — language preference is managed separately via `LanguageProfile`.
#[resource(
    sync = crud,
    list = get("/api/v3/qualityprofile"),
    create = post("/api/v3/qualityprofile"),
    update = put("/api/v3/qualityprofile/${self.id}"),
    delete = delete("/api/v3/qualityprofile/${self.id}"),
)]
pub struct QualityProfile {
    /// Server-assigned profile id.
    #[id]
    pub id: Option<i32>,
    /// Natural key — referenced in `${ref.quality_profile.<name>}`.
    #[key]
    pub name: String,
    /// When `true`, Sonarr will seek a better-quality release after the initial
    /// download.
    pub upgrade_allowed: bool,
    /// Id of the cutoff quality tier; Sonarr will not seek upgrades past this
    /// point.
    pub cutoff: i32,
    /// Ordered quality ladder — all quality tiers and groups this profile
    /// considers.
    pub items: Vec<QualityProfileItem>,
    /// Minimum aggregate custom-format score a release must reach to be grabbed.
    pub min_format_score: i32,
    /// Minimum format score that satisfies the upgrade cutoff.
    pub cutoff_format_score: i32,
    /// Minimum improvement in custom-format score required to trigger an upgrade.
    pub min_upgrade_format_score: i32,
    /// Custom-format score contributions attached to this profile.
    pub format_items: Vec<ProfileFormatItem>,
}
