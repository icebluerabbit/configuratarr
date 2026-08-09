use core_macros::nested;

/// Subtitle synchronisation settings (`settings-subsync-*`).
#[nested(case = snake)]
pub struct Subsync {
    /// Automatically sync subtitles to the audio track after download.
    pub use_subsync: Option<bool>,
    /// Only auto-sync when the subtitle score is below a threshold.
    pub use_subsync_threshold: Option<bool>,
    /// Series score threshold under which to auto-sync.
    pub subsync_threshold: Option<i32>,
    /// Only auto-sync movies when the score is below a threshold.
    pub use_subsync_movie_threshold: Option<bool>,
    /// Movie score threshold under which to auto-sync.
    pub subsync_movie_threshold: Option<i32>,
    /// Keep sync debug output.
    pub debug: Option<bool>,
    /// Force audio-track–based sync.
    pub force_audio: Option<bool>,
    /// Sync against the original-language audio.
    pub use_original_language: Option<bool>,
    /// Automatically prefer the original-language audio for sync.
    pub auto_use_original_language: Option<bool>,
    /// Don't correct the subtitle framerate.
    pub no_fix_framerate: Option<bool>,
    /// Use the golden-section search algorithm.
    pub gss: Option<bool>,
    /// Maximum allowed offset, in seconds.
    pub max_offset_seconds: Option<i32>,
    /// Post-sync quality checker: languages/providers excluded from the sync
    /// verification pass.
    pub checker: Option<SubsyncChecker>,
}

/// Subsync post-sync checker settings (`settings-subsync-checker-*`).
#[nested(case = snake)]
pub struct SubsyncChecker {
    /// Language codes excluded from the post-sync quality check.
    pub blacklisted_languages: Vec<String>,
    /// Provider ids excluded from the post-sync quality check.
    pub blacklisted_providers: Vec<String>,
}
