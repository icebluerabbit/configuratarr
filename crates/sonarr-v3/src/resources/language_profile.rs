//! Language profile — ordered language preference list with an upgrade cutoff.
//!
//! Sonarr-only concept (Radarr uses per-quality-profile language settings
//! instead). A language profile defines which languages Sonarr will accept for a
//! series and the language at which it stops seeking upgrades.

use core_macros::{nested, resource};

/// A language entry as returned by Sonarr.
///
/// Used both as the cutoff language on a `LanguageProfile` and as the
/// `language` sub-object inside each `LanguageProfileItem`.
#[nested]
pub struct Language {
    /// Server-assigned language id (e.g. `1` for English, `2` for French).
    pub id: Option<i32>,
    /// Language name, e.g. `"English"`.
    pub name: Option<String>,
}

/// One language entry in a language profile's ordered list.
///
/// `allowed: true` means Sonarr will download releases in this language;
/// `allowed: false` means it is listed but not acceptable. The order of the
/// `languages` array determines preference when multiple allowed languages are
/// available.
#[nested]
pub struct LanguageProfileItem {
    /// Server-assigned item id.
    pub id: Option<i32>,
    /// The language this entry represents.
    pub language: Option<Language>,
    /// When `true`, Sonarr will accept releases in this language.
    pub allowed: bool,
}

/// Language profile — ordered list of acceptable languages with an upgrade cutoff.
///
/// Sonarr assigns one language profile to each series. The profile controls:
/// - which languages are acceptable (`languages` list with `allowed: true`),
/// - the upgrade cutoff (`cutoff`) — Sonarr stops seeking upgrades once it has
///   a release at or above this language.
/// - whether Sonarr will upgrade from a lower-ranked language (`upgrade_allowed`).
#[resource(
    sync = crud,
    list = get("/api/v3/languageprofile"),
    create = post("/api/v3/languageprofile"),
    update = put("/api/v3/languageprofile/${self.id}"),
    delete = delete("/api/v3/languageprofile/${self.id}"),
)]
pub struct LanguageProfile {
    /// Server-assigned profile id.
    #[id]
    pub id: Option<i32>,
    /// Natural key — referenced in `${ref.language_profile.<name>}`.
    #[key]
    pub name: String,
    /// When `true`, Sonarr will seek a release in a higher-preference language
    /// after an initial download.
    pub upgrade_allowed: bool,
    /// Language at which Sonarr stops seeking upgrades. Must be an `allowed`
    /// entry in the `languages` list.
    pub cutoff: Option<Language>,
    /// Ordered list of languages and their acceptance state; preference
    /// decreases toward the end of the list.
    pub languages: Vec<LanguageProfileItem>,
}
