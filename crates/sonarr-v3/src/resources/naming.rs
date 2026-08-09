use core_macros::resource;

/// `/api/v3/config/naming` — episode file and folder naming configuration.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/naming"),
    update = put("/api/v3/config/naming/${self.id}"),
)]
pub struct Naming {
    #[id]
    pub id: Option<i32>,
    /// Renames existing episode files to match the configured naming format on import or refresh.
    pub rename_episodes: bool,
    /// Replaces characters that are illegal on common filesystems in file and folder names.
    #[default(true)]
    pub replace_illegal_characters: bool,
    /// How to handle colons in series/episode titles; integer code (0 = delete, 1 = dash, 2 = space dash, 3 = space dash space, 4 = smart).
    pub colon_replacement_format: i32,
    /// Custom colon replacement string; used when `colon_replacement_format` is set to a custom mode.
    pub custom_colon_replacement_format: Option<String>,
    /// Style for multi-episode file naming; integer code (0 = extend, 1 = duplicate, 2 = repeat, 3 = scene, 4 = range, 5 = prefixed range).
    pub multi_episode_style: i32,
    /// Naming template for standard (non-daily, non-anime) episode files; uses Sonarr naming tokens.
    pub standard_episode_format: Option<String>,
    /// Naming template for daily (date-based) episode files; uses Sonarr naming tokens.
    pub daily_episode_format: Option<String>,
    /// Naming template for anime episode files; uses Sonarr naming tokens.
    pub anime_episode_format: Option<String>,
    /// Naming template for series root folders; uses Sonarr naming tokens.
    pub series_folder_format: Option<String>,
    /// Naming template for season subfolders; uses Sonarr naming tokens.
    pub season_folder_format: Option<String>,
    /// Naming template for the Specials season subfolder; uses Sonarr naming tokens.
    pub specials_folder_format: Option<String>,
}
