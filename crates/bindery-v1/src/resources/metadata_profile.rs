//! Metadata profile — filters deciding which discovered books get added to
//! the catalogue. Per-user since migration 025 server-side, but the owner
//! column is never serialised nor accepted on input, so it is invisible at
//! this layer. Bindery seeds one profile named `Standard` with
//! `allowed_languages: eng`.

use core_macros::resource;

use crate::resources::unknown_language_behavior::UnknownLanguageBehavior;

/// Named metadata profile controlling which discovered books get added.
#[resource(
    sync = crud,
    list = get("/api/v1/metadataprofile"),
    create = post("/api/v1/metadataprofile"),
    update = put("/api/v1/metadataprofile/${self.id}"),
    delete = delete("/api/v1/metadataprofile/${self.id}"),
)]
pub struct MetadataProfile {
    /// Server-assigned row id (SQLite `AUTOINCREMENT`). Ignored on input — on
    /// create the assigned id is written back, on update the path id wins.
    #[id]
    pub id: Option<i64>,
    /// Natural key — referenced in `${ref.metadata_profile.<name>}`.
    /// Required on create (empty is rejected); NOT re-validated on update.
    /// Not unique — no server-side collision check.
    #[key]
    pub name: String,
    /// Minimum popularity score a book must reach to be added. `0` disables
    /// the filter.
    #[default(0)]
    pub min_popularity: i32,
    /// Minimum page count a book must reach to be added. `0` disables the
    /// filter.
    #[default(0)]
    pub min_pages: i32,
    /// Skip books with no release date.
    #[default(false)]
    pub skip_missing_date: bool,
    /// Skip books with no ISBN.
    #[default(false)]
    pub skip_missing_isbn: bool,
    /// Skip books that are parts/volumes of a larger work.
    #[default(false)]
    pub skip_part_books: bool,
    /// Language filter as a raw string, matched verbatim rather than as a
    /// list. On create, an empty value is replaced server-side with `eng`;
    /// that fallback is **not** applied on update, so sending an empty
    /// string there clears the filter. This codec always emits the field
    /// (its default is `eng`), so an omitted config value never triggers the
    /// update-time gap.
    #[default("eng")]
    pub allowed_languages: String,
    /// What to do when the metadata source reports no language for a book
    /// while `allowed_languages` is non-empty: `pass` imports it anyway,
    /// `fail` skips it. Coerced server-side on both create and update — any
    /// value other than exactly `fail` becomes `pass`, so an omitted or
    /// unrecognised value is never rejected, it silently falls back to
    /// `pass` (the API's own default). See [`UnknownLanguageBehavior`].
    pub unknown_language_behavior: Option<UnknownLanguageBehavior>,
    /// Row creation time (SQLite `CURRENT_TIMESTAMP`). Server-managed, never
    /// writable.
    #[wire(read_only)]
    pub created_at: Option<String>,
}
