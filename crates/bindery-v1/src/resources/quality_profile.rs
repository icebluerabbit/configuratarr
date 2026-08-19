//! Quality profile — an ordered file-format preference list plus an upgrade
//! cutoff, assignable to authors. The Go struct also carries an owner column
//! (per-user ownership, derived from the session at create time), but it is
//! never serialised nor accepted on input, so it is invisible at this layer.

use core_macros::resource;

use crate::resources::quality_item::QualityItem;

/// Named quality profile — ordered file-format preference list with an
/// upgrade cutoff.
#[resource(
    sync = crud,
    list = get("/api/v1/qualityprofile"),
    create = post("/api/v1/qualityprofile"),
    update = put("/api/v1/qualityprofile/${self.id}"),
    delete = delete("/api/v1/qualityprofile/${self.id}"),
)]
pub struct QualityProfile {
    /// Server-assigned row id (SQLite `AUTOINCREMENT`). Forced to `0` on
    /// create and taken from the path on update, so any client-supplied
    /// value is ignored.
    #[id]
    pub id: Option<i64>,
    /// Natural key — referenced in `${ref.quality_profile.<name>}`. Trimmed
    /// server-side; must be non-empty and unique across **all** profiles
    /// (409 on collision).
    #[key]
    pub name: String,
    /// Whether an existing file may be replaced by a better-ranked one.
    #[default(false)]
    pub upgrade_allowed: bool,
    /// Format at which upgrading stops. **Trimmed and lower-cased
    /// server-side** — write it lower-case or every read-back will show a
    /// diff and the config will churn forever. Must be non-empty and must
    /// equal the `quality` of one of `items` with `allowed: true`.
    pub cutoff: String,
    /// Ordered preference list of formats. Must contain at least one entry
    /// and at least one entry with `allowed: true`; duplicate (lower-cased)
    /// qualities are rejected server-side.
    pub items: Vec<QualityItem>,
    /// Row creation time (SQLite `CURRENT_TIMESTAMP`). Server-managed, never
    /// writable. Note: create/update do not re-read the row, so the
    /// 201/200 response body echoes whatever was sent (the Go zero time
    /// when omitted) — only GET/LIST carry the real value.
    #[wire(read_only)]
    pub created_at: Option<String>,
}
