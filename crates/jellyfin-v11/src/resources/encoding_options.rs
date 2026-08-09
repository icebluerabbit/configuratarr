use core_macros::resource;

/// `/System/Configuration/encoding` — transcoding / ffmpeg options.
#[resource(
    sync = singleton,
    case = pascal,
    read = get("/System/Configuration/encoding"),
    update = post("/System/Configuration/encoding"),
)]
pub struct EncodingOptions {
    /// Encoding Thread Count
    pub encoding_thread_count: i32,
    /// Transcoding Temp Path
    pub transcoding_temp_path: Option<String>,
    /// Fallback Font Path
    pub fallback_font_path: Option<String>,
    /// Enable Fallback Font
    pub enable_fallback_font: bool,
    /// Enable Audio Vbr
    pub enable_audio_vbr: bool,
    /// Down Mix Audio Boost
    pub down_mix_audio_boost: f64,
    /// Down Mix Stereo Algorithm. One of: `None`, `Dave750`, `NightmodeDialogue`, `Rfc7845`, `Ac4`.
    pub down_mix_stereo_algorithm: Option<String>,
    /// Max Muxing Queue Size
    pub max_muxing_queue_size: i32,
    /// Enable Throttling
    pub enable_throttling: bool,
    /// Throttle Delay Seconds
    pub throttle_delay_seconds: i32,
    /// Enable Segment Deletion
    pub enable_segment_deletion: bool,
    /// Segment Keep Seconds
    pub segment_keep_seconds: i32,
    /// Hardware Acceleration Type. One of: `none`, `amf`, `qsv`, `nvenc`, `v4l2m2m`, `vaapi`, `videotoolbox`, `rkmpp`.
    pub hardware_acceleration_type: Option<String>,
    /// Encoder App Path
    pub encoder_app_path: Option<String>,
    /// Encoder App Path Display
    pub encoder_app_path_display: Option<String>,
    /// Vaapi Device
    pub vaapi_device: Option<String>,
    /// Qsv Device
    pub qsv_device: Option<String>,
    /// Enable Tonemapping
    pub enable_tonemapping: bool,
    /// Enable Vpp Tonemapping
    pub enable_vpp_tonemapping: bool,
    /// Enable Video Toolbox Tonemapping
    pub enable_video_toolbox_tonemapping: bool,
    /// Tonemapping Algorithm. One of: `none`, `clip`, `linear`, `gamma`, `reinhard`, `hable`, `mobius`, `bt2390`.
    pub tonemapping_algorithm: Option<String>,
    /// Tonemapping Mode. One of: `auto`, `max`, `rgb`, `lum`, `itp`.
    pub tonemapping_mode: Option<String>,
    /// Tonemapping Range. One of: `auto`, `tv`, `pc`.
    pub tonemapping_range: Option<String>,
    /// Tonemapping Desat
    pub tonemapping_desat: f64,
    /// Tonemapping Peak
    pub tonemapping_peak: f64,
    /// Tonemapping Param
    pub tonemapping_param: f64,
    /// Vpp Tonemapping Brightness
    pub vpp_tonemapping_brightness: f64,
    /// Vpp Tonemapping Contrast
    pub vpp_tonemapping_contrast: f64,
    /// H 264 Crf
    pub h264_crf: i32,
    /// H 265 Crf
    pub h265_crf: i32,
    /// Encoder Preset. One of: `auto`, `placebo`, `veryslow`, `slower`, `slow`, `medium`, `fast`, `faster`, `veryfast`, `superfast`, `ultrafast`.
    pub encoder_preset: Option<String>,
    /// Deinterlace Double Rate
    pub deinterlace_double_rate: bool,
    /// Deinterlace Method. One of: `yadif`, `bwdif`.
    pub deinterlace_method: Option<String>,
    /// Enable Decoding Color Depth 10 Hevc
    pub enable_decoding_color_depth10_hevc: bool,
    /// Enable Decoding Color Depth 10 Vp 9
    pub enable_decoding_color_depth10_vp9: bool,
    /// Enable Decoding Color Depth 10 Hevc Rext
    pub enable_decoding_color_depth10_hevc_rext: bool,
    /// Enable Decoding Color Depth 12 Hevc Rext
    pub enable_decoding_color_depth12_hevc_rext: bool,
    /// Enable Enhanced Nvdec Decoder
    pub enable_enhanced_nvdec_decoder: bool,
    /// Prefer System Native Hw Decoder
    pub prefer_system_native_hw_decoder: bool,
    /// Enable Intel Low Power H 264 Hw Encoder
    pub enable_intel_low_power_h264_hw_encoder: bool,
    /// Enable Intel Low Power Hevc Hw Encoder
    pub enable_intel_low_power_hevc_hw_encoder: bool,
    /// Enable Hardware Encoding
    pub enable_hardware_encoding: bool,
    /// Allow Hevc Encoding
    pub allow_hevc_encoding: bool,
    /// Allow Av 1 Encoding
    pub allow_av1_encoding: bool,
    /// Enable Subtitle Extraction
    pub enable_subtitle_extraction: bool,
    /// Hardware Decoding Codecs
    pub hardware_decoding_codecs: Vec<String>,
    /// Allow On Demand Metadata Based Keyframe Extraction For Extensions
    pub allow_on_demand_metadata_based_keyframe_extraction_for_extensions: Vec<String>,
}
