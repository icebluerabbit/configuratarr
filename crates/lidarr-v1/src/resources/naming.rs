use core_macros::resource;

/// `/api/v1/config/naming` — track file and artist folder naming configuration.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/naming"),
    update = put("/api/v1/config/naming/${self.id}"),
)]
pub struct Naming {
    #[id]
    pub id: Option<i32>,
    /// Renames existing track files to match the configured naming format on import or refresh.
    pub rename_tracks: bool,
    /// Replaces characters that are illegal on common filesystems in file and folder names.
    #[default(true)]
    pub replace_illegal_characters: bool,
    /// How to handle colons in artist/album/track titles; integer code (0 = delete, 1 = dash, 2 = space dash, 3 = space dash space, 4 = smart).
    pub colon_replacement_format: i32,
    /// Naming template for standard (single-disc) track files; uses Lidarr naming tokens.
    pub standard_track_format: Option<String>,
    /// Naming template for multi-disc track files; uses Lidarr naming tokens.
    pub multi_disc_track_format: Option<String>,
    /// Naming template for artist root folders; uses Lidarr naming tokens.
    pub artist_folder_format: Option<String>,
    /// Includes the artist name in the auto-generated folder name components.
    pub include_artist_name: bool,
    /// Includes the album title in the auto-generated folder name components.
    pub include_album_title: bool,
    /// Includes the quality profile name in the auto-generated folder name components.
    pub include_quality: bool,
    /// Replaces spaces with the configured separator character in generated names.
    pub replace_spaces: bool,
    /// Separator character used between tokens when `replace_spaces` is enabled (e.g. `.` or `_`).
    pub separator: Option<String>,
    /// Style for track number formatting (e.g. `1` for plain, `01` for zero-padded).
    pub number_style: Option<String>,
}
