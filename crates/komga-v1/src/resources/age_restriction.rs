//! Embedded sub-resource: age restriction settings for a [`super::user::User`].
//!
//! Written as `AgeRestrictionUpdateDto` (create/update), which — unlike the
//! read shape `AgeRestrictionDto` — allows `restriction: "NONE"` as a way to
//! *clear* an existing restriction. Komga's read side has no such value: once
//! cleared, `GET` simply omits `ageRestriction` entirely rather than echoing
//! `{"restriction": "NONE", ...}` back. [`super::user::User::reconcile`]'s
//! `in_sync` check accounts for this (`restriction: NONE` in config is
//! considered satisfied by an *absent* live `ageRestriction`, not by a
//! present one).

use core_macros::{nested, wire_enum};

/// Age-based content restriction applied to a user's account.
#[nested]
pub struct AgeRestriction {
    /// The age boundary the restriction is evaluated against.
    pub age: i32,
    /// How `age` is applied.
    pub restriction: AgeRestrictionKind,
}

/// How an [`AgeRestriction::age`] boundary is applied to content.
#[wire_enum(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgeRestrictionKind {
    /// Only content rated for `age` or younger is allowed.
    AllowOnly,
    /// Content rated for `age` or older is excluded.
    Exclude,
    /// No restriction — write-only: clears an existing restriction. Komga
    /// never reads a restriction back as `NONE`; it reads back as an absent
    /// [`AgeRestriction`] instead.
    None,
    /// Unknown or future restriction kind not yet modelled by this version.
    #[fallback]
    Unknown,
}
