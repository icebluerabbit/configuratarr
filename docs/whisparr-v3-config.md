# Whisparr v3 Configuration

Whisparr v3 (eros) — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Tag

A label applied to movies (scenes), indexers, download clients, etc.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the name referenced in `${ref.tag.<label>}`. |

### Quality Profile

Named quality profile — ordered quality ladder with format-score gates.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.quality_profile.<name>}`. |
| `upgrade_allowed` | boolean | yes |  | When `true`, Whisparr will seek a better-quality release after the initial download. |
| `cutoff` | integer | yes |  | Id of the cutoff quality; Whisparr will not seek upgrades past this point. |
| `items` | array of [`quality_profile_item`](#quality-profile-item) | no |  | Ordered quality ladder — all quality tiers and groups this profile considers. |
| `min_format_score` | integer | yes |  | Minimum aggregate custom-format score a release must reach to be grabbed. |
| `cutoff_format_score` | integer | yes |  | Minimum format score that satisfies the upgrade cutoff. |
| `min_upgrade_format_score` | integer | yes |  | Minimum improvement in custom-format score required to trigger an upgrade. |
| `format_items` | array of [`profile_format_item`](#profile-format-item) | no |  | Custom-format score contributions attached to this profile. |
| `language` | [`language`](#language) | no |  | Language requirement for grabbed releases. |

### Quality Definition

`/api/v3/qualitydefinition` — per-quality-tier size limits.

Quality definitions are server-managed entries (one per quality tier); they cannot be created
or deleted via the API. Only `title`, `min_size`, `max_size`, and `preferred_size` are
user-configurable. Configure only the entries you want to adjust — unlisted tiers keep their
current server-side values.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `title` | string | yes |  | Display name for this quality tier; used as the natural key to match against live entries. |
| `min_size` | number | no |  | Minimum acceptable size in MB for releases of this quality; `null` = no minimum. |
| `max_size` | number | no |  | Maximum acceptable size in MB for releases of this quality; `null` = no maximum. |
| `preferred_size` | number | no |  | Preferred size in MB for releases of this quality; used for scoring when multiple options exist. |

### Custom Format

A custom format — a named collection of specification conditions Whisparr
uses to score releases. The score influences download decisions via
quality profiles.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.custom_format.<name>}`. |
| `include_custom_format_when_renaming` | boolean | no |  | When true, the format name is included in Whisparr's file rename template. |
| `specifications` | array of any | no |  | Specification conditions, each a provider-shaped object (`implementation` + `fields[]`). Raw JSON — see the module docs. |

### Custom Filter

A saved custom filter for a Whisparr UI page.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the user-visible label for this filter. |
| `filter_type` | string | no |  | The UI page context this filter applies to (e.g. `MovieIndex`, `MovieFile`). Wire name is `type` (a Rust keyword). |
| `filters` | array of any | no |  | Filter conditions, each a raw object with `key`, `value`, and `type`. Raw JSON — the condition shape is not described in the static spec. |

### Delay Profile

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enable_usenet` | boolean | yes |  | Whether usenet releases are subject to this profile. |
| `enable_torrent` | boolean | yes |  | Whether torrent releases are subject to this profile. |
| `preferred_protocol` | [`download_protocol`](#download-protocol) | yes |  | Which protocol wins when a release is available on both. |
| `usenet_delay` | integer | yes |  | Minutes to wait before grabbing a usenet release. |
| `torrent_delay` | integer | yes |  | Minutes to wait before grabbing a torrent release. |
| `bypass_if_highest_quality` | boolean | yes |  | Grab immediately when the release is already at the profile's cutoff. |
| `bypass_if_above_custom_format_score` | boolean | yes |  | Grab immediately when the release scores above `minimum_custom_format_score`. |
| `minimum_custom_format_score` | integer | yes |  | The score `bypass_if_above_custom_format_score` compares against. |
| `tags` | array of integer | no |  | Tag references — this profile's identity. eros permits at most one delay profile per tag. An **empty** list addresses the seeded global profile (`id: 1`), which governs everything untagged; any other profile must carry at least one tag or eros rejects it with `'Tags' must not be empty`. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |

### Release Profile

Release profile — term-based acceptance and rejection filter for grabbed releases.

When `enabled`, Whisparr checks every candidate release title against the
`required` and `ignored` term lists before deciding to grab it:
- `required`: at least one term must appear in the release title.
- `ignored`: none of the terms may appear in the release title.

Setting `indexer_id` to `0` (the default) applies the profile to releases
from all indexers; a non-zero value restricts it to a specific indexer.
Tag-scoped profiles (via `tags`) apply only to movies that carry one of
the listed tags; an empty `tags` list means the profile applies globally.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.release_profile.<name>}`. |
| `enabled` | boolean | yes |  | When `false`, this profile is saved but not applied to any grabs. |
| `required` | any | no |  | Terms that must appear in a release title for it to be accepted. Untyped in the spec; in practice an array of term strings. `None` means no required-term constraint. |
| `ignored` | any | no |  | Terms that must **not** appear in a release title; releases containing any of these terms are rejected. Untyped in the spec; in practice an array of term strings. `None` means no ignored-term constraint. |
| `indexer_id` | integer | no | `0` | Id of the indexer this profile is restricted to; `0` means all indexers. |
| `tags` | array of integer | no |  | Movie tag ids this profile applies to; resolved from `${ref.tag.<label>}` at apply. An empty list means the profile is applied globally to all movies. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |

### Remote Path Mapping

A remote-to-local path mapping for a download client host.

Whisparr uses these to translate paths returned by download clients that
run on a different host (or container) where filesystem paths differ.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Hostname or IP of the download client that uses the remote path. |
| `remote_path` | string | yes |  | Natural key — the path as the remote download client reports it. |
| `local_path` | string | yes |  | The local filesystem path that corresponds to `remote_path`. |

### Download Client

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable` | boolean | yes |  | Whether this download client is active. |
| `protocol` | [`download_protocol`](#download-protocol) | yes |  | Download protocol used by this client (torrent or usenet). |
| `priority` | integer | no | `1` | Client priority relative to other configured download clients. |
| `remove_completed_downloads` | boolean | no | `true` | Remove downloads from the client once Whisparr has imported them. |
| `remove_failed_downloads` | boolean | yes |  | Remove downloads from the client if they fail to complete. |

Set `implementation` to one of: [`Aria2`](#download-client-aria2) / [`Deluge`](#download-client-deluge) / [`Flood`](#download-client-flood) / [`TorrentFreeboxDownload`](#download-client-freebox) / [`Hadouken`](#download-client-hadouken) / [`Nzbget`](#download-client-nzbget) / [`NzbVortex`](#download-client-nzbvortex) / [`Pneumatic`](#download-client-pneumatic) / [`QBittorrent`](#download-client-qbittorrent) / [`RTorrent`](#download-client-rtorrent) / [`Sabnzbd`](#download-client-sabnzbd) / [`TorrentBlackhole`](#download-client-torrentblackhole) / [`TorrentDownloadStation`](#download-client-torrentdownloadstation) / [`Transmission`](#download-client-transmission) / [`UsenetBlackhole`](#download-client-usenetblackhole) / [`UsenetDownloadStation`](#download-client-usenetdownloadstation) / [`UTorrent`](#download-client-utorrent) / [`Vuze`](#download-client-vuze).

#### Download Client: Aria2

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Aria2 RPC server. |
| `port` | integer | no |  | TCP port the Aria2 RPC server listens on. |
| `secret_token` | secret string | no |  | Secret token for authenticating with the Aria2 RPC interface. Credential — redacted in plan output. |
| `rpc_path` | string | no |  | Path to the Aria2 JSON-RPC endpoint (default: `/rpc`). |
| `use_ssl` | boolean | no |  | Connect to the Aria2 RPC server over HTTPS. |
| `directory` | string | no |  | Directory Aria2 saves downloaded files to. |

#### Download Client: Deluge

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Deluge daemon. |
| `port` | integer | no |  | TCP port the Deluge daemon listens on. |
| `password` | secret string | no |  | Password for authenticating with the Deluge daemon. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if Deluge is hosted behind a reverse proxy. |
| `movie_category` | string | no |  | Label assigned to movie downloads in Deluge. |
| `movie_imported_category` | string | no |  | Label the client moves completed downloads to after Whisparr imports them. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `add_paused` | boolean | no |  | Add torrents to Deluge in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Deluge over HTTPS. |
| `download_directory` | string | no |  | Directory Deluge saves in-progress downloads to. |
| `completed_directory` | string | no |  | Directory Deluge moves downloads to once complete. |

#### Download Client: Flood

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Flood server. |
| `port` | integer | no |  | TCP port the Flood web UI listens on. |
| `username` | string | no |  | Username for authenticating with Flood. |
| `password` | secret string | no |  | Password for authenticating with Flood. Credential — redacted in plan output. |
| `destination` | string | no |  | Directory Flood saves downloaded files to. |
| `url_base` | string | no |  | URL base path if Flood is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Flood in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Flood over HTTPS. |
| `field_tags` | array of string | no |  | Tags applied to the torrent in Flood (string labels, not Whisparr tag ids) |
| `additional_tags` | array of integer | no |  | Additional Whisparr-managed metadata tags appended to the torrent (integer codes). |
| `post_import_tags` | array of string | no |  | Tags applied to the torrent in Flood after Whisparr imports it. |

#### Download Client: Freebox

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Freebox server. |
| `port` | integer | no |  | TCP port the Freebox API listens on. |
| `api_url` | string | no |  | Base URL of the Freebox HTTP API (e.g. `http://mafreebox.freebox.fr/`). |
| `app_id` | string | no |  | Application ID registered with the Freebox for OAuth-style access. |
| `app_token` | secret string | no |  | Application token obtained during the Freebox authorisation flow. Credential — redacted in plan output. |
| `category` | string | no |  | Download category assigned to movie torrents in FreeboxOS. |
| `destination_directory` | string | no |  | Directory the Freebox saves movie downloads to. |
| `recent_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `add_paused` | boolean | no |  | Add torrents to FreeboxOS in a paused state. |
| `use_ssl` | boolean | no |  | Connect to the Freebox API over HTTPS. |

#### Download Client: Hadouken

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Hadouken server. |
| `port` | integer | no |  | TCP port the Hadouken web UI listens on. |
| `username` | string | no |  | Username for authenticating with Hadouken. |
| `password` | secret string | no |  | Password for authenticating with Hadouken. Credential — redacted in plan output. |
| `category` | string | no |  | Category assigned to downloads in Hadouken. |
| `url_base` | string | no |  | URL base path if Hadouken is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to Hadouken over HTTPS. |

#### Download Client: Nzbget

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBGet server. |
| `port` | integer | no |  | TCP port the NZBGet web UI listens on. |
| `username` | string | no |  | Username for authenticating with NZBGet. |
| `password` | secret string | no |  | Password for authenticating with NZBGet. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category assigned to movie downloads in NZBGet. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `url_base` | string | no |  | URL base path if NZBGet is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add downloads to NZBGet in a paused state. |
| `use_ssl` | boolean | no |  | Connect to NZBGet over HTTPS. |

#### Download Client: NzbVortex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBVortex server. |
| `port` | integer | no |  | TCP port the NZBVortex server listens on. |
| `url_base` | string | no |  | URL base path if NZBVortex is hosted behind a reverse proxy. |
| `api_key` | secret string | no |  | API key used to authenticate with NZBVortex. Credential — redacted in plan output. |
| `tv_category` | string | no |  | Group/category assigned to downloads in NZBVortex. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |

#### Download Client: Pneumatic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Whisparr drops NZB files into for Pneumatic to pick up. |
| `strm_folder` | string | no |  | Folder Pneumatic writes `.strm` stream files to. |

#### Download Client: QBittorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the qBittorrent server. |
| `port` | integer | no |  | TCP port the qBittorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with qBittorrent. |
| `password` | secret string | no |  | Password for authenticating with qBittorrent. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category assigned to movie downloads in qBittorrent. |
| `movie_imported_category` | string | no |  | Category the client moves completed downloads to after Whisparr imports them. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `initial_state` | integer | no |  | 0 = Start, 1 = ForceStart, 2 = Pause |
| `url_base` | string | no |  | URL base path if qBittorrent is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to qBittorrent over HTTPS. |
| `sequential_order` | boolean | no |  | Download pieces in sequential order to enable early playback. |
| `first_and_last` | boolean | no |  | Prioritise downloading the first and last pieces of each file first. |
| `content_layout` | integer | no |  | How the downloaded content is laid out on disk. 0 = Default, 1 = Original, 2 = Subfolder. |

#### Download Client: RTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the rTorrent SCGI/HTTP endpoint. |
| `port` | integer | no |  | TCP port the rTorrent SCGI or HTTP interface listens on. |
| `username` | string | no |  | Username for authenticating with rTorrent (used when fronted by a web server). |
| `password` | secret string | no |  | Password for authenticating with rTorrent (used when fronted by a web server). Credential — redacted in plan output. |
| `movie_category` | string | no |  | Label assigned to movie torrents in rTorrent. |
| `movie_directory` | string | no |  | Directory rTorrent saves movie downloads to. |
| `movie_imported_category` | string | no |  | Label the client moves completed downloads to after Whisparr imports them. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `url_base` | string | no |  | URL base path if rTorrent is hosted behind a reverse proxy. |
| `add_stopped` | boolean | no |  | Add torrents to rTorrent in a stopped state rather than starting immediately. |
| `use_ssl` | boolean | no |  | Connect to rTorrent over HTTPS. |

#### Download Client: Sabnzbd

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the SABnzbd server. |
| `port` | integer | no |  | TCP port the SABnzbd web UI listens on. |
| `username` | string | no |  | Username for authenticating with SABnzbd. |
| `password` | secret string | no |  | Password for authenticating with SABnzbd. Credential — redacted in plan output. |
| `api_key` | secret string | no |  | SABnzbd API key used as an alternative to username/password auth. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category assigned to movie downloads in SABnzbd. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `url_base` | string | no |  | URL base path if SABnzbd is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to SABnzbd over HTTPS. |

#### Download Client: TorrentBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `torrent_folder` | string | no |  | Folder Whisparr drops `.torrent` files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Whisparr watches for completed downloads from the external client. |
| `magnet_file_extension` | string | no |  | File extension used when saving magnet links as files (e.g. `.magnet`). |
| `save_magnet_files` | boolean | no |  | Save magnet links as files in the torrent folder instead of ignoring them. |
| `read_only` | boolean | no |  | Do not move or delete files from the watch folder after import (read-only mode). |

#### Download Client: TorrentDownloadStation

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Synology NAS running Download Station. |
| `port` | integer | no |  | TCP port the Synology DSM web interface listens on. |
| `username` | string | no |  | Username for authenticating with Synology DSM. |
| `password` | secret string | no |  | Password for authenticating with Synology DSM. Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to Synology DSM over HTTPS. |
| `tv_category` | string | no |  | Category assigned to downloads in Download Station. |
| `tv_directory` | string | no |  | Directory Download Station saves downloads to. |

#### Download Client: Transmission

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Transmission server. |
| `port` | integer | no |  | TCP port the Transmission RPC interface listens on. |
| `username` | string | no |  | Username for authenticating with Transmission. |
| `password` | secret string | no |  | Password for authenticating with Transmission. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category (label) assigned to movie downloads in Transmission. |
| `movie_imported_category` | string | no |  | Category the client moves completed downloads to after Whisparr imports them. |
| `movie_directory` | string | no |  | Directory Transmission saves movie downloads to. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `url_base` | string | no |  | URL base path if Transmission is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Transmission in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Transmission over HTTPS. |

#### Download Client: UsenetBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Whisparr drops NZB files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Whisparr watches for completed downloads from the external client. |

#### Download Client: UsenetDownloadStation

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Synology NAS running Download Station. |
| `port` | integer | no |  | TCP port the Synology DSM web interface listens on. |
| `username` | string | no |  | Username for authenticating with Synology DSM. |
| `password` | secret string | no |  | Password for authenticating with Synology DSM. Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to Synology DSM over HTTPS. |
| `tv_category` | string | no |  | Category assigned to downloads in Download Station. |
| `tv_directory` | string | no |  | Directory Download Station saves downloads to. |

#### Download Client: UTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the uTorrent web UI. |
| `port` | integer | no |  | TCP port the uTorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with uTorrent. |
| `password` | secret string | no |  | Password for authenticating with uTorrent. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category assigned to movie downloads in uTorrent. |
| `movie_imported_category` | string | no |  | Category the client moves completed downloads to after Whisparr imports them. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `initial_state` | integer | no |  | Initial state. 0 = Start, 1 = ForceStart, 2 = Pause, 3 = Stop Note: the eros C# property is misspelled `IntialState` (missing an 'i'), and that typo is what actually ships on the wire. |
| `url_base` | string | no |  | URL base path if uTorrent is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to uTorrent over HTTPS. |

#### Download Client: Vuze

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Vuze remote UI server. |
| `port` | integer | no |  | TCP port the Vuze remote interface listens on. |
| `username` | string | no |  | Username for authenticating with Vuze. |
| `password` | secret string | no |  | Password for authenticating with Vuze. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category (label) assigned to movie downloads in Vuze. |
| `movie_imported_category` | string | no |  | Category the client moves completed downloads to after Whisparr imports them. |
| `movie_directory` | string | no |  | Directory Vuze saves movie downloads to. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `url_base` | string | no |  | URL base path if Vuze is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Vuze in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Vuze over HTTPS. |

### Indexer

Indexer definition — connects Whisparr to a usenet or torrent search source.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable_rss` | boolean | yes |  | Whether to include this indexer in RSS sync feeds. |
| `enable_automatic_search` | boolean | yes |  | Whether to use this indexer for automatic (monitored) searches. |
| `enable_interactive_search` | boolean | yes |  | Whether to use this indexer for interactive (manual) searches. |
| `protocol` | [`download_protocol`](#download-protocol) | yes |  | Transport protocol used by this indexer (usenet or torrent). |
| `priority` | integer | no | `25` | Indexer priority; lower values are preferred when multiple indexers match a grab. |
| `download_client_id` | integer | no |  | Download client to use exclusively for grabs from this indexer; absent means use the default. References a [`download_client`](#download-client) by name (`${ref.download_client.<key>}`). |

Set `implementation` to one of: [`FileList`](#indexer-filelist) / [`HDBits`](#indexer-hdbits) / [`IPTorrents`](#indexer-iptorrents) / [`Newznab`](#indexer-newznab) / [`TorrentRssIndexer`](#indexer-torrentrss) / [`Torznab`](#indexer-torznab).

#### Indexer: FileList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | yes |  | FileList account username. |
| `passkey` | secret string | no |  | FileList account passkey used for API authentication. Credential — redacted in plan output. |
| `base_url` | string | no |  | Base URL of the FileList tracker. |
| `categories` | array of integer | no |  | FileList category IDs to include in searches. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |
| `seed_ratio` | number | no |  | Minimum seed ratio Whisparr must reach before stopping seeding. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Whisparr must seed after download. |
| `reject_blocklisted_torrent_hashes_while_grabbing` | boolean | yes |  | Reject grabs whose torrent hash is on the blocklist. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `fail_downloads` | array of integer | no |  | Download outcomes (e.g. executables, potentially dangerous files) that should be treated as a failed grab. |

#### Indexer: HdBits

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the HDBits tracker. |
| `username` | string | yes |  | HDBits account username. |
| `api_key` | secret string | no |  | HDBits API key for authentication. Credential — redacted in plan output. |
| `categories` | array of integer | no |  | HDBits category IDs to include in searches. |
| `codecs` | array of integer | no |  | HDBits codec filter IDs; empty means no codec restriction. |
| `mediums` | array of integer | no |  | HDBits medium (source) filter IDs; empty means no medium restriction. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_ratio` | number | no |  | Minimum seed ratio Whisparr must reach before stopping seeding. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Whisparr must seed after download. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `fail_downloads` | array of integer | no |  | Download outcomes (e.g. executables, potentially dangerous files) that should be treated as a failed grab. |
| `reject_blocklisted_torrent_hashes_while_grabbing` | boolean | yes |  | Reject grabs whose torrent hash is on the blocklist. |

#### Indexer: IpTorrents

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | RSS feed URL including the user's passkey (provided by IPTorrents). |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_ratio` | number | no |  | Minimum seed ratio Whisparr must reach before stopping seeding. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Whisparr must seed after download. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `fail_downloads` | array of integer | no |  | Download outcomes (e.g. executables, potentially dangerous files) that should be treated as a failed grab. |
| `reject_blocklisted_torrent_hashes_while_grabbing` | boolean | yes |  | Reject grabs whose torrent hash is on the blocklist. |

#### Indexer: Newznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Newznab indexer. |
| `api_path` | string | no |  | URL path to the Newznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Newznab indexer. Credential — redacted in plan output. |
| `categories` | array of integer | no |  | Newznab category IDs to include in searches. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `remove_year` | boolean | yes |  | Strip the release year from search queries before sending them to the indexer. |
| `fail_downloads` | array of integer | no |  | Download outcomes (e.g. executables, potentially dangerous files) that should be treated as a failed grab. |

#### Indexer: TorrentRss

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | URL of the torrent RSS feed. |
| `cookie` | secret string | no |  | Session cookie sent with RSS requests for authenticated feeds. Credential — redacted in plan output. |
| `allow_zero_size` | boolean | yes |  | Allow releases that report a size of zero bytes. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_ratio` | number | no |  | Minimum seed ratio Whisparr must reach before stopping seeding. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Whisparr must seed after download. |
| `reject_blocklisted_torrent_hashes_while_grabbing` | boolean | yes |  | Reject grabs whose torrent hash is on the blocklist. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `fail_downloads` | array of integer | no |  | Download outcomes (e.g. executables, potentially dangerous files) that should be treated as a failed grab. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

#### Indexer: Torznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Torznab indexer. |
| `api_path` | string | no |  | URL path to the Torznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Torznab indexer. Credential — redacted in plan output. |
| `categories` | array of integer | no |  | Torznab category IDs to include in searches. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `fail_downloads` | array of integer | no |  | Download outcomes (e.g. executables, potentially dangerous files) that should be treated as a failed grab. |
| `remove_year` | boolean | yes |  | Strip the release year from search queries before sending them to the indexer. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_ratio` | number | no |  | Minimum seed ratio Whisparr must reach before stopping seeding. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Whisparr must seed after download. |
| `reject_blocklisted_torrent_hashes_while_grabbing` | boolean | yes |  | Reject grabs whose torrent hash is on the blocklist. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

### Notification

Notification connection — pushes Whisparr events to an external service.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `on_grab` | boolean | yes |  | Fire notification when a release is grabbed for download. |
| `on_download` | boolean | yes |  | Fire notification when a scene file is imported after download. |
| `on_upgrade` | boolean | yes |  | Fire notification when a file is upgraded to a higher-quality version. |
| `on_rename` | boolean | yes |  | Fire notification when a scene file is renamed. |
| `on_movie_added` | boolean | yes |  | Fire notification when a scene is added to the Whisparr library. |
| `on_movie_delete` | boolean | yes |  | Fire notification when a scene is deleted from the library. |
| `on_movie_file_delete` | boolean | yes |  | Fire notification when a scene file is deleted. |
| `on_movie_file_delete_for_upgrade` | boolean | yes |  | Fire notification when a file is deleted to make room for an upgrade. |
| `on_health_issue` | boolean | yes |  | Fire notification when a health-check issue is detected. |
| `on_health_restored` | boolean | yes |  | Fire notification when a previously detected health-check issue is resolved. |
| `on_application_update` | boolean | yes |  | Fire notification when a Whisparr application update is available. |
| `on_manual_interaction_required` | boolean | yes |  | Fire notification when a download requires manual interaction. |
| `include_health_warnings` | boolean | yes |  | Include health warnings (not just errors) in health-issue notifications. |

Set `implementation` to one of: [`Apprise`](#notification-apprise) / [`CustomScript`](#notification-customscript) / [`Discord`](#notification-discord) / [`Email`](#notification-email) / [`MediaBrowser`](#notification-emby) / [`Gotify`](#notification-gotify) / [`Join`](#notification-join) / [`Xbmc`](#notification-kodi) / [`MailGun`](#notification-mailgun) / [`Notifiarr`](#notification-notifiarr) / [`Ntfy`](#notification-ntfy) / [`PlexServer`](#notification-plex) / [`Prowl`](#notification-prowl) / [`PushBullet`](#notification-pushbullet) / [`Pushcut`](#notification-pushcut) / [`Pushover`](#notification-pushover) / [`Pushsafer`](#notification-pushsafer) / [`SendGrid`](#notification-sendgrid) / [`Signal`](#notification-signal) / [`Simplepush`](#notification-simplepush) / [`Slack`](#notification-slack) / [`Stash`](#notification-stash) / [`SynologyIndexer`](#notification-synologyindexer) / [`Telegram`](#notification-telegram) / [`Trakt`](#notification-trakt) / [`Twitter`](#notification-twitter) / [`Webhook`](#notification-webhook).

#### Notification: Apprise

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | yes |  | Base URL of the Apprise API server (e.g. `http://localhost:8000`). |
| `configuration_key` | string | no |  | Apprise persistent-store configuration key. Mutually exclusive with `stateless_urls`; allowed characters are `a-z`, `0-9` and `-`. |
| `stateless_urls` | string | no |  | Comma-separated stateless Apprise notification URLs (e.g. `slack://…`). Mutually exclusive with `configuration_key`. |
| `notification_type` | integer | no |  | Notification type/category identifier sent to Apprise (0 = Info). |
| `message_tags` | array of string | no |  | Tag filters applied to the Apprise notification dispatch. Not supported when `stateless_urls` is used. |
| `include_poster` | boolean | no |  | Attach the scene poster image to the notification. |
| `auth_username` | string | no |  | HTTP basic-auth username for the Apprise server. |
| `auth_password` | secret string | no |  | HTTP basic-auth password for the Apprise server. Credential — redacted in plan output. |

#### Notification: CustomScript

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `path` | string | yes |  | Absolute filesystem path to the script to execute. |
| `arguments` | string | no |  | Legacy argument string. No longer supported — the API rejects a non-empty value; kept so an existing definition round-trips. |

#### Notification: Discord

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `web_hook_url` | string | yes |  | Discord incoming webhook URL. |
| `username` | string | no |  | Display name override for the webhook bot. |
| `avatar` | string | no |  | Avatar image URL for the webhook bot. |
| `author` | string | no |  | Author name shown in the Discord embed header. |
| `grab_fields` | array of integer | no |  | Field indices included in grab-event notification embeds. |
| `import_fields` | array of integer | no |  | Field indices included in import-event notification embeds. |
| `manual_interaction_fields` | array of integer | no |  | Field indices included in manual-interaction-required notification embeds. |

#### Notification: Email

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | yes |  | SMTP server hostname or IP address. |
| `port` | integer | yes |  | SMTP server port number (default 587). |
| `use_encryption` | integer | yes |  | Encryption mode: 0 = preferred, 1 = always, 2 = never. |
| `username` | string | no |  | SMTP authentication username. |
| `password` | secret string | no |  | SMTP authentication password. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `to` | array of string | no |  | Primary recipient email addresses. |
| `cc` | array of string | no |  | Carbon-copy recipient email addresses. |
| `bcc` | array of string | no |  | Blind carbon-copy recipient email addresses. |

#### Notification: Emby

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Emby/Jellyfin server hostname or IP address. |
| `port` | integer | yes |  | Emby/Jellyfin server HTTP port (default 8096). |
| `use_ssl` | boolean | no |  | Connect to Emby/Jellyfin over HTTPS. |
| `url_base` | string | no |  | URL base path when Emby/Jellyfin is hosted behind a reverse proxy. |
| `api_key` | secret string | yes |  | Emby/Jellyfin API key for authentication. Credential — redacted in plan output. |
| `notify` | boolean | no |  | Send an on-screen notification to Emby/Jellyfin users on events. |
| `update_library` | boolean | no |  | Trigger an Emby/Jellyfin library refresh after a scene is imported. |
| `map_from` | string | no |  | Whisparr-side path prefix to rewrite when Emby/Jellyfin mounts the library at a different location. |
| `map_to` | string | no |  | Emby/Jellyfin-side path prefix that `map_from` is rewritten to. |

#### Notification: Gotify

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | yes |  | Gotify server URL (e.g. `http://gotify.example.com`). |
| `app_token` | secret string | yes |  | Gotify application token used to publish messages. Credential — redacted in plan output. |
| `priority` | integer | no |  | Message priority level sent with each notification (default 5). |
| `include_movie_poster` | boolean | no |  | Attach the scene poster image to the notification. |
| `metadata_links` | array of integer | no |  | Metadata link types to append to the message body. |
| `preferred_metadata_link` | integer | no |  | Metadata link type used for the message's primary link. Must be one of the types selected in `metadata_links` when that list is non-empty. |

#### Notification: Join

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Join API key for authentication. Credential — redacted in plan output. |
| `device_ids` | string | no |  | Legacy comma-separated device ids. Deprecated by the API in favour of `device_names`; kept so an existing definition round-trips. |
| `device_names` | string | no |  | Comma-separated target device names; leave empty to send to all devices. |
| `priority` | integer | no |  | Notification priority level. |

#### Notification: Kodi

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Kodi hostname or IP address. |
| `port` | integer | yes |  | Kodi JSON-RPC HTTP port (default 8080). |
| `use_ssl` | boolean | no |  | Connect to Kodi over HTTPS. |
| `url_base` | string | no |  | URL base path for the Kodi JSON-RPC endpoint (default `/jsonrpc`). |
| `username` | string | no |  | Kodi authentication username. |
| `password` | secret string | no |  | Kodi authentication password. Credential — redacted in plan output. |
| `display_time` | integer | no |  | Duration in seconds to display the on-screen notification (minimum 2). |
| `notify` | boolean | no |  | Display an on-screen notification in Kodi on events. |
| `update_library` | boolean | no |  | Trigger a Kodi video library update after a scene is imported. |
| `clean_library` | boolean | no |  | Trigger a Kodi video library clean after a scene is deleted. |
| `always_update` | boolean | no |  | Always update the library on every event, not just import events. |

#### Notification: Mailgun

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Mailgun API key for authentication. Credential — redacted in plan output. |
| `use_eu_endpoint` | boolean | no |  | Use the EU Mailgun API endpoint instead of the US endpoint. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `sender_domain` | string | yes |  | Mailgun sending domain registered in your account. |
| `recipients` | array of string | no |  | Recipient email addresses. |

#### Notification: Notifiarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Notifiarr API key for authentication.  The eros C# property is `APIKey`, which the *arr field-name derivation lower-cases only in the first position — the wire key is `aPIKey`, not `apiKey`. Credential — redacted in plan output. |

#### Notification: Ntfy

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | yes |  | Base URL of the ntfy server (e.g. `https://ntfy.sh`). |
| `access_token` | secret string | no |  | Bearer access token for ntfy authentication (alternative to username/password). Credential — redacted in plan output. |
| `user_name` | string | no |  | HTTP basic-auth username for the ntfy server. The eros C# property is `UserName`, so the wire key is `userName` (radarr's is `username`). |
| `password` | secret string | no |  | HTTP basic-auth password for the ntfy server. Credential — redacted in plan output. |
| `priority` | integer | no |  | Message priority level (1 = min … 5 = max, default 3). |
| `topics` | array of string | no |  | ntfy topic names to publish notifications to. |
| `message_tags` | array of string | no |  | ntfy message tags applied to the notification (emoji shortcodes accepted). |
| `click_url` | string | no |  | URL opened when the notification is tapped by the user. |

#### Notification: Plex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | no |  | Plex server selected from the plex.tv account, populated by the UI's `servers` action. Not persisted with the settings blob. |
| `host` | string | yes |  | Plex Media Server hostname or IP address. |
| `port` | integer | yes |  | Plex Media Server HTTP port (default 32400). |
| `use_ssl` | boolean | no |  | Connect to Plex over HTTPS. |
| `url_base` | string | no |  | URL base path when Plex is hosted behind a reverse proxy. |
| `auth_token` | secret string | yes |  | Plex authentication token (X-Plex-Token). Credential — redacted in plan output. |
| `sign_in` | string | no |  | OAuth sign-in marker used by the UI to start the plex.tv flow (default `startOAuth`). |
| `update_library` | boolean | no |  | Trigger a Plex library section refresh after a scene is imported. |
| `map_from` | string | no |  | Whisparr-side path prefix to rewrite when Plex mounts the library at a different location. |
| `map_to` | string | no |  | Plex-side path prefix that `map_from` is rewritten to. |

#### Notification: Prowl

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Prowl API key for authentication. Credential — redacted in plan output. |
| `priority` | integer | no |  | Notification priority level (-2 = very low … 2 = emergency). |

#### Notification: Pushbullet

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | PushBullet access token used for authentication. Credential — redacted in plan output. |
| `device_ids` | array of string | no |  | Target device identifiers to receive the push notification; leave empty to send to all devices. |
| `channel_tags` | array of string | no |  | PushBullet channel tags to publish the notification to. |
| `sender_id` | string | no |  | Sender device identifier shown as the push source. |

#### Notification: Pushcut

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `notification_name` | string | yes |  | Name of the Pushcut notification definition to trigger. |
| `api_key` | secret string | yes |  | Pushcut API key for authentication. Credential — redacted in plan output. |
| `time_sensitive` | boolean | no |  | Deliver as a time-sensitive notification (bypasses focus modes on iOS). |
| `include_poster` | boolean | no |  | Attach the scene poster image to the notification. |
| `metadata_links` | array of integer | no |  | Metadata link types to append to the message body. |

#### Notification: Pushover

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Pushover application API token. Credential — redacted in plan output. |
| `user_key` | secret string | yes |  | Pushover user or group key identifying the recipient. Credential — redacted in plan output. |
| `devices` | array of string | no |  | Target device names; leave empty to send to all registered devices. |
| `priority` | integer | no |  | Notification priority (-2 = lowest … 2 = emergency). |
| `retry` | integer | no |  | Retry interval in seconds for emergency-priority notifications (30 … 86400). |
| `expire` | integer | no |  | Expiration time in seconds after which emergency retries stop. |
| `sound` | string | no |  | Notification sound name played on the device. |

#### Notification: Pushsafer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Pushsafer private or alias key used for authentication. Credential — redacted in plan output. |
| `device_ids` | array of string | no |  | Device group id or list of device ids; leave empty to send to all devices. |
| `priority` | integer | no |  | Notification priority level. |
| `retry` | integer | no |  | Retry interval in seconds for emergency-priority notifications (60 … 10800). |
| `expire` | integer | no |  | Expiration time in seconds after which emergency retries stop (60 … 10800). |
| `sound` | string | no |  | Notification sound number 0-62; leave empty for the device default. |
| `vibration` | string | no |  | Vibration pattern 1-3; leave empty for the device default. |
| `icon` | string | no |  | Icon number 1-181; leave empty for the default Pushsafer icon. |
| `icon_color` | string | no |  | Icon colour in hex format (e.g. `#ff0000`); leave empty for the default. |

#### Notification: Sendgrid

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | SendGrid API key for authentication. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `recipients` | array of string | no |  | Recipient email addresses. |

#### Notification: Signal

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Hostname or IP address of the signal-cli REST API gateway. |
| `port` | integer | yes |  | TCP port the signal-cli REST API listens on. |
| `use_ssl` | boolean | no |  | Connect to the gateway over HTTPS. |
| `sender_number` | secret string | yes |  | Registered Signal phone number messages are sent from. Credential — redacted in plan output. |
| `receiver_id` | string | yes |  | Recipient group id or phone number. |
| `auth_username` | string | no |  | HTTP basic-auth username for the gateway. |
| `auth_password` | secret string | no |  | HTTP basic-auth password for the gateway. Credential — redacted in plan output. |

#### Notification: Simplepush

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `key` | secret string | yes |  | Simplepush API key identifying the recipient device. Credential — redacted in plan output. |
| `event` | string | no |  | Custom event name for categorizing notifications in Simplepush. |

#### Notification: Slack

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `web_hook_url` | string | yes |  | Slack incoming webhook URL. |
| `username` | string | yes |  | Display name for the webhook bot. |
| `icon` | string | no |  | Emoji name or image URL to use as the bot's icon (e.g. `:ghost:`). |
| `channel` | string | no |  | Slack channel to post to, overriding the webhook's default channel. |

#### Notification: Stash

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Stash hostname or IP address. |
| `port` | integer | yes |  | Stash HTTP port (default 9999). |
| `use_ssl` | boolean | no |  | Connect to Stash over HTTPS instead of HTTP. |
| `api_key` | secret string | no |  | Stash API key for authentication. Credential — redacted in plan output. |
| `generate_covers` | boolean | no |  | Generate covers for new media during the Stash scan. |
| `generate_previews` | boolean | no |  | Generate previews for new media during the Stash scan. |
| `generate_image_previews` | boolean | no |  | Generate image previews during the Stash scan. Requires `generate_previews` to also be enabled. |
| `generate_sprites` | boolean | no |  | Generate sprites for new media during the Stash scan. |
| `generate_phashes` | boolean | no |  | Generate perceptual hashes for new media during the Stash scan. |
| `metadata_identify` | boolean | no |  | Run the Stash metadata Identify task on newly scanned files. |
| `stash_box_endpoint` | string | no |  | Stash Box GraphQL endpoint used by the Identify task (default `https://stashdb.org/graphql`). |
| `builtin_autotag` | boolean | no |  | Use Stash's builtin autotag source during the Identify task. |
| `include_male_performers` | boolean | no |  | Include male performers during the Identify task. |
| `set_cover_image` | boolean | no |  | Set the scene cover image during the Identify task. |
| `skip_multiple_matches` | boolean | no |  | Skip matches that return more than one result. |
| `skip_multiple_match_tag` | integer | no |  | Stash tag id applied to scenes whose match was skipped. |
| `set_organized` | boolean | no |  | Mark scenes organized during the Identify task. |
| `map_from` | string | no |  | Whisparr-side path prefix to rewrite when Stash mounts the library at a different location. |
| `map_to` | string | no |  | Stash-side path prefix that `map_from` is rewritten to. |

#### Notification: SynologyIndexer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `update_library` | boolean | no |  | Trigger a Synology media library update after a scene is imported. |

#### Notification: Telegram

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bot_token` | secret string | yes |  | Telegram bot token issued by BotFather. Credential — redacted in plan output. |
| `chat_id` | string | yes |  | Target chat, group, or channel ID to send messages to. |
| `topic_id` | integer | no |  | Topic (message thread) ID for supergroup forums. Must be greater than 1. |
| `send_silently` | boolean | no |  | Send the notification silently (no sound or alert on the recipient's device). |
| `include_app_name_in_title` | boolean | no |  | Prefix the message title with the application name. |
| `include_instance_name_in_title` | boolean | no |  | Prefix the message title with this instance's name. |
| `metadata_links` | array of integer | no |  | Metadata link types to append to the message body. |

#### Notification: Trakt

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | yes |  | Trakt OAuth access token. Credential — redacted in plan output. |
| `refresh_token` | secret string | yes |  | Trakt OAuth refresh token used to obtain a new access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 timestamp at which the access token expires. |
| `auth_user` | string | no |  | Trakt username associated with the authenticated account. |
| `sign_in` | string | no |  | OAuth sign-in marker used by the UI to start the Trakt flow (default `startOAuth`). |

#### Notification: Twitter

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `consumer_key` | secret string | yes |  | Twitter application consumer key (API key). Credential — redacted in plan output. |
| `consumer_secret` | secret string | yes |  | Twitter application consumer secret (API secret). Credential — redacted in plan output. |
| `access_token` | secret string | yes |  | Twitter user OAuth access token. Credential — redacted in plan output. |
| `access_token_secret` | secret string | yes |  | Twitter user OAuth access token secret. Credential — redacted in plan output. |
| `mention` | string | no |  | Twitter username to mention in the notification tweet. |
| `direct_message` | boolean | no |  | Send the notification as a direct message rather than a public tweet. |
| `authorize_notification` | string | no |  | OAuth sign-in marker used by the UI to start the Twitter authorisation flow (default `startOAuth`). Must be empty until the access token pair is set. |

#### Notification: Webhook

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `url` | string | yes |  | Webhook endpoint URL that receives the HTTP request. |
| `method` | integer | yes |  | HTTP method to use: 1 = POST, 2 = PUT. |
| `username` | string | no |  | HTTP basic-auth username sent with the request. |
| `password` | secret string | no |  | HTTP basic-auth password sent with the request. Credential — redacted in plan output. |

### Metadata

Metadata consumer — instructs Whisparr to write sidecar metadata files and
artwork alongside downloaded media using a specific plugin.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable` | boolean | yes |  | Whether this metadata consumer is active. |

Set `implementation` to one of: [`XbmcMetadata`](#metadata-xbmc) / [`RoksboxMetadata`](#metadata-roksbox) / [`WdtvMetadata`](#metadata-wdtv) / [`KometaMetadata`](#metadata-kometa) / [`MediaBrowserMetadata`](#metadata-mediabrowser).

#### Metadata: Xbmc

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `movie_metadata` | boolean | no |  | Write movie-level NFO metadata files. |
| `movie_metadata_url` | boolean | no |  | Include the tmdb/imdb url inside NFO files. |
| `movie_metadata_language` | integer | no |  | Language to write metadata in, if available. |
| `movie_images` | boolean | no |  | Download and store movie-level artwork. |
| `use_movie_nfo` | boolean | no |  | Write metadata to movie.nfo instead of the default `<movie-filename>.nfo`. |
| `add_collection_name` | boolean | no |  | Write the collection name to the .nfo file. |

#### Metadata: Roksbox

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `movie_metadata` | boolean | no |  | Write movie-level metadata files. |
| `movie_images` | boolean | no |  | Download and store movie-level artwork. |

#### Metadata: Wdtv

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `movie_metadata` | boolean | no |  | Write movie-level metadata files. |
| `movie_images` | boolean | no |  | Download and store movie-level artwork. |

#### Metadata: Kometa

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `movie_images` | boolean | no |  | Download and store movie-level artwork. |

#### Metadata: MediaBrowser

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `movie_metadata` | boolean | no |  | Write movie-level metadata files. |

### Import List

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enabled` | boolean | yes |  | Whether the import list is active and will be processed. |
| `enable_auto` | boolean | yes |  | Whether items from this list are automatically added to Whisparr. |
| `monitor` | string | no |  | Monitoring strategy applied to added items. One of `movieOnly`, `movieAndScene`, `sceneOnly`, `none`. |
| `root_folder_path` | string | no |  | Root folder where items added by this list are placed. |
| `quality_profile_id` | integer | yes |  | Quality profile assigned to items added by this list. References a [`quality_profile`](#quality-profile) by name (`${ref.quality_profile.<key>}`). |
| `search_on_add` | boolean | yes |  | Whether to trigger an immediate search when an item is added. |
| `list_order` | integer | yes |  | Display sort order of this list in the UI. |

Set `implementation` to one of: [`RSSImport`](#import-list-rss) / [`StashDBFavoriteImport`](#import-list-stashdbfavorite) / [`StashDBPerformerImport`](#import-list-stashdbperformer) / [`StashDBStudioImport`](#import-list-stashdbstudio) / [`StashDBTagsImport`](#import-list-stashdbtags) / [`TMDbCompanyImport`](#import-list-tmdbcompany) / [`TMDbKeywordImport`](#import-list-tmdbkeyword) / [`TMDbListImport`](#import-list-tmdblist) / [`TMDbPersonImport`](#import-list-tmdbperson) / [`TMDbPopularImport`](#import-list-tmdbpopular) / [`TMDbUserImport`](#import-list-tmdbuser) / [`WhisparrImport`](#import-list-whisparr).

#### Import List: Rss

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `link` | string | no |  | URL of the RSS feed to import from. |

#### Import List: StashDbFavorite

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | no |  | StashDB API key. Required; the API returns it masked once saved. Credential — redacted in plan output. |
| `limit` | integer | no |  | Maximum number of scenes to fetch per sync. Must be greater than 0; StashDB caps a page at 100. |
| `sort` | string | no |  | Sort order for the fetched scenes (always descending). One of `released`, `created`, `trending`. |
| `after_date` | string | no |  | Only import scenes released on or after this date (`YYYY-MM-DD`). |
| `filter` | string | no |  | Which kind of favorited entity to follow. One of `all`, `performer`, `studio`. |
| `stash_tags` | string | no |  | Tag StashIDs to filter on, comma-separated. Optional. |
| `tags_filter` | string | no |  | How `tags` is applied. One of `includes`, `excludes`. |

#### Import List: StashDbPerformer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | no |  | StashDB API key. Required; the API returns it masked once saved. Credential — redacted in plan output. |
| `limit` | integer | no |  | Maximum number of scenes to fetch per sync. Must be greater than 0; StashDB caps a page at 100. |
| `sort` | string | no |  | Sort order for the fetched scenes (always descending). One of `released`, `created`, `trending`. |
| `after_date` | string | no |  | Only import scenes released on or after this date (`YYYY-MM-DD`). |
| `performers` | string | no |  | Performer StashIDs to follow, comma-separated. Required. |
| `studios` | string | no |  | Studio StashIDs to filter on, comma-separated. Optional. |
| `studios_filter` | string | no |  | How `studios` is applied. One of `includes`, `excludes`. |
| `stash_tags` | string | no |  | Tag StashIDs to filter on, comma-separated. Optional. |
| `tags_filter` | string | no |  | How `tags` is applied. One of `includes`, `excludes`. |
| `only_favorite_studios` | boolean | no |  | Restrict results to studios the authenticated account has favorited. |

#### Import List: StashDbStudio

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | no |  | StashDB API key. Required; the API returns it masked once saved. Credential — redacted in plan output. |
| `limit` | integer | no |  | Maximum number of scenes to fetch per sync. Must be greater than 0; StashDB caps a page at 100. |
| `sort` | string | no |  | Sort order for the fetched scenes (always descending). One of `released`, `created`, `trending`. |
| `after_date` | string | no |  | Only import scenes released on or after this date (`YYYY-MM-DD`). |
| `studios` | string | no |  | Studio StashIDs to follow, comma-separated. Required. |
| `stash_tags` | string | no |  | Tag StashIDs to filter on, comma-separated. Optional. |
| `tags_filter` | string | no |  | How `tags` is applied. One of `includes`, `excludes`. |
| `only_favorite_performers` | boolean | no |  | Restrict results to performers the authenticated account has favorited. |

#### Import List: StashDbTags

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | no |  | StashDB API key. Required; the API returns it masked once saved. Credential — redacted in plan output. |
| `limit` | integer | no |  | Maximum number of scenes to fetch per sync. Must be greater than 0; StashDB caps a page at 100. |
| `sort` | string | no |  | Sort order for the fetched scenes (always descending). One of `released`, `created`, `trending`. |
| `after_date` | string | no |  | Only import scenes released on or after this date (`YYYY-MM-DD`). |
| `stash_tags` | string | no |  | Tag StashIDs to follow, comma-separated. |
| `tags_filter` | string | no |  | How `tags` is applied. One of `includes`, `excludes`. |

#### Import List: TmdbCompany

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `company_id` | string | no |  | TMDb company identifier whose productions are imported. |

#### Import List: TmdbKeyword

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `keyword_id` | string | no |  | TMDb keyword identifier used to filter titles. |

#### Import List: TmdbList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_id` | string | no |  | TMDb list identifier to import titles from. |

#### Import List: TmdbPerson

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `person_id` | string | no |  | TMDb person identifier whose associated titles are imported. |
| `person_cast` | boolean | no |  | Include titles where the person appears as a cast member. |
| `person_cast_director` | boolean | no |  | Include titles where the person is credited as director. |
| `person_cast_producer` | boolean | no |  | Include titles where the person is credited as producer. |
| `person_cast_sound` | boolean | no |  | Include titles where the person has a sound department credit. |
| `person_cast_writing` | boolean | no |  | Include titles where the person is credited as a writer. |

#### Import List: TmdbPopular

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tmdb_list_type` | integer | no |  | Category of popular titles to import. Integer enum: 1 = In Theaters, 2 = Popular, 3 = Top Rated, 4 = Upcoming.  The wire name derives from the C# property `TMDbListType` — only the first character is lower-cased, giving `tMDbListType`. |
| `min_vote_average` | string | no |  | `filterCriteria.minVoteAverage` — minimum TMDb vote average (0.0–10.0). |
| `min_votes` | string | no |  | `filterCriteria.minVotes` — minimum number of TMDb votes. |
| `certification` | string | no |  | `filterCriteria.certification` — single certification filter (`NR`, `G`, `PG`, `PG-13`, `R`, `NC-17`). |
| `include_genre_ids` | string | no |  | `filterCriteria.includeGenreIds` — TMDb genre ids to include, comma- or pipe-separated. |
| `exclude_genre_ids` | string | no |  | `filterCriteria.excludeGenreIds` — TMDb genre ids to exclude, comma- or pipe-separated. |
| `include_company_ids` | string | no |  | `filterCriteria.includeCompanyIds` — TMDb company ids to include, comma- or pipe-separated. |
| `exclude_company_ids` | string | no |  | `filterCriteria.excludeCompanyIds` — TMDb company ids to exclude, comma- or pipe-separated. |
| `language_code` | integer | no |  | `filterCriteria.languageCode` — original-language filter (integer enum corresponding to a TMDb language code). |

#### Import List: TmdbUser

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `account_id` | string | no |  | TMDb account identifier for the target user. |
| `access_token` | secret string | no |  | TMDb v4 read access token for the user's account. Credential — redacted in plan output. |
| `list_type` | integer | no |  | Type of user list to import. Integer enum: 1 = Watchlist, 2 = Recommendations, 3 = Rated, 4 = Favorite. |
| `sign_in` | string | no |  | OAuth handshake slot the UI's "Authenticate with TMDB" button writes. Normally left unset in config — the account id and access token above are what the list actually authenticates with. |

#### Import List: Whisparr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the source Whisparr instance, e.g. `http://localhost:6969`. |
| `api_key` | secret string | no |  | API key of the source Whisparr instance. Credential — redacted in plan output. |
| `profile_ids` | array of integer | no |  | Only import items whose quality profile on the source instance is one of these ids. Empty imports every profile. |
| `tag_ids` | array of integer | no |  | Only import items carrying one of these tag ids on the source instance. Empty imports regardless of tags. |
| `root_folder_paths` | array of string | no |  | Only import items stored under one of these root folder paths on the source instance. Empty imports every root folder. |

### Root Folder

A root folder Whisparr watches for movies.

The API exposes GET/POST on the collection and GET/DELETE on `/{id}` —
there is no PUT, matching the (unlisted-in-schema) live endpoint set. All
mutable fields below are server-reported and read-only, so there is
nothing to update in place regardless.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `path` | string | yes |  | Natural key — the absolute filesystem path. |

### Import List Exclusion

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `foreign_id` | string | yes |  | Natural key — the foreign (StashDB/ThePornDB-style) id of the excluded item. |
| `exclusion_type` | string | no |  | Kind of item excluded: `scene`, `movie`, `studio`, `performer`, or `tag`. |
| `movie_title` | string | yes |  | Title of the excluded item. **Required** — eros validates `RuleFor(c => c.MovieTitle).NotEmpty()` (`ImportListExclusionController.cs:43`) on POST *and* PUT (`RestController.cs:72`), so omitting it 400s on every write. |
| `movie_year` | integer | yes |  | Release year of the excluded item. **Required and must be > 0** — eros validates `RuleFor(c => c.MovieYear).GreaterThan(0)` (`ImportListExclusionController.cs:44`) on POST *and* PUT. |

### Auto Tag

Automatic tagging rule — applies tags to movies (scenes) matching its specifications.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — the rule name referenced in `${ref.auto_tag.<name>}`. |
| `remove_tags_automatically` | boolean | yes |  | When `true`, tags added by this rule are removed if the movie no longer matches its specifications. |
| `tags` | array of integer | no |  | Tag ids applied when the specifications match. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `specifications` | array of any | no |  | Specification conditions (dynamic fields blob — stored as opaque JSON). |

### Media Management

`/api/v3/config/mediamanagement` — file handling + media management config.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `auto_unmonitor_previously_downloaded_movies` | boolean | yes |  | Automatically unmonitors a movie after its file has been downloaded. |
| `recycle_bin` | string | no |  | Path to the recycle bin folder for deleted movie files; empty disables the recycle bin. |
| `recycle_bin_cleanup_days` | integer | no | `7` | Number of days before files in the recycle bin are permanently deleted; 0 disables automatic cleanup. |
| `download_propers_and_repacks` | string | no | `doNotPrefer` | Whether to download proper/repack releases: `preferAndUpgrade`, `doNotUpgrade`, or `doNotPrefer`. |
| `create_empty_movie_folders` | boolean | yes |  | Creates a folder for a movie even before its file has been downloaded. |
| `delete_empty_folders` | boolean | yes |  | Removes empty movie folders after a file is deleted or moved. |
| `file_date` | string | no | `none` | Sets the file modification date to the movie release date: `none`, `cinemas`, or `release`. |
| `rescan_after_refresh` | string | no | `always` | When to rescan the movie folder after a library refresh: `always`, `afterManual`, or `never`. |
| `auto_rename_folders` | boolean | yes |  | Automatically renames movie folders when the movie title or year changes. |
| `paths_default_static` | boolean | yes |  | Makes movie root folder paths non-editable in the UI. |
| `set_permissions_linux` | boolean | yes |  | Sets file and folder permissions on imported files (Linux/macOS only). |
| `chmod_folder` | string | no |  | Octal permission bits applied to imported movie folders (e.g. `755`); requires `set_permissions_linux`. |
| `chown_group` | string | no |  | Group name or GID to chown imported files and folders to; requires `set_permissions_linux`. |
| `skip_free_space_check_when_importing` | boolean | yes |  | Skips the available disk space check before importing a movie file. |
| `minimum_free_space_when_importing` | integer | no | `100` | Minimum free disk space in MB required on the destination before Whisparr will import. |
| `copy_using_hardlinks` | boolean | no | `true` | Uses hardlinks instead of copying when source and destination are on the same filesystem. |
| `use_script_import` | boolean | yes |  | Delegates file import handling to an external script instead of the built-in importer. |
| `script_import_path` | string | no |  | Absolute path to the script used for custom imports; required when `use_script_import` is true. |
| `import_extra_files` | boolean | yes |  | Imports extra files (subtitles, NFO, etc.) alongside the movie file. |
| `extra_file_extensions` | string | no |  | Comma-separated list of file extensions to import alongside the movie file (e.g. `srt,nfo`). |
| `enable_media_info` | boolean | no | `true` | Reads and stores media info (codec, resolution, audio channels) for imported files. |
| `whisparr_folder_limit` | integer | no | `100` | "Import File Limit" — caps how many files a *manual* import considers within one folder (`ManualImportService.cs` truncates the candidate list to this many). Nothing to do with folder creation. |

### Naming

`/api/v3/config/naming` — movie and scene file/folder naming configuration.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `rename_movies` | boolean | yes |  | Renames existing movie files to match the configured naming format on import or refresh. |
| `rename_scenes` | boolean | yes |  | Renames existing scene files to match the configured scene naming format on import or refresh. |
| `replace_illegal_characters` | boolean | no | `true` | Replaces characters that are illegal on common filesystems in file and folder names. |
| `colon_replacement_format` | string | no | `delete` | How to handle colons in movie titles: `delete`, `dash`, `spaceDash`, `spaceDashSpace`, or `smart`. |
| `standard_movie_format` | string | no |  | Naming template string for movie files; uses Whisparr naming tokens (e.g. `{Movie Title}`). |
| `movie_folder_format` | string | no |  | Naming template string for movie folders; uses Whisparr naming tokens. |
| `standard_scene_format` | string | no |  | Naming template string for scene files; uses Whisparr scene naming tokens. |
| `scene_folder_format` | string | no |  | Naming template string for scene folders; uses Whisparr scene naming tokens. |
| `scene_import_folder_format` | string | no |  | Naming template string for the folder scenes are imported into. |
| `max_folder_path_length` | integer | yes |  | Maximum length in characters allowed for a generated folder path. |
| `max_file_path_length` | integer | yes |  | Maximum length in characters allowed for a generated file path. |

### Ui Config

`/api/v3/config/ui` — UI display and localisation settings.

Unlike Radarr, eros has no `movieInfoLanguage` setting — scene/performer
metadata language follows the library's configured language elsewhere.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `first_day_of_week` | integer | yes |  | Day the calendar week starts on: 0 = Sunday, 1 = Monday. |
| `calendar_week_column_header` | string | no |  | Format string for the column header in the calendar week view (e.g. `ddd M/D`). |
| `movie_runtime_format` | string | no |  | How movie runtimes are displayed in the UI: `hoursMinutes` or `minutes`. |
| `short_date_format` | string | no |  | Short date format string used throughout the UI (e.g. `MMM D YYYY`). |
| `long_date_format` | string | no |  | Long date format string used in detail views (e.g. `dddd, MMMM D YYYY`). |
| `time_format` | string | no |  | Time format string used in the UI: e.g. `h(:mm)a` (12-hour) or `HH:mm` (24-hour). |
| `show_relative_dates` | boolean | yes |  | Displays dates as relative time (e.g. "2 days ago") rather than absolute dates. |
| `enable_color_impaired_mode` | boolean | yes |  | Enables a colour-blind-friendly UI mode with adjusted colour palettes. |
| `ui_language` | integer | yes |  | Language ID for the Whisparr UI interface itself. |
| `theme` | string | no |  | UI colour theme name (e.g. `dark`, `light`, `auto`). |

### Indexer Config

`/api/v3/config/indexer` — global indexer, RSS sync, and eros search-tuning
settings. eros indexers can search by studio code/title/date in addition to
the standard release title search, and the extra fields here tune that.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `minimum_age` | integer | yes |  | Minimum age in minutes a Usenet release must be before Whisparr will grab it. |
| `maximum_size` | integer | yes |  | Maximum release size in MB that Whisparr will grab; 0 = unlimited. |
| `retention` | integer | yes |  | Usenet retention period in days; 0 = unlimited. |
| `rss_sync_interval` | integer | no | `60` | Interval in minutes between RSS feed syncs; 0 = disable RSS sync. |
| `prefer_indexer_flags` | boolean | yes |  | Prefers releases flagged by indexers (e.g. freeleech on torrents) when scoring candidates. |
| `availability_delay` | integer | yes |  | Number of days before (`-`) or after (`+`) a movie's availability date to start searching. |
| `allow_hardcoded_subs` | boolean | yes |  | Allows grabbing releases that contain hardcoded (burned-in) subtitles. |
| `whitelisted_hardcoded_subs` | string | no |  | Comma-separated list of subtitle language codes whose hardcoded releases are permitted. |
| `search_studio_code` | boolean | yes |  | Includes the studio's release code (e.g. scene id) in indexer searches. |
| `search_title_only` | boolean | yes |  | Restricts indexer searches to the release title only, skipping studio/date variants. |
| `search_title_date` | boolean | yes |  | Includes the release date alongside the title in indexer searches. |
| `search_studio_date` | boolean | yes |  | Includes the release date alongside the studio name in indexer searches. |
| `search_studio_title` | boolean | yes |  | Includes the studio name alongside the title in indexer searches. |
| `search_date_format` | string | yes |  | Date format used when building date-based search queries: `yymmdd`, `ddmmyyyy`, or `both`. |
| `search_studio_format` | string | yes |  | Studio name format used when building studio-based search queries: `original`, `clean`, or `both`. |

### Download Client Config

`/api/v3/config/downloadclient` — download client handling settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `download_client_working_folders` | string | no |  | Pipe-separated list of category or folder names that download clients use for in-progress downloads (e.g. `_UNPACK_|_FAILED_`). |
| `enable_completed_download_handling` | boolean | no | `true` | Automatically imports completed downloads from the download client. |
| `check_for_finished_download_interval` | integer | no | `1` | Interval in minutes between checks for finished downloads when completed download handling is enabled. |
| `auto_redownload_failed` | boolean | no | `true` | Automatically searches for a replacement release when a download fails. |
| `auto_redownload_failed_from_interactive_search` | boolean | no | `true` | Automatically re-downloads a failed release that was found via interactive search. |

### Import List Config

`/api/v3/config/importlist` — import list sync level configuration.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_sync_level` | string | no |  | Action taken when a movie is removed from all import lists: e.g. `disabled`, `logOnly`, `removeAndKeep`, or `removeAndDelete`. |

### Host Config

`/api/v3/config/host` — Whisparr host, network, authentication, proxy, and
backup settings, plus eros-specific caching / validation / matching
behaviour (`whisparr*` fields below).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bind_address` | string | no |  | IP address or hostname Whisparr binds to; `*` binds to all interfaces. |
| `port` | integer | yes |  | HTTP port Whisparr listens on. |
| `ssl_port` | integer | yes |  | HTTPS port Whisparr listens on when SSL is enabled. |
| `enable_ssl` | boolean | yes |  | Enables HTTPS/TLS for the Whisparr web UI. |
| `launch_browser` | boolean | yes |  | Opens the Whisparr web UI in the default browser on startup. |
| `authentication_method` | string | no | `none` | Authentication method for the Whisparr web UI: `none`, `basic`, `forms`, or `external`. |
| `authentication_required` | string | no | `enabled` | Whether authentication is required: `enabled` or `disabledForLocalAddresses`. |
| `analytics_enabled` | boolean | yes |  | Sends anonymised usage and error data to the Whisparr team. |
| `username` | string | no |  | Username for basic or forms authentication. |
| `password` | secret string | no |  | Password for basic or forms authentication. Credential — redacted in plan output. |
| `password_confirmation` | secret string | no |  | Password confirmation field; must match `password` when changing credentials. Credential — redacted in plan output. |
| `log_level` | string | no |  | Log verbosity level (e.g. `info`, `debug`, `trace`). |
| `log_size_limit` | integer | yes |  | Maximum size in MB for each log file before it is rotated. |
| `console_log_level` | string | no |  | Log level for console output; overrides `log_level` for stdout. |
| `branch` | string | no |  | Update channel or branch Whisparr checks for updates (e.g. `main`, `develop`). |
| `api_key` | secret string | no |  | Whisparr API key used to authenticate API requests. Credential — redacted in plan output. |
| `ssl_cert_path` | string | no |  | Absolute path to the SSL certificate file (PEM/PFX). |
| `ssl_cert_password` | secret string | no |  | Password for the SSL certificate if it is password-protected. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path for reverse-proxy deployments (e.g. `/whisparr`). |
| `instance_name` | string | no |  | Display name for this Whisparr instance shown in the browser title and notifications. |
| `application_url` | string | no |  | Externally reachable URL for this instance, used in notifications. |
| `update_automatically` | boolean | yes |  | Allows Whisparr to update itself automatically when a new version is available. |
| `update_mechanism` | string | no | `docker` | How Whisparr applies updates: `builtIn`, `script`, `external`, `apt`, or `docker`. |
| `update_script_path` | string | no |  | Absolute path to the update script; required when `update_mechanism` is `script`. |
| `proxy_enabled` | boolean | yes |  | Routes Whisparr's outbound HTTP traffic through a proxy server. |
| `proxy_type` | string | no | `http` | Proxy protocol: `http`, `socks4`, or `socks5`. |
| `proxy_hostname` | string | no |  | Hostname or IP address of the proxy server. |
| `proxy_port` | integer | yes |  | Port of the proxy server. |
| `proxy_username` | string | no |  | Username for proxy authentication. |
| `proxy_password` | secret string | no |  | Password for proxy authentication. Credential — redacted in plan output. |
| `proxy_bypass_filter` | string | no |  | Comma-separated list of hosts or IP ranges that bypass the proxy. |
| `proxy_bypass_local_addresses` | boolean | yes |  | Bypasses the proxy for connections to local/private addresses. |
| `certificate_validation` | string | no | `enabled` | TLS certificate validation mode: `enabled`, `disabledForLocalAddresses`, or `disabled`. |
| `backup_folder` | string | no |  | Folder path where Whisparr stores automatic database backups. |
| `backup_interval` | integer | yes |  | Interval in days between automatic backups. |
| `backup_retention` | integer | yes |  | Number of days to retain automatic backups before they are deleted. |
| `trust_cgnat_ip_addresses` | boolean | yes |  | Trusts Carrier-Grade NAT (CGNAT) IP address ranges for source IP determination. |
| `whisparr_always_exclude_performers_tag` | string | no |  | Name of the tag added to a scene left unmonitored because of its performer. When empty, an import-list exclusion is created instead. |
| `whisparr_always_exclude_studios_tag` | string | no |  | Name of the tag added to a scene left unmonitored because of its studio. When empty, an import-list exclusion is created instead. |
| `whisparr_always_exclude_studios_after_tag` | string | no |  | Name of the tag added to a scene left unmonitored because of its studio's after-date rule. When empty, an import-list exclusion is created instead. |
| `whisparr_always_exclude_tags_tag` | string | no |  | Name of the tag added to a scene left unmonitored because of its tags. When empty, an import-list exclusion is created instead. |
| `whisparr_auto_match_on_date` | boolean | yes |  | Accept a scene on studio and date alone. **Can cause false positives.** |
| `whisparr_cache_exclusion_api` | boolean | yes |  | Caches responses from the exclusion metadata API to reduce repeat lookups. |
| `whisparr_cache_movie_api` | boolean | yes |  | Caches responses from the movie metadata API to reduce repeat lookups. |
| `whisparr_cache_performer_api` | boolean | yes |  | Caches responses from the performer metadata API to reduce repeat lookups. |
| `whisparr_cache_studio_api` | boolean | yes |  | Caches responses from the studio metadata API to reduce repeat lookups. |
| `whisparr_corrupt_file_detection` | boolean | yes |  | Check the file on import; if it is not found to be valid it fails to import automatically (it is not re-downloaded). |
| `whisparr_movie_metadata_source` | string | yes |  | Preferred source for movie metadata: `none`, `tmdb`, or `tpdb` (ThePornDB). |
| `whisparr_validate_runtime` | boolean | yes |  | Extract the runtime from the file and validate it against the scene's expected duration. |
| `whisparr_validate_runtime_limit` | integer | yes |  | Runtime tolerance in minutes used by `whisparr_validate_runtime` when comparing the file against the scene's expected duration. |

## Types

### Quality Profile Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  |  |
| `name` | string | no |  | Group or quality tier label displayed in the UI, e.g. `"HD Web"`. |
| `quality` | [`quality`](#quality) | no |  | Quality definition for leaf items; `None` for group items. |
| `items` | array of [`quality_profile_item`](#quality-profile-item) | no |  | Nested group members — empty for leaf items. |
| `allowed` | boolean | yes |  | When `true`, Whisparr will accept releases at this quality tier. |

### Profile Format Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  |  |
| `format` | integer | yes |  | Custom-format id — resolved from `${ref.custom_format.<name>}` at apply. References a [`custom_format`](#custom-format) by name (`${ref.custom_format.<key>}`). |
| `name` | string | no |  | Custom-format name, mirrored from the format definition. |
| `score` | integer | no |  | Points awarded to a release matching this format; negative values penalise. |

### Language

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | yes |  |  |
| `name` | string | no |  | Language name, e.g. `"English"`. |

### Download Protocol

Allowed values: `usenet` / `torrent`.

### Quality

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  |  |
| `name` | string | no |  | Quality tier name, e.g. `"WEBDL-1080p"`. |
| `source` | string | no |  | Source medium string, e.g. `"web"`, `"bluray"`, `"dvd"`. |
| `resolution` | integer | no |  | Vertical pixel resolution for this quality tier, e.g. `1080`. |

