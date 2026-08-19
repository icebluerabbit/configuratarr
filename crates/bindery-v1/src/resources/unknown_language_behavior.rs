use core_macros::wire_enum;

/// What [`crate::resources::metadata_profile::MetadataProfile`] does when the
/// metadata source reports no language for a book while `allowed_languages`
/// is non-empty: `pass` imports it anyway, `fail` skips it.
///
/// **Coerced server-side on both create and update** — any value other than
/// exactly `fail` becomes `pass`, so an unrecognised value is never rejected,
/// it silently falls back. The `#[fallback]` variant below mirrors that: a
/// value this version doesn't recognise decodes to `Unknown` rather than
/// erroring, but should never actually appear on the wire since the server
/// only ever persists `pass` or `fail`.
#[wire_enum(rename_all = "lowercase")]
pub enum UnknownLanguageBehavior {
    /// Import the book anyway despite the missing language.
    Pass,
    /// Skip the book.
    Fail,
    /// Unknown or future behavior not yet modelled by this version.
    #[fallback]
    Unknown,
}
