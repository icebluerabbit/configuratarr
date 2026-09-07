use core_macros::resource;

/// `/api/v3/config/naming` — movie and scene file/folder naming configuration.
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
    /// Renames existing scene files to match the configured scene naming format on import or refresh.
    pub rename_scenes: bool,
    /// Replaces characters that are illegal on common filesystems in file and folder names.
    #[default(true)]
    pub replace_illegal_characters: bool,
    /// How to handle colons in movie titles: `delete`, `dash`, `spaceDash`, `spaceDashSpace`, or `smart`.
    #[default("delete")]
    pub colon_replacement_format: String,
    /// Naming template string for movie files; uses Whisparr naming tokens (e.g. `{Movie Title}`).
    pub standard_movie_format: Option<String>,
    /// Naming template string for movie folders; uses Whisparr naming tokens.
    pub movie_folder_format: Option<String>,
    /// Naming template string for scene files; uses Whisparr scene naming tokens.
    pub standard_scene_format: Option<String>,
    /// Naming template string for scene folders; uses Whisparr scene naming tokens.
    pub scene_folder_format: Option<String>,
    /// Naming template string for the folder scenes are imported into.
    pub scene_import_folder_format: Option<String>,
    /// Maximum length in characters allowed for a generated folder path.
    pub max_folder_path_length: i32,
    /// Maximum length in characters allowed for a generated file path.
    pub max_file_path_length: i32,
}
