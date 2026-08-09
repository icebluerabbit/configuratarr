use core_macros::nested;

/// One indexer a filter is attached to. autobrr attaches by **numeric id**, so
/// the id is the field that matters on the wire; reference a managed indexer by
/// name with `id: "${ref.indexer.<name>}"`. `name`/`identifier` are display-only
/// (autobrr fills them on read and ignores them on write).
#[nested(case = snake)]
pub struct FilterIndexer {
    /// Server indexer id — attach a managed indexer via `${ref.indexer.<name>}`.
    #[reference(indexer)]
    pub id: Option<i32>,
    /// Indexer display name (read-only; autobrr fills it).
    pub name: Option<String>,
    /// Indexer identifier, e.g. `torznab-<name>` (read-only; autobrr fills it).
    pub identifier: Option<String>,
}
