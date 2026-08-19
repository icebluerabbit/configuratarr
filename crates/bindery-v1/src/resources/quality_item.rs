use core_macros::nested;

/// One entry in a [`crate::resources::quality_profile::QualityProfile`]'s
/// preference list.
#[nested]
pub struct QualityItem {
    /// File-format token. **Trimmed and lower-cased server-side** — a config
    /// that writes mixed case (e.g. `EPUB`) churns forever, since every read
    /// comes back lower-case. Must be non-empty and unique within the
    /// profile. Not restricted to a fixed set, but only the tokens in
    /// Bindery's `QualityRank` carry an ordering: `unknown`, `txt`, `rtf`,
    /// `pdf`, `mobi`/`azw` (tied), `epub`, `azw3`, `mp3`, `m4a`, `m4b`,
    /// `flac`.
    pub quality: String,
    /// Whether releases of this format may be grabbed. At least one item in
    /// a profile must set this `true`.
    #[default(false)]
    pub allowed: bool,
}
