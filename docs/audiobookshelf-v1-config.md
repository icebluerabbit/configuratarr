# Audiobookshelf v1 Configuration

Audiobookshelf — desired-state config for one instance.

The credential is a **bearer token**, not an API key: log in at
`POST /login` and read `user.accessToken`, or mint one under
`/api/api-keys`. It is named `token` rather than `api_key` because that is
what it is — and because doc-gen documents the connection from these very
field names.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Library

`/api/libraries` — a library of either books or podcasts (never mixed).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.library.<name>}`). Required. |
| `folders` | array of [`library_folder`](#library-folder) | no |  | Filesystem folders scanned into this library. Required, non-empty on create. See the module doc + [`LibraryFolder`] for the destructive replace-by-id danger on update. |
| `display_order` | integer | no |  | 1-based position among all libraries. Server-computed on create (always appended as `max+1`; the create request never accepts it, so this hook strips it from the `POST` body even if declared). Settable via this resource's `PATCH`, but note the live API doesn't renormalize other libraries' `display_order` when this one changes. |
| `icon` | string | no |  | Icon key shown in the UI (e.g. `database`, `audiobookshelf`, `podcast`); not validated against a fixed enum server-side. Server default on create: `"database"`. |
| `media_type` | [`media_type`](#media-type) | no |  | Selects which `LibrarySettings` default template seeds this library and which content model its items use. Server default on create: `book`. Effectively create-only — see the module doc. |
| `provider` | string | no |  | Preferred metadata provider slug for matching (e.g. `google`, `audible`, `itunes`, or `custom-<slug>` for a configured custom provider, which must already exist). Server default on create: `"google"`. |
| `settings` | [`library_settings`](#library-settings) | no |  | Per-library scan/matching/display settings. |

### Notification

`/api/notifications` — one rule: fire `event_name` (optionally scoped to
`library_id`) to `urls` via Apprise.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `library_id` | string | no |  | Restrict this rule to one library; unset applies to all libraries. Part of this rule's identity alongside `event_name` — audiobookshelf allows one rule per event per library, plus one "all libraries" rule per event. References a [`library`](#library) by name (`${ref.library.<key>}`). |
| `event_name` | string | yes |  | The event that fires this rule. This rule's other identity component. One of `onPodcastEpisodeDownloaded`, `onBackupCompleted`, `onBackupFailed`, `onRSSFeedFailed`, `onRSSFeedDisabled`, `onTest`. |
| `urls` | array of string | no |  | Apprise URLs (<https://github.com/caronc/apprise>) the payload is POSTed to via the configured Apprise API. Must be non-empty for the rule to actually fire. |
| `title_template` | string | no |  | Title template; `{{variable}}` placeholders are filled from the firing event's data. Absent from a fixture that doesn't override it — audiobookshelf may substitute its own per-event default text, which this resource has no way to read back and compare against. |
| `body_template` | string | no |  | Body template; `{{variable}}` placeholders are filled from the firing event's data. Same per-event-default caveat as `title_template`. |
| `enabled` | boolean | no | `false` | Whether this rule fires. Toggling disabled → enabled resets `last_fired_at`, `last_attempt_failed`, and `num_consecutive_failed_attempts` server-side. Defaults to `false` (the API's own POST default) so an omitted value still round-trips against a live `false` rather than a missing key. |
| `notification_type` | string | no |  | Free-form client-UI styling hint — the server does not validate it against its own documented enum (`info`/`success`/`warning`/ `failure`). Omitted ⇒ wire `null` (`#[wire(null)]`), matching the server's own "defaults to null, not `info`, if omitted" behavior. |

### User

`/api/users` — a local or OpenID-linked account.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | yes |  | Login name — this account's identity. Changing it server-side regenerates the legacy API token and invalidates JWT sessions; a `username` change via config is not specially handled and, like any resource keyed by a mutable field, just looks like a new user to this hook's keyed match (the old name is orphaned, not renamed). |
| `email` | string | no |  | Account email, if any. |
| `user_type` | [`user_type`](#user-type) | no |  | The account's role. Required on create; the API itself defaults to `user` if omitted there. |
| `password` | secret string | no |  | Initial password, plaintext — bcrypt-hashed server-side, never echoed back on any read. **Create-only and required to create**: ABS rejects a `POST /api/users` without one. It is never sent on update and never diffed, so leaving it in config costs nothing. To change an existing account's password, use `reset_password`. Credential — redacted in plan output. |
| `reset_password` | secret string | no |  | Opt-in password reset for an existing account. When set, **every apply** PATCHes this value — remove the field from config once the reset has taken effect, or it keeps firing. Deliberately separate from the create-only `password`; see the module doc. Credential — redacted in plan output. |
| `is_active` | boolean | no |  | Whether the account can log in. API create default: `false` — an omitted value here creates an inactive account. |
| `last_seen` | integer | no |  | Epoch millis of last activity. Rarely client-set; primarily server-maintained. Accepted on update but **not** on create (`CreateUserRequest` has no such property), so [`create_body`] strips it. |
| `permissions` | [`user_permissions`](#user-permissions) | no |  | Capability flags + library/tag scoping. See [`UserPermissions`] for how an omitted key behaves on create vs. update. |

### Ereader Device

`/api/emails/ereader-devices` — one e-reader device.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Unique display name — this device's whole identity. Also the name `POST /api/emails/send-ebook-to-device` targets it by. |
| `email` | string | yes |  | Destination email address for this device. |
| `availability_option` | [`ereader_availability`](#ereader-availability) | no |  | Who may send to this device. Unset ⇒ server default `adminOrUp`. |
| `users` | array of string | no |  | User ids allowed to use the device; meaningful only when `availability_option` is `specific_users` (the server clears this to `[]` otherwise). `${ref.user.<username>}` resolves to the referenced user's server id. References a [`user`](#user) by name (`${ref.user.<key>}`). |

### Notification Settings

`/api/notifications` (read: the `.settings` half of the envelope; write:
`PATCH /api/notifications`, three keys only) — Apprise delivery settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `apprise_api_url` | string | no |  | Base URL of the Apprise API server notifications are POSTed to. Notifications are inert while this is unset; setting it to `null` disables them. |
| `max_failed_attempts` | integer | no |  | Consecutive failures after which a rule auto-disables. API default `5` when unset. |
| `max_notification_queue` | integer | no |  | Max events queued while a prior notification send is in flight; once full, further events are dropped. API default `20` when unset. |

### Email Settings

SMTP configuration for outgoing mail. See <https://nodemailer.com/smtp/>.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | string | no | `email-settings` | Always the literal `email-settings` — this singleton's fixed id. There is no reason to declare it in config. |
| `host` | string | no |  | SMTP host. Unset/`null` disables outgoing email. |
| `port` | integer | no | `465` | SMTP port. Only port 465 is treated as implicit TLS (`secure`); any other port forces `secure: false` at connection time regardless of the stored `secure` value. API default `465`. |
| `secure` | boolean | no | `true` | Whether to use TLS. Coerced to a strict boolean server-side. API default `true`. |
| `reject_unauthorized` | boolean | no | `true` | Whether to reject self-signed/invalid TLS certificates. API default `true`. |
| `user` | string | no |  | SMTP auth username. |
| `pass` | secret string | no |  | SMTP auth password. Stored and echoed back **in plaintext** on subsequent `GET`s (`x-sensitive` in the spec, but not write-only) — see the module doc for why this is nonetheless modeled `SecretValue`. Credential — redacted in plan output. |
| `test_address` | string | no |  | Recipient used by `POST /api/emails/test`; falls back to `from_address` if unset. |
| `from_address` | string | no |  | The `From:` address for outgoing mail. |
| `ereader_devices` | array of any | no |  | Registered e-reader devices, as `GET` reports them — informational only. Manage them under `ereader_devices` at the top level, not here; declaring the field here has no effect. |

### Server Settings

Server-wide configuration. See the module doc for the "no live baseline"
caveat that applies to every field below.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `scanner_find_covers` | boolean | no |  | Extract embedded cover art during scans. Read-side default `false`. |
| `scanner_cover_provider` | string | no |  | Cover art provider used by `scanner_find_covers`. Read-side default `"google"`. |
| `scanner_parse_subtitle` | boolean | no |  | Parse a subtitle out of the filename during scans. Read-side default `false`. |
| `scanner_prefer_matched_metadata` | boolean | no |  | Prefer metadata already matched over freshly scanned file metadata. Read-side default `false`. |
| `scanner_disable_watcher` | boolean | no |  | Disable the filesystem watcher globally. Read-side default `false`. |
| `store_cover_with_item` | boolean | no |  | Store a cover image file alongside each library item. Read-side default `false`. |
| `store_metadata_with_item` | boolean | no |  | Store a metadata file alongside each library item. Read-side default `false`. |
| `metadata_file_format` | string | no |  | Metadata file format. As of ABS v2.4.5 only `"json"` is supported; any other stored value is coerced back to `"json"`. Read-side default `"json"`. |
| `rate_limit_login_requests` | integer | no |  | Max login attempts within `rate_limit_login_window` before rate-limiting. Read-side default `10`. |
| `rate_limit_login_window` | integer | no |  | Login rate-limit window, in milliseconds. Read-side default `600000` (10 minutes). |
| `allow_iframe` | boolean | no |  | Allow embedding the UI in an `<iframe>`. Forced `true`, and not settable back to `false` (400), while the `ALLOW_IFRAME=1` environment variable is set. Read-side default `false`. |
| `backup_path` | string | no |  | Backup destination directory. Prefer `PATCH /api/backups/path` instead (not covered by this slice), which also validates the path and reloads the backup manager — this key alone does neither. Read-side default: `<MetadataPath>/backups`. |
| `backup_schedule` | any | no |  | Cron expression for automatic backups, or `false` to disable them. Changing this immediately reloads the backup cron schedule. Heterogeneous type (string or the literal `false`) — modeled `Json` rather than a typed field. Read-side default `false`. |
| `backups_to_keep` | integer | no |  | Number of backups to retain. Read-side default `2`. |
| `max_backup_size` | number | no |  | Max size, in GB, a new backup may reach before it's refused. Read-side default `1`. |
| `logger_daily_logs_to_keep` | integer | no |  | Days of daily log files to retain. Read-side default `7`. |
| `logger_scanner_logs_to_keep` | integer | no |  | Days of scanner log files to retain. Read-side default `2`. |
| `home_bookshelf_view` | integer | no |  | Home shelf view: `0` = standard, `1` = detail. Read-side default `1`. |
| `bookshelf_view` | integer | no |  | Library bookshelf view: `0` = standard, `1` = detail. Read-side default `1`. |
| `podcast_episode_schedule` | string | no |  | Cron expression for podcast episode checks. Read-side default `"0 * * * *"`. |
| `sorting_ignore_prefix` | boolean | no |  | Ignore configured sorting prefixes (`"the"`, `"a"`, ...) when sort-titling. Read-side default `false`. (The prefix list itself is not settable through this route — see the module doc.) |
| `chromecast_enabled` | boolean | no |  | Enable Chromecast support. Read-side default `false`. |
| `date_format` | string | no |  | Display date format. Read-side default `"MM/dd/yyyy"`. |
| `time_format` | string | no |  | Display time format. Read-side default `"HH:mm"`. |
| `language` | string | no |  | UI language tag. Read-side default `"en-us"`. |
| `allowed_origins` | array of string | no |  | Extra CORS origins allowed beyond the server's own. Rejected with 400 if not an array. Read-side default `[]`. |
| `log_level` | integer | no |  | Server log verbosity: `0`=TRACE `1`=DEBUG `2`=INFO `3`=WARN `4`=ERROR `5`=FATAL `6`=???  (per the read schema's enum). Setting this immediately calls `Logger.setLogLevel`. Read-side default `2`. |
| `auth_login_custom_message` | string | no |  | Custom message shown on the login screen. |
| `auth_active_auth_methods` | array of string | no |  | Which auth methods are active (`local`, `openid`). Read-side default `["local"]`. |
| `auth_open_id_issuer_url` | string | no |  | OpenID issuer URL. |
| `auth_open_id_authorization_url` | string | no |  | OpenID authorization endpoint URL. |
| `auth_open_id_token_url` | string | no |  | OpenID token endpoint URL. |
| `auth_open_id_user_info_url` | string | no |  | OpenID userinfo endpoint URL. |
| `auth_open_id_jwks_url` | string | no |  | OpenID JWKS URL. |
| `auth_open_id_logout_url` | string | no |  | OpenID logout endpoint URL. |
| `auth_open_id_client_id` | string | no |  | OpenID client id. Per `ServerSettings`' own doc, this is stripped before any settings response reaches the browser — never echoed back by anything this crate can read, same undetectable-convergence caveat as the rest of this resource (see the module doc), just permanent rather than incidental. |
| `auth_open_id_client_secret` | secret string | no |  | OpenID client secret. Never echoed back by any settings response — modeled `SecretValue` both because it's a genuine credential and so plan output redacts it. Credential — redacted in plan output. |
| `auth_open_id_token_signing_algorithm` | string | no |  | OpenID token signing algorithm. Read-side default `"RS256"`. |
| `auth_open_id_button_text` | string | no |  | Login button label. Read-side default `"Login with OpenId"`. |
| `auth_open_id_auto_launch` | boolean | no |  | Auto-redirect to the OpenID provider instead of showing the local login form. Read-side default `false`. |
| `auth_open_id_auto_register` | boolean | no |  | Auto-create a local account on first successful OpenID login. Read-side default `false`. |
| `auth_open_id_match_existing_by` | string | no |  | Match an OpenID login to an existing local account by `"username"` or `"email"`; unset disables matching (a new account is always created). |
| `auth_open_id_mobile_redirect_uris` | array of string | no |  | App-scheme redirect URIs for the mobile OpenID flow. Never echoed back by any settings response. |
| `auth_open_id_group_claim` | string | no |  | OpenID claim used for group-based permission mapping. Per `ServerSettings`' own doc, stripped before any settings response reaches the browser — never echoed back. |
| `auth_open_id_advanced_perms_claim` | string | no |  | OpenID claim used for advanced-permissions mapping. Same never-echoed-back caveat as `auth_open_id_group_claim`. |
| `auth_open_id_subfolder_for_redirect_urls` | string | no |  | Subfolder prefix for OpenID redirect URLs, when Audiobookshelf is served from a subpath. |

## Types

### Library Folder

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `full_path` | string | yes |  | Absolute filesystem path on the Audiobookshelf server to scan into this library (e.g. `/audiobooks`). Matched against the library's live folders by exact string equality to decide create-vs-keep on update. |

### Media Type

Allowed values: `book` / `podcast`.

### Library Settings

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `cover_aspect_ratio` | integer | no |  | Cover thumbnail aspect ratio: `0` = standard (2:3), `1` = square. Shared by both media types. Server default: `1`. |
| `disable_watcher` | boolean | no |  | Disable the filesystem watcher for this library. Shared. Toggling this restarts the watcher. Server default: `false`. |
| `auto_scan_cron_expression` | string | no |  | Standard 6-field (seconds-first) cron expression for the scheduled scan (e.g. `0 0 0 * * *`); absent disables the scheduled scan. Shared. Changing this on update reprograms the scan cron job. **Known limitation:** the live API accepts an explicit `null` on update to *clear* an existing schedule, but this crate's config codec treats an explicit YAML `null` identically to omitting the key — so this resource can declare "leave unset" or "set a value" but has no way to declare "explicitly clear a previously-set schedule"; clearing it today requires an out-of-band call. |
| `skip_matching_media_with_asin` | boolean | no |  | Book libraries only. Skip quick-match for books that already have an ASIN. Server default: `false`. |
| `skip_matching_media_with_isbn` | boolean | no |  | Book libraries only. Skip quick-match for books that already have an ISBN. Server default: `false`. |
| `audiobooks_only` | boolean | no |  | Book libraries only. Ignore ebook files entirely except as supplementary files to an audiobook. Server default: `false`. |
| `epubs_allow_scripted_content` | boolean | no |  | Book libraries only. Allow scripted (JS) content inside served epubs. Server default: `false`. |
| `hide_single_book_series` | boolean | no |  | Book libraries only. Hide series that contain only one book from series views. Server default: `false`. |
| `only_show_later_books_in_continue_series` | boolean | no |  | Book libraries only. "Continue series" shelves skip books at or before the highest sequence number already read. Server default: `false`. |
| `metadata_precedence` | array of string | no |  | Book libraries only. Ordered precedence of metadata sources used when scanning/matching (e.g. `folderStructure`, `audioMetatags`, `nfoFile`, `txtFiles`, `opfFile`, `absMetadata`). Sent value replaces the array wholesale — the server does not merge it element-wise. |
| `podcast_search_region` | string | no |  | Podcast libraries only. Region code used when searching for podcasts to add. Server default: `"us"`. |
| `mark_as_finished_percent_complete` | number | no |  | Shared. Percent (0-100) of playback progress at which an item is auto-marked finished; when set, takes precedence over `mark_as_finished_time_remaining`. Server default: unset (disabled). |
| `mark_as_finished_time_remaining` | number | no |  | Shared. Seconds of playback remaining at which an item is auto-marked finished. Ignored when `mark_as_finished_percent_complete` is set. Server default: `10`. |

### User Type

Allowed values: `admin` / `user` / `guest`.

### User Permissions

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `download` | boolean | no |  | Defaults true for all account types. |
| `update` | boolean | no |  | Defaults true for root/admin, false for user/guest. |
| `delete` | boolean | no |  | Defaults true for root only. |
| `upload` | boolean | no |  | Defaults true for root/admin. |
| `create_ereader` | boolean | no |  | Defaults true for root/admin. |
| `access_all_libraries` | boolean | no |  | Defaults true. When true, `libraries_accessible` is ignored/cleared server-side. |
| `access_all_tags` | boolean | no |  | Defaults true. When true, `item_tags_selected` is unused. |
| `access_explicit_content` | boolean | no |  | Defaults true for root/admin. |
| `selected_tags_not_accessible` | boolean | no |  | Inverts `item_tags_selected` into a denylist instead of an allowlist when true. Defaults false. |
| `libraries_accessible` | array of string | no |  | Library ids the user may access when `access_all_libraries` is false. References a [`library`](#library) by name (`${ref.library.<key>}`). |
| `item_tags_selected` | array of string | no |  | Tag names selected as allow- (or deny-, if `selected_tags_not_accessible`) list when `access_all_tags` is false. |

### Ereader Availability

Allowed values: `adminOrUp` / `userOrUp` / `guestOrUp` / `specificUsers`.

