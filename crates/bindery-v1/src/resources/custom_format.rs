//! Custom format — a named set of release-matching conditions (the
//! TRaSH-style scoring/filtering primitive). Conditions are persisted
//! server-side as a JSON blob in a single `TEXT` column. There is no owner
//! column: custom formats are shared deployment config.

use core_macros::resource;

use crate::resources::custom_condition::CustomCondition;

/// A named set of release-matching conditions used to score/filter releases.
#[resource(
    sync = crud,
    list = get("/api/v1/customformat"),
    create = post("/api/v1/customformat"),
    update = put("/api/v1/customformat/${self.id}"),
    delete = delete("/api/v1/customformat/${self.id}"),
)]
pub struct CustomFormat {
    /// Server-assigned row id (SQLite `AUTOINCREMENT`). Ignored on input — on
    /// create the assigned id is written back, on update the path id wins.
    #[id]
    pub id: Option<i64>,
    /// Natural key — referenced in `${ref.custom_format.<name>}`. Required
    /// on create (empty is rejected); NOT re-validated on update, so a `PUT`
    /// omitting it would store an empty string — this codec always emits it.
    /// Not unique — no server-side collision check.
    #[key]
    pub name: String,
    /// Conditions that must match for the format to apply. The API
    /// normalises a `null`/omitted value to `[]` on both create and update,
    /// so a plain always-emitted `Vec` here is correct — never distinguish
    /// "omitted" from "empty" for this field. Fully replaced on update;
    /// there is no per-condition endpoint.
    pub conditions: Vec<CustomCondition>,
    /// Row creation time (SQLite `CURRENT_TIMESTAMP`). Server-managed, never
    /// writable.
    #[wire(read_only)]
    pub created_at: Option<String>,
}
