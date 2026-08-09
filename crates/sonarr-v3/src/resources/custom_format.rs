//! Custom format resource — a named set of specification conditions that Sonarr
//! evaluates to award a quality score to a release.
//!
//! Specifications are provider-shaped on the wire (`{implementation, negate,
//! required, fields:[{name, value}]}`) and modelled as raw [`Json`], mirroring
//! [`super::auto_tag`]. The *arr `fields` array is implementation-defined, so
//! the static spec can't validate per-spec field names — typing each variant
//! would need struct-variant + fields-blob composition for zero validation gain.

use core_lib::Json;
use core_macros::resource;

/// A custom format — a named collection of specification conditions Sonarr uses
/// to score releases. The score influences download decisions via quality profiles.
#[resource(
    sync = crud,
    list = get("/api/v3/customformat"),
    create = post("/api/v3/customformat"),
    update = put("/api/v3/customformat/${self.id}"),
    delete = delete("/api/v3/customformat/${self.id}"),
)]
pub struct CustomFormat {
    #[id]
    pub id: Option<i32>,
    /// Natural key — referenced in `${ref.custom_format.<name>}`.
    #[key]
    pub name: String,
    /// When true, the format name is included in Sonarr's file rename template.
    pub include_custom_format_when_renaming: Option<bool>,
    /// Specification conditions, each a provider-shaped object
    /// (`implementation` + `fields[]`). Raw JSON — see the module docs.
    pub specifications: Vec<Json>,
}
