use core_macros::nested;

/// Trickplay (scrubbing preview) generation options.
#[nested(case = pascal)]
pub struct TrickplayOptions {
    /// Enable Hw Acceleration
    pub enable_hw_acceleration: bool,
    /// Enable Hw Encoding
    pub enable_hw_encoding: bool,
    /// Enable Key Frame Only Extraction
    pub enable_key_frame_only_extraction: bool,
    /// Scan Behavior. One of: `Blocking`, `NonBlocking`.
    pub scan_behavior: Option<String>,
    /// Process Priority. One of: `Normal`, `Idle`, `High`, `RealTime`, `BelowNormal`, `AboveNormal`.
    pub process_priority: Option<String>,
    /// Interval
    pub interval: i32,
    /// Width Resolutions
    pub width_resolutions: Vec<i32>,
    /// Tile Width
    pub tile_width: i32,
    /// Tile Height
    pub tile_height: i32,
    /// Qscale
    pub qscale: i32,
    /// Jpeg Quality
    pub jpeg_quality: i32,
    /// Process Threads
    pub process_threads: i32,
}
