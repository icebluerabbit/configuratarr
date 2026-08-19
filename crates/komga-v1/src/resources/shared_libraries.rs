//! Embedded sub-resource: which libraries a [`super::user::User`] can access.
//!
//! Written as `SharedLibrariesUpdateDto` (`{all, libraryIds}`) on create/
//! update. The **read** shape (`UserDto`) disagrees: it flattens this to two
//! top-level fields, `sharedAllLibraries` (bool) and `sharedLibrariesIds`
//! (array). [`super::user::User::reconcile`]'s `in_sync` check bridges the
//! two shapes directly against the raw live JSON rather than decoding it
//! through this type.

use core_macros::nested;

/// Which libraries a user can see: either all of them, or an explicit subset.
#[nested]
pub struct SharedLibraries {
    /// Grant access to every library, including ones created later.
    /// When `true`, `library_ids` is ignored by Komga.
    pub all: bool,
    /// Explicit library ids to grant access to (used when `all` is `false`).
    #[reference(library)]
    pub library_ids: Vec<String>,
}
