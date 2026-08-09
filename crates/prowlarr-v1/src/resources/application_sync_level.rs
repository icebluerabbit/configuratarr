//! Unit-enum for `ApplicationSyncLevel` — the wire string that controls how
//! Prowlarr pushes indexers to an application. `#[fallback]` absorbs any new
//! level the API introduces without breaking the codec.

use core_macros::wire_enum;

/// How Prowlarr synchronises indexers to an application.
#[wire_enum(rename_all = "camelCase")]
pub enum ApplicationSyncLevel {
    /// Prowlarr does not sync indexers to this application.
    Disabled,
    /// Prowlarr adds new indexers but does not update or remove existing ones.
    AddOnly,
    /// Prowlarr adds, updates, and removes indexers to keep the application in sync.
    FullSync,
    /// Unknown sync level returned by a newer API version.
    #[fallback]
    Unknown,
}
