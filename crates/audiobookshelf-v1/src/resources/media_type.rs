//! `Library.mediaType` — which content model a library's items use.

use core_macros::wire_enum;

/// A library's content model: selects which [`super::library_settings::LibrarySettings`]
/// default template seeds it and how its items are typed. Technically
/// PATCH-able, but changing it on an existing library does not migrate
/// `settings` or existing items to the new shape — treat as effectively
/// create-only (`Library`'s reconcile hook never compares it).
#[wire_enum(rename_all = "lowercase")]
pub enum MediaType {
    /// Audiobooks/ebooks. The default on create.
    Book,
    /// Podcasts.
    Podcast,
    /// A value this version doesn't yet model.
    #[fallback]
    Unknown,
}
