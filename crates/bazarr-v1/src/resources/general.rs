use core_macros::nested;

/// General settings (`settings-general-*`) — instance behaviour and which
/// downstream apps bazarr talks to.
#[nested(case = snake)]
pub struct General {
    /// Bind address bazarr listens on.
    pub ip: Option<String>,
    /// Port bazarr listens on.
    pub port: Option<i32>,
    /// Base URL bazarr is served under (reverse-proxy subpath).
    pub base_url: Option<String>,
    /// Display name for this instance.
    pub instance_name: Option<String>,
    /// Update branch (`master` / `development`).
    pub branch: Option<String>,
    /// Automatically install updates.
    pub auto_update: Option<bool>,
    /// Only ever download subtitles in a single language.
    pub single_language: Option<bool>,
    /// Minimum subtitle score for series (0–100).
    pub minimum_score: Option<i32>,
    /// Minimum subtitle score for movies (0–100).
    pub minimum_score_movie: Option<i32>,
    /// Use the scene name when searching.
    pub use_scenename: Option<bool>,
    /// Run a post-processing command after download.
    pub use_postprocessing: Option<bool>,
    /// The post-processing command to run.
    pub postprocessing_cmd: Option<String>,
    /// Manage subtitles for Sonarr-tracked series.
    pub use_sonarr: Option<bool>,
    /// Manage subtitles for Radarr-tracked movies.
    pub use_radarr: Option<bool>,
    /// Enable the Plex integration.
    pub use_plex: Option<bool>,
    /// Enable the Jellyfin integration.
    pub use_jellyfin: Option<bool>,
    /// Consider embedded subtitles already present in the media.
    pub use_embedded_subs: Option<bool>,
    /// Show embedded subtitles in the desired-languages view.
    pub embedded_subs_show_desired: Option<bool>,
    /// Ignore embedded PGS (image) subtitles.
    pub ignore_pgs_subs: Option<bool>,
    /// Ignore embedded VobSub (image) subtitles.
    pub ignore_vobsub_subs: Option<bool>,
    /// Ignore embedded ASS/SSA subtitles.
    pub ignore_ass_subs: Option<bool>,
    /// Space searches out over time as media ages.
    pub adaptive_searching: Option<bool>,
    /// Enabled subtitle provider ids (e.g. `opensubtitlescom`, `addic7ed`).
    pub enabled_providers: Vec<String>,
    /// Enabled integration ids.
    pub enabled_integrations: Vec<String>,
    /// Search providers in parallel.
    pub multithreading: Option<bool>,
    /// Keep upgrading subtitles to better matches after the first download.
    pub upgrade_subs: Option<bool>,
    /// Upgrade check frequency, in hours.
    pub upgrade_frequency: Option<i32>,
    /// How many days back to keep upgrading a subtitle.
    pub days_to_upgrade_subs: Option<i32>,
    /// Wanted-search frequency for series, in hours.
    pub wanted_search_frequency: Option<i32>,
    /// Wanted-search frequency for movies, in hours.
    pub wanted_search_frequency_movie: Option<i32>,
    /// Languages treated as equal to one another (`from:to` rules).
    pub language_equals: Vec<String>,
    /// Automatically assign a default language profile to newly-tracked series.
    pub serie_default_enabled: Option<bool>,
    /// Language-profile id (as a string) assigned to new series when
    /// `serie_default_enabled` is set. References a `language_profiles` entry.
    pub serie_default_profile: Option<String>,
    /// Automatically assign a default language profile to newly-tracked movies.
    pub movie_default_enabled: Option<bool>,
    /// Language-profile id (as a string) assigned to new movies when
    /// `movie_default_enabled` is set. References a `language_profiles` entry.
    pub movie_default_profile: Option<String>,
}
