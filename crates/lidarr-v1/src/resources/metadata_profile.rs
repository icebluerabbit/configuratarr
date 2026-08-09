//! Metadata profile resource — controls which album types and release statuses
//! Lidarr monitors for a given artist.
//!
//! A metadata profile is a named set of allowed/enabled toggles over three
//! dimensions:
//! - **Primary album types**: Album, Single, Other, Broadcast, EP (each toggled
//!   on or off via the `allowed` flag).
//! - **Secondary album types**: Studio, Remix, Live, Compilation, Soundtrack,
//!   Spokenword, Interview, Audiobook, Other (each toggled on or off).
//! - **Release statuses**: Official, Promotional, Bootleg, Pseudo-Release (each
//!   toggled on or off).
//!
//! Each item in these lists is a server-managed entry that carries the type
//! descriptor (id + name) plus a user-configurable `allowed` flag. The lists are
//! always populated by the server; the user only controls `allowed`.
//!
//! There is no sonarr analog for this resource.

use core_macros::{nested, resource};

/// Album type descriptor returned by Lidarr for primary album type items.
///
/// The id and name are server-assigned and read-only; they identify the
/// album type (e.g. id=1 name="Album", id=2 name="Single").
#[nested]
pub struct PrimaryAlbumType {
    /// Server-assigned album type id.
    pub id: Option<i32>,
    /// Album type label, e.g. `"Album"`, `"Single"`, `"EP"`.
    pub name: Option<String>,
}

/// One primary-album-type toggle inside a `MetadataProfile`.
///
/// Each item pairs an album type with a user-configurable `allowed` flag.
/// Lidarr monitors albums of the given type for artists under this profile
/// when `allowed` is `true`.
#[nested]
pub struct PrimaryAlbumTypeItem {
    /// Server-assigned item id.
    pub id: Option<i32>,
    /// The primary album type described by this item.
    pub album_type: Option<PrimaryAlbumType>,
    /// When `true`, albums of this type are monitored.
    pub allowed: bool,
}

/// Album type descriptor returned by Lidarr for secondary album type items.
///
/// The id and name are server-assigned and read-only; they identify the
/// secondary classification (e.g. "Studio", "Live", "Compilation").
#[nested]
pub struct SecondaryAlbumType {
    /// Server-assigned secondary album type id.
    pub id: Option<i32>,
    /// Secondary album type label, e.g. `"Studio"`, `"Live"`, `"Remix"`.
    pub name: Option<String>,
}

/// One secondary-album-type toggle inside a `MetadataProfile`.
///
/// Each item pairs a secondary album type with a user-configurable `allowed`
/// flag. Lidarr applies this filter in addition to the primary type filter.
#[nested]
pub struct SecondaryAlbumTypeItem {
    /// Server-assigned item id.
    pub id: Option<i32>,
    /// The secondary album type described by this item.
    pub album_type: Option<SecondaryAlbumType>,
    /// When `true`, albums with this secondary classification are monitored.
    pub allowed: bool,
}

/// Release status descriptor returned by Lidarr for release status items.
///
/// The id and name are server-assigned and read-only; they identify the
/// release status (e.g. "Official", "Promotional", "Bootleg").
#[nested]
pub struct ReleaseStatus {
    /// Server-assigned release status id.
    pub id: Option<i32>,
    /// Release status label, e.g. `"Official"`, `"Bootleg"`, `"Promotional"`.
    pub name: Option<String>,
}

/// One release-status toggle inside a `MetadataProfile`.
///
/// Each item pairs a release status with a user-configurable `allowed` flag.
/// Lidarr monitors releases with the given status for artists under this profile
/// when `allowed` is `true`.
#[nested]
pub struct ReleaseStatusItem {
    /// Server-assigned item id.
    pub id: Option<i32>,
    /// The release status described by this item.
    pub release_status: Option<ReleaseStatus>,
    /// When `true`, releases with this status are monitored.
    pub allowed: bool,
}

/// Named metadata profile — controls which album types and release statuses
/// Lidarr monitors for artists assigned this profile.
///
/// The three toggle lists (`primary_album_types`, `secondary_album_types`,
/// `release_statuses`) are always fully populated by the server. The user
/// configures only the `allowed` flag on each item. The server assigns the
/// type descriptors (ids and names).
#[resource(
    sync = crud,
    list = get("/api/v1/metadataprofile"),
    create = post("/api/v1/metadataprofile"),
    update = put("/api/v1/metadataprofile/${self.id}"),
    delete = delete("/api/v1/metadataprofile/${self.id}"),
)]
pub struct MetadataProfile {
    /// Server-assigned profile id.
    #[id]
    pub id: Option<i32>,
    /// Natural key — referenced in `${ref.metadata_profile.<name>}`.
    #[key]
    pub name: String,
    /// Toggles for each primary album type (Album, Single, EP, etc.).
    /// Each entry's `allowed` flag controls whether that type is monitored.
    pub primary_album_types: Vec<PrimaryAlbumTypeItem>,
    /// Toggles for each secondary album type (Studio, Live, Remix, etc.).
    /// Each entry's `allowed` flag controls whether that classification is
    /// monitored.
    pub secondary_album_types: Vec<SecondaryAlbumTypeItem>,
    /// Toggles for each release status (Official, Promotional, Bootleg, etc.).
    /// Each entry's `allowed` flag controls whether that status is monitored.
    pub release_statuses: Vec<ReleaseStatusItem>,
}
