//! One filesystem folder scanned into a [`crate::resources::library::Library`].
//!
//! This struct models only the *config* shape a user authors — a bare path —
//! which matches what `POST /api/libraries` (`CreateLibraryRequest.folders`,
//! `{fullPath}`) accepts. It is deliberately **not** the shape `GET
//! /api/libraries` returns for `Library.folders` (server-enriched: `{id,
//! fullPath, libraryId, addedAt}`), nor the shape `PATCH /api/libraries/{id}`
//! accepts for an update (`{id}`-or-`{fullPath}`, id-discriminated). The
//! `Library` reconcile hook bridges all three shapes itself: it matches an
//! existing live folder to a declared one by `full_path` and echoes the live
//! folder's server-assigned `id` back into the `PATCH` body — see the danger
//! note on `Library.folders` for why that echo is load-bearing, not optional.

use core_macros::nested;

/// A folder path to scan into a library, as the user declares it.
///
/// **Danger — read before removing a folder from config.** Audiobookshelf's
/// folder update is a destructive replace-by-id, not a merge: any folder the
/// library currently has that isn't matched back (by `full_path`) in the next
/// apply is deleted, and deleting a folder cascades to every library item
/// (and now-orphaned author/series) that was scanned from it. Removing a
/// `library_folder` entry from this resource's config is therefore
/// equivalent to deleting that folder's entire scanned contents on the next
/// apply — there is no dry, reversible "unlink" here.
#[nested]
pub struct LibraryFolder {
    /// Absolute filesystem path on the Audiobookshelf server to scan into
    /// this library (e.g. `/audiobooks`). Matched against the library's live
    /// folders by exact string equality to decide create-vs-keep on update.
    pub full_path: String,
}
