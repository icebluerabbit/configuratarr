//! Custom filter resource — a user-defined saved filter for a Whisparr UI
//! page (movie index, movie file list, etc.).
//!
//! The `filters` array holds raw filter condition objects (`{key, value, type}`)
//! whose shape is opaque in the static OpenAPI spec, so they are stored as
//! [`Json`].
//!
//! Note: the wire field name `type` is a Rust keyword; the field is renamed to
//! `filter_type` with `#[wire(name = "type")]`.

use core_lib::Json;
use core_macros::resource;

/// A saved custom filter for a Whisparr UI page.
#[resource(
    sync = crud,
    list = get("/api/v3/customfilter"),
    create = post("/api/v3/customfilter"),
    update = put("/api/v3/customfilter/${self.id}"),
    delete = delete("/api/v3/customfilter/${self.id}"),
)]
pub struct CustomFilter {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the user-visible label for this filter.
    #[key]
    pub label: String,
    /// The UI page context this filter applies to (e.g. `MovieIndex`,
    /// `MovieFile`). Wire name is `type` (a Rust keyword).
    #[wire(name = "type")]
    pub filter_type: Option<String>,
    /// Filter conditions, each a raw object with `key`, `value`, and `type`.
    /// Raw JSON — the condition shape is not described in the static spec.
    pub filters: Vec<Json>,
}
