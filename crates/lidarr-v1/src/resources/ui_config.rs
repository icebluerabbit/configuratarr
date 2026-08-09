use core_macros::resource;

/// `/api/v1/config/ui` — UI display and localisation settings.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/ui"),
    update = put("/api/v1/config/ui/${self.id}"),
)]
pub struct UiConfig {
    #[id]
    pub id: Option<i32>,
    /// Day the calendar week starts on: 0 = Sunday, 1 = Monday.
    pub first_day_of_week: i32,
    /// Format string for the column header in the calendar week view (e.g. `ddd M/D`).
    pub calendar_week_column_header: Option<String>,
    /// Short date format string used throughout the UI (e.g. `MMM D YYYY`).
    pub short_date_format: Option<String>,
    /// Long date format string used in detail views (e.g. `dddd, MMMM D YYYY`).
    pub long_date_format: Option<String>,
    /// Time format string used in the UI: e.g. `h(:mm)a` (12-hour) or `HH:mm` (24-hour).
    pub time_format: Option<String>,
    /// Displays dates as relative time (e.g. "2 days ago") rather than absolute dates.
    pub show_relative_dates: bool,
    /// Enables a colour-blind-friendly UI mode with adjusted colour palettes.
    pub enable_color_impaired_mode: bool,
    /// Language ID for the Lidarr UI interface.
    pub ui_language: i32,
    /// Expands albums by default in the artist detail view.
    pub expand_album_by_default: bool,
    /// Expands singles by default in the artist detail view.
    pub expand_single_by_default: bool,
    /// Expands EPs by default in the artist detail view.
    #[wire(name = "expandEPByDefault")]
    pub expand_ep_by_default: bool,
    /// Expands broadcast albums by default in the artist detail view.
    pub expand_broadcast_by_default: bool,
    /// Expands other release types by default in the artist detail view.
    pub expand_other_by_default: bool,
    /// UI colour theme name (e.g. `dark`, `light`, `auto`).
    pub theme: Option<String>,
}
