# Bazarr v1 Configuration

Bazarr v1 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Settings

`/api/system/settings` — bazarr's whole configuration. `case = snake` because
bazarr's JSON keys are the snake field names verbatim; each section is
`Option`: present = manage it, absent = leave bazarr's current values.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `general` | [`general`](#general) | no |  | General instance behaviour and downstream-app toggles. |
| `sonarr` | [`sonarr`](#sonarr) | no |  | Sonarr connection. |
| `radarr` | [`radarr`](#radarr) | no |  | Radarr connection. |
| `jellyfin` | [`jellyfin`](#jellyfin) | no |  | Jellyfin connection. |
| `proxy` | [`proxy`](#proxy) | no |  | Outbound proxy. |
| `backup` | [`backup`](#backup) | no |  | Automatic backups. |
| `subsync` | [`subsync`](#subsync) | no |  | Subtitle synchronisation. |
| `auth` | [`auth`](#auth) | no |  | Web-UI authentication. |
| `plex` | [`plex`](#plex) | no |  | Plex integration. |
| `postgresql` | [`postgresql`](#postgresql) | no |  | PostgreSQL backend. |
| `translator` | [`translator`](#translator) | no |  | Machine-translation engine. |
| `log` | [`log`](#log) | no |  | Log filtering. |
| `addic7ed` | [`addic7ed`](#addic7ed) | no |  | Addic7ed provider settings. |
| `anidb` | [`ani_db`](#ani-db) | no |  | AniDB provider settings. |
| `animetosho` | [`anime_tosho`](#anime-tosho) | no |  | AnimeTosho provider settings. |
| `anticaptcha` | [`anti_captcha`](#anti-captcha) | no |  | Anti-Captcha provider settings. |
| `assrt` | [`assrt`](#assrt) | no |  | Assrt provider settings. |
| `avistaz` | [`avista_z`](#avista-z) | no |  | AvistaZ provider settings. |
| `betaseries` | [`beta_series`](#beta-series) | no |  | BetaSeries provider settings. |
| `captchaai` | [`captcha_ai`](#captcha-ai) | no |  | CaptchaAI provider settings. |
| `cinemaz` | [`cinema_z`](#cinema-z) | no |  | CinemaZ provider settings. |
| `deathbycaptcha` | [`death_by_captcha`](#death-by-captcha) | no |  | Death by Captcha provider settings. |
| `embeddedsubtitles` | [`embedded_subtitles`](#embedded-subtitles) | no |  | Embedded subtitles provider settings. |
| `hdbits` | [`hd_bits`](#hd-bits) | no |  | HDBits provider settings. |
| `jimaku` | [`jimaku`](#jimaku) | no |  | Jimaku provider settings. |
| `karagarga` | [`kara_garga`](#kara-garga) | no |  | KaraGarga provider settings. |
| `ktuvit` | [`ktuvit`](#ktuvit) | no |  | Ktuvit provider settings. |
| `legendasdivx` | [`legendas_divx`](#legendas-divx) | no |  | LegendasDivx provider settings. |
| `legendasnet` | [`legendas_net`](#legendas-net) | no |  | Legendas.net provider settings. |
| `napiprojekt` | [`napi_projekt`](#napi-projekt) | no |  | NapiProjekt provider settings. |
| `napisy24` | [`napisy24`](#napisy24) | no |  | Napisy24 provider settings. |
| `opensubtitlescom` | [`open_subtitles_com`](#open-subtitles-com) | no |  | OpenSubtitles.com provider settings. |
| `pipocas` | [`pipocas`](#pipocas) | no |  | Pipocas provider settings. |
| `subdl` | [`sub_dl`](#sub-dl) | no |  | SubDL provider settings. |
| `subf2m` | [`subf2m`](#subf2m) | no |  | Subf2m provider settings. |
| `subsarr` | [`subsarr`](#subsarr) | no |  | Subsarr provider settings. |
| `subsource` | [`sub_source`](#sub-source) | no |  | SubSource provider settings. |
| `subsro` | [`subs_ro`](#subs-ro) | no |  | Subs.ro provider settings. |
| `subx` | [`sub_x`](#sub-x) | no |  | SubX provider settings. |
| `titlovi` | [`titlovi`](#titlovi) | no |  | Titlovi provider settings. |
| `titulky` | [`titulky`](#titulky) | no |  | Titulky provider settings. |
| `turkcealtyaziorg` | [`turkce_altyazi_org`](#turkce-altyazi-org) | no |  | Turkcealtyazi.org provider settings. |
| `whisperai` | [`whisper_ai`](#whisper-ai) | no |  | Whisper AI provider settings. |
| `xsubs` | [`x_subs`](#x-subs) | no |  | XSubs provider settings. |

### Languages

Enabled subtitle languages + the language-profile set, reconciled together
(profiles reference enabled languages). Written via the settings POST.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled_languages` | array of string | no |  | Enabled subtitle-language codes (alpha-2, e.g. `en`). Replaces the enabled set; declare the languages your profiles reference. |
| `language_profiles` | array of [`language_profile`](#language-profile) | no |  | Language profiles (full-replace by `profile_id`): the declared list is the complete desired set; bazarr deletes any profile not listed. |

### Notifications

Notification providers, reconciled sparsely by `name`: only the providers you
declare are updated; bazarr's other providers are left as they are. Written
via the settings POST.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `providers` | array of [`notifier`](#notifier) | no |  | Notification providers to manage (sparse, keyed by `name`). Each declared provider is updated; providers you don't list are left untouched. |

## Types

### General

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `ip` | string | no |  | Bind address bazarr listens on. |
| `port` | integer | no |  | Port bazarr listens on. |
| `base_url` | string | no |  | Base URL bazarr is served under (reverse-proxy subpath). |
| `instance_name` | string | no |  | Display name for this instance. |
| `branch` | string | no |  | Update branch (`master` / `development`). |
| `auto_update` | boolean | no |  | Automatically install updates. |
| `single_language` | boolean | no |  | Only ever download subtitles in a single language. |
| `minimum_score` | integer | no |  | Minimum subtitle score for series (0–100). |
| `minimum_score_movie` | integer | no |  | Minimum subtitle score for movies (0–100). |
| `use_scenename` | boolean | no |  | Use the scene name when searching. |
| `use_postprocessing` | boolean | no |  | Run a post-processing command after download. |
| `postprocessing_cmd` | string | no |  | The post-processing command to run. |
| `use_sonarr` | boolean | no |  | Manage subtitles for Sonarr-tracked series. |
| `use_radarr` | boolean | no |  | Manage subtitles for Radarr-tracked movies. |
| `use_plex` | boolean | no |  | Enable the Plex integration. |
| `use_jellyfin` | boolean | no |  | Enable the Jellyfin integration. |
| `use_embedded_subs` | boolean | no |  | Consider embedded subtitles already present in the media. |
| `embedded_subs_show_desired` | boolean | no |  | Show embedded subtitles in the desired-languages view. |
| `ignore_pgs_subs` | boolean | no |  | Ignore embedded PGS (image) subtitles. |
| `ignore_vobsub_subs` | boolean | no |  | Ignore embedded VobSub (image) subtitles. |
| `ignore_ass_subs` | boolean | no |  | Ignore embedded ASS/SSA subtitles. |
| `adaptive_searching` | boolean | no |  | Space searches out over time as media ages. |
| `enabled_providers` | array of string | no |  | Enabled subtitle provider ids (e.g. `opensubtitlescom`, `addic7ed`). |
| `enabled_integrations` | array of string | no |  | Enabled integration ids. |
| `multithreading` | boolean | no |  | Search providers in parallel. |
| `upgrade_subs` | boolean | no |  | Keep upgrading subtitles to better matches after the first download. |
| `upgrade_frequency` | integer | no |  | Upgrade check frequency, in hours. |
| `days_to_upgrade_subs` | integer | no |  | How many days back to keep upgrading a subtitle. |
| `wanted_search_frequency` | integer | no |  | Wanted-search frequency for series, in hours. |
| `wanted_search_frequency_movie` | integer | no |  | Wanted-search frequency for movies, in hours. |
| `language_equals` | array of string | no |  | Languages treated as equal to one another (`from:to` rules). |
| `serie_default_enabled` | boolean | no |  | Automatically assign a default language profile to newly-tracked series. |
| `serie_default_profile` | string | no |  | Language-profile id (as a string) assigned to new series when `serie_default_enabled` is set. References a `language_profiles` entry. |
| `movie_default_enabled` | boolean | no |  | Automatically assign a default language profile to newly-tracked movies. |
| `movie_default_profile` | string | no |  | Language-profile id (as a string) assigned to new movies when `movie_default_enabled` is set. References a `language_profiles` entry. |

### Sonarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `ip` | string | no |  | Sonarr host/IP. |
| `port` | integer | no |  | Sonarr port. |
| `base_url` | string | no |  | Sonarr base URL (reverse-proxy subpath). |
| `ssl` | boolean | no |  | Use HTTPS to reach Sonarr. |
| `http_timeout` | integer | no |  | HTTP request timeout, in seconds. |
| `apikey` | string | no |  | Sonarr API key. |
| `full_update` | string | no |  | Full-sync cadence (`Manually` / `Daily` / `Weekly`). |
| `full_update_day` | integer | no |  | Day of week for a weekly full sync (0–6). |
| `full_update_hour` | integer | no |  | Hour of day for the full sync (0–23). |
| `only_monitored` | boolean | no |  | Only manage monitored series. |
| `series_sync_on_live` | boolean | no |  | Keep the series list in sync via SignalR live updates. |
| `series_sync` | integer | no |  | Series-sync frequency, in minutes. |
| `excluded_tags` | array of string | no |  | Sonarr tags to exclude. |
| `excluded_series_types` | array of string | no |  | Sonarr series types to exclude. |
| `use_ffprobe_cache` | boolean | no |  | Cache ffprobe results. |
| `exclude_season_zero` | boolean | no |  | Exclude season 0 (specials). |
| `defer_search_signalr` | boolean | no |  | Defer searches triggered by SignalR events. |
| `sync_only_monitored_series` | boolean | no |  | Only sync monitored series. |
| `sync_only_monitored_episodes` | boolean | no |  | Only sync monitored episodes. |

### Radarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `ip` | string | no |  | Radarr host/IP. |
| `port` | integer | no |  | Radarr port. |
| `base_url` | string | no |  | Radarr base URL (reverse-proxy subpath). |
| `ssl` | boolean | no |  | Use HTTPS to reach Radarr. |
| `http_timeout` | integer | no |  | HTTP request timeout, in seconds. |
| `apikey` | string | no |  | Radarr API key. |
| `full_update` | string | no |  | Full-sync cadence (`Manually` / `Daily` / `Weekly`). |
| `full_update_day` | integer | no |  | Day of week for a weekly full sync (0–6). |
| `full_update_hour` | integer | no |  | Hour of day for the full sync (0–23). |
| `only_monitored` | boolean | no |  | Only manage monitored movies. |
| `movies_sync_on_live` | boolean | no |  | Keep the movie list in sync via SignalR live updates. |
| `movies_sync` | integer | no |  | Movie-sync frequency, in minutes. |
| `excluded_tags` | array of string | no |  | Radarr tags to exclude. |
| `use_ffprobe_cache` | boolean | no |  | Cache ffprobe results. |
| `defer_search_signalr` | boolean | no |  | Defer searches triggered by SignalR events. |
| `sync_only_monitored_movies` | boolean | no |  | Only sync monitored movies. |

### Jellyfin

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `url` | string | no |  | Jellyfin base URL. |
| `apikey` | string | no |  | Jellyfin API key. |
| `update_movie_library` | boolean | no |  | Update the movie library after a subtitle change. |
| `update_series_library` | boolean | no |  | Update the series library after a subtitle change. |
| `movie_library_ids` | array of string | no |  | Movie library ids to refresh. |
| `series_library_ids` | array of string | no |  | Series library ids to refresh. |
| `refresh_method` | string | no |  | Refresh method (`immediate` / `scan`). |

### Proxy

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `kind` | string | no |  | Proxy type: `socks5` / `http`, or unset to disable. Declare it as `kind:` under `proxy:` — it maps to bazarr's `type` key (`type` is a Rust keyword, so the field is named `kind`). |
| `url` | string | no |  | Proxy host. |
| `port` | string | no |  | Proxy port. |
| `username` | string | no |  | Proxy username. |
| `password` | string | no |  | Proxy password. |
| `exclude` | array of string | no |  | Hosts to reach directly, bypassing the proxy. |

### Backup

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `folder` | string | no |  | Directory backups are written to. |
| `retention` | integer | no |  | How many days of backups to retain. |
| `frequency` | string | no |  | Backup cadence (`Manually` / `Daily` / `Weekly`). |
| `day` | integer | no |  | Day of week for a weekly backup (0–6). |
| `hour` | integer | no |  | Hour of day for the backup (0–23). |

### Subsync

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_subsync` | boolean | no |  | Automatically sync subtitles to the audio track after download. |
| `use_subsync_threshold` | boolean | no |  | Only auto-sync when the subtitle score is below a threshold. |
| `subsync_threshold` | integer | no |  | Series score threshold under which to auto-sync. |
| `use_subsync_movie_threshold` | boolean | no |  | Only auto-sync movies when the score is below a threshold. |
| `subsync_movie_threshold` | integer | no |  | Movie score threshold under which to auto-sync. |
| `debug` | boolean | no |  | Keep sync debug output. |
| `force_audio` | boolean | no |  | Force audio-track–based sync. |
| `use_original_language` | boolean | no |  | Sync against the original-language audio. |
| `auto_use_original_language` | boolean | no |  | Automatically prefer the original-language audio for sync. |
| `no_fix_framerate` | boolean | no |  | Don't correct the subtitle framerate. |
| `gss` | boolean | no |  | Use the golden-section search algorithm. |
| `max_offset_seconds` | integer | no |  | Maximum allowed offset, in seconds. |
| `checker` | [`subsync_checker`](#subsync-checker) | no |  | Post-sync quality checker: languages/providers excluded from the sync verification pass. |

### Auth

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `kind` | string | no |  | Authentication method: unset = none, `basic`, or `form`. Declare it as `kind:` under `auth:` — it maps to bazarr's `type` key (`type` is a Rust keyword, so the field is named `kind`). |
| `username` | string | no |  | Login username. |
| `password` | string | no |  | Login password (sent in plaintext; bazarr stores it md5-hashed). |

### Plex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `ip` | string | no |  | Plex host/IP. |
| `port` | integer | no |  | Plex port. |
| `ssl` | boolean | no |  | Use HTTPS to reach Plex. |
| `apikey` | string | no |  | Plex API key. |
| `token` | string | no |  | Plex auth token. |
| `auth_method` | string | no |  | How to authenticate (`apikey` / `oauth`). |
| `update_movie_library` | boolean | no |  | Refresh the movie library after a subtitle change. |
| `update_series_library` | boolean | no |  | Refresh the series library after a subtitle change. |
| `movie_library_ids` | array of string | no |  | Movie library ids to refresh. |
| `series_library_ids` | array of string | no |  | Series library ids to refresh. |
| `set_movie_added` | boolean | no |  | Set the "added" date on movies from the subtitle date. |
| `set_episode_added` | boolean | no |  | Set the "added" date on episodes from the subtitle date. |

### Postgresql

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Use PostgreSQL instead of the bundled SQLite database. |
| `host` | string | no |  | PostgreSQL host. |
| `port` | integer | no |  | PostgreSQL port. |
| `database` | string | no |  | Database name. |
| `username` | string | no |  | Database username. |
| `password` | string | no |  | Database password. |
| `url` | string | no |  | Full connection URL (overrides the discrete host/port/… fields). |

### Translator

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `translator_type` | string | no |  | Translation engine (`google_translate`, `gemini`, `lingarr`, …). |
| `default_score` | integer | no |  | Minimum score a translated subtitle must reach to be kept. |
| `translator_info` | boolean | no |  | Show translation info/attribution in the UI. |
| `gemini_key` | string | no |  | Google Gemini API key (used when `translator_type` is `gemini`). |
| `gemini_model` | string | no |  | Gemini model id (e.g. `gemini-2.0-flash`). |
| `lingarr_url` | string | no |  | Lingarr base URL (used when `translator_type` is `lingarr`). |
| `lingarr_token` | string | no |  | Lingarr API token. |

### Log

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `include_filter` | string | no |  | Only keep log lines matching this filter. |
| `exclude_filter` | string | no |  | Drop log lines matching this filter. |
| `use_regex` | boolean | no |  | Treat the filters as regular expressions. |
| `ignore_case` | boolean | no |  | Match the filters case-insensitively. |

### Addic7ed

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |
| `cookies` | string | no |  | Session cookies. |
| `user_agent` | string | no |  | User-agent to send with cookie auth. |
| `vip` | boolean | no |  | Account has VIP access. |

### Ani Db

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_client` | string | no |  | API client id. |
| `api_client_ver` | integer | no |  | API client version. |

### Anime Tosho

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `search_threshold` | integer | no |  | Search score threshold. |
| `anidb_api_client` | string | no |  | AniDB API client id. |
| `anidb_api_client_ver` | integer | no |  | AniDB API client version. |

### Anti Captcha

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `anti_captcha_key` | string | no |  | Anti-Captcha API key. |

### Assrt

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `token` | string | no |  | API token. |

### Avista Z

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `cookies` | string | no |  | Session cookies. |
| `user_agent` | string | no |  | User-agent to send with cookie auth. |

### Beta Series

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `token` | string | no |  | API token. |

### Captcha Ai

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `captchaai_key` | string | no |  | CaptchaAI API key. |

### Cinema Z

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `cookies` | string | no |  | Session cookies. |
| `user_agent` | string | no |  | User-agent to send with cookie auth. |

### Death By Captcha

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |

### Embedded Subtitles

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `included_codecs` | array of string | no |  | Subtitle codecs to include. |
| `hi_fallback` | boolean | no |  | Fall back to hearing-impaired subtitles. |
| `timeout` | integer | no |  | Request timeout, in seconds. |
| `unknown_as_fallback` | boolean | no |  | Treat unknown-language tracks as a fallback. |
| `fallback_lang` | string | no |  | Fallback language code. |

### Hd Bits

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `passkey` | string | no |  | Account passkey. |

### Jimaku

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | string | no |  | API key. |
| `enable_name_search_fallback` | boolean | no |  | Fall back to name search. |
| `enable_archives_download` | boolean | no |  | Allow downloading subtitle archives. |
| `enable_ai_subs` | boolean | no |  | Enable AI-generated subtitles. |

### Kara Garga

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |
| `f_username` | string | no |  | Forum username. |
| `f_password` | string | no |  | Forum password. |

### Ktuvit

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `email` | string | no |  | Account email. |
| `hashed_password` | string | no |  | Pre-hashed account password (provide the hash bazarr expects). |

### Legendas Divx

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |
| `skip_wrong_fps` | boolean | no |  | Skip subtitles with a mismatched framerate. |

### Legendas Net

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |

### Napi Projekt

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `only_authors` | boolean | no |  | Only match subtitles by known authors. |
| `only_real_names` | boolean | no |  | Only match subtitles with real author names. |

### Napisy24

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |

### Open Subtitles Com

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |
| `use_hash` | boolean | no |  | Match by file hash. |
| `include_ai_translated` | boolean | no |  | Include AI-translated subtitles. |
| `include_machine_translated` | boolean | no |  | Include machine-translated subtitles. |

### Pipocas

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |

### Sub Dl

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | string | no |  | API key. |

### Subf2m

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `verify_ssl` | boolean | no |  | Verify the provider's TLS certificate. |
| `user_agent` | string | no |  | User-agent to send with cookie auth. |

### Subsarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Provider base URL. |

### Sub Source

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `apikey` | string | no |  | API key. |

### Subs Ro

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | string | no |  | API key. |

### Sub X

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | string | no |  | API key. |

### Titlovi

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |

### Titulky

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |
| `approved_only` | boolean | no |  | Only use approved subtitles. |
| `skip_wrong_fps` | boolean | no |  | Skip subtitles with a mismatched framerate. |

### Turkce Altyazi Org

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `cookies` | string | no |  | Session cookies. |
| `user_agent` | string | no |  | User-agent to send with cookie auth. |

### Whisper Ai

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `endpoint` | string | no |  | Service endpoint URL. |
| `response` | integer | no |  | Response timeout, in seconds. |
| `timeout` | integer | no |  | Request timeout, in seconds. |
| `pass_video_name` | boolean | no |  | Pass the video filename to the service. |
| `loglevel` | string | no |  | Log level. |

### X Subs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | no |  | Account username. |
| `password` | string | no |  | Account password. |

### Language Profile

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `profile_id` | integer | yes |  | Stable profile id (the natural key; profiles are replaced by this id). |
| `name` | string | no |  | Display name. |
| `cutoff` | integer | no |  | Cutoff item `id` — stop searching once this language is found (`65535` = any of the profile's languages; unset = never cut off early). |
| `items` | array of [`language_profile_item`](#language-profile-item) | no |  | The languages this profile wants, in priority order. |
| `must_contain` | array of string | no |  | Release must contain all of these strings. |
| `must_not_contain` | array of string | no |  | Release must contain none of these strings. |
| `original_format` | boolean | no |  | Prefer the original-format subtitle. Stored by bazarr as the int `0`/`1` (or null when unset). |
| `tag` | string | no |  | Sonarr/Radarr tag this profile is scoped to. |

### Notifier

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider name — must match one of bazarr's built-in providers exactly (e.g. `"Discord"`, `"Telegram"`, `"Pushover"`). |
| `enabled` | boolean | no | `false` | Whether this provider is active. Omitted = disabled. |
| `url` | string | no |  | Apprise notification URL (e.g. `discord://webhook_id/webhook_token`). Omitted = cleared (encoded as an explicit `null`, matching bazarr's store). |

### Subsync Checker

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `blacklisted_languages` | array of string | no |  | Language codes excluded from the post-sync quality check. |
| `blacklisted_providers` | array of string | no |  | Provider ids excluded from the post-sync quality check. |

### Language Profile Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | yes |  | Item id (unique within the profile; referenced by the profile `cutoff`). |
| `language` | string | yes |  | Language code (alpha-2, e.g. `en`). |
| `forced` | string | no | `False` | Match forced subtitles: `"True"` / `"False"` / `"Both"`. |
| `hi` | string | no | `False` | Match hearing-impaired subtitles: `"True"` / `"False"` / `"Both"`. |
| `audio_exclude` | string | no | `False` | Skip when this language is present in the audio: `"True"` / `"False"`. |
| `audio_only_include` | string | no | `False` | Only include when this language is the audio: `"True"` / `"False"`. |

