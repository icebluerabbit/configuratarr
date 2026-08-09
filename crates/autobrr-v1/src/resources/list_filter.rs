use core_macros::nested;

/// One filter a list feeds into. autobrr attaches by **numeric id**, so the id is
/// the field that matters on the wire; reference a managed filter by name with
/// `id: "${ref.filter.<name>}"`. `name` is display-only (autobrr fills it on read
/// and ignores it on write).
#[nested(case = snake)]
pub struct ListFilter {
    /// Server filter id — attach a managed filter via `${ref.filter.<name>}`.
    #[reference(filter)]
    pub id: Option<i32>,
    /// Filter display name (read-only; autobrr fills it).
    pub name: Option<String>,
}
