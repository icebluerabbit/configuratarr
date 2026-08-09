use core_lib::Json;
use core_macros::resource;

/// Automatic tagging rule — applies tags to movies matching its specifications.
#[resource(
    sync = crud,
    list = get("/api/v3/autotagging"),
    create = post("/api/v3/autotagging"),
    update = put("/api/v3/autotagging/${self.id}"),
    delete = delete("/api/v3/autotagging/${self.id}"),
)]
pub struct AutoTag {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the rule name referenced in `${ref.auto_tag.<name>}`.
    #[key]
    pub name: String,
    /// When `true`, tags added by this rule are removed if the movie no longer matches its specifications.
    pub remove_tags_automatically: bool,
    /// Tag ids applied when the specifications match.
    #[reference(tag)]
    pub tags: Vec<i32>,
    /// Specification conditions (dynamic fields blob — stored as opaque JSON).
    pub specifications: Vec<Json>,
}
