# Autobrr v1 Configuration

Autobrr v1 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Api Key

`/api/keys` — an API key issued under a name, with a set of scopes.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Key name — its identity. |
| `scopes` | array of string | no |  | Access scopes granted to the key (e.g. `["read", "write"]`). |

### Notification

`/api/notification` — a notification target.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity. |
| `notification_type` | string | yes |  | Provider kind. One of: `DISCORD`, `NOTIFIARR`, `IFTTT`, `JOIN`, `MATTERMOST`, `MATRIX`, `PUSH_BULLET`, `PUSHOVER`, `ROCKETCHAT`, `SLACK`, `TELEGRAM`, `GOTIFY`, `NTFY`, `LUNASEA`, `SHOUTRRR`, `WEBHOOK`. |
| `enabled` | boolean | yes |  | Whether the target is active. |
| `events` | array of [`notification_event`](#notification-event) | no |  | Events that trigger this notification. |
| `webhook` | string | no |  | Webhook URL (Discord and similar). |
| `api_key` | secret string | no |  | Provider API key / auth token where required. Credential — redacted in plan output. |
| `token` | secret string | no |  | Bot token (Telegram and similar). Credential — redacted in plan output. |
| `channel` | string | no |  | Target channel / chat id. |
| `topic` | string | no |  | Topic (ntfy and similar). |
| `host` | string | no |  | Provider host (self-hosted Gotify/ntfy). |
| `title` | string | no |  | Message title (ntfy and similar). |
| `icon` | string | no |  | Icon / avatar override. |
| `username` | string | no |  | Username for auth-protected providers (ntfy, Matrix). |
| `password` | secret string | no |  | Password for auth-protected providers. Credential — redacted in plan output. |
| `rooms` | string | no |  | Rooms to post to (Matrix). |
| `targets` | string | no |  | Explicit targets (Shoutrrr and similar). |
| `devices` | string | no |  | Target device names (Pushover). |
| `priority` | integer | no |  | Message priority (Pushover / Gotify / ntfy). |
| `sound` | string | no |  | Notification sound (Pushover / ntfy). |
| `method` | string | no |  | HTTP method (generic webhook providers). |
| `headers` | string | no |  | Extra headers (generic webhook providers). |
| `event_sounds` | any | no |  | Per-event sound overrides as an `EVENT: sound` map (Pushover / ntfy), e.g. `{ PUSH_APPROVED: "magic", PUSH_REJECTED: "falling" }`. Overrides `sound` for the listed events only. |

### Proxy

`/api/proxy` — a proxy autobrr can route indexer/IRC traffic through.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.proxy.<name>}`). |
| `enabled` | boolean | yes |  | Whether the proxy is active. |
| `proxy_type` | string | yes |  | Proxy kind: `SOCKS5` or `HTTP`. |
| `addr` | string | yes |  | Proxy URL, including scheme (e.g. `socks5://127.0.0.1:1080`). |
| `user` | string | no |  | Username, where the proxy authenticates. |
| `pass` | secret string | no |  | Password, where the proxy authenticates. Credential — redacted in plan output. |
| `timeout` | integer | no |  | Connection timeout in seconds (`0` = client default). |

### Download Client

`/api/download_clients` — a download client autobrr pushes releases to.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.download_client.<name>}`). |
| `client_type` | string | yes |  | Client kind: `QBITTORRENT`, `DELUGE_V2`, `RADARR`, … |
| `enabled` | boolean | yes |  | Whether the client is active. |
| `host` | string | yes |  | Client host (URL or hostname). |
| `port` | integer | no |  | Client port. |
| `tls` | boolean | no |  | Connect over TLS. |
| `tls_skip_verify` | boolean | no |  | Skip TLS certificate verification. |
| `username` | string | no |  | Username, where the client authenticates by user/pass. |
| `password` | secret string | no |  | Password, where the client authenticates by user/pass. Credential — redacted in plan output. |
| `settings` | [`download_client_settings`](#download-client-settings) | no |  | Client-specific settings (auth, rules, delegation). |

### Indexer

`/api/indexer` — a configured indexer instance.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.indexer.<name>}`). |
| `identifier` | string | yes |  | Definition id to instantiate (e.g. `torznab`, `beyond-hd`). Sent verbatim on create; autobrr namespaces the stored value per instance. |
| `implementation` | string | yes |  | Definition implementation: `torznab`, `newznab`, `rss`, or `irc`. |
| `identifier_external` | string | no |  | Alternate identifier autobrr matches announces against, where the tracker announces under a different name than the definition id. |
| `base_url` | string | no |  | Tracker base URL. **Required for `irc` indexers** — autobrr rejects an empty `base_url` there (`indexer baseURL must not be empty`); it maps the indexer into the IRC announce handler by it. A top-level field, not a `settings` entry. |
| `enabled` | boolean | no | `true` | Whether the indexer is active. |
| `use_proxy` | boolean | no |  | Route this indexer's HTTP through a proxy. |
| `proxy_id` | integer | no |  | Proxy to route through (`${ref.proxy.<name>}`). References a [`proxy`](#proxy) by name (`${ref.proxy.<key>}`). |
| `settings` | any | yes |  | Definition settings as a flat `name: value` map. For a torznab/newznab indexer: `{ url: "...", api_key: "..." }`. For an `irc` indexer, the IRC login: `{ nick: "...", "auth.account": "...", "auth.password": "..." }` — autobrr derives the IRC network from the indexer, so the login lives here, not on a separate `irc_networks` entry. Write-only — never returned on read. |

### Irc Network

`/api/irc` — an IRC network with its channels and auth.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Network name — its identity (`${ref.irc_network.<name>}`). |
| `enabled` | boolean | yes |  | Whether autobrr connects to this network. |
| `server` | string | yes |  | IRC server hostname. |
| `port` | integer | yes |  | IRC server port. |
| `tls` | boolean | no |  | Connect over TLS. |
| `tls_skip_verify` | boolean | no |  | Skip TLS certificate verification. |
| `nick` | string | yes |  | Bot nick to use on the network. |
| `pass` | secret string | no |  | Server password (PASS), where required (write-only). Credential — redacted in plan output. |
| `auth` | [`irc_auth`](#irc-auth) | no |  | NickServ / SASL authentication. |
| `invite_command` | string | no |  | Command sent to request an invite (e.g. `/msg gatekeeper !invite`). |
| `use_bouncer` | boolean | no |  | Connect through a bouncer instead of directly. |
| `bouncer_addr` | string | no |  | Bouncer address, where `use_bouncer` is set. |
| `bot_mode` | boolean | no |  | Enable IRCv3 bot mode. |
| `use_proxy` | boolean | no |  | Route this network through a proxy. |
| `proxy_id` | integer | no |  | Proxy to route through (`${ref.proxy.<name>}`). References a [`proxy`](#proxy) by name (`${ref.proxy.<key>}`). |
| `channels` | array of [`irc_channel`](#irc-channel) | no |  | Channels to join. |

### Release Profile Duplicate

`/api/release/profiles/duplicate` — a dedup profile.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.release_profile_duplicate.<name>}`). |
| `protocol` | boolean | no |  | Treat differing protocol (torrent vs usenet) as distinct. |
| `release_name` | boolean | no |  | Match on the full release name. |
| `hash` | boolean | no |  | Match on the release hash. |
| `title` | boolean | no |  | Match on parsed title. |
| `sub_title` | boolean | no |  | Match on parsed sub-title. |
| `year` | boolean | no |  | Match on year. |
| `month` | boolean | no |  | Match on month. |
| `day` | boolean | no |  | Match on day. |
| `source` | boolean | no |  | Match on source (BluRay, WEB-DL, …). |
| `resolution` | boolean | no |  | Match on resolution. |
| `codec` | boolean | no |  | Match on video codec. |
| `container` | boolean | no |  | Match on container. |
| `dynamic_range` | boolean | no |  | Match on dynamic range (HDR/DV). |
| `audio` | boolean | no |  | Match on audio. |
| `group` | boolean | no |  | Match on release group. |
| `season` | boolean | no |  | Match on season. |
| `episode` | boolean | no |  | Match on episode. |
| `website` | boolean | no |  | Match on website/source tag. |
| `proper` | boolean | no |  | Match on PROPER. |
| `repack` | boolean | no |  | Match on REPACK. |
| `edition` | boolean | no |  | Match on edition. |
| `hybrid` | boolean | no |  | Match on hybrid. |
| `language` | boolean | no |  | Match on language. |

### Filter

`/api/filters` — a release-matching filter.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Filter name — its identity. |
| `enabled` | boolean | no |  | Whether the filter is active. |
| `priority` | integer | no |  | Evaluation priority (higher wins). |
| `use_regex` | boolean | no |  | Treat match/except patterns as regular expressions. |
| `years` | string | no |  | Match release years (e.g. `2020-2024`). |
| `seasons` | string | no |  | Match seasons (e.g. `1,3-5`). |
| `episodes` | string | no |  | Match episodes. |
| `match_releases` | string | no |  | Releases must match these terms. |
| `except_releases` | string | no |  | Releases must not match these terms. |
| `match_release_groups` | string | no |  | Match these release groups. |
| `except_release_groups` | string | no |  | Exclude these release groups. |
| `match_categories` | string | no |  | Match these categories. |
| `except_categories` | string | no |  | Exclude these categories. |
| `tags` | string | no |  | Match these tags. |
| `except_tags` | string | no |  | Exclude these tags. |
| `tags_match_logic` | string | no |  | Logic for `tags` (`ANY` / `ALL`). |
| `except_tags_match_logic` | string | no |  | Logic for `except_tags` (`ANY` / `ALL`). |
| `smart_episode` | boolean | no |  | Smarter episode/season handling. |
| `resolutions` | array of string | no |  | Match these resolutions. |
| `codecs` | array of string | no |  | Match these video codecs. |
| `sources` | array of string | no |  | Match these sources. |
| `containers` | array of string | no |  | Match these containers. |
| `match_hdr` | array of string | no |  | Match these HDR formats. |
| `except_hdr` | array of string | no |  | Exclude these HDR formats. |
| `match_other` | array of string | no |  | Match these "other" tags (REPACK, PROPER, …). |
| `except_other` | array of string | no |  | Exclude these "other" tags. |
| `match_language` | array of string | no |  | Match these languages. |
| `except_language` | array of string | no |  | Exclude these languages. |
| `formats` | array of string | no |  | Match these audio formats. |
| `quality` | array of string | no |  | Match these audio qualities. |
| `media` | array of string | no |  | Match these media types. |
| `match_release_types` | array of string | no |  | Match these release types. |
| `origins` | array of string | no |  | Match these origins (INTERNAL, SCENE, …). |
| `except_origins` | array of string | no |  | Exclude these origins. |
| `min_size` | string | no |  | Minimum release size (e.g. `1GB`). |
| `max_size` | string | no |  | Maximum release size. |
| `delay` | integer | no |  | Delay before pushing, seconds. |
| `max_downloads` | integer | no |  | Cap the number of grabs. |
| `max_downloads_unit` | string | no |  | Window the `max_downloads` cap applies over: `HOUR`, `DAY`, `WEEK`, `MONTH`, or `EVER`. |
| `announce_types` | array of string | no |  | Match these announce types (`NEW`, `PROMO`, `PROMO_GP`, `RESURRECTED`). |
| `scene` | boolean | no |  | Match only scene releases. |
| `bonus` | array of string | no |  | Match these bonus/reward tags (tracker-specific). |
| `freeleech` | boolean | no |  | Match only freeleech releases. |
| `freeleech_percent` | string | no |  | Match these freeleech percentages (e.g. `50,100`). |
| `shows` | string | no |  | Match these show/title terms. |
| `months` | string | no |  | Match months (e.g. `1,6-8`). |
| `days` | string | no |  | Match days. |
| `artists` | string | no |  | Match these artists (music). |
| `albums` | string | no |  | Match these albums (music). |
| `except_release_types` | string | no |  | Exclude these release types. |
| `perfect_flac` | boolean | no |  | Match only perfect FLAC (music). |
| `cue` | boolean | no |  | Require a CUE file (music). |
| `log` | boolean | no |  | Require a log (music). |
| `log_score` | integer | no |  | Minimum log score (music). |
| `match_uploaders` | string | no |  | Match these uploaders. |
| `except_uploaders` | string | no |  | Exclude these uploaders. |
| `match_record_labels` | string | no |  | Match these record labels (music). |
| `except_record_labels` | string | no |  | Exclude these record labels (music). |
| `tags_any` | string | no |  | Match if the release carries any of these tags. |
| `except_tags_any` | string | no |  | Exclude if the release carries any of these tags. |
| `match_release_tags` | string | no |  | Match these release tags. |
| `except_release_tags` | string | no |  | Exclude these release tags. |
| `use_regex_release_tags` | boolean | no |  | Treat release-tag patterns as regular expressions. |
| `match_description` | string | no |  | Match these terms in the release description. |
| `except_description` | string | no |  | Exclude these terms in the release description. |
| `use_regex_description` | boolean | no |  | Treat description patterns as regular expressions. |
| `min_seeders` | integer | no |  | Minimum seeders. |
| `max_seeders` | integer | no |  | Maximum seeders. |
| `min_leechers` | integer | no |  | Minimum leechers. |
| `max_leechers` | integer | no |  | Maximum leechers. |
| `release_profile_duplicate_id` | integer | no |  | Dedup profile to apply (`${ref.release_profile_duplicate.<name>}`). References a [`release_profile_duplicate`](#release-profile-duplicate) by name (`${ref.release_profile_duplicate.<key>}`). |
| `indexers` | array of [`filter_indexer`](#filter-indexer) | no |  | Indexers this filter is attached to. |
| `actions` | array of [`action`](#action) | no |  | Actions run on a matched release. |
| `external` | array of [`external_filter`](#external-filter) | no |  | External (webhook/exec) checks. |

### List

`/api/lists` — a configured list.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.list.<name>}`). |
| `list_type` | string | yes |  | List type: `RADARR`, `SONARR`, `LIDARR`, `READARR`, `WHISPARR` (*arr types, need `client_id`), or `MDBLIST`, `METACRITIC`, `PLAINTEXT`, `TRAKT`, `STEAM`, `ANILIST` (external types, need `url`). |
| `enabled` | boolean | no |  | Whether the list is active. |
| `client_id` | integer | no |  | Download client to pull wanted titles from, for the *arr list types (`${ref.download_client.<name>}`). References a [`download_client`](#download-client) by name (`${ref.download_client.<key>}`). |
| `url` | string | no |  | Source URL, for the external list types. |
| `headers` | array of string | no |  | Extra HTTP headers to send when fetching an external list. |
| `api_key` | secret string | no |  | API key for the external source, where required. Credential — redacted in plan output. |
| `filters` | array of [`list_filter`](#list-filter) | no |  | Filters this list feeds into (`${ref.filter.<name>}`). autobrr requires at least one for the *arr list types. |
| `match_release` | boolean | no |  | Match on the release name rather than the parsed title. |
| `tags_included` | array of string | no |  | Only include titles carrying these tags. |
| `tags_excluded` | array of string | no |  | Exclude titles carrying these tags. |
| `include_unmonitored` | boolean | no |  | Include unmonitored titles (*arr types). |
| `include_alternate_titles` | boolean | no |  | Include alternate titles (*arr types). |
| `include_year` | boolean | no |  | Append the year to the matched title. |
| `skip_clean_sanitize` | boolean | no |  | Skip autobrr's title clean/sanitize pass. |

### Feed

`/api/feeds` — a configured feed.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.feed.<name>}`). |
| `indexer_id` | integer | no |  | The feed-type indexer this feed polls (`${ref.indexer.<name>}`). References a [`indexer`](#indexer) by name (`${ref.indexer.<key>}`). |
| `feed_type` | string | yes |  | Feed protocol: `TORZNAB`, `NEWZNAB`, or `RSS`. |
| `enabled` | boolean | no |  | Whether the feed is active. |
| `url` | string | no |  | Feed URL. |
| `interval` | integer | no |  | Poll interval, minutes. |
| `timeout` | integer | no |  | Per-request timeout, seconds. |
| `max_age` | integer | no |  | Ignore items older than this many minutes (`0` = no limit). |
| `categories` | array of integer | no |  | Torznab/Newznab category ids to fetch. |
| `api_key` | secret string | no |  | API key for the feed source, where required. Credential — redacted in plan output. |
| `cookie` | secret string | no |  | Cookie to send with feed requests (private trackers). Credential — redacted in plan output. |
| `tls_skip_verify` | boolean | no |  | Skip TLS certificate verification. |
| `settings` | [`feed_settings`](#feed-settings) | no |  | Feed settings blob (currently just `download_type`). |

## Types

### Notification Event

Allowed values: `PUSH_APPROVED` / `PUSH_REJECTED` / `PUSH_ERROR` / `IRC_DISCONNECTED` / `IRC_RECONNECTED` / `APP_UPDATE_AVAILABLE` / `RELEASE_NEW` / `TEST`.

### Download Client Settings

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `apikey` | secret string | no |  | API key (arr-style clients that authenticate by key rather than user/pass). Credential — redacted in plan output. |
| `basic` | [`download_client_basic`](#download-client-basic) | no |  | HTTP basic-auth credentials (deprecated — prefer `auth`). |
| `auth` | [`download_client_auth`](#download-client-auth) | no |  | Auth for the client's web endpoint (supersedes `basic`). |
| `rules` | [`download_client_rules`](#download-client-rules) | no |  | Throughput/queue rules applied before pushing releases. |
| `external_download_client_id` | integer | no |  | Id of another download client to delegate to (proxy setups). |
| `external_download_client` | string | no |  | Name of another download client to delegate to (proxy setups). |

### Irc Auth

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `mechanism` | string | no |  | Auth mechanism: `NONE`, `SASL_PLAIN`, or `NICKSERV`. |
| `account` | string | no |  | Account / login name. |
| `password` | secret string | no |  | Account password (write-only; returned `<redacted>` on read). Credential — redacted in plan output. |

### Irc Channel

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Channel name (e.g. `#announce`). |
| `password` | secret string | no |  | Channel key/password, where the channel is protected (write-only). Credential — redacted in plan output. |
| `enabled` | boolean | no | `true` | Whether autobrr joins this channel. Must always be written: autobrr's channel struct decodes an absent `enabled` key as `false`, and its join workflow skips disabled channels — so omitting it silently stops announces. Non-optional with a `true` default for that reason. |

### Filter Indexer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server indexer id — attach a managed indexer via `${ref.indexer.<name>}`. References a [`indexer`](#indexer) by name (`${ref.indexer.<key>}`). |
| `name` | string | no |  | Indexer display name (read-only; autobrr fills it). |
| `identifier` | string | no |  | Indexer identifier, e.g. `torznab-<name>` (read-only; autobrr fills it). |

### Action

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned id. |
| `name` | string | no |  | Display name. |
| `action_type` | string | no |  | Action kind: `QBITTORRENT`, `DELUGE_V2`, `WEBHOOK`, `EXEC`, `TEST`, … |
| `enabled` | boolean | no |  | Whether the action is active. |
| `category` | string | no |  | Category to file the release under (client actions). |
| `tags` | string | no |  | Tags to apply (client actions). |
| `label` | string | no |  | Label to apply (Deluge/qBittorrent). |
| `save_path` | string | no |  | Save path override (client actions). |
| `download_path` | string | no |  | Download path override (client actions). |
| `paused` | boolean | no |  | Add the torrent in a paused state. |
| `ignore_rules` | boolean | no |  | Ignore the client's own throughput rules for this push. |
| `first_last_piece_prio` | boolean | no |  | Prioritise the first and last pieces (streaming). |
| `skip_hash_check` | boolean | no |  | Skip the client's hash check on add. |
| `content_layout` | string | no |  | Torrent content layout: `ORIGINAL`, `SUBFOLDER_CREATE`, `SUBFOLDER_NONE`. |
| `limit_upload_speed` | integer | no |  | Upload speed limit, KiB/s. |
| `limit_download_speed` | integer | no |  | Download speed limit, KiB/s. |
| `limit_ratio` | number | no |  | Seeding ratio limit. |
| `limit_seed_time` | integer | no |  | Seeding time limit, minutes. |
| `priority` | string | no |  | Queue priority: `max` / `min` (qBittorrent). |
| `reannounce_skip` | boolean | no |  | Skip reannounce handling. |
| `reannounce_delete` | boolean | no |  | Delete the torrent if reannounce never succeeds. |
| `reannounce_interval` | integer | no |  | Reannounce interval, seconds. |
| `reannounce_max_attempts` | integer | no |  | Max reannounce attempts. |
| `client_id` | integer | no |  | Download client this action pushes to (`${ref.download_client.<name>}`). References a [`download_client`](#download-client) by name (`${ref.download_client.<key>}`). |
| `external_download_client_id` | integer | no |  | Secondary client to delegate the push to (`${ref.download_client.<name>}`). References a [`download_client`](#download-client) by name (`${ref.download_client.<key>}`). |
| `external_download_client` | string | no |  | Secondary client name (free-form, where not referenced by id). |
| `webhook_host` | string | no |  | Webhook URL (`WEBHOOK` type). |
| `webhook_method` | string | no |  | HTTP method for the webhook. |
| `webhook_type` | string | no |  | Webhook payload content type. |
| `webhook_data` | string | no |  | Request body sent to the webhook. |
| `webhook_headers` | array of string | no |  | Extra webhook headers (`Key: value`). |
| `exec_cmd` | string | no |  | Command to run (`EXEC` type). |
| `exec_args` | string | no |  | Arguments passed to the command. |
| `watch_folder` | string | no |  | Folder to watch for `.torrent` files (`WATCH_FOLDER` type). |

### External Filter

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned id. |
| `name` | string | no |  | Display name. |
| `index` | integer | no |  | Evaluation order among external checks. |
| `external_type` | string | no |  | Check kind: `EXEC` or `WEBHOOK`. |
| `enabled` | boolean | no |  | Whether the check is active. |
| `webhook_host` | string | no |  | Webhook URL (`WEBHOOK` type). |
| `webhook_method` | string | no |  | HTTP method for the webhook. |
| `webhook_data` | string | no |  | Request body sent to the webhook. |
| `webhook_headers` | string | no |  | Extra webhook headers (`Key=value,Key2=value2`). |
| `webhook_expect_status` | integer | no |  | HTTP status the webhook must return to pass. |
| `webhook_retry_status` | string | no |  | HTTP status(es) that trigger a retry. |
| `webhook_retry_attempts` | integer | no |  | Number of times to retry the webhook. |
| `webhook_retry_delay_seconds` | integer | no |  | Delay between webhook retries, seconds. |
| `exec_cmd` | string | no |  | Command to run (`EXEC` type). |
| `exec_args` | string | no |  | Arguments passed to the command. |
| `exec_expect_status` | integer | no |  | Exit status the command must return to pass. |
| `on_error` | string | no |  | Behaviour when the check errors: `CONTINUE` or `REJECT`. |

### List Filter

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server filter id — attach a managed filter via `${ref.filter.<name>}`. References a [`filter`](#filter) by name (`${ref.filter.<key>}`). |
| `name` | string | no |  | Filter display name (read-only; autobrr fills it). |

### Feed Settings

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `download_type` | string | no |  | What to hand the download client for each item: `TORRENT` (default), `MAGNET`, or `NZB` (autobrr 1.76+, for usenet RSS feeds). |

### Download Client Basic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `auth` | boolean | no |  | Whether basic auth is required. |
| `username` | string | no |  | Basic-auth username. |
| `password` | secret string | no |  | Basic-auth password. Credential — redacted in plan output. |

### Download Client Auth

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Whether auth is required. |
| `auth_type` | string | no |  | Auth scheme: `NONE`, `BASIC_AUTH`, or `DIGEST_AUTH`. |
| `username` | string | no |  | Auth username. |
| `password` | secret string | no |  | Auth password. Credential — redacted in plan output. |

### Download Client Rules

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Whether the rules are enforced. |
| `max_active_downloads` | integer | no |  | Cap on simultaneously active downloads (0 = unlimited). |
| `ignore_slow_torrents` | boolean | no |  | Skip pushing when existing torrents are slow. |
| `ignore_slow_torrents_condition` | string | no |  | When the slow-torrent check applies (`ALWAYS` / `MAX_DOWNLOADS_REACHED`). |
| `download_speed_threshold` | integer | no |  | Download-speed threshold (KB/s) for the slow check. |
| `upload_speed_threshold` | integer | no |  | Upload-speed threshold (KB/s) for the slow check. |

