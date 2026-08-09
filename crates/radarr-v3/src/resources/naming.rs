use core_macros::resource;

/// `/api/v3/config/naming` — movie file and folder naming configuration.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/naming"),
    update = put("/api/v3/config/naming/${self.id}"),
)]
pub struct Naming {
    #[id]
    pub id: Option<i32>,
    /// Renames existing movie files to match the configured naming format on import or refresh.
    pub rename_movies: bool,
    /// Replaces characters that are illegal on common filesystems in file and folder names.
    #[default(true)]
    pub replace_illegal_characters: bool,
    /// How to handle colons in movie titles: `delete`, `dash`, `spaceDash`, `spaceDashSpace`, or `smart`.
    #[default("delete")]
    pub colon_replacement_format: String,
    /// Naming template string for movie files; uses Radarr naming tokens (e.g. `{Movie Title}`).
    pub standard_movie_format: Option<String>,
    /// Naming template string for movie folders; uses Radarr naming tokens.
    pub movie_folder_format: Option<String>,
}
