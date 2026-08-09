# LazyLibrarian v1 Configuration

LazyLibrarian v1 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Config

LazyLibrarian configuration — every managed section (all optional).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `general` | [`general`](#general) | no |  | The `[General]` section. |
| `api` | [`api`](#api) | no |  | The `[Api]` section. |
| `proxy` | [`proxy`](#proxy) | no |  | The `[Proxy]` section. |
| `postprocess` | [`postprocess`](#postprocess) | no |  | The `[Postprocess]` section. |
| `web_server` | [`web_server`](#web-server) | no |  | The `[Webserver]` section. |
| `logging` | [`logging`](#logging) | no |  | The `[Logging]` section. |
| `importer` | [`importer`](#importer) | no |  | The `[Importer]` section. |
| `calibre` | [`calibre`](#calibre) | no |  | The `[Calibre]` section. |
| `sabnzbd` | [`sabnzbd`](#sabnzbd) | no |  | The `[Sabnzbd]` section. |
| `nzbget` | [`nzbget`](#nzbget) | no |  | The `[Nzbget]` section. |
| `usenet` | [`usenet`](#usenet) | no |  | The `[Usenet]` section. |
| `nzbmatrix` | [`nzbmatrix`](#nzbmatrix) | no |  | The `[Nzbmatrix]` section. |
| `torrent` | [`torrent`](#torrent) | no |  | The `[Torrent]` section. |
| `rtorrent` | [`rtorrent`](#rtorrent) | no |  | The `[Rtorrent]` section. |
| `utorrent` | [`utorrent`](#utorrent) | no |  | The `[Utorrent]` section. |
| `qbittorrent` | [`qbittorrent`](#qbittorrent) | no |  | The `[Qbittorrent]` section. |
| `transmission` | [`transmission`](#transmission) | no |  | The `[Transmission]` section. |
| `deluge` | [`deluge`](#deluge) | no |  | The `[Deluge]` section. |
| `synology` | [`synology`](#synology) | no |  | The `[Synology]` section. |
| `abb` | [`abb`](#abb) | no |  | The `[Abb]` section. |
| `kat` | [`kat`](#kat) | no |  | The `[Kat]` section. |
| `tpb` | [`tpb`](#tpb) | no |  | The `[Tpb]` section. |
| `tdl` | [`tdl`](#tdl) | no |  | The `[Tdl]` section. |
| `bok` | [`bok`](#bok) | no |  | The `[Bok]` section. |
| `slsk` | [`slsk`](#slsk) | no |  | The `[Slsk]` section. |
| `anna` | [`anna`](#anna) | no |  | The `[Anna]` section. |
| `lime` | [`lime`](#lime) | no |  | The `[Lime]` section. |
| `newzbin` | [`newzbin`](#newzbin) | no |  | The `[Newzbin]` section. |
| `searchscan` | [`searchscan`](#searchscan) | no |  | The `[Searchscan]` section. |
| `libraryscan` | [`libraryscan`](#libraryscan) | no |  | The `[Libraryscan]` section. |
| `comics` | [`comics`](#comics) | no |  | The `[Comics]` section. |
| `magazines` | [`magazines`](#magazines) | no |  | The `[Magazines]` section. |
| `twitter` | [`twitter`](#twitter) | no |  | The `[Twitter]` section. |
| `boxcar` | [`boxcar`](#boxcar) | no |  | The `[Boxcar]` section. |
| `pushbullet` | [`pushbullet`](#pushbullet) | no |  | The `[Pushbullet]` section. |
| `pushover` | [`pushover`](#pushover) | no |  | The `[Pushover]` section. |
| `androidpn` | [`android_pn`](#android-pn) | no |  | The `[Androidpn]` section. |
| `telegram` | [`telegram`](#telegram) | no |  | The `[Telegram]` section. |
| `prowl` | [`prowl`](#prowl) | no |  | The `[Prowl]` section. |
| `growl` | [`growl`](#growl) | no |  | The `[Growl]` section. |
| `slack` | [`slack`](#slack) | no |  | The `[Slack]` section. |
| `custom` | [`custom`](#custom) | no |  | The `[Custom]` section. |
| `email` | [`email`](#email) | no |  | The `[Email]` section. |
| `fmt` | [`fmt`](#fmt) | no |  | The `[Fmt]` section. |
| `opds` | [`opds`](#opds) | no |  | The `[Opds]` section. |
| `rss_config` | [`rss_config`](#rss-config) | no |  | The `[Rss]` section. |
| `preprocess` | [`preprocess`](#preprocess) | no |  | The `[Preprocess]` section. |

### Newznab Provider

A Newznab (usenet) provider.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `dispname` | string | yes |  | Display name — its identity (`DISPNAME`). |
| `enabled` | boolean | no |  | Whether the provider is enabled. |
| `host` | string | no |  | Base host URL. |
| `api` | secret string | no |  | Provider API key. Credential — redacted in plan output. |
| `book_cat` | string | no |  | eBook category ids (CSV). |
| `mag_cat` | string | no |  | Magazine category ids (CSV). |
| `audio_cat` | string | no |  | Audiobook category ids (CSV). |
| `comic_cat` | string | no |  | Comic category ids (CSV). |
| `extended` | integer | no |  | Use the extended API (`1`/`0`). |
| `api_limit` | integer | no |  | Daily API call limit (`0` = unlimited). |
| `dl_priority` | integer | no |  | Download priority (lower = higher). |
| `dl_types` | string | no |  | Download types this provider serves (CSV of `A`,`E`,`M`,`C`). |

### Torznab Provider

A Torznab (torrent-over-newznab) provider.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `dispname` | string | yes |  | Display name — its identity (`DISPNAME`). |
| `enabled` | boolean | no |  | Whether the provider is enabled. |
| `host` | string | no |  | Base host URL. |
| `api` | secret string | no |  | Provider API key. Credential — redacted in plan output. |
| `book_cat` | string | no |  | eBook category ids (CSV). |
| `mag_cat` | string | no |  | Magazine category ids (CSV). |
| `audio_cat` | string | no |  | Audiobook category ids (CSV). |
| `comic_cat` | string | no |  | Comic category ids (CSV). |
| `extended` | integer | no |  | Use the extended API (`1`/`0`). |
| `dl_priority` | integer | no |  | Download priority (lower = higher). |
| `dl_types` | string | no |  | Download types this provider serves (CSV of `A`,`E`,`M`,`C`). |
| `seeders` | integer | no |  | Minimum seeders to accept a torrent. |
| `seed_ratio` | number | no |  | Required seed ratio. |
| `seed_duration` | integer | no |  | Required seed duration (minutes). |

### Rss Provider

An RSS / wishlist provider.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `dispname` | string | yes |  | Display name — its identity (`DISPNAME`). |
| `enabled` | boolean | no |  | Whether the provider is enabled. |
| `host` | string | no |  | Feed host URL. |
| `dl_priority` | integer | no |  | Download priority (lower = higher). |
| `dl_types` | string | no |  | Download types this provider serves (CSV of `A`,`E`,`M`,`C`). |
| `label` | string | no |  | Download-client label to tag grabs from this feed. |

### Irc Provider

An IRC provider.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `dispname` | string | yes |  | Display name — its identity (`DISPNAME`). |
| `enabled` | boolean | no |  | Whether the provider is enabled. |
| `server` | string | no |  | IRC server host. |
| `channel` | string | no |  | Channel to join. |
| `bot_nick` | string | no |  | Bot nick to search under. |
| `search` | string | no |  | Search command prefix (default `@search`). |
| `dl_priority` | integer | no |  | Download priority (lower = higher). |
| `dl_types` | string | no |  | Download types this provider serves (CSV of `A`,`E`,`M`,`C`). |

### Gen Provider

A direct/generic (GEN) provider.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `dispname` | string | yes |  | Display name — its identity (`DISPNAME`). |
| `enabled` | boolean | no |  | Whether the provider is enabled. |
| `host` | string | no |  | Base host URL. |
| `search` | string | no |  | Search path/template. |
| `dl_priority` | integer | no |  | Download priority (lower = higher). |
| `dl_types` | string | no |  | Download types this provider serves (CSV of `A`,`E`,`M`,`C`). |

### Magazine

A tracked magazine.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `title` | string | yes |  | Magazine title — its identity (wire/JSON key `Title`). |

### Author

A tracked author.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Author name — its identity (wire/JSON key `AuthorName`). |
| `add_books` | boolean | no |  | Also fetch the author's books when first added (`&books`). Create-time hint only — not part of the author's persisted identity. |

## Types

### General

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `ol_url` | string | no |  | `OL_URL` |
| `gr_url` | string | no |  | `GR_URL` |
| `gb_url` | string | no |  | `GB_URL` |
| `lt_url` | string | no |  | `LT_URL` |
| `cv_url` | string | no |  | `CV_URL` |
| `cx_url` | string | no |  | `CX_URL` |
| `show_newz_prov` | boolean | no |  | `SHOW_NEWZ_PROV` |
| `show_torz_prov` | boolean | no |  | `SHOW_TORZ_PROV` |
| `show_tor_prov` | boolean | no |  | `SHOW_TOR_PROV` |
| `show_rss_prov` | boolean | no |  | `SHOW_RSS_PROV` |
| `show_irc_prov` | boolean | no |  | `SHOW_IRC_PROV` |
| `show_gen_prov` | boolean | no |  | `SHOW_GEN_PROV` |
| `show_direct_prov` | boolean | no |  | `SHOW_DIRECT_PROV` |
| `user_accounts` | boolean | no |  | `USER_ACCOUNTS` |
| `single_user` | boolean | no |  | `SINGLE_USER` |
| `admin_email` | string | no |  | `ADMIN_EMAIL` |
| `whitelist` | string | no |  | `WHITELIST` |
| `sys_encoding` | string | no |  | `SYS_ENCODING` |
| `homepage` | string | no |  | `HOMEPAGE` |
| `auth_type` | string | no |  | `AUTH_TYPE` |
| `wall_columns` | integer | no |  | `WALL_COLUMNS` |
| `file_perm` | string | no |  | `FILE_PERM` |
| `dir_perm` | string | no |  | `DIR_PERM` |
| `blocklist_timer` | integer | no |  | `BLOCKLIST_TIMER` |
| `max_pages` | integer | no |  | `MAX_PAGES` |
| `max_bookpages` | integer | no |  | `MAX_BOOKPAGES` |
| `max_wall` | integer | no |  | `MAX_WALL` |
| `match_ratio` | integer | no |  | `MATCH_RATIO` |
| `dload_ratio` | integer | no |  | `DLOAD_RATIO` |
| `name_ratio` | integer | no |  | `NAME_RATIO` |
| `name_partial` | integer | no |  | `NAME_PARTIAL` |
| `name_partname` | integer | no |  | `NAME_PARTNAME` |
| `displaylength` | integer | no |  | `DISPLAYLENGTH` |
| `hist_refresh` | integer | no |  | `HIST_REFRESH` |
| `no_ipv6` | boolean | no |  | `NO_IPV6` |
| `bookstrap_theme` | string | no |  | `BOOKSTRAP_THEME` |
| `series_tab` | boolean | no |  | `SERIES_TAB` |
| `audio_tab` | boolean | no |  | `AUDIO_TAB` |
| `ebook_tab` | boolean | no |  | `EBOOK_TAB` |
| `config_tab_num` | integer | no |  | `CONFIG_TAB_NUM` |
| `toggles` | boolean | no |  | `TOGGLES` |
| `sort_definite` | boolean | no |  | `SORT_DEFINITE` |
| `sort_surname` | boolean | no |  | `SORT_SURNAME` |
| `show_reason` | boolean | no |  | `SHOW_REASON` |
| `show_genres` | boolean | no |  | `SHOW_GENRES` |
| `ignore_paused` | boolean | no |  | `IGNORE_PAUSED` |
| `launch_browser` | boolean | no |  | `LAUNCH_BROWSER` |
| `name_postfix` | string | no |  | `NAME_POSTFIX` |
| `name_definite` | string | no |  | `NAME_DEFINITE` |
| `multi_author_split` | string | no |  | `MULTI_AUTHOR_SPLIT` |
| `google_trans_id` | boolean | no |  | `GOOGLE_TRANS_ID` |
| `imp_preflang` | string | no |  | `IMP_PREFLANG` |
| `pref_maglang` | string | no |  | `PREF_MAGLANG` |
| `iss_format` | string | no |  | `ISS_FORMAT` |
| `date_format` | string | no |  | `DATE_FORMAT` |
| `date_lang` | string | no |  | `DATE_LANG` |
| `author_date_format` | string | no |  | `AUTHOR_DATE_FORMAT` |
| `issue_nouns` | string | no |  | `ISSUE_NOUNS` |
| `volume_nouns` | string | no |  | `VOLUME_NOUNS` |
| `imp_monthlang` | string | no |  | `IMP_MONTHLANG` |
| `imp_autoadd` | string | no |  | `IMP_AUTOADD` |
| `imp_autoadd_copy` | boolean | no |  | `IMP_AUTOADD_COPY` |
| `imp_autoadd_bookonly` | boolean | no |  | `IMP_AUTOADD_BOOKONLY` |
| `imp_autosearch` | boolean | no |  | `IMP_AUTOSEARCH` |
| `blacklist_failed` | boolean | no |  | `BLACKLIST_FAILED` |
| `blacklist_processed` | boolean | no |  | `BLACKLIST_PROCESSED` |
| `ssl_certs` | string | no |  | `SSL_CERTS` |
| `ssl_verify` | boolean | no |  | `SSL_VERIFY` |
| `http_timeout` | integer | no |  | `HTTP_TIMEOUT` |
| `http_ext_timeout` | integer | no |  | `HTTP_EXT_TIMEOUT` |
| `imp_singlebook` | boolean | no |  | `IMP_SINGLEBOOK` |
| `imp_rename` | boolean | no |  | `IMP_RENAME` |
| `mag_rename` | boolean | no |  | `MAG_RENAME` |
| `imp_comicopf` | boolean | no |  | `IMP_COMICOPF` |
| `imp_comiccover` | boolean | no |  | `IMP_COMICCOVER` |
| `imp_convert` | string | no |  | `IMP_CONVERT` |
| `imp_nosplit` | string | no |  | `IMP_NOSPLIT` |
| `ext_preprocess` | string | no |  | `EXT_PREPROCESS` |
| `git_program` | string | no |  | `GIT_PROGRAM` |
| `cache_age` | integer | no |  | `CACHE_AGE` |
| `backup_db` | integer | no |  | `BACKUP_DB` |
| `task_age` | integer | no |  | `TASK_AGE` |
| `opf_tags` | boolean | no |  | `OPF_TAGS` |
| `genre_tags` | boolean | no |  | `GENRE_TAGS` |
| `wishlist_tags` | boolean | no |  | `WISHLIST_TAGS` |
| `wishlist_genres` | boolean | no |  | `WISHLIST_GENRES` |
| `notify_with_title` | boolean | no |  | `NOTIFY_WITH_TITLE` |
| `notify_with_url` | boolean | no |  | `NOTIFY_WITH_URL` |
| `destination_copy` | boolean | no |  | `DESTINATION_COPY` |
| `ebook_dir` | string | no |  | `EBOOK_DIR` |
| `audio_dir` | string | no |  | `AUDIO_DIR` |
| `alternate_dir` | string | no |  | `ALTERNATE_DIR` |
| `testdata_dir` | string | no |  | `TESTDATA_DIR` |
| `delete_csv` | boolean | no |  | `DELETE_CSV` |
| `download_dir` | string | no |  | `DOWNLOAD_DIR` |
| `ebook_type` | string | no |  | `EBOOK_TYPE` |
| `audiobook_type` | string | no |  | `AUDIOBOOK_TYPE` |
| `mag_type` | string | no |  | `MAG_TYPE` |
| `reject_publisher` | string | no |  | `REJECT_PUBLISHER` |
| `reject_words` | string | no |  | `REJECT_WORDS` |
| `prefer_words` | string | no |  | `PREFER_WORDS` |
| `reject_audio` | string | no |  | `REJECT_AUDIO` |
| `mag_age` | integer | no |  | `MAG_AGE` |
| `hide_old_notifiers` | boolean | no |  | `HIDE_OLD_NOTIFIERS` |
| `pref_unrarlib` | integer | no |  | `PREF_UNRARLIB` |
| `user_agent` | string | no |  | `USER_AGENT` |
| `ratestars` | boolean | no |  | `RATESTARS` |

### Api

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | `API_ENABLED` |
| `key` | secret string | no |  | `API_KEY` Credential — redacted in plan output. |
| `ro_key` | secret string | no |  | `API_RO_KEY` Credential — redacted in plan output. |
| `book_api` | string | no |  | `BOOK_API` |
| `lt_devkey` | secret string | no |  | `LT_DEVKEY` Credential — redacted in plan output. |
| `cv_websearch` | boolean | no |  | `CV_WEBSEARCH` |
| `ol_api` | boolean | no |  | `OL_API` |
| `dnb_api` | boolean | no |  | `DNB_API` |
| `hc_api` | boolean | no |  | `HC_API` |
| `hc_sync` | boolean | no |  | `HC_SYNC` |
| `hc_syncreadonly` | boolean | no |  | `HC_SYNCREADONLY` |
| `hc_sync_limit` | integer | no |  | `HC_SYNC_LIMIT` |
| `gr_api` | secret string | no |  | `GR_API` Credential — redacted in plan output. |
| `gr_sync` | boolean | no |  | `GR_SYNC` |
| `gr_syncuser` | boolean | no |  | `GR_SYNCUSER` |
| `gr_user` | string | no |  | `GR_USER` |
| `gr_syncreadonly` | boolean | no |  | `GR_SYNCREADONLY` |
| `gr_secret` | secret string | no |  | `GR_SECRET` Credential — redacted in plan output. |
| `gr_oauth_token` | secret string | no |  | `GR_OAUTH_TOKEN` Credential — redacted in plan output. |
| `gr_oauth_secret` | secret string | no |  | `GR_OAUTH_SECRET` Credential — redacted in plan output. |
| `gr_wanted` | string | no |  | `GR_WANTED` |
| `gr_owned` | string | no |  | `GR_OWNED` |
| `gr_awanted` | string | no |  | `GR_AWANTED` |
| `gr_aowned` | string | no |  | `GR_AOWNED` |
| `gr_unique` | boolean | no |  | `GR_UNIQUE` |
| `gr_follow` | boolean | no |  | `GR_FOLLOW` |
| `gr_follownew` | boolean | no |  | `GR_FOLLOWNEW` |
| `gb_api` | secret string | no |  | `GB_API` Credential — redacted in plan output. |
| `gb_country` | string | no |  | `GB_COUNTRY` |

### Proxy

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `PROXY_HOST` |
| `type_` | string | no |  | `PROXY_TYPE` |
| `local` | string | no |  | `PROXY_LOCAL` |
| `auth` | boolean | no |  | `PROXY_AUTH` |
| `register` | boolean | no |  | `PROXY_REGISTER` |
| `auth_user` | string | no |  | `PROXY_AUTH_USER` |
| `auth_email` | string | no |  | `PROXY_AUTH_EMAIL` |
| `auth_name` | string | no |  | `PROXY_AUTH_NAME` |

### Postprocess

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `skipped_ext` | string | no |  | `SKIPPED_EXT` |
| `banned_ext` | string | no |  | `BANNED_EXT` |
| `create_link` | string | no |  | `CREATE_LINK` |
| `ebook_dest_folder` | string | no |  | `EBOOK_DEST_FOLDER` |
| `ebook_dest_file` | string | no |  | `EBOOK_DEST_FILE` |
| `audiobook_dest_file` | string | no |  | `AUDIOBOOK_DEST_FILE` |
| `audiobook_single_file` | string | no |  | `AUDIOBOOK_SINGLE_FILE` |
| `audiobook_dest_folder` | string | no |  | `AUDIOBOOK_DEST_FOLDER` |
| `one_format` | boolean | no |  | `ONE_FORMAT` |
| `del_downloadfailed` | boolean | no |  | `DEL_DOWNLOADFAILED` |
| `pp_delay` | integer | no |  | `PP_DELAY` |
| `del_failed` | boolean | no |  | `DEL_FAILED` |
| `del_completed` | boolean | no |  | `DEL_COMPLETED` |

### Web Server

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `http_port` | integer | no |  | `HTTP_PORT` |
| `http_host` | string | no |  | `HTTP_HOST` |
| `http_user` | string | no |  | `HTTP_USER` |
| `http_pass` | secret string | no |  | `HTTP_PASS` Credential — redacted in plan output. |
| `http_proxy` | boolean | no |  | `HTTP_PROXY` |
| `http_root` | string | no |  | `HTTP_ROOT` |
| `http_look` | string | no |  | `HTTP_LOOK` |
| `https_enabled` | boolean | no |  | `HTTPS_ENABLED` |
| `https_cert` | string | no |  | `HTTPS_CERT` |
| `https_key` | secret string | no |  | `HTTPS_KEY` Credential — redacted in plan output. |

### Logging

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `logdir` | string | no |  | `LOGDIR` |
| `loglimit` | integer | no |  | `LOGLIMIT` |
| `logfiles` | integer | no |  | `LOGFILES` |
| `logsize` | integer | no |  | `LOGSIZE` |
| `detaileduilog` | boolean | no |  | `DETAILEDUILOG` |
| `logredact` | boolean | no |  | `LOGREDACT` |
| `hostredact` | boolean | no |  | `HOSTREDACT` |
| `logfileredact` | boolean | no |  | `LOGFILEREDACT` |
| `loglevel` | integer | no |  | `LOGLEVEL` |
| `logspecialdebug` | string | no |  | `LOGSPECIALDEBUG` |

### Importer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `multi_source` | boolean | no |  | `MULTI_SOURCE` |

### Calibre

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_server` | boolean | no |  | `CALIBRE_USE_SERVER` |
| `server` | string | no |  | `CALIBRE_SERVER` |
| `user` | string | no |  | `CALIBRE_USER` |
| `pass_` | secret string | no |  | `CALIBRE_PASS` Credential — redacted in plan output. |
| `rename` | boolean | no |  | `CALIBRE_RENAME` |
| `imp_calibredb` | string | no |  | `IMP_CALIBREDB` |
| `imp_calibreoverwrite` | boolean | no |  | `IMP_CALIBREOVERWRITE` |
| `imp_calibre_ebook` | boolean | no |  | `IMP_CALIBRE_EBOOK` |
| `imp_calibre_comic` | boolean | no |  | `IMP_CALIBRE_COMIC` |
| `imp_calibre_magazine` | boolean | no |  | `IMP_CALIBRE_MAGAZINE` |
| `imp_calibre_magtitle` | boolean | no |  | `IMP_CALIBRE_MAGTITLE` |
| `imp_calibre_magissue` | boolean | no |  | `IMP_CALIBRE_MAGISSUE` |

### Sabnzbd

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `sab_host` | string | no |  | `SAB_HOST` |
| `sab_port` | integer | no |  | `SAB_PORT` |
| `sab_subdir` | string | no |  | `SAB_SUBDIR` |
| `sab_user` | string | no |  | `SAB_USER` |
| `sab_pass` | secret string | no |  | `SAB_PASS` Credential — redacted in plan output. |
| `sab_api` | secret string | no |  | `SAB_API` Credential — redacted in plan output. |
| `sab_cat` | string | no |  | `SAB_CAT` |
| `sab_delete` | boolean | no |  | `SAB_DELETE` |
| `sab_external_host` | string | no |  | `SAB_EXTERNAL_HOST` |
| `sab_remote` | string | no |  | `SAB_REMOTE` |
| `sab_local` | string | no |  | `SAB_LOCAL` |

### Nzbget

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `NZBGET_HOST` |
| `port` | integer | no |  | `NZBGET_PORT` |
| `user` | string | no |  | `NZBGET_USER` |
| `pass_` | secret string | no |  | `NZBGET_PASS` Credential — redacted in plan output. |
| `category` | string | no |  | `NZBGET_CATEGORY` |
| `priority` | integer | no |  | `NZBGET_PRIORITY` |
| `remote` | string | no |  | `NZBGET_REMOTE` |
| `local` | string | no |  | `NZBGET_LOCAL` |

### Usenet

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_downloader_sabnzbd` | boolean | no |  | `NZB_DOWNLOADER_SABNZBD` |
| `nzb_downloader_nzbget` | boolean | no |  | `NZB_DOWNLOADER_NZBGET` |
| `nzb_downloader_synology` | boolean | no |  | `NZB_DOWNLOADER_SYNOLOGY` |
| `nzb_downloader_blackhole` | boolean | no |  | `NZB_DOWNLOADER_BLACKHOLE` |
| `nzb_blackholedir` | string | no |  | `NZB_BLACKHOLEDIR` |
| `nzb_paused` | boolean | no |  | `NZB_PAUSED` |
| `retention` | integer | no |  | `USENET_RETENTION` |

### Nzbmatrix

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `user` | string | no |  | `NZBMATRIX_USER` |
| `api` | secret string | no |  | `NZBMATRIX_API` Credential — redacted in plan output. |
| `nzbmatrix` | boolean | no |  | `NZBMATRIX` |

### Torrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tor_downloader_blackhole` | boolean | no |  | `TOR_DOWNLOADER_BLACKHOLE` |
| `tor_convert_magnet` | boolean | no |  | `TOR_CONVERT_MAGNET` |
| `tor_downloader_utorrent` | boolean | no |  | `TOR_DOWNLOADER_UTORRENT` |
| `tor_downloader_rtorrent` | boolean | no |  | `TOR_DOWNLOADER_RTORRENT` |
| `tor_downloader_qbittorrent` | boolean | no |  | `TOR_DOWNLOADER_QBITTORRENT` |
| `tor_downloader_transmission` | boolean | no |  | `TOR_DOWNLOADER_TRANSMISSION` |
| `tor_downloader_synology` | boolean | no |  | `TOR_DOWNLOADER_SYNOLOGY` |
| `tor_downloader_deluge` | boolean | no |  | `TOR_DOWNLOADER_DELUGE` |
| `paused` | boolean | no |  | `TORRENT_PAUSED` |
| `numberofseeders` | integer | no |  | `NUMBEROFSEEDERS` |
| `keep_seeding` | boolean | no |  | `KEEP_SEEDING` |
| `seed_wait` | boolean | no |  | `SEED_WAIT` |
| `prefer_magnet` | boolean | no |  | `PREFER_MAGNET` |
| `dir` | string | no |  | `TORRENT_DIR` |

### Rtorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `RTORRENT_HOST` |
| `user` | string | no |  | `RTORRENT_USER` |
| `pass_` | secret string | no |  | `RTORRENT_PASS` Credential — redacted in plan output. |
| `label` | string | no |  | `RTORRENT_LABEL` |
| `dir` | string | no |  | `RTORRENT_DIR` |
| `remote` | string | no |  | `RTORRENT_REMOTE` |
| `local` | string | no |  | `RTORRENT_LOCAL` |

### Utorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `UTORRENT_HOST` |
| `port` | integer | no |  | `UTORRENT_PORT` |
| `base` | string | no |  | `UTORRENT_BASE` |
| `user` | string | no |  | `UTORRENT_USER` |
| `pass_` | secret string | no |  | `UTORRENT_PASS` Credential — redacted in plan output. |
| `label` | string | no |  | `UTORRENT_LABEL` |
| `remote` | string | no |  | `UTORRENT_REMOTE` |
| `local` | string | no |  | `UTORRENT_LOCAL` |

### Qbittorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `QBITTORRENT_HOST` |
| `port` | integer | no |  | `QBITTORRENT_PORT` |
| `base` | string | no |  | `QBITTORRENT_BASE` |
| `user` | string | no |  | `QBITTORRENT_USER` |
| `pass_` | secret string | no |  | `QBITTORRENT_PASS` Credential — redacted in plan output. |
| `label` | string | no |  | `QBITTORRENT_LABEL` |
| `dir` | string | no |  | `QBITTORRENT_DIR` |
| `remote` | string | no |  | `QBITTORRENT_REMOTE` |
| `local` | string | no |  | `QBITTORRENT_LOCAL` |
| `ignore_ssl` | boolean | no |  | `QBITTORRENT_IGNORE_SSL` |

### Transmission

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `TRANSMISSION_HOST` |
| `base` | string | no |  | `TRANSMISSION_BASE` |
| `port` | integer | no |  | `TRANSMISSION_PORT` |
| `user` | string | no |  | `TRANSMISSION_USER` |
| `pass_` | secret string | no |  | `TRANSMISSION_PASS` Credential — redacted in plan output. |
| `dir` | string | no |  | `TRANSMISSION_DIR` |
| `label` | string | no |  | `TRANSMISSION_LABEL` |
| `remote` | string | no |  | `TRANSMISSION_REMOTE` |
| `local` | string | no |  | `TRANSMISSION_LOCAL` |

### Deluge

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `cert` | string | no |  | `DELUGE_CERT` |
| `host` | string | no |  | `DELUGE_HOST` |
| `base` | string | no |  | `DELUGE_BASE` |
| `port` | integer | no |  | `DELUGE_PORT` |
| `user` | string | no |  | `DELUGE_USER` |
| `pass_` | secret string | no |  | `DELUGE_PASS` Credential — redacted in plan output. |
| `label` | string | no |  | `DELUGE_LABEL` |
| `dir` | string | no |  | `DELUGE_DIR` |
| `timeout` | integer | no |  | `DELUGE_TIMEOUT` |
| `remote` | string | no |  | `DELUGE_REMOTE` |
| `local` | string | no |  | `DELUGE_LOCAL` |

### Synology

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `SYNOLOGY_HOST` |
| `port` | integer | no |  | `SYNOLOGY_PORT` |
| `user` | string | no |  | `SYNOLOGY_USER` |
| `pass_` | secret string | no |  | `SYNOLOGY_PASS` Credential — redacted in plan output. |
| `dir` | string | no |  | `SYNOLOGY_DIR` |
| `use_synology` | boolean | no |  | `USE_SYNOLOGY` |
| `remote` | string | no |  | `SYNOLOGY_REMOTE` |
| `local` | string | no |  | `SYNOLOGY_LOCAL` |

### Abb

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `ABB_HOST` |
| `abb` | boolean | no |  | `ABB` |
| `dlpriority` | integer | no |  | `ABB_DLPRIORITY` |
| `dltypes` | string | no |  | `ABB_DLTYPES` |
| `seeders` | integer | no |  | `ABB_SEEDERS` |

### Kat

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `KAT_HOST` |
| `kat` | boolean | no |  | `KAT` |
| `dlpriority` | integer | no |  | `KAT_DLPRIORITY` |
| `dltypes` | string | no |  | `KAT_DLTYPES` |
| `seeders` | integer | no |  | `KAT_SEEDERS` |

### Tpb

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `TPB_HOST` |
| `tpb` | boolean | no |  | `TPB` |
| `dlpriority` | integer | no |  | `TPB_DLPRIORITY` |
| `dltypes` | string | no |  | `TPB_DLTYPES` |
| `seeders` | integer | no |  | `TPB_SEEDERS` |

### Tdl

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `TDL_HOST` |
| `tdl` | boolean | no |  | `TDL` |
| `dlpriority` | integer | no |  | `TDL_DLPRIORITY` |
| `dltypes` | string | no |  | `TDL_DLTYPES` |
| `seeders` | integer | no |  | `TDL_SEEDERS` |

### Bok

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `BOK_HOST` |
| `pass_` | secret string | no |  | `BOK_PASS` Credential — redacted in plan output. |
| `email` | string | no |  | `BOK_EMAIL` |
| `remix_userid` | string | no |  | `BOK_REMIX_USERID` |
| `remix_userkey` | string | no |  | `BOK_REMIX_USERKEY` |
| `search_lang` | string | no |  | `BOK_SEARCH_LANG` |
| `bok` | boolean | no |  | `BOK` |
| `dlpriority` | integer | no |  | `BOK_DLPRIORITY` |
| `dllimit` | integer | no |  | `BOK_DLLIMIT` |
| `dltypes` | string | no |  | `BOK_DLTYPES` |

### Slsk

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `SLSK_HOST` |
| `api` | secret string | no |  | `SLSK_API` Credential — redacted in plan output. |
| `urlbase` | string | no |  | `SLSK_URLBASE` |
| `slsk` | boolean | no |  | `SLSK` |
| `dlpriority` | integer | no |  | `SLSK_DLPRIORITY` |
| `remote` | string | no |  | `SLSK_REMOTE` |
| `local` | string | no |  | `SLSK_LOCAL` |
| `dltypes` | string | no |  | `SLSK_DLTYPES` |

### Anna

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `ANNA_HOST` |
| `key` | secret string | no |  | `ANNA_KEY` Credential — redacted in plan output. |
| `anna` | boolean | no |  | `ANNA` |
| `dlpriority` | integer | no |  | `ANNA_DLPRIORITY` |
| `dllimit` | integer | no |  | `ANNA_DLLIMIT` |
| `max_servers` | integer | no |  | `ANNA_MAX_SERVERS` |
| `search_lang` | string | no |  | `ANNA_SEARCH_LANG` |
| `dltypes` | string | no |  | `ANNA_DLTYPES` |

### Lime

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | `LIME_HOST` |
| `lime` | boolean | no |  | `LIME` |
| `dlpriority` | integer | no |  | `LIME_DLPRIORITY` |
| `dltypes` | string | no |  | `LIME_DLTYPES` |
| `seeders` | integer | no |  | `LIME_SEEDERS` |

### Newzbin

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `uid` | string | no |  | `NEWZBIN_UID` |
| `pass_` | secret string | no |  | `NEWZBIN_PASS` Credential — redacted in plan output. |
| `newzbin` | boolean | no |  | `NEWZBIN` |

### Searchscan

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `search_bookinterval` | integer | no |  | `SEARCH_BOOKINTERVAL` |
| `search_maginterval` | integer | no |  | `SEARCH_MAGINTERVAL` |
| `scan_interval` | integer | no |  | `SCAN_INTERVAL` |
| `searchrss_interval` | integer | no |  | `SEARCHRSS_INTERVAL` |
| `wishlist_interval` | integer | no |  | `WISHLIST_INTERVAL` |
| `search_comicinterval` | integer | no |  | `SEARCH_COMICINTERVAL` |
| `versioncheck_interval` | integer | no |  | `VERSIONCHECK_INTERVAL` |
| `goodreads_interval` | integer | no |  | `GOODREADS_INTERVAL` |
| `hardcover_interval` | integer | no |  | `HARDCOVER_INTERVAL` |
| `clean_cache_interval` | integer | no |  | `CLEAN_CACHE_INTERVAL` |
| `backup_interval` | integer | no |  | `BACKUP_INTERVAL` |
| `authorupdate_interval` | integer | no |  | `AUTHORUPDATE_INTERVAL` |
| `seriesupdate_interval` | integer | no |  | `SERIESUPDATE_INTERVAL` |
| `delaysearch` | boolean | no |  | `DELAYSEARCH` |
| `search_ratelimit` | integer | no |  | `SEARCH_RATELIMIT` |

### Libraryscan

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `full_scan` | boolean | no |  | `FULL_SCAN` |
| `add_author` | boolean | no |  | `ADD_AUTHOR` |
| `add_series` | boolean | no |  | `ADD_SERIES` |
| `notfound_status` | string | no |  | `NOTFOUND_STATUS` |
| `found_status` | string | no |  | `FOUND_STATUS` |
| `no_single_book_series` | boolean | no |  | `NO_SINGLE_BOOK_SERIES` |
| `no_noninteger_series` | boolean | no |  | `NO_NONINTEGER_SERIES` |
| `newseries_status` | string | no |  | `NEWSERIES_STATUS` |
| `newbook_status` | string | no |  | `NEWBOOK_STATUS` |
| `newaudio_status` | string | no |  | `NEWAUDIO_STATUS` |
| `newauthor_status` | string | no |  | `NEWAUTHOR_STATUS` |
| `newauthor_audio` | string | no |  | `NEWAUTHOR_AUDIO` |
| `newauthor_books` | boolean | no |  | `NEWAUTHOR_BOOKS` |
| `no_future` | boolean | no |  | `NO_FUTURE` |
| `no_pubdate` | boolean | no |  | `NO_PUBDATE` |
| `no_isbn` | boolean | no |  | `NO_ISBN` |
| `no_sets` | boolean | no |  | `NO_SETS` |
| `no_lang` | boolean | no |  | `NO_LANG` |
| `isbn_lookup` | boolean | no |  | `ISBN_LOOKUP` |
| `imp_ignore` | boolean | no |  | `IMP_IGNORE` |
| `contributing_authors` | boolean | no |  | `CONTRIBUTING_AUTHORS` |

### Comics

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `comic_dest_folder` | string | no |  | `COMIC_DEST_FOLDER` |
| `comic_tab` | boolean | no |  | `COMIC_TAB` |
| `comic_relative` | boolean | no |  | `COMIC_RELATIVE` |
| `comic_delfolder` | boolean | no |  | `COMIC_DELFOLDER` |
| `comic_type` | string | no |  | `COMIC_TYPE` |
| `comic_single` | boolean | no |  | `COMIC_SINGLE` |
| `reject_comic` | string | no |  | `REJECT_COMIC` |
| `reject_maxcomic` | integer | no |  | `REJECT_MAXCOMIC` |
| `reject_mincomic` | integer | no |  | `REJECT_MINCOMIC` |
| `cv_apikey` | secret string | no |  | `CV_APIKEY` Credential — redacted in plan output. |

### Magazines

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `mag_tab` | boolean | no |  | `MAG_TAB` |
| `mag_coverswap` | string | no |  | `MAG_COVERSWAP` |
| `mag_dest_folder` | string | no |  | `MAG_DEST_FOLDER` |
| `mag_dest_file` | string | no |  | `MAG_DEST_FILE` |
| `mag_relative` | boolean | no |  | `MAG_RELATIVE` |
| `mag_delfolder` | boolean | no |  | `MAG_DELFOLDER` |
| `reject_mags` | string | no |  | `REJECT_MAGS` |
| `reject_maxsize` | integer | no |  | `REJECT_MAXSIZE` |
| `reject_minsize` | integer | no |  | `REJECT_MINSIZE` |
| `reject_maxaudio` | integer | no |  | `REJECT_MAXAUDIO` |
| `reject_minaudio` | integer | no |  | `REJECT_MINAUDIO` |
| `reject_magsize` | integer | no |  | `REJECT_MAGSIZE` |
| `reject_magmin` | integer | no |  | `REJECT_MAGMIN` |
| `imp_magopf` | boolean | no |  | `IMP_MAGOPF` |
| `imp_magcover` | boolean | no |  | `IMP_MAGCOVER` |
| `mag_single` | boolean | no |  | `MAG_SINGLE` |
| `mag_nouns` | string | no |  | `MAG_NOUNS` |
| `imp_autoaddmag` | string | no |  | `IMP_AUTOADDMAG` |
| `imp_autoaddmag_copy` | boolean | no |  | `IMP_AUTOADDMAG_COPY` |
| `imp_autoadd_magonly` | boolean | no |  | `IMP_AUTOADD_MAGONLY` |

### Twitter

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_twitter` | boolean | no |  | `USE_TWITTER` |
| `notify_onsnatch` | boolean | no |  | `TWITTER_NOTIFY_ONSNATCH` |
| `notify_ondownload` | boolean | no |  | `TWITTER_NOTIFY_ONDOWNLOAD` |
| `username` | string | no |  | `TWITTER_USERNAME` |
| `password` | secret string | no |  | `TWITTER_PASSWORD` Credential — redacted in plan output. |
| `prefix` | string | no |  | `TWITTER_PREFIX` |

### Boxcar

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_boxcar` | boolean | no |  | `USE_BOXCAR` |
| `notify_onsnatch` | boolean | no |  | `BOXCAR_NOTIFY_ONSNATCH` |
| `notify_ondownload` | boolean | no |  | `BOXCAR_NOTIFY_ONDOWNLOAD` |
| `token` | secret string | no |  | `BOXCAR_TOKEN` Credential — redacted in plan output. |

### Pushbullet

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_pushbullet` | boolean | no |  | `USE_PUSHBULLET` |
| `notify_onsnatch` | boolean | no |  | `PUSHBULLET_NOTIFY_ONSNATCH` |
| `notify_ondownload` | boolean | no |  | `PUSHBULLET_NOTIFY_ONDOWNLOAD` |
| `token` | secret string | no |  | `PUSHBULLET_TOKEN` Credential — redacted in plan output. |
| `deviceid` | string | no |  | `PUSHBULLET_DEVICEID` |

### Pushover

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_pushover` | boolean | no |  | `USE_PUSHOVER` |
| `onsnatch` | boolean | no |  | `PUSHOVER_ONSNATCH` |
| `ondownload` | boolean | no |  | `PUSHOVER_ONDOWNLOAD` |
| `keys` | string | no |  | `PUSHOVER_KEYS` |
| `apitoken` | secret string | no |  | `PUSHOVER_APITOKEN` Credential — redacted in plan output. |
| `priority` | integer | no |  | `PUSHOVER_PRIORITY` |
| `device` | string | no |  | `PUSHOVER_DEVICE` |

### Android Pn

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_androidpn` | boolean | no |  | `USE_ANDROIDPN` |
| `notify_onsnatch` | boolean | no |  | `ANDROIDPN_NOTIFY_ONSNATCH` |
| `notify_ondownload` | boolean | no |  | `ANDROIDPN_NOTIFY_ONDOWNLOAD` |
| `url` | string | no |  | `ANDROIDPN_URL` |
| `username` | string | no |  | `ANDROIDPN_USERNAME` |
| `broadcast` | boolean | no |  | `ANDROIDPN_BROADCAST` |

### Telegram

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_telegram` | boolean | no |  | `USE_TELEGRAM` |
| `token` | secret string | no |  | `TELEGRAM_TOKEN` Credential — redacted in plan output. |
| `userid` | string | no |  | `TELEGRAM_USERID` |
| `onsnatch` | boolean | no |  | `TELEGRAM_ONSNATCH` |
| `ondownload` | boolean | no |  | `TELEGRAM_ONDOWNLOAD` |

### Prowl

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_prowl` | boolean | no |  | `USE_PROWL` |
| `apikey` | secret string | no |  | `PROWL_APIKEY` Credential — redacted in plan output. |
| `priority` | integer | no |  | `PROWL_PRIORITY` |
| `onsnatch` | boolean | no |  | `PROWL_ONSNATCH` |
| `ondownload` | boolean | no |  | `PROWL_ONDOWNLOAD` |

### Growl

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_growl` | boolean | no |  | `USE_GROWL` |
| `host` | string | no |  | `GROWL_HOST` |
| `password` | secret string | no |  | `GROWL_PASSWORD` Credential — redacted in plan output. |
| `onsnatch` | boolean | no |  | `GROWL_ONSNATCH` |
| `ondownload` | boolean | no |  | `GROWL_ONDOWNLOAD` |

### Slack

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_slack` | boolean | no |  | `USE_SLACK` |
| `notify_onsnatch` | boolean | no |  | `SLACK_NOTIFY_ONSNATCH` |
| `notify_ondownload` | boolean | no |  | `SLACK_NOTIFY_ONDOWNLOAD` |
| `token` | secret string | no |  | `SLACK_TOKEN` Credential — redacted in plan output. |
| `url` | string | no |  | `SLACK_URL` |

### Custom

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_custom` | boolean | no |  | `USE_CUSTOM` |
| `notify_onsnatch` | boolean | no |  | `CUSTOM_NOTIFY_ONSNATCH` |
| `notify_ondownload` | boolean | no |  | `CUSTOM_NOTIFY_ONDOWNLOAD` |
| `script` | string | no |  | `CUSTOM_SCRIPT` |

### Email

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `use_email` | boolean | no |  | `USE_EMAIL` |
| `notify_onsnatch` | boolean | no |  | `EMAIL_NOTIFY_ONSNATCH` |
| `notify_ondownload` | boolean | no |  | `EMAIL_NOTIFY_ONDOWNLOAD` |
| `sendfile_ondownload` | boolean | no |  | `EMAIL_SENDFILE_ONDOWNLOAD` |
| `from_` | string | no |  | `EMAIL_FROM` |
| `to` | string | no |  | `EMAIL_TO` |
| `ssl` | boolean | no |  | `EMAIL_SSL` |
| `smtp_server` | string | no |  | `EMAIL_SMTP_SERVER` |
| `smtp_port` | integer | no |  | `EMAIL_SMTP_PORT` |
| `tls` | boolean | no |  | `EMAIL_TLS` |
| `smtp_user` | string | no |  | `EMAIL_SMTP_USER` |
| `smtp_password` | secret string | no |  | `EMAIL_SMTP_PASSWORD` Credential — redacted in plan output. |
| `limit` | integer | no |  | `EMAIL_LIMIT` |
| `use_email_custom_format` | boolean | no |  | `USE_EMAIL_CUSTOM_FORMAT` |
| `convert_from` | string | no |  | `EMAIL_CONVERT_FROM` |
| `send_type` | string | no |  | `EMAIL_SEND_TYPE` |

### Fmt

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `sername` | string | no |  | `FMT_SERNAME` |
| `sernum` | string | no |  | `FMT_SERNUM` |
| `series` | string | no |  | `FMT_SERIES` |

### Opds

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | `OPDS_ENABLED` |
| `authentication` | boolean | no |  | `OPDS_AUTHENTICATION` |
| `username` | string | no |  | `OPDS_USERNAME` |
| `password` | secret string | no |  | `OPDS_PASSWORD` Credential — redacted in plan output. |
| `metainfo` | boolean | no |  | `OPDS_METAINFO` |
| `page` | integer | no |  | `OPDS_PAGE` |

### Rss Config

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | `RSS_ENABLED` |
| `podcast` | boolean | no |  | `RSS_PODCAST` |
| `host` | string | no |  | `RSS_HOST` |

### Preprocess

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `ebook_wanted_formats` | string | no |  | `EBOOK_WANTED_FORMATS` |
| `delete_other_formats` | boolean | no |  | `DELETE_OTHER_FORMATS` |
| `ebook_convert` | string | no |  | `EBOOK_CONVERT` |
| `keep_opf` | boolean | no |  | `KEEP_OPF` |
| `keep_jpg` | boolean | no |  | `KEEP_JPG` |
| `ffmpeg` | string | no |  | `FFMPEG` |
| `ffmpeg_out` | string | no |  | `FFMPEG_OUT` |
| `audio_options` | string | no |  | `AUDIO_OPTIONS` |
| `create_singleaudio` | boolean | no |  | `CREATE_SINGLEAUDIO` |
| `keep_separateaudio` | boolean | no |  | `KEEP_SEPARATEAUDIO` |
| `write_audiotags` | boolean | no |  | `WRITE_AUDIOTAGS` |
| `zip_audioparts` | boolean | no |  | `ZIP_AUDIOPARTS` |
| `swap_coverpage` | boolean | no |  | `SWAP_COVERPAGE` |
| `tag_pdf` | boolean | no |  | `TAG_PDF` |
| `shrink_mag` | integer | no |  | `SHRINK_MAG` |

