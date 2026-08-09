//! LazyLibrarian config sections (generated from the spec `Config` schema).
//!
//! One `#[nested]` struct per `[Section]`; each field carries its exact `KEY` as the
//! wire name so the section-generic reconcile in [`super`] can `readCFG`/`writeCFG`
//! it. Runtime-only sections (`TELEMETRY`, `TESTING`, `GIT`) are excluded. A field is
//! `SecretValue` (redacted in plans) only when it is string-typed **and** looks like a
//! credential — booleans like `OL_API`/`DNB_API` ("use this source") stay `bool`.
//!
//! Some sections key fields with a different prefix than the section name (`[Sabnzbd]`
//! → `SAB_*`), so those config field names keep the prefix (`sab_host`).

use core_lib::SecretValue;
use core_macros::nested;

/// The `[General]` config section (107 keys).
#[nested(case = snake)]
pub struct General {
    /// `OL_URL`
    #[wire(name = "OL_URL")]
    pub ol_url: Option<String>,
    /// `GR_URL`
    #[wire(name = "GR_URL")]
    pub gr_url: Option<String>,
    /// `GB_URL`
    #[wire(name = "GB_URL")]
    pub gb_url: Option<String>,
    /// `LT_URL`
    #[wire(name = "LT_URL")]
    pub lt_url: Option<String>,
    /// `CV_URL`
    #[wire(name = "CV_URL")]
    pub cv_url: Option<String>,
    /// `CX_URL`
    #[wire(name = "CX_URL")]
    pub cx_url: Option<String>,
    /// `SHOW_NEWZ_PROV`
    #[wire(name = "SHOW_NEWZ_PROV")]
    pub show_newz_prov: Option<bool>,
    /// `SHOW_TORZ_PROV`
    #[wire(name = "SHOW_TORZ_PROV")]
    pub show_torz_prov: Option<bool>,
    /// `SHOW_TOR_PROV`
    #[wire(name = "SHOW_TOR_PROV")]
    pub show_tor_prov: Option<bool>,
    /// `SHOW_RSS_PROV`
    #[wire(name = "SHOW_RSS_PROV")]
    pub show_rss_prov: Option<bool>,
    /// `SHOW_IRC_PROV`
    #[wire(name = "SHOW_IRC_PROV")]
    pub show_irc_prov: Option<bool>,
    /// `SHOW_GEN_PROV`
    #[wire(name = "SHOW_GEN_PROV")]
    pub show_gen_prov: Option<bool>,
    /// `SHOW_DIRECT_PROV`
    #[wire(name = "SHOW_DIRECT_PROV")]
    pub show_direct_prov: Option<bool>,
    /// `USER_ACCOUNTS`
    #[wire(name = "USER_ACCOUNTS")]
    pub user_accounts: Option<bool>,
    /// `SINGLE_USER`
    #[wire(name = "SINGLE_USER")]
    pub single_user: Option<bool>,
    /// `ADMIN_EMAIL`
    #[wire(name = "ADMIN_EMAIL")]
    pub admin_email: Option<String>,
    /// `WHITELIST`
    #[wire(name = "WHITELIST")]
    pub whitelist: Option<String>,
    /// `SYS_ENCODING`
    #[wire(name = "SYS_ENCODING")]
    pub sys_encoding: Option<String>,
    /// `HOMEPAGE`
    #[wire(name = "HOMEPAGE")]
    pub homepage: Option<String>,
    /// `AUTH_TYPE`
    #[wire(name = "AUTH_TYPE")]
    pub auth_type: Option<String>,
    /// `WALL_COLUMNS`
    #[wire(name = "WALL_COLUMNS")]
    pub wall_columns: Option<i32>,
    /// `FILE_PERM`
    #[wire(name = "FILE_PERM")]
    pub file_perm: Option<String>,
    /// `DIR_PERM`
    #[wire(name = "DIR_PERM")]
    pub dir_perm: Option<String>,
    /// `BLOCKLIST_TIMER`
    #[wire(name = "BLOCKLIST_TIMER")]
    pub blocklist_timer: Option<i32>,
    /// `MAX_PAGES`
    #[wire(name = "MAX_PAGES")]
    pub max_pages: Option<i32>,
    /// `MAX_BOOKPAGES`
    #[wire(name = "MAX_BOOKPAGES")]
    pub max_bookpages: Option<i32>,
    /// `MAX_WALL`
    #[wire(name = "MAX_WALL")]
    pub max_wall: Option<i32>,
    /// `MATCH_RATIO`
    #[wire(name = "MATCH_RATIO")]
    pub match_ratio: Option<i32>,
    /// `DLOAD_RATIO`
    #[wire(name = "DLOAD_RATIO")]
    pub dload_ratio: Option<i32>,
    /// `NAME_RATIO`
    #[wire(name = "NAME_RATIO")]
    pub name_ratio: Option<i32>,
    /// `NAME_PARTIAL`
    #[wire(name = "NAME_PARTIAL")]
    pub name_partial: Option<i32>,
    /// `NAME_PARTNAME`
    #[wire(name = "NAME_PARTNAME")]
    pub name_partname: Option<i32>,
    /// `DISPLAYLENGTH`
    #[wire(name = "DISPLAYLENGTH")]
    pub displaylength: Option<i32>,
    /// `HIST_REFRESH`
    #[wire(name = "HIST_REFRESH")]
    pub hist_refresh: Option<i32>,
    /// `NO_IPV6`
    #[wire(name = "NO_IPV6")]
    pub no_ipv6: Option<bool>,
    /// `BOOKSTRAP_THEME`
    #[wire(name = "BOOKSTRAP_THEME")]
    pub bookstrap_theme: Option<String>,
    /// `SERIES_TAB`
    #[wire(name = "SERIES_TAB")]
    pub series_tab: Option<bool>,
    /// `AUDIO_TAB`
    #[wire(name = "AUDIO_TAB")]
    pub audio_tab: Option<bool>,
    /// `EBOOK_TAB`
    #[wire(name = "EBOOK_TAB")]
    pub ebook_tab: Option<bool>,
    /// `CONFIG_TAB_NUM`
    #[wire(name = "CONFIG_TAB_NUM")]
    pub config_tab_num: Option<i32>,
    /// `TOGGLES`
    #[wire(name = "TOGGLES")]
    pub toggles: Option<bool>,
    /// `SORT_DEFINITE`
    #[wire(name = "SORT_DEFINITE")]
    pub sort_definite: Option<bool>,
    /// `SORT_SURNAME`
    #[wire(name = "SORT_SURNAME")]
    pub sort_surname: Option<bool>,
    /// `SHOW_REASON`
    #[wire(name = "SHOW_REASON")]
    pub show_reason: Option<bool>,
    /// `SHOW_GENRES`
    #[wire(name = "SHOW_GENRES")]
    pub show_genres: Option<bool>,
    /// `IGNORE_PAUSED`
    #[wire(name = "IGNORE_PAUSED")]
    pub ignore_paused: Option<bool>,
    /// `LAUNCH_BROWSER`
    #[wire(name = "LAUNCH_BROWSER")]
    pub launch_browser: Option<bool>,
    /// `NAME_POSTFIX`
    #[wire(name = "NAME_POSTFIX")]
    pub name_postfix: Option<String>,
    /// `NAME_DEFINITE`
    #[wire(name = "NAME_DEFINITE")]
    pub name_definite: Option<String>,
    /// `MULTI_AUTHOR_SPLIT`
    #[wire(name = "MULTI_AUTHOR_SPLIT")]
    pub multi_author_split: Option<String>,
    /// `GOOGLE_TRANS_ID`
    #[wire(name = "GOOGLE_TRANS_ID")]
    pub google_trans_id: Option<bool>,
    /// `IMP_PREFLANG`
    #[wire(name = "IMP_PREFLANG")]
    pub imp_preflang: Option<String>,
    /// `PREF_MAGLANG`
    #[wire(name = "PREF_MAGLANG")]
    pub pref_maglang: Option<String>,
    /// `ISS_FORMAT`
    #[wire(name = "ISS_FORMAT")]
    pub iss_format: Option<String>,
    /// `DATE_FORMAT`
    #[wire(name = "DATE_FORMAT")]
    pub date_format: Option<String>,
    /// `DATE_LANG`
    #[wire(name = "DATE_LANG")]
    pub date_lang: Option<String>,
    /// `AUTHOR_DATE_FORMAT`
    #[wire(name = "AUTHOR_DATE_FORMAT")]
    pub author_date_format: Option<String>,
    /// `ISSUE_NOUNS`
    #[wire(name = "ISSUE_NOUNS")]
    pub issue_nouns: Option<String>,
    /// `VOLUME_NOUNS`
    #[wire(name = "VOLUME_NOUNS")]
    pub volume_nouns: Option<String>,
    /// `IMP_MONTHLANG`
    #[wire(name = "IMP_MONTHLANG")]
    pub imp_monthlang: Option<String>,
    /// `IMP_AUTOADD`
    #[wire(name = "IMP_AUTOADD")]
    pub imp_autoadd: Option<String>,
    /// `IMP_AUTOADD_COPY`
    #[wire(name = "IMP_AUTOADD_COPY")]
    pub imp_autoadd_copy: Option<bool>,
    /// `IMP_AUTOADD_BOOKONLY`
    #[wire(name = "IMP_AUTOADD_BOOKONLY")]
    pub imp_autoadd_bookonly: Option<bool>,
    /// `IMP_AUTOSEARCH`
    #[wire(name = "IMP_AUTOSEARCH")]
    pub imp_autosearch: Option<bool>,
    /// `BLACKLIST_FAILED`
    #[wire(name = "BLACKLIST_FAILED")]
    pub blacklist_failed: Option<bool>,
    /// `BLACKLIST_PROCESSED`
    #[wire(name = "BLACKLIST_PROCESSED")]
    pub blacklist_processed: Option<bool>,
    /// `SSL_CERTS`
    #[wire(name = "SSL_CERTS")]
    pub ssl_certs: Option<String>,
    /// `SSL_VERIFY`
    #[wire(name = "SSL_VERIFY")]
    pub ssl_verify: Option<bool>,
    /// `HTTP_TIMEOUT`
    #[wire(name = "HTTP_TIMEOUT")]
    pub http_timeout: Option<i32>,
    /// `HTTP_EXT_TIMEOUT`
    #[wire(name = "HTTP_EXT_TIMEOUT")]
    pub http_ext_timeout: Option<i32>,
    /// `IMP_SINGLEBOOK`
    #[wire(name = "IMP_SINGLEBOOK")]
    pub imp_singlebook: Option<bool>,
    /// `IMP_RENAME`
    #[wire(name = "IMP_RENAME")]
    pub imp_rename: Option<bool>,
    /// `MAG_RENAME`
    #[wire(name = "MAG_RENAME")]
    pub mag_rename: Option<bool>,
    /// `IMP_COMICOPF`
    #[wire(name = "IMP_COMICOPF")]
    pub imp_comicopf: Option<bool>,
    /// `IMP_COMICCOVER`
    #[wire(name = "IMP_COMICCOVER")]
    pub imp_comiccover: Option<bool>,
    /// `IMP_CONVERT`
    #[wire(name = "IMP_CONVERT")]
    pub imp_convert: Option<String>,
    /// `IMP_NOSPLIT`
    #[wire(name = "IMP_NOSPLIT")]
    pub imp_nosplit: Option<String>,
    /// `EXT_PREPROCESS`
    #[wire(name = "EXT_PREPROCESS")]
    pub ext_preprocess: Option<String>,
    /// `GIT_PROGRAM`
    #[wire(name = "GIT_PROGRAM")]
    pub git_program: Option<String>,
    /// `CACHE_AGE`
    #[wire(name = "CACHE_AGE")]
    pub cache_age: Option<i32>,
    /// `BACKUP_DB`
    #[wire(name = "BACKUP_DB")]
    pub backup_db: Option<i32>,
    /// `TASK_AGE`
    #[wire(name = "TASK_AGE")]
    pub task_age: Option<i32>,
    /// `OPF_TAGS`
    #[wire(name = "OPF_TAGS")]
    pub opf_tags: Option<bool>,
    /// `GENRE_TAGS`
    #[wire(name = "GENRE_TAGS")]
    pub genre_tags: Option<bool>,
    /// `WISHLIST_TAGS`
    #[wire(name = "WISHLIST_TAGS")]
    pub wishlist_tags: Option<bool>,
    /// `WISHLIST_GENRES`
    #[wire(name = "WISHLIST_GENRES")]
    pub wishlist_genres: Option<bool>,
    /// `NOTIFY_WITH_TITLE`
    #[wire(name = "NOTIFY_WITH_TITLE")]
    pub notify_with_title: Option<bool>,
    /// `NOTIFY_WITH_URL`
    #[wire(name = "NOTIFY_WITH_URL")]
    pub notify_with_url: Option<bool>,
    /// `DESTINATION_COPY`
    #[wire(name = "DESTINATION_COPY")]
    pub destination_copy: Option<bool>,
    /// `EBOOK_DIR`
    #[wire(name = "EBOOK_DIR")]
    pub ebook_dir: Option<String>,
    /// `AUDIO_DIR`
    #[wire(name = "AUDIO_DIR")]
    pub audio_dir: Option<String>,
    /// `ALTERNATE_DIR`
    #[wire(name = "ALTERNATE_DIR")]
    pub alternate_dir: Option<String>,
    /// `TESTDATA_DIR`
    #[wire(name = "TESTDATA_DIR")]
    pub testdata_dir: Option<String>,
    /// `DELETE_CSV`
    #[wire(name = "DELETE_CSV")]
    pub delete_csv: Option<bool>,
    /// `DOWNLOAD_DIR`
    #[wire(name = "DOWNLOAD_DIR")]
    pub download_dir: Option<String>,
    /// `EBOOK_TYPE`
    #[wire(name = "EBOOK_TYPE")]
    pub ebook_type: Option<String>,
    /// `AUDIOBOOK_TYPE`
    #[wire(name = "AUDIOBOOK_TYPE")]
    pub audiobook_type: Option<String>,
    /// `MAG_TYPE`
    #[wire(name = "MAG_TYPE")]
    pub mag_type: Option<String>,
    /// `REJECT_PUBLISHER`
    #[wire(name = "REJECT_PUBLISHER")]
    pub reject_publisher: Option<String>,
    /// `REJECT_WORDS`
    #[wire(name = "REJECT_WORDS")]
    pub reject_words: Option<String>,
    /// `PREFER_WORDS`
    #[wire(name = "PREFER_WORDS")]
    pub prefer_words: Option<String>,
    /// `REJECT_AUDIO`
    #[wire(name = "REJECT_AUDIO")]
    pub reject_audio: Option<String>,
    /// `MAG_AGE`
    #[wire(name = "MAG_AGE")]
    pub mag_age: Option<i32>,
    /// `HIDE_OLD_NOTIFIERS`
    #[wire(name = "HIDE_OLD_NOTIFIERS")]
    pub hide_old_notifiers: Option<bool>,
    /// `PREF_UNRARLIB`
    #[wire(name = "PREF_UNRARLIB")]
    pub pref_unrarlib: Option<i32>,
    /// `USER_AGENT`
    #[wire(name = "USER_AGENT")]
    pub user_agent: Option<String>,
    /// `RATESTARS`
    #[wire(name = "RATESTARS")]
    pub ratestars: Option<bool>,
}

/// The `[Api]` config section (29 keys).
#[nested(case = snake)]
pub struct Api {
    /// `API_ENABLED`
    #[wire(name = "API_ENABLED")]
    pub enabled: Option<bool>,
    /// `API_KEY`
    #[wire(name = "API_KEY")]
    pub key: Option<SecretValue>,
    /// `API_RO_KEY`
    #[wire(name = "API_RO_KEY")]
    pub ro_key: Option<SecretValue>,
    /// `BOOK_API`
    #[wire(name = "BOOK_API")]
    pub book_api: Option<String>,
    /// `LT_DEVKEY`
    #[wire(name = "LT_DEVKEY")]
    pub lt_devkey: Option<SecretValue>,
    /// `CV_WEBSEARCH`
    #[wire(name = "CV_WEBSEARCH")]
    pub cv_websearch: Option<bool>,
    /// `OL_API`
    #[wire(name = "OL_API")]
    pub ol_api: Option<bool>,
    /// `DNB_API`
    #[wire(name = "DNB_API")]
    pub dnb_api: Option<bool>,
    /// `HC_API`
    #[wire(name = "HC_API")]
    pub hc_api: Option<bool>,
    /// `HC_SYNC`
    #[wire(name = "HC_SYNC")]
    pub hc_sync: Option<bool>,
    /// `HC_SYNCREADONLY`
    #[wire(name = "HC_SYNCREADONLY")]
    pub hc_syncreadonly: Option<bool>,
    /// `HC_SYNC_LIMIT`
    #[wire(name = "HC_SYNC_LIMIT")]
    pub hc_sync_limit: Option<i32>,
    /// `GR_API`
    #[wire(name = "GR_API")]
    pub gr_api: Option<SecretValue>,
    /// `GR_SYNC`
    #[wire(name = "GR_SYNC")]
    pub gr_sync: Option<bool>,
    /// `GR_SYNCUSER`
    #[wire(name = "GR_SYNCUSER")]
    pub gr_syncuser: Option<bool>,
    /// `GR_USER`
    #[wire(name = "GR_USER")]
    pub gr_user: Option<String>,
    /// `GR_SYNCREADONLY`
    #[wire(name = "GR_SYNCREADONLY")]
    pub gr_syncreadonly: Option<bool>,
    /// `GR_SECRET`
    #[wire(name = "GR_SECRET")]
    pub gr_secret: Option<SecretValue>,
    /// `GR_OAUTH_TOKEN`
    #[wire(name = "GR_OAUTH_TOKEN")]
    pub gr_oauth_token: Option<SecretValue>,
    /// `GR_OAUTH_SECRET`
    #[wire(name = "GR_OAUTH_SECRET")]
    pub gr_oauth_secret: Option<SecretValue>,
    /// `GR_WANTED`
    #[wire(name = "GR_WANTED")]
    pub gr_wanted: Option<String>,
    /// `GR_OWNED`
    #[wire(name = "GR_OWNED")]
    pub gr_owned: Option<String>,
    /// `GR_AWANTED`
    #[wire(name = "GR_AWANTED")]
    pub gr_awanted: Option<String>,
    /// `GR_AOWNED`
    #[wire(name = "GR_AOWNED")]
    pub gr_aowned: Option<String>,
    /// `GR_UNIQUE`
    #[wire(name = "GR_UNIQUE")]
    pub gr_unique: Option<bool>,
    /// `GR_FOLLOW`
    #[wire(name = "GR_FOLLOW")]
    pub gr_follow: Option<bool>,
    /// `GR_FOLLOWNEW`
    #[wire(name = "GR_FOLLOWNEW")]
    pub gr_follownew: Option<bool>,
    /// `GB_API`
    #[wire(name = "GB_API")]
    pub gb_api: Option<SecretValue>,
    /// `GB_COUNTRY`
    #[wire(name = "GB_COUNTRY")]
    pub gb_country: Option<String>,
}

/// The `[Proxy]` config section (8 keys).
#[nested(case = snake)]
pub struct Proxy {
    /// `PROXY_HOST`
    #[wire(name = "PROXY_HOST")]
    pub host: Option<String>,
    /// `PROXY_TYPE`
    #[wire(name = "PROXY_TYPE")]
    pub type_: Option<String>,
    /// `PROXY_LOCAL`
    #[wire(name = "PROXY_LOCAL")]
    pub local: Option<String>,
    /// `PROXY_AUTH`
    #[wire(name = "PROXY_AUTH")]
    pub auth: Option<bool>,
    /// `PROXY_REGISTER`
    #[wire(name = "PROXY_REGISTER")]
    pub register: Option<bool>,
    /// `PROXY_AUTH_USER`
    #[wire(name = "PROXY_AUTH_USER")]
    pub auth_user: Option<String>,
    /// `PROXY_AUTH_EMAIL`
    #[wire(name = "PROXY_AUTH_EMAIL")]
    pub auth_email: Option<String>,
    /// `PROXY_AUTH_NAME`
    #[wire(name = "PROXY_AUTH_NAME")]
    pub auth_name: Option<String>,
}

/// The `[Postprocess]` config section (13 keys).
#[nested(case = snake)]
pub struct Postprocess {
    /// `SKIPPED_EXT`
    #[wire(name = "SKIPPED_EXT")]
    pub skipped_ext: Option<String>,
    /// `BANNED_EXT`
    #[wire(name = "BANNED_EXT")]
    pub banned_ext: Option<String>,
    /// `CREATE_LINK`
    #[wire(name = "CREATE_LINK")]
    pub create_link: Option<String>,
    /// `EBOOK_DEST_FOLDER`
    #[wire(name = "EBOOK_DEST_FOLDER")]
    pub ebook_dest_folder: Option<String>,
    /// `EBOOK_DEST_FILE`
    #[wire(name = "EBOOK_DEST_FILE")]
    pub ebook_dest_file: Option<String>,
    /// `AUDIOBOOK_DEST_FILE`
    #[wire(name = "AUDIOBOOK_DEST_FILE")]
    pub audiobook_dest_file: Option<String>,
    /// `AUDIOBOOK_SINGLE_FILE`
    #[wire(name = "AUDIOBOOK_SINGLE_FILE")]
    pub audiobook_single_file: Option<String>,
    /// `AUDIOBOOK_DEST_FOLDER`
    #[wire(name = "AUDIOBOOK_DEST_FOLDER")]
    pub audiobook_dest_folder: Option<String>,
    /// `ONE_FORMAT`
    #[wire(name = "ONE_FORMAT")]
    pub one_format: Option<bool>,
    /// `DEL_DOWNLOADFAILED`
    #[wire(name = "DEL_DOWNLOADFAILED")]
    pub del_downloadfailed: Option<bool>,
    /// `PP_DELAY`
    #[wire(name = "PP_DELAY")]
    pub pp_delay: Option<i32>,
    /// `DEL_FAILED`
    #[wire(name = "DEL_FAILED")]
    pub del_failed: Option<bool>,
    /// `DEL_COMPLETED`
    #[wire(name = "DEL_COMPLETED")]
    pub del_completed: Option<bool>,
}

/// The `[Webserver]` config section (10 keys).
#[nested(case = snake)]
pub struct WebServer {
    /// `HTTP_PORT`
    #[wire(name = "HTTP_PORT")]
    pub http_port: Option<i32>,
    /// `HTTP_HOST`
    #[wire(name = "HTTP_HOST")]
    pub http_host: Option<String>,
    /// `HTTP_USER`
    #[wire(name = "HTTP_USER")]
    pub http_user: Option<String>,
    /// `HTTP_PASS`
    #[wire(name = "HTTP_PASS")]
    pub http_pass: Option<SecretValue>,
    /// `HTTP_PROXY`
    #[wire(name = "HTTP_PROXY")]
    pub http_proxy: Option<bool>,
    /// `HTTP_ROOT`
    #[wire(name = "HTTP_ROOT")]
    pub http_root: Option<String>,
    /// `HTTP_LOOK`
    #[wire(name = "HTTP_LOOK")]
    pub http_look: Option<String>,
    /// `HTTPS_ENABLED`
    #[wire(name = "HTTPS_ENABLED")]
    pub https_enabled: Option<bool>,
    /// `HTTPS_CERT`
    #[wire(name = "HTTPS_CERT")]
    pub https_cert: Option<String>,
    /// `HTTPS_KEY`
    #[wire(name = "HTTPS_KEY")]
    pub https_key: Option<SecretValue>,
}

/// The `[Logging]` config section (10 keys).
#[nested(case = snake)]
pub struct Logging {
    /// `LOGDIR`
    #[wire(name = "LOGDIR")]
    pub logdir: Option<String>,
    /// `LOGLIMIT`
    #[wire(name = "LOGLIMIT")]
    pub loglimit: Option<i32>,
    /// `LOGFILES`
    #[wire(name = "LOGFILES")]
    pub logfiles: Option<i32>,
    /// `LOGSIZE`
    #[wire(name = "LOGSIZE")]
    pub logsize: Option<i32>,
    /// `DETAILEDUILOG`
    #[wire(name = "DETAILEDUILOG")]
    pub detaileduilog: Option<bool>,
    /// `LOGREDACT`
    #[wire(name = "LOGREDACT")]
    pub logredact: Option<bool>,
    /// `HOSTREDACT`
    #[wire(name = "HOSTREDACT")]
    pub hostredact: Option<bool>,
    /// `LOGFILEREDACT`
    #[wire(name = "LOGFILEREDACT")]
    pub logfileredact: Option<bool>,
    /// `LOGLEVEL`
    #[wire(name = "LOGLEVEL")]
    pub loglevel: Option<i32>,
    /// `LOGSPECIALDEBUG`
    #[wire(name = "LOGSPECIALDEBUG")]
    pub logspecialdebug: Option<String>,
}

/// The `[Importer]` config section (1 keys).
#[nested(case = snake)]
pub struct Importer {
    /// `MULTI_SOURCE`
    #[wire(name = "MULTI_SOURCE")]
    pub multi_source: Option<bool>,
}

/// The `[Calibre]` config section (12 keys).
#[nested(case = snake)]
pub struct Calibre {
    /// `CALIBRE_USE_SERVER`
    #[wire(name = "CALIBRE_USE_SERVER")]
    pub use_server: Option<bool>,
    /// `CALIBRE_SERVER`
    #[wire(name = "CALIBRE_SERVER")]
    pub server: Option<String>,
    /// `CALIBRE_USER`
    #[wire(name = "CALIBRE_USER")]
    pub user: Option<String>,
    /// `CALIBRE_PASS`
    #[wire(name = "CALIBRE_PASS")]
    pub pass_: Option<SecretValue>,
    /// `CALIBRE_RENAME`
    #[wire(name = "CALIBRE_RENAME")]
    pub rename: Option<bool>,
    /// `IMP_CALIBREDB`
    #[wire(name = "IMP_CALIBREDB")]
    pub imp_calibredb: Option<String>,
    /// `IMP_CALIBREOVERWRITE`
    #[wire(name = "IMP_CALIBREOVERWRITE")]
    pub imp_calibreoverwrite: Option<bool>,
    /// `IMP_CALIBRE_EBOOK`
    #[wire(name = "IMP_CALIBRE_EBOOK")]
    pub imp_calibre_ebook: Option<bool>,
    /// `IMP_CALIBRE_COMIC`
    #[wire(name = "IMP_CALIBRE_COMIC")]
    pub imp_calibre_comic: Option<bool>,
    /// `IMP_CALIBRE_MAGAZINE`
    #[wire(name = "IMP_CALIBRE_MAGAZINE")]
    pub imp_calibre_magazine: Option<bool>,
    /// `IMP_CALIBRE_MAGTITLE`
    #[wire(name = "IMP_CALIBRE_MAGTITLE")]
    pub imp_calibre_magtitle: Option<bool>,
    /// `IMP_CALIBRE_MAGISSUE`
    #[wire(name = "IMP_CALIBRE_MAGISSUE")]
    pub imp_calibre_magissue: Option<bool>,
}

/// The `[Sabnzbd]` config section (11 keys).
#[nested(case = snake)]
pub struct Sabnzbd {
    /// `SAB_HOST`
    #[wire(name = "SAB_HOST")]
    pub sab_host: Option<String>,
    /// `SAB_PORT`
    #[wire(name = "SAB_PORT")]
    pub sab_port: Option<i32>,
    /// `SAB_SUBDIR`
    #[wire(name = "SAB_SUBDIR")]
    pub sab_subdir: Option<String>,
    /// `SAB_USER`
    #[wire(name = "SAB_USER")]
    pub sab_user: Option<String>,
    /// `SAB_PASS`
    #[wire(name = "SAB_PASS")]
    pub sab_pass: Option<SecretValue>,
    /// `SAB_API`
    #[wire(name = "SAB_API")]
    pub sab_api: Option<SecretValue>,
    /// `SAB_CAT`
    #[wire(name = "SAB_CAT")]
    pub sab_cat: Option<String>,
    /// `SAB_DELETE`
    #[wire(name = "SAB_DELETE")]
    pub sab_delete: Option<bool>,
    /// `SAB_EXTERNAL_HOST`
    #[wire(name = "SAB_EXTERNAL_HOST")]
    pub sab_external_host: Option<String>,
    /// `SAB_REMOTE`
    #[wire(name = "SAB_REMOTE")]
    pub sab_remote: Option<String>,
    /// `SAB_LOCAL`
    #[wire(name = "SAB_LOCAL")]
    pub sab_local: Option<String>,
}

/// The `[Nzbget]` config section (8 keys).
#[nested(case = snake)]
pub struct Nzbget {
    /// `NZBGET_HOST`
    #[wire(name = "NZBGET_HOST")]
    pub host: Option<String>,
    /// `NZBGET_PORT`
    #[wire(name = "NZBGET_PORT")]
    pub port: Option<i32>,
    /// `NZBGET_USER`
    #[wire(name = "NZBGET_USER")]
    pub user: Option<String>,
    /// `NZBGET_PASS`
    #[wire(name = "NZBGET_PASS")]
    pub pass_: Option<SecretValue>,
    /// `NZBGET_CATEGORY`
    #[wire(name = "NZBGET_CATEGORY")]
    pub category: Option<String>,
    /// `NZBGET_PRIORITY`
    #[wire(name = "NZBGET_PRIORITY")]
    pub priority: Option<i32>,
    /// `NZBGET_REMOTE`
    #[wire(name = "NZBGET_REMOTE")]
    pub remote: Option<String>,
    /// `NZBGET_LOCAL`
    #[wire(name = "NZBGET_LOCAL")]
    pub local: Option<String>,
}

/// The `[Usenet]` config section (7 keys).
#[nested(case = snake)]
pub struct Usenet {
    /// `NZB_DOWNLOADER_SABNZBD`
    #[wire(name = "NZB_DOWNLOADER_SABNZBD")]
    pub nzb_downloader_sabnzbd: Option<bool>,
    /// `NZB_DOWNLOADER_NZBGET`
    #[wire(name = "NZB_DOWNLOADER_NZBGET")]
    pub nzb_downloader_nzbget: Option<bool>,
    /// `NZB_DOWNLOADER_SYNOLOGY`
    #[wire(name = "NZB_DOWNLOADER_SYNOLOGY")]
    pub nzb_downloader_synology: Option<bool>,
    /// `NZB_DOWNLOADER_BLACKHOLE`
    #[wire(name = "NZB_DOWNLOADER_BLACKHOLE")]
    pub nzb_downloader_blackhole: Option<bool>,
    /// `NZB_BLACKHOLEDIR`
    #[wire(name = "NZB_BLACKHOLEDIR")]
    pub nzb_blackholedir: Option<String>,
    /// `NZB_PAUSED`
    #[wire(name = "NZB_PAUSED")]
    pub nzb_paused: Option<bool>,
    /// `USENET_RETENTION`
    #[wire(name = "USENET_RETENTION")]
    pub retention: Option<i32>,
}

/// The `[Nzbmatrix]` config section (3 keys).
#[nested(case = snake)]
pub struct Nzbmatrix {
    /// `NZBMATRIX_USER`
    #[wire(name = "NZBMATRIX_USER")]
    pub user: Option<String>,
    /// `NZBMATRIX_API`
    #[wire(name = "NZBMATRIX_API")]
    pub api: Option<SecretValue>,
    /// `NZBMATRIX`
    #[wire(name = "NZBMATRIX")]
    pub nzbmatrix: Option<bool>,
}

/// The `[Torrent]` config section (14 keys).
#[nested(case = snake)]
pub struct Torrent {
    /// `TOR_DOWNLOADER_BLACKHOLE`
    #[wire(name = "TOR_DOWNLOADER_BLACKHOLE")]
    pub tor_downloader_blackhole: Option<bool>,
    /// `TOR_CONVERT_MAGNET`
    #[wire(name = "TOR_CONVERT_MAGNET")]
    pub tor_convert_magnet: Option<bool>,
    /// `TOR_DOWNLOADER_UTORRENT`
    #[wire(name = "TOR_DOWNLOADER_UTORRENT")]
    pub tor_downloader_utorrent: Option<bool>,
    /// `TOR_DOWNLOADER_RTORRENT`
    #[wire(name = "TOR_DOWNLOADER_RTORRENT")]
    pub tor_downloader_rtorrent: Option<bool>,
    /// `TOR_DOWNLOADER_QBITTORRENT`
    #[wire(name = "TOR_DOWNLOADER_QBITTORRENT")]
    pub tor_downloader_qbittorrent: Option<bool>,
    /// `TOR_DOWNLOADER_TRANSMISSION`
    #[wire(name = "TOR_DOWNLOADER_TRANSMISSION")]
    pub tor_downloader_transmission: Option<bool>,
    /// `TOR_DOWNLOADER_SYNOLOGY`
    #[wire(name = "TOR_DOWNLOADER_SYNOLOGY")]
    pub tor_downloader_synology: Option<bool>,
    /// `TOR_DOWNLOADER_DELUGE`
    #[wire(name = "TOR_DOWNLOADER_DELUGE")]
    pub tor_downloader_deluge: Option<bool>,
    /// `TORRENT_PAUSED`
    #[wire(name = "TORRENT_PAUSED")]
    pub paused: Option<bool>,
    /// `NUMBEROFSEEDERS`
    #[wire(name = "NUMBEROFSEEDERS")]
    pub numberofseeders: Option<i32>,
    /// `KEEP_SEEDING`
    #[wire(name = "KEEP_SEEDING")]
    pub keep_seeding: Option<bool>,
    /// `SEED_WAIT`
    #[wire(name = "SEED_WAIT")]
    pub seed_wait: Option<bool>,
    /// `PREFER_MAGNET`
    #[wire(name = "PREFER_MAGNET")]
    pub prefer_magnet: Option<bool>,
    /// `TORRENT_DIR`
    #[wire(name = "TORRENT_DIR")]
    pub dir: Option<String>,
}

/// The `[Rtorrent]` config section (7 keys).
#[nested(case = snake)]
pub struct Rtorrent {
    /// `RTORRENT_HOST`
    #[wire(name = "RTORRENT_HOST")]
    pub host: Option<String>,
    /// `RTORRENT_USER`
    #[wire(name = "RTORRENT_USER")]
    pub user: Option<String>,
    /// `RTORRENT_PASS`
    #[wire(name = "RTORRENT_PASS")]
    pub pass_: Option<SecretValue>,
    /// `RTORRENT_LABEL`
    #[wire(name = "RTORRENT_LABEL")]
    pub label: Option<String>,
    /// `RTORRENT_DIR`
    #[wire(name = "RTORRENT_DIR")]
    pub dir: Option<String>,
    /// `RTORRENT_REMOTE`
    #[wire(name = "RTORRENT_REMOTE")]
    pub remote: Option<String>,
    /// `RTORRENT_LOCAL`
    #[wire(name = "RTORRENT_LOCAL")]
    pub local: Option<String>,
}

/// The `[Utorrent]` config section (8 keys).
#[nested(case = snake)]
pub struct Utorrent {
    /// `UTORRENT_HOST`
    #[wire(name = "UTORRENT_HOST")]
    pub host: Option<String>,
    /// `UTORRENT_PORT`
    #[wire(name = "UTORRENT_PORT")]
    pub port: Option<i32>,
    /// `UTORRENT_BASE`
    #[wire(name = "UTORRENT_BASE")]
    pub base: Option<String>,
    /// `UTORRENT_USER`
    #[wire(name = "UTORRENT_USER")]
    pub user: Option<String>,
    /// `UTORRENT_PASS`
    #[wire(name = "UTORRENT_PASS")]
    pub pass_: Option<SecretValue>,
    /// `UTORRENT_LABEL`
    #[wire(name = "UTORRENT_LABEL")]
    pub label: Option<String>,
    /// `UTORRENT_REMOTE`
    #[wire(name = "UTORRENT_REMOTE")]
    pub remote: Option<String>,
    /// `UTORRENT_LOCAL`
    #[wire(name = "UTORRENT_LOCAL")]
    pub local: Option<String>,
}

/// The `[Qbittorrent]` config section (10 keys).
#[nested(case = snake)]
pub struct Qbittorrent {
    /// `QBITTORRENT_HOST`
    #[wire(name = "QBITTORRENT_HOST")]
    pub host: Option<String>,
    /// `QBITTORRENT_PORT`
    #[wire(name = "QBITTORRENT_PORT")]
    pub port: Option<i32>,
    /// `QBITTORRENT_BASE`
    #[wire(name = "QBITTORRENT_BASE")]
    pub base: Option<String>,
    /// `QBITTORRENT_USER`
    #[wire(name = "QBITTORRENT_USER")]
    pub user: Option<String>,
    /// `QBITTORRENT_PASS`
    #[wire(name = "QBITTORRENT_PASS")]
    pub pass_: Option<SecretValue>,
    /// `QBITTORRENT_LABEL`
    #[wire(name = "QBITTORRENT_LABEL")]
    pub label: Option<String>,
    /// `QBITTORRENT_DIR`
    #[wire(name = "QBITTORRENT_DIR")]
    pub dir: Option<String>,
    /// `QBITTORRENT_REMOTE`
    #[wire(name = "QBITTORRENT_REMOTE")]
    pub remote: Option<String>,
    /// `QBITTORRENT_LOCAL`
    #[wire(name = "QBITTORRENT_LOCAL")]
    pub local: Option<String>,
    /// `QBITTORRENT_IGNORE_SSL`
    #[wire(name = "QBITTORRENT_IGNORE_SSL")]
    pub ignore_ssl: Option<bool>,
}

/// The `[Transmission]` config section (9 keys).
#[nested(case = snake)]
pub struct Transmission {
    /// `TRANSMISSION_HOST`
    #[wire(name = "TRANSMISSION_HOST")]
    pub host: Option<String>,
    /// `TRANSMISSION_BASE`
    #[wire(name = "TRANSMISSION_BASE")]
    pub base: Option<String>,
    /// `TRANSMISSION_PORT`
    #[wire(name = "TRANSMISSION_PORT")]
    pub port: Option<i32>,
    /// `TRANSMISSION_USER`
    #[wire(name = "TRANSMISSION_USER")]
    pub user: Option<String>,
    /// `TRANSMISSION_PASS`
    #[wire(name = "TRANSMISSION_PASS")]
    pub pass_: Option<SecretValue>,
    /// `TRANSMISSION_DIR`
    #[wire(name = "TRANSMISSION_DIR")]
    pub dir: Option<String>,
    /// `TRANSMISSION_LABEL`
    #[wire(name = "TRANSMISSION_LABEL")]
    pub label: Option<String>,
    /// `TRANSMISSION_REMOTE`
    #[wire(name = "TRANSMISSION_REMOTE")]
    pub remote: Option<String>,
    /// `TRANSMISSION_LOCAL`
    #[wire(name = "TRANSMISSION_LOCAL")]
    pub local: Option<String>,
}

/// The `[Deluge]` config section (11 keys).
#[nested(case = snake)]
pub struct Deluge {
    /// `DELUGE_CERT`
    #[wire(name = "DELUGE_CERT")]
    pub cert: Option<String>,
    /// `DELUGE_HOST`
    #[wire(name = "DELUGE_HOST")]
    pub host: Option<String>,
    /// `DELUGE_BASE`
    #[wire(name = "DELUGE_BASE")]
    pub base: Option<String>,
    /// `DELUGE_PORT`
    #[wire(name = "DELUGE_PORT")]
    pub port: Option<i32>,
    /// `DELUGE_USER`
    #[wire(name = "DELUGE_USER")]
    pub user: Option<String>,
    /// `DELUGE_PASS`
    #[wire(name = "DELUGE_PASS")]
    pub pass_: Option<SecretValue>,
    /// `DELUGE_LABEL`
    #[wire(name = "DELUGE_LABEL")]
    pub label: Option<String>,
    /// `DELUGE_DIR`
    #[wire(name = "DELUGE_DIR")]
    pub dir: Option<String>,
    /// `DELUGE_TIMEOUT`
    #[wire(name = "DELUGE_TIMEOUT")]
    pub timeout: Option<i32>,
    /// `DELUGE_REMOTE`
    #[wire(name = "DELUGE_REMOTE")]
    pub remote: Option<String>,
    /// `DELUGE_LOCAL`
    #[wire(name = "DELUGE_LOCAL")]
    pub local: Option<String>,
}

/// The `[Synology]` config section (8 keys).
#[nested(case = snake)]
pub struct Synology {
    /// `SYNOLOGY_HOST`
    #[wire(name = "SYNOLOGY_HOST")]
    pub host: Option<String>,
    /// `SYNOLOGY_PORT`
    #[wire(name = "SYNOLOGY_PORT")]
    pub port: Option<i32>,
    /// `SYNOLOGY_USER`
    #[wire(name = "SYNOLOGY_USER")]
    pub user: Option<String>,
    /// `SYNOLOGY_PASS`
    #[wire(name = "SYNOLOGY_PASS")]
    pub pass_: Option<SecretValue>,
    /// `SYNOLOGY_DIR`
    #[wire(name = "SYNOLOGY_DIR")]
    pub dir: Option<String>,
    /// `USE_SYNOLOGY`
    #[wire(name = "USE_SYNOLOGY")]
    pub use_synology: Option<bool>,
    /// `SYNOLOGY_REMOTE`
    #[wire(name = "SYNOLOGY_REMOTE")]
    pub remote: Option<String>,
    /// `SYNOLOGY_LOCAL`
    #[wire(name = "SYNOLOGY_LOCAL")]
    pub local: Option<String>,
}

/// The `[Abb]` config section (5 keys).
#[nested(case = snake)]
pub struct Abb {
    /// `ABB_HOST`
    #[wire(name = "ABB_HOST")]
    pub host: Option<String>,
    /// `ABB`
    #[wire(name = "ABB")]
    pub abb: Option<bool>,
    /// `ABB_DLPRIORITY`
    #[wire(name = "ABB_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `ABB_DLTYPES`
    #[wire(name = "ABB_DLTYPES")]
    pub dltypes: Option<String>,
    /// `ABB_SEEDERS`
    #[wire(name = "ABB_SEEDERS")]
    pub seeders: Option<i32>,
}

/// The `[Kat]` config section (5 keys).
#[nested(case = snake)]
pub struct Kat {
    /// `KAT_HOST`
    #[wire(name = "KAT_HOST")]
    pub host: Option<String>,
    /// `KAT`
    #[wire(name = "KAT")]
    pub kat: Option<bool>,
    /// `KAT_DLPRIORITY`
    #[wire(name = "KAT_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `KAT_DLTYPES`
    #[wire(name = "KAT_DLTYPES")]
    pub dltypes: Option<String>,
    /// `KAT_SEEDERS`
    #[wire(name = "KAT_SEEDERS")]
    pub seeders: Option<i32>,
}

/// The `[Tpb]` config section (5 keys).
#[nested(case = snake)]
pub struct Tpb {
    /// `TPB_HOST`
    #[wire(name = "TPB_HOST")]
    pub host: Option<String>,
    /// `TPB`
    #[wire(name = "TPB")]
    pub tpb: Option<bool>,
    /// `TPB_DLPRIORITY`
    #[wire(name = "TPB_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `TPB_DLTYPES`
    #[wire(name = "TPB_DLTYPES")]
    pub dltypes: Option<String>,
    /// `TPB_SEEDERS`
    #[wire(name = "TPB_SEEDERS")]
    pub seeders: Option<i32>,
}

/// The `[Tdl]` config section (5 keys).
#[nested(case = snake)]
pub struct Tdl {
    /// `TDL_HOST`
    #[wire(name = "TDL_HOST")]
    pub host: Option<String>,
    /// `TDL`
    #[wire(name = "TDL")]
    pub tdl: Option<bool>,
    /// `TDL_DLPRIORITY`
    #[wire(name = "TDL_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `TDL_DLTYPES`
    #[wire(name = "TDL_DLTYPES")]
    pub dltypes: Option<String>,
    /// `TDL_SEEDERS`
    #[wire(name = "TDL_SEEDERS")]
    pub seeders: Option<i32>,
}

/// The `[Bok]` config section (10 keys).
#[nested(case = snake)]
pub struct Bok {
    /// `BOK_HOST`
    #[wire(name = "BOK_HOST")]
    pub host: Option<String>,
    /// `BOK_PASS`
    #[wire(name = "BOK_PASS")]
    pub pass_: Option<SecretValue>,
    /// `BOK_EMAIL`
    #[wire(name = "BOK_EMAIL")]
    pub email: Option<String>,
    /// `BOK_REMIX_USERID`
    #[wire(name = "BOK_REMIX_USERID")]
    pub remix_userid: Option<String>,
    /// `BOK_REMIX_USERKEY`
    #[wire(name = "BOK_REMIX_USERKEY")]
    pub remix_userkey: Option<String>,
    /// `BOK_SEARCH_LANG`
    #[wire(name = "BOK_SEARCH_LANG")]
    pub search_lang: Option<String>,
    /// `BOK`
    #[wire(name = "BOK")]
    pub bok: Option<bool>,
    /// `BOK_DLPRIORITY`
    #[wire(name = "BOK_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `BOK_DLLIMIT`
    #[wire(name = "BOK_DLLIMIT")]
    pub dllimit: Option<i32>,
    /// `BOK_DLTYPES`
    #[wire(name = "BOK_DLTYPES")]
    pub dltypes: Option<String>,
}

/// The `[Slsk]` config section (8 keys).
#[nested(case = snake)]
pub struct Slsk {
    /// `SLSK_HOST`
    #[wire(name = "SLSK_HOST")]
    pub host: Option<String>,
    /// `SLSK_API`
    #[wire(name = "SLSK_API")]
    pub api: Option<SecretValue>,
    /// `SLSK_URLBASE`
    #[wire(name = "SLSK_URLBASE")]
    pub urlbase: Option<String>,
    /// `SLSK`
    #[wire(name = "SLSK")]
    pub slsk: Option<bool>,
    /// `SLSK_DLPRIORITY`
    #[wire(name = "SLSK_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `SLSK_REMOTE`
    #[wire(name = "SLSK_REMOTE")]
    pub remote: Option<String>,
    /// `SLSK_LOCAL`
    #[wire(name = "SLSK_LOCAL")]
    pub local: Option<String>,
    /// `SLSK_DLTYPES`
    #[wire(name = "SLSK_DLTYPES")]
    pub dltypes: Option<String>,
}

/// The `[Anna]` config section (8 keys).
#[nested(case = snake)]
pub struct Anna {
    /// `ANNA_HOST`
    #[wire(name = "ANNA_HOST")]
    pub host: Option<String>,
    /// `ANNA_KEY`
    #[wire(name = "ANNA_KEY")]
    pub key: Option<SecretValue>,
    /// `ANNA`
    #[wire(name = "ANNA")]
    pub anna: Option<bool>,
    /// `ANNA_DLPRIORITY`
    #[wire(name = "ANNA_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `ANNA_DLLIMIT`
    #[wire(name = "ANNA_DLLIMIT")]
    pub dllimit: Option<i32>,
    /// `ANNA_MAX_SERVERS`
    #[wire(name = "ANNA_MAX_SERVERS")]
    pub max_servers: Option<i32>,
    /// `ANNA_SEARCH_LANG`
    #[wire(name = "ANNA_SEARCH_LANG")]
    pub search_lang: Option<String>,
    /// `ANNA_DLTYPES`
    #[wire(name = "ANNA_DLTYPES")]
    pub dltypes: Option<String>,
}

/// The `[Lime]` config section (5 keys).
#[nested(case = snake)]
pub struct Lime {
    /// `LIME_HOST`
    #[wire(name = "LIME_HOST")]
    pub host: Option<String>,
    /// `LIME`
    #[wire(name = "LIME")]
    pub lime: Option<bool>,
    /// `LIME_DLPRIORITY`
    #[wire(name = "LIME_DLPRIORITY")]
    pub dlpriority: Option<i32>,
    /// `LIME_DLTYPES`
    #[wire(name = "LIME_DLTYPES")]
    pub dltypes: Option<String>,
    /// `LIME_SEEDERS`
    #[wire(name = "LIME_SEEDERS")]
    pub seeders: Option<i32>,
}

/// The `[Newzbin]` config section (3 keys).
#[nested(case = snake)]
pub struct Newzbin {
    /// `NEWZBIN_UID`
    #[wire(name = "NEWZBIN_UID")]
    pub uid: Option<String>,
    /// `NEWZBIN_PASS`
    #[wire(name = "NEWZBIN_PASS")]
    pub pass_: Option<SecretValue>,
    /// `NEWZBIN`
    #[wire(name = "NEWZBIN")]
    pub newzbin: Option<bool>,
}

/// The `[Searchscan]` config section (15 keys).
#[nested(case = snake)]
pub struct Searchscan {
    /// `SEARCH_BOOKINTERVAL`
    #[wire(name = "SEARCH_BOOKINTERVAL")]
    pub search_bookinterval: Option<i32>,
    /// `SEARCH_MAGINTERVAL`
    #[wire(name = "SEARCH_MAGINTERVAL")]
    pub search_maginterval: Option<i32>,
    /// `SCAN_INTERVAL`
    #[wire(name = "SCAN_INTERVAL")]
    pub scan_interval: Option<i32>,
    /// `SEARCHRSS_INTERVAL`
    #[wire(name = "SEARCHRSS_INTERVAL")]
    pub searchrss_interval: Option<i32>,
    /// `WISHLIST_INTERVAL`
    #[wire(name = "WISHLIST_INTERVAL")]
    pub wishlist_interval: Option<i32>,
    /// `SEARCH_COMICINTERVAL`
    #[wire(name = "SEARCH_COMICINTERVAL")]
    pub search_comicinterval: Option<i32>,
    /// `VERSIONCHECK_INTERVAL`
    #[wire(name = "VERSIONCHECK_INTERVAL")]
    pub versioncheck_interval: Option<i32>,
    /// `GOODREADS_INTERVAL`
    #[wire(name = "GOODREADS_INTERVAL")]
    pub goodreads_interval: Option<i32>,
    /// `HARDCOVER_INTERVAL`
    #[wire(name = "HARDCOVER_INTERVAL")]
    pub hardcover_interval: Option<i32>,
    /// `CLEAN_CACHE_INTERVAL`
    #[wire(name = "CLEAN_CACHE_INTERVAL")]
    pub clean_cache_interval: Option<i32>,
    /// `BACKUP_INTERVAL`
    #[wire(name = "BACKUP_INTERVAL")]
    pub backup_interval: Option<i32>,
    /// `AUTHORUPDATE_INTERVAL`
    #[wire(name = "AUTHORUPDATE_INTERVAL")]
    pub authorupdate_interval: Option<i32>,
    /// `SERIESUPDATE_INTERVAL`
    #[wire(name = "SERIESUPDATE_INTERVAL")]
    pub seriesupdate_interval: Option<i32>,
    /// `DELAYSEARCH`
    #[wire(name = "DELAYSEARCH")]
    pub delaysearch: Option<bool>,
    /// `SEARCH_RATELIMIT`
    #[wire(name = "SEARCH_RATELIMIT")]
    pub search_ratelimit: Option<i32>,
}

/// The `[Libraryscan]` config section (21 keys).
#[nested(case = snake)]
pub struct Libraryscan {
    /// `FULL_SCAN`
    #[wire(name = "FULL_SCAN")]
    pub full_scan: Option<bool>,
    /// `ADD_AUTHOR`
    #[wire(name = "ADD_AUTHOR")]
    pub add_author: Option<bool>,
    /// `ADD_SERIES`
    #[wire(name = "ADD_SERIES")]
    pub add_series: Option<bool>,
    /// `NOTFOUND_STATUS`
    #[wire(name = "NOTFOUND_STATUS")]
    pub notfound_status: Option<String>,
    /// `FOUND_STATUS`
    #[wire(name = "FOUND_STATUS")]
    pub found_status: Option<String>,
    /// `NO_SINGLE_BOOK_SERIES`
    #[wire(name = "NO_SINGLE_BOOK_SERIES")]
    pub no_single_book_series: Option<bool>,
    /// `NO_NONINTEGER_SERIES`
    #[wire(name = "NO_NONINTEGER_SERIES")]
    pub no_noninteger_series: Option<bool>,
    /// `NEWSERIES_STATUS`
    #[wire(name = "NEWSERIES_STATUS")]
    pub newseries_status: Option<String>,
    /// `NEWBOOK_STATUS`
    #[wire(name = "NEWBOOK_STATUS")]
    pub newbook_status: Option<String>,
    /// `NEWAUDIO_STATUS`
    #[wire(name = "NEWAUDIO_STATUS")]
    pub newaudio_status: Option<String>,
    /// `NEWAUTHOR_STATUS`
    #[wire(name = "NEWAUTHOR_STATUS")]
    pub newauthor_status: Option<String>,
    /// `NEWAUTHOR_AUDIO`
    #[wire(name = "NEWAUTHOR_AUDIO")]
    pub newauthor_audio: Option<String>,
    /// `NEWAUTHOR_BOOKS`
    #[wire(name = "NEWAUTHOR_BOOKS")]
    pub newauthor_books: Option<bool>,
    /// `NO_FUTURE`
    #[wire(name = "NO_FUTURE")]
    pub no_future: Option<bool>,
    /// `NO_PUBDATE`
    #[wire(name = "NO_PUBDATE")]
    pub no_pubdate: Option<bool>,
    /// `NO_ISBN`
    #[wire(name = "NO_ISBN")]
    pub no_isbn: Option<bool>,
    /// `NO_SETS`
    #[wire(name = "NO_SETS")]
    pub no_sets: Option<bool>,
    /// `NO_LANG`
    #[wire(name = "NO_LANG")]
    pub no_lang: Option<bool>,
    /// `ISBN_LOOKUP`
    #[wire(name = "ISBN_LOOKUP")]
    pub isbn_lookup: Option<bool>,
    /// `IMP_IGNORE`
    #[wire(name = "IMP_IGNORE")]
    pub imp_ignore: Option<bool>,
    /// `CONTRIBUTING_AUTHORS`
    #[wire(name = "CONTRIBUTING_AUTHORS")]
    pub contributing_authors: Option<bool>,
}

/// The `[Comics]` config section (10 keys).
#[nested(case = snake)]
pub struct Comics {
    /// `COMIC_DEST_FOLDER`
    #[wire(name = "COMIC_DEST_FOLDER")]
    pub comic_dest_folder: Option<String>,
    /// `COMIC_TAB`
    #[wire(name = "COMIC_TAB")]
    pub comic_tab: Option<bool>,
    /// `COMIC_RELATIVE`
    #[wire(name = "COMIC_RELATIVE")]
    pub comic_relative: Option<bool>,
    /// `COMIC_DELFOLDER`
    #[wire(name = "COMIC_DELFOLDER")]
    pub comic_delfolder: Option<bool>,
    /// `COMIC_TYPE`
    #[wire(name = "COMIC_TYPE")]
    pub comic_type: Option<String>,
    /// `COMIC_SINGLE`
    #[wire(name = "COMIC_SINGLE")]
    pub comic_single: Option<bool>,
    /// `REJECT_COMIC`
    #[wire(name = "REJECT_COMIC")]
    pub reject_comic: Option<String>,
    /// `REJECT_MAXCOMIC`
    #[wire(name = "REJECT_MAXCOMIC")]
    pub reject_maxcomic: Option<i32>,
    /// `REJECT_MINCOMIC`
    #[wire(name = "REJECT_MINCOMIC")]
    pub reject_mincomic: Option<i32>,
    /// `CV_APIKEY`
    #[wire(name = "CV_APIKEY")]
    pub cv_apikey: Option<SecretValue>,
}

/// The `[Magazines]` config section (20 keys).
#[nested(case = snake)]
pub struct Magazines {
    /// `MAG_TAB`
    #[wire(name = "MAG_TAB")]
    pub mag_tab: Option<bool>,
    /// `MAG_COVERSWAP`
    #[wire(name = "MAG_COVERSWAP")]
    pub mag_coverswap: Option<String>,
    /// `MAG_DEST_FOLDER`
    #[wire(name = "MAG_DEST_FOLDER")]
    pub mag_dest_folder: Option<String>,
    /// `MAG_DEST_FILE`
    #[wire(name = "MAG_DEST_FILE")]
    pub mag_dest_file: Option<String>,
    /// `MAG_RELATIVE`
    #[wire(name = "MAG_RELATIVE")]
    pub mag_relative: Option<bool>,
    /// `MAG_DELFOLDER`
    #[wire(name = "MAG_DELFOLDER")]
    pub mag_delfolder: Option<bool>,
    /// `REJECT_MAGS`
    #[wire(name = "REJECT_MAGS")]
    pub reject_mags: Option<String>,
    /// `REJECT_MAXSIZE`
    #[wire(name = "REJECT_MAXSIZE")]
    pub reject_maxsize: Option<i32>,
    /// `REJECT_MINSIZE`
    #[wire(name = "REJECT_MINSIZE")]
    pub reject_minsize: Option<i32>,
    /// `REJECT_MAXAUDIO`
    #[wire(name = "REJECT_MAXAUDIO")]
    pub reject_maxaudio: Option<i32>,
    /// `REJECT_MINAUDIO`
    #[wire(name = "REJECT_MINAUDIO")]
    pub reject_minaudio: Option<i32>,
    /// `REJECT_MAGSIZE`
    #[wire(name = "REJECT_MAGSIZE")]
    pub reject_magsize: Option<i32>,
    /// `REJECT_MAGMIN`
    #[wire(name = "REJECT_MAGMIN")]
    pub reject_magmin: Option<i32>,
    /// `IMP_MAGOPF`
    #[wire(name = "IMP_MAGOPF")]
    pub imp_magopf: Option<bool>,
    /// `IMP_MAGCOVER`
    #[wire(name = "IMP_MAGCOVER")]
    pub imp_magcover: Option<bool>,
    /// `MAG_SINGLE`
    #[wire(name = "MAG_SINGLE")]
    pub mag_single: Option<bool>,
    /// `MAG_NOUNS`
    #[wire(name = "MAG_NOUNS")]
    pub mag_nouns: Option<String>,
    /// `IMP_AUTOADDMAG`
    #[wire(name = "IMP_AUTOADDMAG")]
    pub imp_autoaddmag: Option<String>,
    /// `IMP_AUTOADDMAG_COPY`
    #[wire(name = "IMP_AUTOADDMAG_COPY")]
    pub imp_autoaddmag_copy: Option<bool>,
    /// `IMP_AUTOADD_MAGONLY`
    #[wire(name = "IMP_AUTOADD_MAGONLY")]
    pub imp_autoadd_magonly: Option<bool>,
}

/// The `[Twitter]` config section (6 keys).
#[nested(case = snake)]
pub struct Twitter {
    /// `USE_TWITTER`
    #[wire(name = "USE_TWITTER")]
    pub use_twitter: Option<bool>,
    /// `TWITTER_NOTIFY_ONSNATCH`
    #[wire(name = "TWITTER_NOTIFY_ONSNATCH")]
    pub notify_onsnatch: Option<bool>,
    /// `TWITTER_NOTIFY_ONDOWNLOAD`
    #[wire(name = "TWITTER_NOTIFY_ONDOWNLOAD")]
    pub notify_ondownload: Option<bool>,
    /// `TWITTER_USERNAME`
    #[wire(name = "TWITTER_USERNAME")]
    pub username: Option<String>,
    /// `TWITTER_PASSWORD`
    #[wire(name = "TWITTER_PASSWORD")]
    pub password: Option<SecretValue>,
    /// `TWITTER_PREFIX`
    #[wire(name = "TWITTER_PREFIX")]
    pub prefix: Option<String>,
}

/// The `[Boxcar]` config section (4 keys).
#[nested(case = snake)]
pub struct Boxcar {
    /// `USE_BOXCAR`
    #[wire(name = "USE_BOXCAR")]
    pub use_boxcar: Option<bool>,
    /// `BOXCAR_NOTIFY_ONSNATCH`
    #[wire(name = "BOXCAR_NOTIFY_ONSNATCH")]
    pub notify_onsnatch: Option<bool>,
    /// `BOXCAR_NOTIFY_ONDOWNLOAD`
    #[wire(name = "BOXCAR_NOTIFY_ONDOWNLOAD")]
    pub notify_ondownload: Option<bool>,
    /// `BOXCAR_TOKEN`
    #[wire(name = "BOXCAR_TOKEN")]
    pub token: Option<SecretValue>,
}

/// The `[Pushbullet]` config section (5 keys).
#[nested(case = snake)]
pub struct Pushbullet {
    /// `USE_PUSHBULLET`
    #[wire(name = "USE_PUSHBULLET")]
    pub use_pushbullet: Option<bool>,
    /// `PUSHBULLET_NOTIFY_ONSNATCH`
    #[wire(name = "PUSHBULLET_NOTIFY_ONSNATCH")]
    pub notify_onsnatch: Option<bool>,
    /// `PUSHBULLET_NOTIFY_ONDOWNLOAD`
    #[wire(name = "PUSHBULLET_NOTIFY_ONDOWNLOAD")]
    pub notify_ondownload: Option<bool>,
    /// `PUSHBULLET_TOKEN`
    #[wire(name = "PUSHBULLET_TOKEN")]
    pub token: Option<SecretValue>,
    /// `PUSHBULLET_DEVICEID`
    #[wire(name = "PUSHBULLET_DEVICEID")]
    pub deviceid: Option<String>,
}

/// The `[Pushover]` config section (7 keys).
#[nested(case = snake)]
pub struct Pushover {
    /// `USE_PUSHOVER`
    #[wire(name = "USE_PUSHOVER")]
    pub use_pushover: Option<bool>,
    /// `PUSHOVER_ONSNATCH`
    #[wire(name = "PUSHOVER_ONSNATCH")]
    pub onsnatch: Option<bool>,
    /// `PUSHOVER_ONDOWNLOAD`
    #[wire(name = "PUSHOVER_ONDOWNLOAD")]
    pub ondownload: Option<bool>,
    /// `PUSHOVER_KEYS`
    #[wire(name = "PUSHOVER_KEYS")]
    pub keys: Option<String>,
    /// `PUSHOVER_APITOKEN`
    #[wire(name = "PUSHOVER_APITOKEN")]
    pub apitoken: Option<SecretValue>,
    /// `PUSHOVER_PRIORITY`
    #[wire(name = "PUSHOVER_PRIORITY")]
    pub priority: Option<i32>,
    /// `PUSHOVER_DEVICE`
    #[wire(name = "PUSHOVER_DEVICE")]
    pub device: Option<String>,
}

/// The `[Androidpn]` config section (6 keys).
#[nested(case = snake)]
pub struct AndroidPn {
    /// `USE_ANDROIDPN`
    #[wire(name = "USE_ANDROIDPN")]
    pub use_androidpn: Option<bool>,
    /// `ANDROIDPN_NOTIFY_ONSNATCH`
    #[wire(name = "ANDROIDPN_NOTIFY_ONSNATCH")]
    pub notify_onsnatch: Option<bool>,
    /// `ANDROIDPN_NOTIFY_ONDOWNLOAD`
    #[wire(name = "ANDROIDPN_NOTIFY_ONDOWNLOAD")]
    pub notify_ondownload: Option<bool>,
    /// `ANDROIDPN_URL`
    #[wire(name = "ANDROIDPN_URL")]
    pub url: Option<String>,
    /// `ANDROIDPN_USERNAME`
    #[wire(name = "ANDROIDPN_USERNAME")]
    pub username: Option<String>,
    /// `ANDROIDPN_BROADCAST`
    #[wire(name = "ANDROIDPN_BROADCAST")]
    pub broadcast: Option<bool>,
}

/// The `[Telegram]` config section (5 keys).
#[nested(case = snake)]
pub struct Telegram {
    /// `USE_TELEGRAM`
    #[wire(name = "USE_TELEGRAM")]
    pub use_telegram: Option<bool>,
    /// `TELEGRAM_TOKEN`
    #[wire(name = "TELEGRAM_TOKEN")]
    pub token: Option<SecretValue>,
    /// `TELEGRAM_USERID`
    #[wire(name = "TELEGRAM_USERID")]
    pub userid: Option<String>,
    /// `TELEGRAM_ONSNATCH`
    #[wire(name = "TELEGRAM_ONSNATCH")]
    pub onsnatch: Option<bool>,
    /// `TELEGRAM_ONDOWNLOAD`
    #[wire(name = "TELEGRAM_ONDOWNLOAD")]
    pub ondownload: Option<bool>,
}

/// The `[Prowl]` config section (5 keys).
#[nested(case = snake)]
pub struct Prowl {
    /// `USE_PROWL`
    #[wire(name = "USE_PROWL")]
    pub use_prowl: Option<bool>,
    /// `PROWL_APIKEY`
    #[wire(name = "PROWL_APIKEY")]
    pub apikey: Option<SecretValue>,
    /// `PROWL_PRIORITY`
    #[wire(name = "PROWL_PRIORITY")]
    pub priority: Option<i32>,
    /// `PROWL_ONSNATCH`
    #[wire(name = "PROWL_ONSNATCH")]
    pub onsnatch: Option<bool>,
    /// `PROWL_ONDOWNLOAD`
    #[wire(name = "PROWL_ONDOWNLOAD")]
    pub ondownload: Option<bool>,
}

/// The `[Growl]` config section (5 keys).
#[nested(case = snake)]
pub struct Growl {
    /// `USE_GROWL`
    #[wire(name = "USE_GROWL")]
    pub use_growl: Option<bool>,
    /// `GROWL_HOST`
    #[wire(name = "GROWL_HOST")]
    pub host: Option<String>,
    /// `GROWL_PASSWORD`
    #[wire(name = "GROWL_PASSWORD")]
    pub password: Option<SecretValue>,
    /// `GROWL_ONSNATCH`
    #[wire(name = "GROWL_ONSNATCH")]
    pub onsnatch: Option<bool>,
    /// `GROWL_ONDOWNLOAD`
    #[wire(name = "GROWL_ONDOWNLOAD")]
    pub ondownload: Option<bool>,
}

/// The `[Slack]` config section (5 keys).
#[nested(case = snake)]
pub struct Slack {
    /// `USE_SLACK`
    #[wire(name = "USE_SLACK")]
    pub use_slack: Option<bool>,
    /// `SLACK_NOTIFY_ONSNATCH`
    #[wire(name = "SLACK_NOTIFY_ONSNATCH")]
    pub notify_onsnatch: Option<bool>,
    /// `SLACK_NOTIFY_ONDOWNLOAD`
    #[wire(name = "SLACK_NOTIFY_ONDOWNLOAD")]
    pub notify_ondownload: Option<bool>,
    /// `SLACK_TOKEN`
    #[wire(name = "SLACK_TOKEN")]
    pub token: Option<SecretValue>,
    /// `SLACK_URL`
    #[wire(name = "SLACK_URL")]
    pub url: Option<String>,
}

/// The `[Custom]` config section (4 keys).
#[nested(case = snake)]
pub struct Custom {
    /// `USE_CUSTOM`
    #[wire(name = "USE_CUSTOM")]
    pub use_custom: Option<bool>,
    /// `CUSTOM_NOTIFY_ONSNATCH`
    #[wire(name = "CUSTOM_NOTIFY_ONSNATCH")]
    pub notify_onsnatch: Option<bool>,
    /// `CUSTOM_NOTIFY_ONDOWNLOAD`
    #[wire(name = "CUSTOM_NOTIFY_ONDOWNLOAD")]
    pub notify_ondownload: Option<bool>,
    /// `CUSTOM_SCRIPT`
    #[wire(name = "CUSTOM_SCRIPT")]
    pub script: Option<String>,
}

/// The `[Email]` config section (16 keys).
#[nested(case = snake)]
pub struct Email {
    /// `USE_EMAIL`
    #[wire(name = "USE_EMAIL")]
    pub use_email: Option<bool>,
    /// `EMAIL_NOTIFY_ONSNATCH`
    #[wire(name = "EMAIL_NOTIFY_ONSNATCH")]
    pub notify_onsnatch: Option<bool>,
    /// `EMAIL_NOTIFY_ONDOWNLOAD`
    #[wire(name = "EMAIL_NOTIFY_ONDOWNLOAD")]
    pub notify_ondownload: Option<bool>,
    /// `EMAIL_SENDFILE_ONDOWNLOAD`
    #[wire(name = "EMAIL_SENDFILE_ONDOWNLOAD")]
    pub sendfile_ondownload: Option<bool>,
    /// `EMAIL_FROM`
    #[wire(name = "EMAIL_FROM")]
    pub from_: Option<String>,
    /// `EMAIL_TO`
    #[wire(name = "EMAIL_TO")]
    pub to: Option<String>,
    /// `EMAIL_SSL`
    #[wire(name = "EMAIL_SSL")]
    pub ssl: Option<bool>,
    /// `EMAIL_SMTP_SERVER`
    #[wire(name = "EMAIL_SMTP_SERVER")]
    pub smtp_server: Option<String>,
    /// `EMAIL_SMTP_PORT`
    #[wire(name = "EMAIL_SMTP_PORT")]
    pub smtp_port: Option<i32>,
    /// `EMAIL_TLS`
    #[wire(name = "EMAIL_TLS")]
    pub tls: Option<bool>,
    /// `EMAIL_SMTP_USER`
    #[wire(name = "EMAIL_SMTP_USER")]
    pub smtp_user: Option<String>,
    /// `EMAIL_SMTP_PASSWORD`
    #[wire(name = "EMAIL_SMTP_PASSWORD")]
    pub smtp_password: Option<SecretValue>,
    /// `EMAIL_LIMIT`
    #[wire(name = "EMAIL_LIMIT")]
    pub limit: Option<i32>,
    /// `USE_EMAIL_CUSTOM_FORMAT`
    #[wire(name = "USE_EMAIL_CUSTOM_FORMAT")]
    pub use_email_custom_format: Option<bool>,
    /// `EMAIL_CONVERT_FROM`
    #[wire(name = "EMAIL_CONVERT_FROM")]
    pub convert_from: Option<String>,
    /// `EMAIL_SEND_TYPE`
    #[wire(name = "EMAIL_SEND_TYPE")]
    pub send_type: Option<String>,
}

/// The `[Fmt]` config section (3 keys).
#[nested(case = snake)]
pub struct Fmt {
    /// `FMT_SERNAME`
    #[wire(name = "FMT_SERNAME")]
    pub sername: Option<String>,
    /// `FMT_SERNUM`
    #[wire(name = "FMT_SERNUM")]
    pub sernum: Option<String>,
    /// `FMT_SERIES`
    #[wire(name = "FMT_SERIES")]
    pub series: Option<String>,
}

/// The `[Opds]` config section (6 keys).
#[nested(case = snake)]
pub struct Opds {
    /// `OPDS_ENABLED`
    #[wire(name = "OPDS_ENABLED")]
    pub enabled: Option<bool>,
    /// `OPDS_AUTHENTICATION`
    #[wire(name = "OPDS_AUTHENTICATION")]
    pub authentication: Option<bool>,
    /// `OPDS_USERNAME`
    #[wire(name = "OPDS_USERNAME")]
    pub username: Option<String>,
    /// `OPDS_PASSWORD`
    #[wire(name = "OPDS_PASSWORD")]
    pub password: Option<SecretValue>,
    /// `OPDS_METAINFO`
    #[wire(name = "OPDS_METAINFO")]
    pub metainfo: Option<bool>,
    /// `OPDS_PAGE`
    #[wire(name = "OPDS_PAGE")]
    pub page: Option<i32>,
}

/// The `[Rss]` config section (3 keys).
#[nested(case = snake)]
pub struct RssConfig {
    /// `RSS_ENABLED`
    #[wire(name = "RSS_ENABLED")]
    pub enabled: Option<bool>,
    /// `RSS_PODCAST`
    #[wire(name = "RSS_PODCAST")]
    pub podcast: Option<bool>,
    /// `RSS_HOST`
    #[wire(name = "RSS_HOST")]
    pub host: Option<String>,
}

/// The `[Preprocess]` config section (15 keys).
#[nested(case = snake)]
pub struct Preprocess {
    /// `EBOOK_WANTED_FORMATS`
    #[wire(name = "EBOOK_WANTED_FORMATS")]
    pub ebook_wanted_formats: Option<String>,
    /// `DELETE_OTHER_FORMATS`
    #[wire(name = "DELETE_OTHER_FORMATS")]
    pub delete_other_formats: Option<bool>,
    /// `EBOOK_CONVERT`
    #[wire(name = "EBOOK_CONVERT")]
    pub ebook_convert: Option<String>,
    /// `KEEP_OPF`
    #[wire(name = "KEEP_OPF")]
    pub keep_opf: Option<bool>,
    /// `KEEP_JPG`
    #[wire(name = "KEEP_JPG")]
    pub keep_jpg: Option<bool>,
    /// `FFMPEG`
    #[wire(name = "FFMPEG")]
    pub ffmpeg: Option<String>,
    /// `FFMPEG_OUT`
    #[wire(name = "FFMPEG_OUT")]
    pub ffmpeg_out: Option<String>,
    /// `AUDIO_OPTIONS`
    #[wire(name = "AUDIO_OPTIONS")]
    pub audio_options: Option<String>,
    /// `CREATE_SINGLEAUDIO`
    #[wire(name = "CREATE_SINGLEAUDIO")]
    pub create_singleaudio: Option<bool>,
    /// `KEEP_SEPARATEAUDIO`
    #[wire(name = "KEEP_SEPARATEAUDIO")]
    pub keep_separateaudio: Option<bool>,
    /// `WRITE_AUDIOTAGS`
    #[wire(name = "WRITE_AUDIOTAGS")]
    pub write_audiotags: Option<bool>,
    /// `ZIP_AUDIOPARTS`
    #[wire(name = "ZIP_AUDIOPARTS")]
    pub zip_audioparts: Option<bool>,
    /// `SWAP_COVERPAGE`
    #[wire(name = "SWAP_COVERPAGE")]
    pub swap_coverpage: Option<bool>,
    /// `TAG_PDF`
    #[wire(name = "TAG_PDF")]
    pub tag_pdf: Option<bool>,
    /// `SHRINK_MAG`
    #[wire(name = "SHRINK_MAG")]
    pub shrink_mag: Option<i32>,
}
