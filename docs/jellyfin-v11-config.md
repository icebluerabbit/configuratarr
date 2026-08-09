# Jellyfin v11 Configuration

Jellyfin v11 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Repository

`/Repositories` — a plugin repository (replaced as part of the whole list).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | no |  | Repository display name. |
| `url` | string | no |  | Manifest URL for the repository. |
| `enabled` | boolean | yes |  | Whether this repository is enabled. |

### Auth Key

`/Auth/Keys` — an API key issued to an application name.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `app` | string | yes |  | Application name the key is issued to — its identity. |

### User

`/Users` — a Jellyfin user account.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Login name — the user's identity. |
| `password` | secret string | no |  | Initial password, set when the user is created (create-only). Credential — redacted in plan output. |
| `is_administrator` | boolean | no |  | Grant full administrator rights. |
| `is_disabled` | boolean | no |  | Disable the account (cannot sign in). |
| `is_hidden` | boolean | no |  | Hide the user from login pages. |
| `enable_remote_access` | boolean | no |  | Allow access from outside the local network. |
| `enable_content_deletion` | boolean | no |  | Allow the user to delete media. |

### Library

`/Library/VirtualFolders` — a media library (name + collection type + paths).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Library display name — its identity. |
| `collection_type` | string | no |  | Collection type: `movies`, `tvshows`, `music`, `books`, … Omit for mixed. |
| `paths` | array of string | no |  | Filesystem paths that make up the library. |

### Server Configuration

`/System/Configuration` — core server settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `log_file_retention_days` | integer | yes |  | Log File Retention Days |
| `is_startup_wizard_completed` | boolean | yes |  | Is Startup Wizard Completed |
| `cache_path` | string | no |  | Cache Path |
| `previous_version` | string | no |  | Previous Version |
| `previous_version_str` | string | no |  | Previous Version Str |
| `enable_metrics` | boolean | yes |  | Enable Metrics |
| `enable_normalized_item_by_name_ids` | boolean | yes |  | Enable Normalized Item By Name Ids |
| `is_port_authorized` | boolean | yes |  | Is Port Authorized |
| `quick_connect_available` | boolean | yes |  | Quick Connect Available |
| `enable_case_sensitive_item_ids` | boolean | yes |  | Enable Case Sensitive Item Ids |
| `disable_live_tv_channel_user_data_name` | boolean | yes |  | Disable Live Tv Channel User Data Name |
| `metadata_path` | string | no |  | Metadata Path |
| `preferred_metadata_language` | string | no |  | Preferred Metadata Language |
| `metadata_country_code` | string | no |  | Metadata Country Code |
| `sort_replace_characters` | array of string | no |  | Sort Replace Characters |
| `sort_remove_characters` | array of string | no |  | Sort Remove Characters |
| `sort_remove_words` | array of string | no |  | Sort Remove Words |
| `min_resume_pct` | integer | yes |  | Min Resume Pct |
| `max_resume_pct` | integer | yes |  | Max Resume Pct |
| `min_resume_duration_seconds` | integer | yes |  | Min Resume Duration Seconds |
| `min_audiobook_resume` | integer | yes |  | Min Audiobook Resume |
| `max_audiobook_resume` | integer | yes |  | Max Audiobook Resume |
| `inactive_session_threshold` | integer | yes |  | Inactive Session Threshold |
| `library_monitor_delay` | integer | yes |  | Library Monitor Delay |
| `library_update_duration` | integer | yes |  | Library Update Duration |
| `cache_size` | integer | yes |  | Cache Size |
| `image_saving_convention` | string | no |  | Image Saving Convention. One of: `Legacy`, `Compatible`. |
| `metadata_options` | array of [`metadata_options`](#metadata-options) | no |  | Metadata Options |
| `skip_deserialization_for_basic_types` | boolean | yes |  | Skip Deserialization For Basic Types |
| `server_name` | string | no |  | Server Name |
| `u_i_culture` | string | no |  | UI Culture |
| `save_metadata_hidden` | boolean | yes |  | Save Metadata Hidden |
| `content_types` | array of [`name_value_pair`](#name-value-pair) | no |  | Content Types |
| `remote_client_bitrate_limit` | integer | yes |  | Remote Client Bitrate Limit |
| `enable_folder_view` | boolean | yes |  | Enable Folder View |
| `enable_grouping_movies_into_collections` | boolean | yes |  | Enable Grouping Movies Into Collections |
| `enable_grouping_shows_into_collections` | boolean | yes |  | Enable Grouping Shows Into Collections |
| `display_specials_within_seasons` | boolean | yes |  | Display Specials Within Seasons |
| `codecs_used` | array of string | no |  | Codecs Used |
| `plugin_repositories` | array of [`repository_info`](#repository-info) | no |  | Plugin Repositories |
| `enable_external_content_in_suggestions` | boolean | yes |  | Enable External Content In Suggestions |
| `image_extraction_timeout_ms` | integer | yes |  | Image Extraction Timeout Ms |
| `path_substitutions` | array of [`path_substitution`](#path-substitution) | no |  | Path Substitutions |
| `enable_slow_response_warning` | boolean | yes |  | Enable Slow Response Warning |
| `slow_response_threshold_ms` | integer | yes |  | Slow Response Threshold Ms |
| `cors_hosts` | array of string | no |  | Cors Hosts |
| `activity_log_retention_days` | integer | yes |  | Activity Log Retention Days |
| `library_scan_fanout_concurrency` | integer | yes |  | Library Scan Fanout Concurrency |
| `library_metadata_refresh_concurrency` | integer | yes |  | Library Metadata Refresh Concurrency |
| `allow_client_log_upload` | boolean | yes |  | Allow Client Log Upload |
| `dummy_chapter_duration` | integer | yes |  | Dummy Chapter Duration |
| `chapter_image_resolution` | string | no |  | Chapter Image Resolution. One of: `MatchSource`, `P144`, `P240`, `P360`, `P480`, `P720`, `P1080`, `P1440`, `P2160`. |
| `parallel_image_encoding_limit` | integer | yes |  | Parallel Image Encoding Limit |
| `cast_receiver_applications` | array of [`cast_receiver_application`](#cast-receiver-application) | no |  | Cast Receiver Applications |
| `trickplay_options` | [`trickplay_options`](#trickplay-options) | no |  | Trickplay Options |
| `enable_legacy_authorization` | boolean | yes |  | Enable Legacy Authorization |

### Network Configuration

`/System/Configuration/network` — HTTP(S), ports, and network access.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base Url |
| `enable_https` | boolean | yes |  | Enable Https |
| `require_https` | boolean | yes |  | Require Https |
| `certificate_path` | string | no |  | Certificate Path |
| `certificate_password` | string | no |  | Certificate Password |
| `internal_http_port` | integer | yes |  | Internal Http Port |
| `internal_https_port` | integer | yes |  | Internal Https Port |
| `public_http_port` | integer | yes |  | Public Http Port |
| `public_https_port` | integer | yes |  | Public Https Port |
| `auto_discovery` | boolean | yes |  | Auto Discovery |
| `enable_u_pn_p` | boolean | yes |  | Enable U Pn P |
| `enable_i_pv4` | boolean | yes |  | Enable I Pv 4 |
| `enable_i_pv6` | boolean | yes |  | Enable I Pv 6 |
| `enable_remote_access` | boolean | yes |  | Enable Remote Access |
| `local_network_subnets` | array of string | no |  | Local Network Subnets |
| `local_network_addresses` | array of string | no |  | Local Network Addresses |
| `known_proxies` | array of string | no |  | Known Proxies |
| `ignore_virtual_interfaces` | boolean | yes |  | Ignore Virtual Interfaces |
| `virtual_interface_names` | array of string | no |  | Virtual Interface Names |
| `enable_published_server_uri_by_request` | boolean | yes |  | Enable Published Server Uri By Request |
| `published_server_uri_by_subnet` | array of string | no |  | Published Server Uri By Subnet |
| `remote_i_p_filter` | array of string | no |  | Remote IP Filter |
| `is_remote_i_p_filter_blacklist` | boolean | yes |  | Is Remote IP Filter Blacklist |

### Encoding Options

`/System/Configuration/encoding` — transcoding / ffmpeg options.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `encoding_thread_count` | integer | yes |  | Encoding Thread Count |
| `transcoding_temp_path` | string | no |  | Transcoding Temp Path |
| `fallback_font_path` | string | no |  | Fallback Font Path |
| `enable_fallback_font` | boolean | yes |  | Enable Fallback Font |
| `enable_audio_vbr` | boolean | yes |  | Enable Audio Vbr |
| `down_mix_audio_boost` | number | yes |  | Down Mix Audio Boost |
| `down_mix_stereo_algorithm` | string | no |  | Down Mix Stereo Algorithm. One of: `None`, `Dave750`, `NightmodeDialogue`, `Rfc7845`, `Ac4`. |
| `max_muxing_queue_size` | integer | yes |  | Max Muxing Queue Size |
| `enable_throttling` | boolean | yes |  | Enable Throttling |
| `throttle_delay_seconds` | integer | yes |  | Throttle Delay Seconds |
| `enable_segment_deletion` | boolean | yes |  | Enable Segment Deletion |
| `segment_keep_seconds` | integer | yes |  | Segment Keep Seconds |
| `hardware_acceleration_type` | string | no |  | Hardware Acceleration Type. One of: `none`, `amf`, `qsv`, `nvenc`, `v4l2m2m`, `vaapi`, `videotoolbox`, `rkmpp`. |
| `encoder_app_path` | string | no |  | Encoder App Path |
| `encoder_app_path_display` | string | no |  | Encoder App Path Display |
| `vaapi_device` | string | no |  | Vaapi Device |
| `qsv_device` | string | no |  | Qsv Device |
| `enable_tonemapping` | boolean | yes |  | Enable Tonemapping |
| `enable_vpp_tonemapping` | boolean | yes |  | Enable Vpp Tonemapping |
| `enable_video_toolbox_tonemapping` | boolean | yes |  | Enable Video Toolbox Tonemapping |
| `tonemapping_algorithm` | string | no |  | Tonemapping Algorithm. One of: `none`, `clip`, `linear`, `gamma`, `reinhard`, `hable`, `mobius`, `bt2390`. |
| `tonemapping_mode` | string | no |  | Tonemapping Mode. One of: `auto`, `max`, `rgb`, `lum`, `itp`. |
| `tonemapping_range` | string | no |  | Tonemapping Range. One of: `auto`, `tv`, `pc`. |
| `tonemapping_desat` | number | yes |  | Tonemapping Desat |
| `tonemapping_peak` | number | yes |  | Tonemapping Peak |
| `tonemapping_param` | number | yes |  | Tonemapping Param |
| `vpp_tonemapping_brightness` | number | yes |  | Vpp Tonemapping Brightness |
| `vpp_tonemapping_contrast` | number | yes |  | Vpp Tonemapping Contrast |
| `h264_crf` | integer | yes |  | H 264 Crf |
| `h265_crf` | integer | yes |  | H 265 Crf |
| `encoder_preset` | string | no |  | Encoder Preset. One of: `auto`, `placebo`, `veryslow`, `slower`, `slow`, `medium`, `fast`, `faster`, `veryfast`, `superfast`, `ultrafast`. |
| `deinterlace_double_rate` | boolean | yes |  | Deinterlace Double Rate |
| `deinterlace_method` | string | no |  | Deinterlace Method. One of: `yadif`, `bwdif`. |
| `enable_decoding_color_depth10_hevc` | boolean | yes |  | Enable Decoding Color Depth 10 Hevc |
| `enable_decoding_color_depth10_vp9` | boolean | yes |  | Enable Decoding Color Depth 10 Vp 9 |
| `enable_decoding_color_depth10_hevc_rext` | boolean | yes |  | Enable Decoding Color Depth 10 Hevc Rext |
| `enable_decoding_color_depth12_hevc_rext` | boolean | yes |  | Enable Decoding Color Depth 12 Hevc Rext |
| `enable_enhanced_nvdec_decoder` | boolean | yes |  | Enable Enhanced Nvdec Decoder |
| `prefer_system_native_hw_decoder` | boolean | yes |  | Prefer System Native Hw Decoder |
| `enable_intel_low_power_h264_hw_encoder` | boolean | yes |  | Enable Intel Low Power H 264 Hw Encoder |
| `enable_intel_low_power_hevc_hw_encoder` | boolean | yes |  | Enable Intel Low Power Hevc Hw Encoder |
| `enable_hardware_encoding` | boolean | yes |  | Enable Hardware Encoding |
| `allow_hevc_encoding` | boolean | yes |  | Allow Hevc Encoding |
| `allow_av1_encoding` | boolean | yes |  | Allow Av 1 Encoding |
| `enable_subtitle_extraction` | boolean | yes |  | Enable Subtitle Extraction |
| `hardware_decoding_codecs` | array of string | no |  | Hardware Decoding Codecs |
| `allow_on_demand_metadata_based_keyframe_extraction_for_extensions` | array of string | no |  | Allow On Demand Metadata Based Keyframe Extraction For Extensions |

### Metadata Configuration

`/System/Configuration/metadata` — metadata storage behaviour.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_file_creation_time_for_date_added` | boolean | yes |  | Use File Creation Time For Date Added |

### Branding Options

Branding — login disclaimer, custom CSS, splashscreen.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `login_disclaimer` | string | no |  | Login Disclaimer |
| `custom_css` | string | no |  | Custom Css |
| `splashscreen_enabled` | boolean | yes |  | Splashscreen Enabled |

## Types

### Metadata Options

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `item_type` | string | no |  | Item Type |
| `disabled_metadata_savers` | array of string | no |  | Disabled Metadata Savers |
| `local_metadata_reader_order` | array of string | no |  | Local Metadata Reader Order |
| `disabled_metadata_fetchers` | array of string | no |  | Disabled Metadata Fetchers |
| `metadata_fetcher_order` | array of string | no |  | Metadata Fetcher Order |
| `disabled_image_fetchers` | array of string | no |  | Disabled Image Fetchers |
| `image_fetcher_order` | array of string | no |  | Image Fetcher Order |

### Name Value Pair

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | no |  | Name |
| `value` | string | no |  | Value |

### Repository Info

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | no |  | Name |
| `url` | string | no |  | Url |
| `enabled` | boolean | yes |  | Enabled |

### Path Substitution

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `from` | string | no |  | From |
| `to` | string | no |  | To |

### Cast Receiver Application

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | string | no |  | Id |
| `name` | string | no |  | Name |

### Trickplay Options

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enable_hw_acceleration` | boolean | yes |  | Enable Hw Acceleration |
| `enable_hw_encoding` | boolean | yes |  | Enable Hw Encoding |
| `enable_key_frame_only_extraction` | boolean | yes |  | Enable Key Frame Only Extraction |
| `scan_behavior` | string | no |  | Scan Behavior. One of: `Blocking`, `NonBlocking`. |
| `process_priority` | string | no |  | Process Priority. One of: `Normal`, `Idle`, `High`, `RealTime`, `BelowNormal`, `AboveNormal`. |
| `interval` | integer | yes |  | Interval |
| `width_resolutions` | array of integer | no |  | Width Resolutions |
| `tile_width` | integer | yes |  | Tile Width |
| `tile_height` | integer | yes |  | Tile Height |
| `qscale` | integer | yes |  | Qscale |
| `jpeg_quality` | integer | yes |  | Jpeg Quality |
| `process_threads` | integer | yes |  | Process Threads |

