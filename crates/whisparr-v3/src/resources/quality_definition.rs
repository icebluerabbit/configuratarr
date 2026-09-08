use core_lib::Json;
use core_macros::resource;

/// `/api/v3/qualitydefinition` — per-quality-tier size limits.
///
/// Quality definitions are server-managed entries (one per quality tier); they cannot be created
/// or deleted via the API. Only `title`, `min_size`, `max_size`, and `preferred_size` are
/// user-configurable. Configure only the entries you want to adjust — unlisted tiers keep their
/// current server-side values.
#[resource(
    sync = crud,
    list = get("/api/v3/qualitydefinition"),
    update = put("/api/v3/qualitydefinition/${self.id}"),
)]
pub struct QualityDefinition {
    #[id]
    pub id: Option<i32>,
    /// Display name for this quality tier; used as the natural key to match against live entries.
    #[key]
    pub title: String,
    /// Server-assigned quality tier descriptor (id, name, source, resolution); read-only identity data.
    #[wire(read_only)]
    pub quality: Option<Json>,
    /// Server-managed sort order weight; read-only.
    #[wire(read_only)]
    pub weight: Option<i32>,
    /// Minimum acceptable size in MB for releases of this quality; `null` = no minimum.
    pub min_size: Option<f64>,
    /// Maximum acceptable size in MB for releases of this quality; `null` = no maximum.
    pub max_size: Option<f64>,
    /// Preferred size in MB for releases of this quality; used for scoring when multiple options exist.
    pub preferred_size: Option<f64>,
}
