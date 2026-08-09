use core_macros::nested;

/// Generate subtitles with a Whisper AI transcription service.
#[nested(case = snake)]
pub struct WhisperAi {
    /// Service endpoint URL.
    pub endpoint: Option<String>,
    /// Response timeout, in seconds.
    pub response: Option<i32>,
    /// Request timeout, in seconds.
    pub timeout: Option<i32>,
    /// Pass the video filename to the service.
    pub pass_video_name: Option<bool>,
    /// Log level.
    pub loglevel: Option<String>,
}
