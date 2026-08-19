//! `/api/settings` — the server-wide configuration singleton.
//!
//! **There is no `GET` for this endpoint at all.** Checked against the raw
//! spec: `/api/settings` declares only `PATCH`; `ServerSettings` never
//! appears as a response schema anywhere else either (`/status`'s
//! `GetStatusResponse`, `/login`'s `UserLoginResponse`, `/api/authorize` —
//! none of them carry it). Real Audiobookshelf pushes the full settings
//! object to the browser over the Socket.IO `init` event, not a REST route;
//! the reverse-engineered spec reflects that, it isn't a coverage gap.
//!
//! So this is more than the "GET/PATCH shapes differ, so `sync = custom`"
//! case this crate's [`crate::resources::notification_settings::NotificationSettings`]
//! already covers — there, `sync = custom` still gets a live baseline (its
//! hook fetches `GET /api/notifications` and reads `.settings`) to diff
//! against. Here there is **no live baseline this crate can read over
//! HTTP, full stop**. Consequence: this hook cannot tell whether a declared
//! value already matches the server. It sends the `PATCH` body = exactly the
//! keys the user wrote (`engine::config_present_to_wire`, the same
//! presence-masking a diffable singleton/custom resource would use to build
//! its desired side) whenever that set is non-empty, and reports
//! `Change::updated` unconditionally when it does — there is no way to earn
//! a legitimate `Unchanged` here, so this resource's plan output should be
//! read as "will (re)send these keys", not "these keys differ". The server
//! is itself idempotent about persisting only actually-changed keys (per its
//! own doc: "Any key omitted is left unchanged"), so a repeated apply is
//! harmless — just permanently noisy in the report, the same tradeoff
//! [`crate::resources::user::User`] documents for an undetectable `password`
//! change, just for the whole resource instead of one field.
//!
//! `sortingPrefixes` is deliberately not modeled: the spec notes it's
//! "accepted but silently ignored" by this route ("use `PATCH
//! /api/sorting-prefixes`" instead, a different route this slice doesn't
//! cover) and it is in fact absent from `UpdateServerSettingsRequest`'s own
//! property list.
//!
//! Fields never echoed by any settings response at all
//! (`authOpenIDClientSecret`, `authOpenIDMobileRedirectURIs`) inherit the
//! same "can't detect convergence" caveat as every other field here — which,
//! given there is no live baseline whatsoever, is not actually a *special*
//! case for this resource the way it is for `User::password`. `tokenSecret`
//! is not modeled at all: it never appears in `UpdateServerSettingsRequest`,
//! i.e. it isn't settable through this route either.
//!
//! Every field is `Option<T>` and none carry `#[default(...)]`, on purpose —
//! same reasoning as [`crate::resources::library_settings::LibrarySettings`]:
//! this hook always presence-masks, so a literal default would only apply to
//! an unmasked encode path this resource never uses; server-side defaults
//! (from the read-side `ServerSettings` schema, since the write side never
//! states them) are documented in prose per field instead.
//!
//! Every `auth_open_id_*` field needs an explicit `#[wire(name = ...)]`:
//! this crate's default snake→camel conversion capitalizes only the first
//! letter after each underscore (`auth_open_id_url` → `authOpenIdUrl`), but
//! the real API spells the acronyms `ID`/`URL`/`URIs` in full caps
//! (`authOpenIDURL`) — a mismatch on every single one of these keys, not
//! just a few, so every `auth_open_id_*` field below carries an override
//! rather than relying on the default.

use core_lib::{
    Change, CustomSync, CustomSyncFuture, HttpClient, Json, RefStore, SecretValue, engine,
};
use core_macros::resource;
use serde_json::Value;

/// `PATCH /api/settings` — see the module doc for why there is no `read`.
const SETTINGS_PATH: &str = "/api/settings";

/// Server-wide configuration. See the module doc for the "no live baseline"
/// caveat that applies to every field below.
#[resource(sync = custom)]
pub struct ServerSettings {
    /// Extract embedded cover art during scans. Read-side default `false`.
    pub scanner_find_covers: Option<bool>,
    /// Cover art provider used by `scanner_find_covers`. Read-side default
    /// `"google"`.
    pub scanner_cover_provider: Option<String>,
    /// Parse a subtitle out of the filename during scans. Read-side default
    /// `false`.
    pub scanner_parse_subtitle: Option<bool>,
    /// Prefer metadata already matched over freshly scanned file metadata.
    /// Read-side default `false`.
    pub scanner_prefer_matched_metadata: Option<bool>,
    /// Disable the filesystem watcher globally. Read-side default `false`.
    pub scanner_disable_watcher: Option<bool>,
    /// Store a cover image file alongside each library item. Read-side
    /// default `false`.
    pub store_cover_with_item: Option<bool>,
    /// Store a metadata file alongside each library item. Read-side default
    /// `false`.
    pub store_metadata_with_item: Option<bool>,
    /// Metadata file format. As of ABS v2.4.5 only `"json"` is supported;
    /// any other stored value is coerced back to `"json"`. Read-side default
    /// `"json"`.
    pub metadata_file_format: Option<String>,
    /// Max login attempts within `rate_limit_login_window` before
    /// rate-limiting. Read-side default `10`.
    pub rate_limit_login_requests: Option<i32>,
    /// Login rate-limit window, in milliseconds. Read-side default
    /// `600000` (10 minutes).
    pub rate_limit_login_window: Option<i32>,
    /// Allow embedding the UI in an `<iframe>`. Forced `true`, and not
    /// settable back to `false` (400), while the `ALLOW_IFRAME=1`
    /// environment variable is set. Read-side default `false`.
    pub allow_iframe: Option<bool>,
    /// Backup destination directory. Prefer `PATCH /api/backups/path`
    /// instead (not covered by this slice), which also validates the path
    /// and reloads the backup manager — this key alone does neither.
    /// Read-side default: `<MetadataPath>/backups`.
    pub backup_path: Option<String>,
    /// Cron expression for automatic backups, or `false` to disable them.
    /// Changing this immediately reloads the backup cron schedule.
    /// Heterogeneous type (string or the literal `false`) — modeled `Json`
    /// rather than a typed field. Read-side default `false`.
    pub backup_schedule: Option<Json>,
    /// Number of backups to retain. Read-side default `2`.
    pub backups_to_keep: Option<i32>,
    /// Max size, in GB, a new backup may reach before it's refused.
    /// Read-side default `1`.
    pub max_backup_size: Option<f64>,
    /// Days of daily log files to retain. Read-side default `7`.
    pub logger_daily_logs_to_keep: Option<i32>,
    /// Days of scanner log files to retain. Read-side default `2`.
    pub logger_scanner_logs_to_keep: Option<i32>,
    /// Home shelf view: `0` = standard, `1` = detail. Read-side default `1`.
    pub home_bookshelf_view: Option<i32>,
    /// Library bookshelf view: `0` = standard, `1` = detail. Read-side
    /// default `1`.
    pub bookshelf_view: Option<i32>,
    /// Cron expression for podcast episode checks. Read-side default
    /// `"0 * * * *"`.
    pub podcast_episode_schedule: Option<String>,
    /// Ignore configured sorting prefixes (`"the"`, `"a"`, ...) when
    /// sort-titling. Read-side default `false`. (The prefix list itself is
    /// not settable through this route — see the module doc.)
    pub sorting_ignore_prefix: Option<bool>,
    /// Enable Chromecast support. Read-side default `false`.
    pub chromecast_enabled: Option<bool>,
    /// Display date format. Read-side default `"MM/dd/yyyy"`.
    pub date_format: Option<String>,
    /// Display time format. Read-side default `"HH:mm"`.
    pub time_format: Option<String>,
    /// UI language tag. Read-side default `"en-us"`.
    pub language: Option<String>,
    /// Extra CORS origins allowed beyond the server's own. Rejected with 400
    /// if not an array. Read-side default `[]`.
    pub allowed_origins: Option<Vec<String>>,
    /// Server log verbosity: `0`=TRACE `1`=DEBUG `2`=INFO `3`=WARN
    /// `4`=ERROR `5`=FATAL `6`=???  (per the read schema's enum). Setting
    /// this immediately calls `Logger.setLogLevel`. Read-side default `2`.
    pub log_level: Option<i32>,
    /// Custom message shown on the login screen.
    pub auth_login_custom_message: Option<String>,
    /// Which auth methods are active (`local`, `openid`). Read-side default
    /// `["local"]`.
    pub auth_active_auth_methods: Option<Vec<String>>,
    #[wire(name = "authOpenIDIssuerURL")]
    /// OpenID issuer URL.
    pub auth_open_id_issuer_url: Option<String>,
    #[wire(name = "authOpenIDAuthorizationURL")]
    /// OpenID authorization endpoint URL.
    pub auth_open_id_authorization_url: Option<String>,
    #[wire(name = "authOpenIDTokenURL")]
    /// OpenID token endpoint URL.
    pub auth_open_id_token_url: Option<String>,
    #[wire(name = "authOpenIDUserInfoURL")]
    /// OpenID userinfo endpoint URL.
    pub auth_open_id_user_info_url: Option<String>,
    #[wire(name = "authOpenIDJwksURL")]
    /// OpenID JWKS URL.
    pub auth_open_id_jwks_url: Option<String>,
    #[wire(name = "authOpenIDLogoutURL")]
    /// OpenID logout endpoint URL.
    pub auth_open_id_logout_url: Option<String>,
    #[wire(name = "authOpenIDClientID")]
    /// OpenID client id. Per `ServerSettings`' own doc, this is stripped
    /// before any settings response reaches the browser — never echoed back
    /// by anything this crate can read, same undetectable-convergence
    /// caveat as the rest of this resource (see the module doc), just
    /// permanent rather than incidental.
    pub auth_open_id_client_id: Option<String>,
    #[wire(name = "authOpenIDClientSecret")]
    /// OpenID client secret. Never echoed back by any settings response —
    /// modeled `SecretValue` both because it's a genuine credential and so
    /// plan output redacts it.
    pub auth_open_id_client_secret: Option<SecretValue>,
    #[wire(name = "authOpenIDTokenSigningAlgorithm")]
    /// OpenID token signing algorithm. Read-side default `"RS256"`.
    pub auth_open_id_token_signing_algorithm: Option<String>,
    #[wire(name = "authOpenIDButtonText")]
    /// Login button label. Read-side default `"Login with OpenId"`.
    pub auth_open_id_button_text: Option<String>,
    #[wire(name = "authOpenIDAutoLaunch")]
    /// Auto-redirect to the OpenID provider instead of showing the local
    /// login form. Read-side default `false`.
    pub auth_open_id_auto_launch: Option<bool>,
    #[wire(name = "authOpenIDAutoRegister")]
    /// Auto-create a local account on first successful OpenID login.
    /// Read-side default `false`.
    pub auth_open_id_auto_register: Option<bool>,
    #[wire(name = "authOpenIDMatchExistingBy")]
    /// Match an OpenID login to an existing local account by `"username"` or
    /// `"email"`; unset disables matching (a new account is always created).
    pub auth_open_id_match_existing_by: Option<String>,
    #[wire(name = "authOpenIDMobileRedirectURIs")]
    /// App-scheme redirect URIs for the mobile OpenID flow. Never echoed
    /// back by any settings response.
    pub auth_open_id_mobile_redirect_uris: Option<Vec<String>>,
    #[wire(name = "authOpenIDGroupClaim")]
    /// OpenID claim used for group-based permission mapping. Per
    /// `ServerSettings`' own doc, stripped before any settings response
    /// reaches the browser — never echoed back.
    pub auth_open_id_group_claim: Option<String>,
    #[wire(name = "authOpenIDAdvancedPermsClaim")]
    /// OpenID claim used for advanced-permissions mapping. Same
    /// never-echoed-back caveat as `auth_open_id_group_claim`.
    pub auth_open_id_advanced_perms_claim: Option<String>,
    #[wire(name = "authOpenIDSubfolderForRedirectURLs")]
    /// Subfolder prefix for OpenID redirect URLs, when Audiobookshelf is
    /// served from a subpath.
    pub auth_open_id_subfolder_for_redirect_urls: Option<String>,
}

impl CustomSync for ServerSettings {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(Vec::new());
            };

            // Only the keys the user's config actually wrote — see the
            // module doc for why there is nothing to merge or diff this
            // against.
            let wire = engine::config_present_to_wire::<Self>(cfg)?;
            let Some(obj) = wire.as_object() else {
                return Ok(Vec::new());
            };
            if obj.is_empty() {
                return Ok(vec![Change::unchanged("server_settings")]);
            }

            if execute {
                let _: Value = client.patch(SETTINGS_PATH, &wire).await?;
            }
            // No live baseline exists to earn a legitimate `Unchanged` — see
            // the module doc.
            Ok(vec![Change::updated("server_settings")])
        })
    }
}
