# Cleanuparr v1 Configuration

Cleanuparr v1 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### General

`/api/configuration/general` — global application behavior, connectivity
checks, logging, and authentication settings.

The GET response additionally carries a server-assigned `id` and other
read-only fields not accepted by the PUT contract; those are left
unmanaged (`merge(live, desired)` keeps them from the live value).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `display_support_banner` | boolean | no | `true` | Shows the "support this project" banner in the Cleanuparr UI. |
| `dry_run` | boolean | no | `false` | When enabled, Cleanuparr evaluates cleanup rules and logs what it would do without making any changes to download clients or *arr apps. |
| `http_max_retries` | integer | no | `0` | Number of times a failed HTTP request to an external service is retried before giving up. |
| `http_timeout` | integer | no | `100` | Timeout, in seconds, for outbound HTTP requests to external services. |
| `http_certificate_validation` | [`certificate_validation_type`](#certificate-validation-type) | no |  | TLS/SSL certificate validation mode for outbound HTTP calls. |
| `status_check_enabled` | boolean | no | `true` | Periodically checks connectivity to configured *arr apps and download clients and surfaces their status in the UI. |
| `encryption_key` | secret string | no |  | Symmetric key used to encrypt stored credentials at rest. Credential — redacted in plan output. |
| `ignored_downloads` | array of string | no |  | Download names or hashes that cleanup rules will never act on. |
| `connectivity_check_enabled` | boolean | no | `false` | Periodically verifies that Cleanuparr can reach the public internet (used to distinguish a real outage from a misconfiguration). |
| `connectivity_check_urls` | array of string | no |  | URLs polled to determine internet connectivity when `connectivity_check_enabled` is set. |
| `strike_inactivity_window_hours` | integer | no | `24` | Number of hours of inactivity on a stalled/slow download before a strike is issued against it. |
| `history_retention_days` | integer | no | `365` | Number of days of cleanup history retained before older entries are purged. |
| `log` | [`logging_config`](#logging-config) | no |  | Structured logging and rolling log-file settings. |
| `auth` | [`auth_config`](#auth-config) | no |  | Authentication bypass and reverse-proxy trust settings. |

### Oidc

`/api/account/oidc` — OpenID Connect single sign-on configuration.

The GET response (`OidcConfig`) additionally carries a read-only
`authorizedSubject` field (the subject claim of the last account that
completed the OIDC flow); the PUT contract does not accept it, so it is
not modelled here and is left unmanaged.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no | `false` | Enables OIDC single sign-on for the Cleanuparr UI. |
| `issuer_url` | string | no | `` | Base URL of the OIDC identity provider (issuer). |
| `client_id` | string | no | `` | OAuth2 client id registered with the identity provider. |
| `client_secret` | secret string | no |  | OAuth2 client secret registered with the identity provider. Credential — redacted in plan output. |
| `scopes` | string | no | `openid profile email` | Space-separated OAuth2 scopes requested during authentication. |
| `provider_name` | string | no | `OIDC` | Display name for the OIDC provider shown on the Cleanuparr login page. |
| `redirect_url` | string | no | `` | URL the identity provider redirects back to after authentication. |
| `exclusive_mode` | boolean | no | `false` | When enabled, OIDC is the only allowed sign-in method and Cleanuparr's built-in username/password login is disabled. |

### Queue Cleaner

`/api/configuration/queue_cleaner` — periodic cleanup of stalled, slow, and
failed-import download-queue items.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables the queue cleaner job. |
| `cron_expression` | string | no | `0 0/5 * * * ?` | Cron expression controlling how often the queue cleaner job runs. |
| `use_advanced_scheduling` | boolean | no |  | Uses `cron_expression` instead of the built-in interval scheduling. |
| `failed_import` | [`failed_import_config`](#failed-import-config) | no |  | Failed-import striking settings. |
| `downloading_metadata_max_strikes` | integer | no |  | Number of strikes for a download stuck downloading metadata before it is removed. |
| `process_no_content_id` | boolean | no |  | Strikes/removes downloads that have no content id (e.g. private-tracker downloads without size/hash info) instead of leaving them queued. |
| `ignored_downloads` | array of string | no |  | Download names or category names excluded from queue cleaning. |

### Malware Blocker

`/api/configuration/malware_blocker` — blocks or strikes downloads whose
content matches a per-*arr malware blocklist (persisted upstream as
`ContentBlockerConfig`).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables the malware blocker job. |
| `cron_expression` | string | no | `0/5 * * * * ?` | Cron expression controlling how often the malware blocker job runs. |
| `use_advanced_scheduling` | boolean | no |  | Uses `cron_expression` instead of the built-in interval scheduling. |
| `ignore_private` | boolean | no |  | Ignores private-tracker downloads when blocking malware. |
| `delete_private` | boolean | no |  | Deletes private-tracker downloads instead of striking them. |
| `process_no_content_id` | boolean | no |  | Strikes/removes downloads that have no content id instead of leaving them queued. |
| `delete_if_any_file_blocked` | boolean | no |  | Deletes the whole download if any of its files are blocked, instead of only removing the blocked files. |
| `sonarr` | [`blocklist_settings`](#blocklist-settings) | no |  | Malware blocklist settings applied to the Sonarr instance(s). |
| `radarr` | [`blocklist_settings`](#blocklist-settings) | no |  | Malware blocklist settings applied to the Radarr instance(s). |
| `lidarr` | [`blocklist_settings`](#blocklist-settings) | no |  | Malware blocklist settings applied to the Lidarr instance(s). |
| `readarr` | [`blocklist_settings`](#blocklist-settings) | no |  | Malware blocklist settings applied to the Readarr instance(s). |
| `whisparr` | [`blocklist_settings`](#blocklist-settings) | no |  | Malware blocklist settings applied to the Whisparr instance(s). |
| `ignored_downloads` | array of string | no |  | Download names or category names excluded from malware blocking. |

### Blacklist Sync

`/api/configuration/blacklist_sync` — periodically publishes a blacklist
file that seeds *arr download-client blocklists.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables the blacklist sync job. |
| `blacklist_path` | string | no |  | http(s) URL or an existing local file path. Required when `enabled`. |

### Download Cleaner

`/api/configuration/download_cleaner` — periodic cleanup of downloads that
have finished seeding or been orphaned/unlinked from their *arr instance.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables the download cleaner job. |
| `cron_expression` | string | no | `0 0 * * * ?` | Cron expression controlling how often the download cleaner job runs. |
| `use_advanced_scheduling` | boolean | no |  | Uses `cron_expression` instead of the built-in interval scheduling. |
| `ignored_downloads` | array of string | no |  | Download names or category names excluded from download cleaning. |

### Sonarr Config

`/api/configuration/sonarr` — Sonarr-wide behaviour shared by every
declared instance.

The `GET` response additionally carries a server-assigned `id`, `type`, and
the live `instances` array; those are left unmanaged
(`merge(live, desired)` keeps them from the live value). Manage the
instances themselves via [`SonarrInstance`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `failed_import_max_strikes` | integer | no | `-1` | Number of failed-import strikes a download may accrue against this app before it is treated as a failed import. `-1` disables the check. |

### Radarr Config

`/api/configuration/radarr` — Radarr-wide behaviour shared by every
declared instance.

The `GET` response additionally carries a server-assigned `id`, `type`, and
the live `instances` array; those are left unmanaged
(`merge(live, desired)` keeps them from the live value). Manage the
instances themselves via [`RadarrInstance`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `failed_import_max_strikes` | integer | no | `-1` | Number of failed-import strikes a download may accrue against this app before it is treated as a failed import. `-1` disables the check. |

### Lidarr Config

`/api/configuration/lidarr` — Lidarr-wide behaviour shared by every
declared instance.

The `GET` response additionally carries a server-assigned `id`, `type`, and
the live `instances` array; those are left unmanaged
(`merge(live, desired)` keeps them from the live value). Manage the
instances themselves via [`LidarrInstance`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `failed_import_max_strikes` | integer | no | `-1` | Number of failed-import strikes a download may accrue against this app before it is treated as a failed import. `-1` disables the check. |

### Readarr Config

`/api/configuration/readarr` — Readarr-wide behaviour shared by every
declared instance.

The `GET` response additionally carries a server-assigned `id`, `type`, and
the live `instances` array; those are left unmanaged
(`merge(live, desired)` keeps them from the live value). Manage the
instances themselves via [`ReadarrInstance`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `failed_import_max_strikes` | integer | no | `-1` | Number of failed-import strikes a download may accrue against this app before it is treated as a failed import. `-1` disables the check. |

### Whisparr Config

`/api/configuration/whisparr` — Whisparr-wide behaviour shared by every
declared instance.

The `GET` response additionally carries a server-assigned `id`, `type`, and
the live `instances` array; those are left unmanaged
(`merge(live, desired)` keeps them from the live value). Manage the
instances themselves via [`WhisparrInstance`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `failed_import_max_strikes` | integer | no | `-1` | Number of failed-import strikes a download may accrue against this app before it is treated as a failed import. `-1` disables the check. |

### Stall Rule

`/api/queue-rules/stall` — a rule that strikes, and eventually removes,
torrents whose download has made no progress for too long.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Rule name. Must be unique among stall rules. |
| `enabled` | boolean | no | `true` | Whether this rule is active. |
| `max_strikes` | integer | no | `3` | Number of times a torrent may be struck for stalling before it is removed. |
| `privacy_type` | [`torrent_privacy_type`](#torrent-privacy-type) | no |  | Restrict this rule to torrents of a given privacy classification. Omitted, the server treats it as `Public`. |
| `min_completion_percentage` | integer | no | `0` | Minimum download completion percentage a torrent must reach before this rule starts evaluating it. |
| `max_completion_percentage` | integer | no | `100` | Maximum download completion percentage this rule applies to; a torrent past this point is left alone. The API has no server default for this field — omitting it binds `0`, which then fails the server's own `[Range(1,100)]` validation, so it is modelled non-optional with a working default rather than `Option`. |
| `delete_private_torrents_from_client` | boolean | no | `false` | Delete a private torrent from the download client (not just the queue) once this rule strikes it out. |
| `change_category` | boolean | no | `false` | Move the torrent to a different category once this rule strikes it out. |
| `reset_strikes_on_progress` | boolean | no | `true` | Reset a torrent's strike count whenever it makes fresh download progress. |
| `minimum_progress` | string | no |  | Minimum progress delta (e.g. `"10MB"`) that counts as "made progress" for `reset_strikes_on_progress` purposes. `None` uses the server's own threshold. |

### Slow Rule

`/api/queue-rules/slow` — a rule that strikes, and eventually removes,
torrents whose download speed drops too low for too long.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Rule name. Must be unique among slow rules. |
| `enabled` | boolean | no | `true` | Whether this rule is active. |
| `max_strikes` | integer | no | `3` | Number of times a torrent may be struck for being slow before it is removed. |
| `privacy_type` | [`torrent_privacy_type`](#torrent-privacy-type) | no |  | Restrict this rule to torrents of a given privacy classification. Omitted, the server treats it as `Public`. |
| `min_completion_percentage` | integer | no | `0` | Minimum download completion percentage a torrent must reach before this rule starts evaluating it. |
| `max_completion_percentage` | integer | no | `100` | Maximum download completion percentage this rule applies to; a torrent past this point is left alone. The API has no server default for this field — omitting it binds `0`, which then fails the server's own `[Range(1,100)]` validation, so it is modelled non-optional with a working default rather than `Option`. |
| `delete_private_torrents_from_client` | boolean | no | `false` | Delete a private torrent from the download client (not just the queue) once this rule strikes it out. |
| `change_category` | boolean | no | `false` | Move the torrent to a different category once this rule strikes it out. |
| `reset_strikes_on_progress` | boolean | no | `true` | Reset a torrent's strike count whenever its speed recovers. |
| `min_speed` | string | no | `` | Minimum download speed (e.g. `"10MB"`) below which a torrent is considered slow. |
| `max_time_hours` | number | no | `0` | Number of hours a torrent may run below `min_speed` before it is struck. |
| `ignore_above_size` | string | no |  | Torrents above this size (e.g. `"5GB"`) are exempt from this rule. `None` applies the rule regardless of torrent size. |
| `ignore_while_alt_speed_active` | boolean | no | `true` | Skip slow-speed evaluation while the download client's alternative speed limits are active. |

### Sonarr Instance

`/api/configuration/sonarr/instances` — one connected Sonarr instance.

`sync = custom`: the list endpoint (`GET /api/configuration/sonarr`)
returns an envelope object (`{ id, type, instances }`), not a bare array,
and `apiKey` reads back masked — see [`crate::resources::arr::reconcile`]
and [`crate::diff::subset`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.sonarr_instance.<name>}`). |
| `enabled` | boolean | no | `true` | Whether Cleanuparr manages downloads against this instance. |
| `url` | string | yes |  | Base URL of the Sonarr instance. Must parse as an absolute URI. |
| `api_key` | secret string | yes |  | Sonarr API key. On create, the masked placeholder is rejected; on update, sending it back keeps the stored key. Credential — redacted in plan output. |
| `version` | number | yes |  | Sonarr's reported API/schema version, used to pick the right request shape for this instance. |
| `external_url` | string | no |  | Externally reachable URL for this instance (e.g. behind a reverse proxy), used when Cleanuparr needs to hand the user a clickable link. |

### Radarr Instance

`/api/configuration/radarr/instances` — one connected Radarr instance.

`sync = custom`: the list endpoint (`GET /api/configuration/radarr`)
returns an envelope object (`{ id, type, instances }`), not a bare array,
and `apiKey` reads back masked — see [`crate::resources::arr::reconcile`]
and [`crate::diff::subset`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.radarr_instance.<name>}`). |
| `enabled` | boolean | no | `true` | Whether Cleanuparr manages downloads against this instance. |
| `url` | string | yes |  | Base URL of the Radarr instance. Must parse as an absolute URI. |
| `api_key` | secret string | yes |  | Radarr API key. On create, the masked placeholder is rejected; on update, sending it back keeps the stored key. Credential — redacted in plan output. |
| `version` | number | yes |  | Radarr's reported API/schema version, used to pick the right request shape for this instance. |
| `external_url` | string | no |  | Externally reachable URL for this instance (e.g. behind a reverse proxy), used when Cleanuparr needs to hand the user a clickable link. |

### Lidarr Instance

`/api/configuration/lidarr/instances` — one connected Lidarr instance.

`sync = custom`: the list endpoint (`GET /api/configuration/lidarr`)
returns an envelope object (`{ id, type, instances }`), not a bare array,
and `apiKey` reads back masked — see [`crate::resources::arr::reconcile`]
and [`crate::diff::subset`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.lidarr_instance.<name>}`). |
| `enabled` | boolean | no | `true` | Whether Cleanuparr manages downloads against this instance. |
| `url` | string | yes |  | Base URL of the Lidarr instance. Must parse as an absolute URI. |
| `api_key` | secret string | yes |  | Lidarr API key. On create, the masked placeholder is rejected; on update, sending it back keeps the stored key. Credential — redacted in plan output. |
| `version` | number | yes |  | Lidarr's reported API/schema version, used to pick the right request shape for this instance. |
| `external_url` | string | no |  | Externally reachable URL for this instance (e.g. behind a reverse proxy), used when Cleanuparr needs to hand the user a clickable link. |

### Readarr Instance

`/api/configuration/readarr/instances` — one connected Readarr instance.

`sync = custom`: the list endpoint (`GET /api/configuration/readarr`)
returns an envelope object (`{ id, type, instances }`), not a bare array,
and `apiKey` reads back masked — see [`crate::resources::arr::reconcile`]
and [`crate::diff::subset`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.readarr_instance.<name>}`). |
| `enabled` | boolean | no | `true` | Whether Cleanuparr manages downloads against this instance. |
| `url` | string | yes |  | Base URL of the Readarr instance. Must parse as an absolute URI. |
| `api_key` | secret string | yes |  | Readarr API key. On create, the masked placeholder is rejected; on update, sending it back keeps the stored key. Credential — redacted in plan output. |
| `version` | number | yes |  | Readarr's reported API/schema version, used to pick the right request shape for this instance. |
| `external_url` | string | no |  | Externally reachable URL for this instance (e.g. behind a reverse proxy), used when Cleanuparr needs to hand the user a clickable link. |

### Whisparr Instance

`/api/configuration/whisparr/instances` — one connected Whisparr instance.

`sync = custom`: the list endpoint (`GET /api/configuration/whisparr`)
returns an envelope object (`{ id, type, instances }`), not a bare array,
and `apiKey` reads back masked — see [`crate::resources::arr::reconcile`]
and [`crate::diff::subset`].

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.whisparr_instance.<name>}`). |
| `enabled` | boolean | no | `true` | Whether Cleanuparr manages downloads against this instance. |
| `url` | string | yes |  | Base URL of the Whisparr instance. Must parse as an absolute URI. |
| `api_key` | secret string | yes |  | Whisparr API key. On create, the masked placeholder is rejected; on update, sending it back keeps the stored key. Credential — redacted in plan output. |
| `version` | number | yes |  | Whisparr's reported API/schema version, used to pick the right request shape for this instance. |
| `external_url` | string | no |  | Externally reachable URL for this instance (e.g. behind a reverse proxy), used when Cleanuparr needs to hand the user a clickable link. |

### Notifiarr Provider

`/api/configuration/notification_providers/notifiarr` — a Notifiarr
notification provider. See `crate::resources::notifications` for the
shared read/write shape and the global-uniqueness note on `name`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name. Required and unique across *all* notification provider types, not just Notifiarr ones. |
| `is_enabled` | boolean | no | `true` | Whether this provider is active. |
| `on_failed_import_strike` | boolean | no |  | Notify when a download is struck for a failed import. |
| `on_stalled_strike` | boolean | no |  | Notify when a download is struck for stalling. |
| `on_slow_strike` | boolean | no |  | Notify when a download is struck for being too slow. |
| `on_queue_item_deleted` | boolean | no |  | Notify when a queue item is deleted. |
| `on_download_cleaned` | boolean | no |  | Notify when a download is cleaned (seeding finished / orphaned). |
| `on_category_changed` | boolean | no |  | Notify when a download's category changes. |
| `on_search_triggered` | boolean | no |  | Notify when a search is triggered. |
| `on_search_item_grabbed` | boolean | no |  | Notify when a search grabs an item. |
| `api_key` | secret string | no |  | Notifiarr API key. At least 10 characters. Masked on read; resending the placeholder keeps the stored value. Credential — redacted in plan output. |
| `channel_id` | string | no |  | Numeric Discord channel id Notifiarr should post to. |

### Apprise Provider

`/api/configuration/notification_providers/apprise` — an Apprise
notification provider. See `crate::resources::notifications` for the
shared read/write shape and the global-uniqueness note on `name`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name. Required and unique across *all* notification provider types, not just Apprise ones. |
| `is_enabled` | boolean | no | `true` | Whether this provider is active. |
| `on_failed_import_strike` | boolean | no |  | Notify when a download is struck for a failed import. |
| `on_stalled_strike` | boolean | no |  | Notify when a download is struck for stalling. |
| `on_slow_strike` | boolean | no |  | Notify when a download is struck for being too slow. |
| `on_queue_item_deleted` | boolean | no |  | Notify when a queue item is deleted. |
| `on_download_cleaned` | boolean | no |  | Notify when a download is cleaned (seeding finished / orphaned). |
| `on_category_changed` | boolean | no |  | Notify when a download's category changes. |
| `on_search_triggered` | boolean | no |  | Notify when a search is triggered. |
| `on_search_item_grabbed` | boolean | no |  | Notify when a search grabs an item. |
| `mode` | [`apprise_mode`](#apprise-mode) | no |  | Whether to talk to an `apprise-api` container (`Api`) or shell out to the `apprise` CLI (`Cli`). |
| `url` | string | no |  | API mode: base URL of the `apprise-api` container. |
| `key` | secret string | no |  | API mode: configuration key, at least 2 characters. Masked on read. Credential — redacted in plan output. |
| `tags` | string | no |  | Comma-separated Apprise tag expression restricting which configured URLs a notification is sent to. |
| `service_urls` | secret string | no |  | CLI mode: one Apprise service URL per line. Masked on read down to the scheme (e.g. `discord://••••••••`). Credential — redacted in plan output. |

### Ntfy Provider

`/api/configuration/notification_providers/ntfy` — an ntfy.sh
notification provider. See `crate::resources::notifications` for the
shared read/write shape and the global-uniqueness note on `name`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name. Required and unique across *all* notification provider types, not just ntfy ones. |
| `is_enabled` | boolean | no | `true` | Whether this provider is active. |
| `on_failed_import_strike` | boolean | no |  | Notify when a download is struck for a failed import. |
| `on_stalled_strike` | boolean | no |  | Notify when a download is struck for stalling. |
| `on_slow_strike` | boolean | no |  | Notify when a download is struck for being too slow. |
| `on_queue_item_deleted` | boolean | no |  | Notify when a queue item is deleted. |
| `on_download_cleaned` | boolean | no |  | Notify when a download is cleaned (seeding finished / orphaned). |
| `on_category_changed` | boolean | no |  | Notify when a download's category changes. |
| `on_search_triggered` | boolean | no |  | Notify when a search is triggered. |
| `on_search_item_grabbed` | boolean | no |  | Notify when a search grabs an item. |
| `server_url` | string | no |  | Base URL of the ntfy server (self-hosted or `https://ntfy.sh`). |
| `topics` | array of string | no |  | Topics to publish to. At least one non-blank topic is required. |
| `authentication_type` | [`ntfy_authentication_type`](#ntfy-authentication-type) | no |  | Authentication method against the ntfy server. |
| `username` | string | no |  | Username, required for `BasicAuth`. |
| `password` | secret string | no |  | Password, required for `BasicAuth`. Masked on read. Credential — redacted in plan output. |
| `access_token` | secret string | no |  | Access token, required for `AccessToken`. Masked on read. Credential — redacted in plan output. |
| `priority` | [`ntfy_priority`](#ntfy-priority) | no |  | Message priority. |
| `tags` | array of string | no |  | ntfy tags/emoji shown alongside the notification. |

### Pushover Provider

`/api/configuration/notification_providers/pushover` — a Pushover
notification provider. See `crate::resources::notifications` for the
shared read/write shape and the global-uniqueness note on `name`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name. Required and unique across *all* notification provider types, not just Pushover ones. |
| `is_enabled` | boolean | no | `true` | Whether this provider is active. |
| `on_failed_import_strike` | boolean | no |  | Notify when a download is struck for a failed import. |
| `on_stalled_strike` | boolean | no |  | Notify when a download is struck for stalling. |
| `on_slow_strike` | boolean | no |  | Notify when a download is struck for being too slow. |
| `on_queue_item_deleted` | boolean | no |  | Notify when a queue item is deleted. |
| `on_download_cleaned` | boolean | no |  | Notify when a download is cleaned (seeding finished / orphaned). |
| `on_category_changed` | boolean | no |  | Notify when a download's category changes. |
| `on_search_triggered` | boolean | no |  | Notify when a search is triggered. |
| `on_search_item_grabbed` | boolean | no |  | Notify when a search grabs an item. |
| `api_token` | secret string | no |  | Pushover application API token. Masked on read. Credential — redacted in plan output. |
| `user_key` | secret string | no |  | Pushover user or group key. Masked on read. Credential — redacted in plan output. |
| `devices` | array of string | no |  | Device names to target (letters, digits, underscore and hyphen only). Empty targets every device on the account. |
| `priority` | [`pushover_priority`](#pushover-priority) | no |  | Message priority. |
| `sound` | string | no |  | Built-in Pushover sound name (`pushover`, `bike`, `siren`, `none`, …) or a custom one. |
| `retry` | integer | no |  | Seconds between repeat notifications. Required for `Emergency` priority; at least 30 seconds. |
| `expire` | integer | no |  | Seconds before Pushover stops repeating an `Emergency` notification. Required for `Emergency` priority; at most 10800 seconds. |
| `tags` | array of string | no |  | Pushover tags attached to the notification. |

### Telegram Provider

`/api/configuration/notification_providers/telegram` — a Telegram
notification provider. See `crate::resources::notifications` for the
shared read/write shape and the global-uniqueness note on `name`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name. Required and unique across *all* notification provider types, not just Telegram ones. |
| `is_enabled` | boolean | no | `true` | Whether this provider is active. |
| `on_failed_import_strike` | boolean | no |  | Notify when a download is struck for a failed import. |
| `on_stalled_strike` | boolean | no |  | Notify when a download is struck for stalling. |
| `on_slow_strike` | boolean | no |  | Notify when a download is struck for being too slow. |
| `on_queue_item_deleted` | boolean | no |  | Notify when a queue item is deleted. |
| `on_download_cleaned` | boolean | no |  | Notify when a download is cleaned (seeding finished / orphaned). |
| `on_category_changed` | boolean | no |  | Notify when a download's category changes. |
| `on_search_triggered` | boolean | no |  | Notify when a search is triggered. |
| `on_search_item_grabbed` | boolean | no |  | Notify when a search grabs an item. |
| `bot_token` | secret string | no |  | Telegram bot token, at least 10 characters. Masked on read. Credential — redacted in plan output. |
| `chat_id` | string | no |  | Target chat id (integer as a string; negative for groups). |
| `topic_id` | string | no |  | Forum topic id (integer as a string), for chats with topics enabled. |
| `send_silently` | boolean | no |  | Send the message without a notification sound. |

### Discord Provider

`/api/configuration/notification_providers/discord` — a Discord webhook
notification provider. See `crate::resources::notifications` for the
shared read/write shape and the global-uniqueness note on `name`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name. Required and unique across *all* notification provider types, not just Discord ones. |
| `is_enabled` | boolean | no | `true` | Whether this provider is active. |
| `on_failed_import_strike` | boolean | no |  | Notify when a download is struck for a failed import. |
| `on_stalled_strike` | boolean | no |  | Notify when a download is struck for stalling. |
| `on_slow_strike` | boolean | no |  | Notify when a download is struck for being too slow. |
| `on_queue_item_deleted` | boolean | no |  | Notify when a queue item is deleted. |
| `on_download_cleaned` | boolean | no |  | Notify when a download is cleaned (seeding finished / orphaned). |
| `on_category_changed` | boolean | no |  | Notify when a download's category changes. |
| `on_search_triggered` | boolean | no |  | Notify when a search is triggered. |
| `on_search_item_grabbed` | boolean | no |  | Notify when a search grabs an item. |
| `webhook_url` | secret string | no |  | Discord webhook URL. Must start with `https://discord.com/api/webhooks/` or `https://discordapp.com/api/webhooks/`. Masked on read. Credential — redacted in plan output. |
| `username` | string | no |  | Override the webhook's posted username. |
| `avatar_url` | string | no |  | Override the webhook's posted avatar image URL. |

### Gotify Provider

`/api/configuration/notification_providers/gotify` — a Gotify
notification provider. See `crate::resources::notifications` for the
shared read/write shape and the global-uniqueness note on `name`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name. Required and unique across *all* notification provider types, not just Gotify ones. |
| `is_enabled` | boolean | no | `true` | Whether this provider is active. |
| `on_failed_import_strike` | boolean | no |  | Notify when a download is struck for a failed import. |
| `on_stalled_strike` | boolean | no |  | Notify when a download is struck for stalling. |
| `on_slow_strike` | boolean | no |  | Notify when a download is struck for being too slow. |
| `on_queue_item_deleted` | boolean | no |  | Notify when a queue item is deleted. |
| `on_download_cleaned` | boolean | no |  | Notify when a download is cleaned (seeding finished / orphaned). |
| `on_category_changed` | boolean | no |  | Notify when a download's category changes. |
| `on_search_triggered` | boolean | no |  | Notify when a search is triggered. |
| `on_search_item_grabbed` | boolean | no |  | Notify when a search grabs an item. |
| `server_url` | string | no |  | Base URL of the Gotify server. |
| `application_token` | secret string | no |  | Gotify application token. Masked on read. Credential — redacted in plan output. |
| `priority` | integer | no | `5` | Message priority, `0`-`10`. |

### Download Client

`/api/configuration/download_client` — a configured download client.

The four nested fields are **not** part of this resource's own wire body
(`CreateDownloadClientRequest`/`UpdateDownloadClientRequest` are both
`additionalProperties: false`) — each is `#[wire(config_only)]` so the
standard codec's encode skips it, while config decode (which doesn't
check `read_only`) still reads it from the user's YAML. See the `impl
CustomSync` for how they're reconciled against their own endpoints.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.download_client.<name>}`). |
| `enabled` | boolean | no | `false` | Whether Cleanuparr manages downloads against this client. |
| `type_name` | [`download_client_type_name`](#download-client-type-name) | no |  | Client implementation (`qBittorrent`, `Deluge`, `Transmission`, `uTorrent`, or `rTorrent`). |
| `protocol` | [`download_client_type`](#download-client-type) | no |  | Protocol family this client handles. |
| `host` | string | no |  | Client host/address. |
| `username` | string | no |  | Client username, where required. |
| `password` | secret string | no |  | Client password, where required. Masked on read; the masked placeholder is rejected on create and, sent back on update, keeps the stored password. Credential — redacted in plan output. |
| `url_base` | string | no |  | Path prefix for clients such as Transmission and Deluge. |
| `external_url` | string | no |  | Externally reachable URL for this client (e.g. behind a reverse proxy), used when Cleanuparr needs to hand the user a clickable link. |
| `download_directory_source` | string | no |  | Path prefix as the client itself reports it. Must be set together with `download_directory_target`. |
| `download_directory_target` | string | no |  | Local mount path substituted for `download_directory_source` when Cleanuparr resolves files on disk. |
| `seeding_rules` | array of [`seeding_rule`](#seeding-rule) | no |  | Seeding rules scoped to this client. Reconciled against `/api/seeding-rules` once this client's GUID is known — see [`seeding_rule`]. |
| `unlinked_config` | [`unlinked_config`](#unlinked-config) | no |  | This client's unlinked-download handling. `None` leaves it unmanaged. Reconciled against `/api/unlinked-config/{id}` — see [`unlinked`]. |
| `dead_torrent_config` | [`dead_torrent_config`](#dead-torrent-config) | no |  | This client's dead-torrent handling. `None` leaves it unmanaged. Reconciled against `/api/dead-torrent-config/{id}` — see [`dead_torrent`]. |
| `orphaned_files_config` | [`orphaned_files_config`](#orphaned-files-config) | no |  | This client's orphaned-file scanning. `None` leaves it unmanaged. Reconciled against `/api/orphaned-files-config/{id}` — see [`orphaned_files`]. |

### Seeker

`/api/configuration/seeker` — proactive search behaviour, global and
per-*arr-instance.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `search_enabled` | boolean | no | `true` | Enables the seeker job entirely (both scheduled and proactive search). |
| `search_interval` | integer | no | `3` | Minutes between seeker runs. Must be one of 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60, 120, 180, 240, 360. |
| `proactive_search_enabled` | boolean | no | `false` | Enables proactively searching for missing/below-cutoff items, rather than only reacting to *arr events. |
| `selection_strategy` | [`selection_strategy`](#selection-strategy) | no |  | How the next item to proactively search is chosen among eligible candidates. |
| `use_round_robin` | boolean | no | `true` | Round-robins proactive search across instances instead of draining one instance's queue before moving to the next. |
| `post_release_grace_hours` | integer | no | `6` | Hours after an item's release before proactive search will consider it, giving indexers time to pick it up. |
| `instances` | array of [`seeker_instance`](#seeker-instance) | no |  | Per-*arr-instance proactive-search settings. Upserted by `arr_instance_id`; instances omitted here keep their stored settings. |

## Types

### Certificate Validation Type

Allowed values: `Enabled` / `DisabledForLocalAddresses` / `Disabled`.

### Logging Config

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `level` | [`log_event_level`](#log-event-level) | no |  | Minimum Serilog level that will be written to the log. |
| `rolling_size_mb` | integer | no | `10` | Maximum size, in megabytes, a single log file may reach before it rolls over into a new file. `0` disables rolling-file logging. |
| `retained_file_count` | integer | no | `5` | Number of rolled-over log files retained on disk before the oldest is deleted. `0` retains files indefinitely. |
| `time_limit_hours` | integer | no | `24` | Maximum age, in hours, a log file is kept before it is eligible for deletion. `0` retains files indefinitely. |
| `archive_enabled` | boolean | no | `true` | Whether old log files are moved into a compressed archive instead of being deleted outright. |
| `archive_retained_count` | integer | no | `60` | Number of archived log files retained on disk. `0` retains files indefinitely. Cannot be `0` at the same time as `archive_time_limit_hours` when archiving is enabled. |
| `archive_time_limit_hours` | integer | no | `720` | Maximum age, in hours, an archived log file is kept before deletion. `0` retains files indefinitely. Cannot be `0` at the same time as `archive_retained_count` when archiving is enabled. |

### Auth Config

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `disable_auth_for_local_addresses` | boolean | no |  | Authenticates callers from local or trusted networks as the admin user without requiring credentials. |
| `trust_forwarded_headers` | boolean | no |  | Honours `X-Forwarded-For` / `-Proto` / `-Host` from trusted hops when resolving the client address. Only enable this when Cleanuparr sits behind a trusted reverse proxy. |
| `trusted_networks` | array of string | no |  | Plain IPs or CIDR ranges treated as trusted networks. Loopback and private ranges are always trusted regardless of this list. |

### Failed Import Config

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `max_strikes` | integer | yes |  | Number of failed-import strikes before the download is removed. `0` disables striking; any other value must be at least 3. |
| `ignore_private` | boolean | no |  | Ignores private-tracker downloads when striking for failed imports. |
| `delete_private` | boolean | no |  | Deletes private-tracker downloads instead of striking them. Mutually exclusive with `change_category`. |
| `skip_if_not_found_in_client` | boolean | no | `true` | Skips striking a download that is no longer present in the download client's queue. |
| `patterns` | array of string | no |  | File-name patterns used to identify failed imports. At least one pattern is required when striking is on and `pattern_mode` is `Include`. |
| `pattern_mode` | [`pattern_mode`](#pattern-mode) | no |  | Whether `patterns` excludes or requires matches. |
| `change_category` | boolean | no |  | Moves the download to a different category instead of striking it. Mutually exclusive with `delete_private`. |

### Blocklist Settings

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables malware blocking for this *arr instance. |
| `blocklist_type` | [`blocklist_type`](#blocklist-type) | no |  | Whether `blocklist_path` is a blacklist or a whitelist. |
| `blocklist_path` | string | no |  | http(s) URL or an existing local file path. |

### Torrent Privacy Type

Allowed values: `Public` / `Private` / `Both`.

### Apprise Mode

Allowed values: `Api` / `Cli`.

### Ntfy Authentication Type

Allowed values: `None` / `BasicAuth` / `AccessToken`.

### Ntfy Priority

Allowed values: `Min` / `Low` / `Default` / `High` / `Max`.

### Pushover Priority

Allowed values: `Lowest` / `Low` / `Normal` / `High` / `Emergency`.

### Download Client Type Name

Allowed values: `qBittorrent` / `Deluge` / `Transmission` / `uTorrent` / `rTorrent`.

### Download Client Type

Allowed values: `Torrent` / `Usenet`.

### Seeding Rule

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Rule name — its identity among the client's seeding rules. |
| `categories` | array of string | no |  | Torrent categories this rule applies to. At least one is required. |
| `tracker_patterns` | array of string | no |  | Tracker domain suffixes this rule applies to, case-insensitive. Empty matches any tracker. |
| `tags_any` | array of string | no |  | Torrent must carry at least one of these tags. Accepted for every client but silently ignored for Deluge, rTorrent, and µTorrent. |
| `tags_all` | array of string | no |  | Torrent must carry all of these tags. Accepted for every client but silently ignored for Deluge, rTorrent, and µTorrent. |
| `privacy_type` | [`torrent_privacy_type`](#torrent-privacy-type) | no |  | Restrict this rule to torrents of a given privacy classification. Omitted, the server treats it as `Public`. |
| `max_ratio` | number | no | `-1` | Seed ratio to reach before removal. `-1` disables. Either `max_ratio` or `max_seed_time` must be non-negative. |
| `min_seed_time` | number | no | `0` | Hours to seed before removal once the ratio is met. |
| `max_seed_time` | number | no | `-1` | Hours to seed before removal regardless of ratio. `-1` disables. |
| `min_seeders` | integer | no | `0` | Minimum seeders required before this rule evaluates the torrent. Ignored for rTorrent, which reports no seeder count. |
| `max_inactive_days` | number | no |  | qBittorrent only: maximum inactive days before removal. `None` (server) / `-1` disables. |
| `delete_source_files` | boolean | no | `true` | Delete the torrent's source files from disk (not just remove it from the client's queue) once this rule removes it. |

### Unlinked Config

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables unlinked-download handling for this client. |
| `target_category` | string | no | `cleanuparr-unlinked` | Category (or, with `use_tag`, tag) applied to unlinked downloads. Must not also appear in `categories`. |
| `use_tag` | boolean | no |  | Add a tag instead of changing the category (qBittorrent and Transmission only). |
| `ignored_root_dirs` | array of string | no |  | Root directories excluded from the unlinked-file scan. Each entry must exist on disk. |
| `categories` | array of string | no |  | Categories this rule watches. At least one is required when `enabled`. |

### Dead Torrent Config

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables dead-torrent handling for this client. Cannot be enabled for rTorrent. |
| `target_category` | string | no | `cleanuparr-dead` | Category (or, with `use_tag`, tag) applied to dead torrents. Must not also appear in `categories`. |
| `use_tag` | boolean | no |  | Add a tag instead of changing the category. |
| `max_strikes` | integer | no |  | Consecutive runs reporting zero seeders before a torrent is moved. Minimum `3`. |
| `categories` | array of string | no |  | Categories this rule watches. At least one is required when `enabled`. |

### Orphaned Files Config

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Enables orphaned-file scanning for this client. |
| `scan_directories` | array of string | no |  | Directories scanned for orphaned entries. At least one is required when `enabled`. |
| `orphaned_directory` | string | yes |  | Where orphaned entries are moved. Must not overlap any scan directory, any other client's scan or orphaned directory, or another client's download directory target. |
| `exclude_patterns` | array of string | no |  | Glob patterns exempt from orphan detection. |
| `min_file_age_hours` | integer | no | `24` | Hours a file must remain untouched before it's eligible for orphan handling. `0` disables the age check. |
| `purge_after_hours` | integer | no |  | Permanently delete moved entries after this many hours. `None` keeps them forever. |

### Selection Strategy

Allowed values: `BalancedWeighted` / `OldestSearchFirst` / `OldestSearchWeighted` / `NewestFirst` / `NewestWeighted` / `Random`.

### Seeker Instance

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `arr_instance_id` | string | no |  | The Sonarr or Radarr instance these settings apply to. References either [`sonarr_instance`](#sonarr-instance) or [`radarr_instance`](#radarr-instance) by name (`${ref.sonarr_instance.<key>}` or `${ref.radarr_instance.<key>}`). |
| `enabled` | boolean | no | `true` | Whether proactive search runs for this instance. |
| `skip_tags` | array of string | no |  | *arr tag ids to exclude from search. |
| `active_download_limit` | integer | no | `3` | Skip a proactive search cycle when this many items are already downloading. `0` disables the limit. |
| `min_cycle_time_days` | integer | no | `7` | Minimum number of days between proactive-search cycles for the same item. |
| `monitored_only` | boolean | no | `true` | Only proactively search monitored items. |
| `use_cutoff` | boolean | no | `false` | Search up to the quality cutoff instead of stopping once an item has any acceptable file. |
| `use_custom_format_score` | boolean | no | `false` | Weigh search candidates by custom format score. |

### Log Event Level

Allowed values: `Verbose` / `Debug` / `Information` / `Warning` / `Error` / `Fatal`.

### Pattern Mode

Allowed values: `Exclude` / `Include`.

### Blocklist Type

Allowed values: `Blacklist` / `Whitelist`.

