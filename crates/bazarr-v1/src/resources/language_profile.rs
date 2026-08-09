//! Language profiles — bazarr's per-title subtitle-language policy.
//!
//! These are written through the **same** `/api/system/settings` POST as the
//! rest of the config, but as a top-level `languages-profiles` form field
//! (a JSON string, **not** a `settings-*` key) with **full-replace-by-`profileId`**
//! semantics: the submitted list is the complete desired set, and bazarr deletes
//! any profile whose `profileId` is absent. Read back from
//! `/api/system/languages/profiles`.
//!
//! The standard codec produces bazarr's exact stored JSON shape: the profile is
//! `case = camel` (`profile_id` → `profileId`, `must_contain` → `mustContain`),
//! `original_format` carries `#[wire(null, int)]` so it renders as the
//! `originalFormat` **int** (`0`/`1`/null) bazarr stores, and the nullable fields
//! carry `#[wire(null)]` so they round-trip as explicit `null`s. Items are a
//! `case = snake` nested struct; their booleans are Python-style
//! `"True"`/`"False"`/`"Both"` strings (defaulting to `"False"`), matching what
//! bazarr returns, so the round-trip is idempotent.

use core_macros::nested;

/// One language entry within a [`LanguageProfile`].
///
/// The `forced`, `hi`, `audio_exclude` and `audio_only_include` flags are
/// bazarr's own tri-/bi-state strings — `"True"`, `"False"` (and `"Both"` for
/// `forced`/`hi`) — mirrored verbatim so a profile round-trips unchanged.
#[nested(case = snake)]
pub struct LanguageProfileItem {
    /// Item id (unique within the profile; referenced by the profile `cutoff`).
    pub id: i32,
    /// Language code (alpha-2, e.g. `en`).
    pub language: String,
    /// Match forced subtitles: `"True"` / `"False"` / `"Both"`.
    #[default("False")]
    pub forced: String,
    /// Match hearing-impaired subtitles: `"True"` / `"False"` / `"Both"`.
    #[default("False")]
    pub hi: String,
    /// Skip when this language is present in the audio: `"True"` / `"False"`.
    #[default("False")]
    pub audio_exclude: String,
    /// Only include when this language is the audio: `"True"` / `"False"`.
    #[default("False")]
    pub audio_only_include: String,
}

/// A language profile — the ordered set of subtitle languages bazarr wants for a
/// title, plus the cutoff and must-/must-not-contain rules.
#[nested]
pub struct LanguageProfile {
    /// Stable profile id (the natural key; profiles are replaced by this id).
    pub profile_id: i32,
    /// Display name.
    #[wire(null)]
    pub name: Option<String>,
    /// Cutoff item `id` — stop searching once this language is found (`65535` =
    /// any of the profile's languages; unset = never cut off early).
    #[wire(null)]
    pub cutoff: Option<i32>,
    /// The languages this profile wants, in priority order.
    pub items: Vec<LanguageProfileItem>,
    /// Release must contain all of these strings.
    pub must_contain: Vec<String>,
    /// Release must contain none of these strings.
    pub must_not_contain: Vec<String>,
    /// Prefer the original-format subtitle. Stored by bazarr as the int `0`/`1`
    /// (or null when unset).
    #[wire(null, int)]
    pub original_format: Option<bool>,
    /// Sonarr/Radarr tag this profile is scoped to.
    #[wire(null)]
    pub tag: Option<String>,
}
