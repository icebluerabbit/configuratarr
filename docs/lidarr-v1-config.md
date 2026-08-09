# Lidarr v1 Configuration

Lidarr v1 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Tag

A label applied to artists, albums, import lists, indexers, download clients,
notifications, etc.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the name referenced in `${ref.tag.<label>}`. |

### Custom Format

A custom format — a named collection of specification conditions Lidarr uses
to score releases. The score influences download decisions via quality profiles.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.custom_format.<name>}`. |
| `include_custom_format_when_renaming` | boolean | no |  | When true, the format name is included in Lidarr's file rename template. |
| `specifications` | array of any | no |  | Specification conditions, each a provider-shaped object (`implementation` + `fields[]`). Raw JSON — see the module docs. |

### Custom Filter

A saved custom filter for a Lidarr UI page.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the user-visible label for this filter. |
| `filter_type` | string | no |  | The UI page context this filter applies to (e.g. `ArtistIndex`, `AlbumFile`). Wire name is `type` (a Rust keyword). |
| `filters` | array of any | no |  | Filter conditions, each a raw object with `key`, `value`, and `type`. Raw JSON — the condition shape is not described in the static spec. |

### Quality Profile

Named quality profile — ordered quality ladder with format-score gates.

Lidarr evaluates profiles top-to-bottom: the first `allowed` quality (or
group) that a release matches determines whether it is grabbed and whether an
upgrade is triggered. Custom-format scores further filter grabs and
upgrades via `min_format_score` and `cutoff_format_score`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.quality_profile.<name>}`. |
| `upgrade_allowed` | boolean | yes |  | When `true`, Lidarr will seek a better-quality release after the initial download. |
| `cutoff` | integer | yes |  | Id of the cutoff quality tier; Lidarr will not seek upgrades past this point. |
| `items` | array of [`quality_profile_quality_item`](#quality-profile-quality-item) | no |  | Ordered quality ladder — all quality tiers and groups this profile considers. |
| `min_format_score` | integer | yes |  | Minimum aggregate custom-format score a release must reach to be grabbed. |
| `cutoff_format_score` | integer | yes |  | Minimum format score that satisfies the upgrade cutoff. |
| `format_items` | array of [`profile_format_item`](#profile-format-item) | no |  | Custom-format score contributions attached to this profile. |

### Metadata Profile

Named metadata profile — controls which album types and release statuses
Lidarr monitors for artists assigned this profile.

The three toggle lists (`primary_album_types`, `secondary_album_types`,
`release_statuses`) are always fully populated by the server. The user
configures only the `allowed` flag on each item. The server assigns the
type descriptors (ids and names).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.metadata_profile.<name>}`. |
| `primary_album_types` | array of [`primary_album_type_item`](#primary-album-type-item) | no |  | Toggles for each primary album type (Album, Single, EP, etc.). Each entry's `allowed` flag controls whether that type is monitored. |
| `secondary_album_types` | array of [`secondary_album_type_item`](#secondary-album-type-item) | no |  | Toggles for each secondary album type (Studio, Live, Remix, etc.). Each entry's `allowed` flag controls whether that classification is monitored. |
| `release_statuses` | array of [`release_status_item`](#release-status-item) | no |  | Toggles for each release status (Official, Promotional, Bootleg, etc.). Each entry's `allowed` flag controls whether that status is monitored. |

### Release Profile

Release profile — term-based acceptance and rejection filter for grabbed releases.

When `enabled`, Lidarr checks every candidate release title against the
`required` and `ignored` term lists before deciding to grab it:
- `required`: at least one term must appear in the release title.
- `ignored`: none of the terms may appear in the release title.

**Key note:** Lidarr's API does not include a `name` field on release profiles.
The `indexer_id` is used as the natural key — keep each value unique across
your profiles to enable reliable diff and idempotent apply.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | yes |  | When `false`, this profile is saved but not applied to any grabs. |
| `required` | any | no |  | Terms that must appear in a release title for it to be accepted. In practice a JSON array of strings; `null` means no required-term constraint. |
| `ignored` | any | no |  | Terms that must **not** appear in a release title; releases containing any of these terms are rejected. In practice a JSON array of strings; `null` means no ignored-term constraint. |
| `indexer_id` | integer | no | `0` | Natural key — id of the indexer this profile is restricted to; `0` means all indexers. Keep unique across profiles to ensure reliable diffing. |
| `tags` | array of integer | no |  | Artist tag ids this profile applies to; resolved from `${ref.tag.<label>}` at apply. An empty list means the profile is applied globally to all artists. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |

### Auto Tag

Automatic tagging rule — applies tags to artists matching its specifications.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — the rule name referenced in `${ref.auto_tag.<name>}`. |
| `remove_tags_automatically` | boolean | yes |  | When `true`, tags added by this rule are removed if the artist no longer matches its specifications. |
| `tags` | array of integer | no |  | Tag ids applied when the specifications match; resolved from `${ref.tag.<label>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `specifications` | array of any | no |  | Specification conditions (dynamic fields blob — stored as opaque JSON). |

### Remote Path Mapping

A remote-to-local path mapping for a download client host.

Lidarr uses these to translate paths returned by download clients that run
on a different host (or container) where filesystem paths differ.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Hostname or IP of the download client that uses the remote path. |
| `remote_path` | string | yes |  | Natural key — the path as the remote download client reports it. |
| `local_path` | string | yes |  | The local filesystem path that corresponds to `remote_path`. |

### Root Folder

A root folder Lidarr watches for artists.

Root folders carry default profile and monitoring settings applied to artists
added under this path. The `path` field is the natural key used to match
desired entries against live ones.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | no |  | Optional display name for the root folder. |
| `path` | string | yes |  | Natural key — the absolute filesystem path Lidarr watches. |
| `default_metadata_profile_id` | integer | yes |  | Id of the metadata profile applied by default to new artists; resolved from `${ref.metadata_profile.<name>}` at apply. References a [`metadata_profile`](#metadata-profile) by name (`${ref.metadata_profile.<key>}`). |
| `default_quality_profile_id` | integer | yes |  | Id of the quality profile applied by default to new artists; resolved from `${ref.quality_profile.<name>}` at apply. References a [`quality_profile`](#quality-profile) by name (`${ref.quality_profile.<key>}`). |
| `default_monitor_option` | [`monitor_types`](#monitor-types) | yes |  | Monitor strategy applied when adding a new artist under this root folder. |
| `default_new_item_monitor_option` | [`new_item_monitor_types`](#new-item-monitor-types) | yes |  | Monitor strategy applied when a new album is added to a monitored artist. |
| `default_tags` | array of integer | no |  | Tag ids applied by default to new artists added under this root folder; resolved from `${ref.tag.<label>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |

### Import List Exclusion

An artist excluded from Lidarr import list processing.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `foreign_id` | string | yes |  | Natural key — the MusicBrainz artist ID being excluded. Referenced as `${ref.import_list_exclusion.<foreign_id>}`. |
| `artist_name` | string | no |  | Display name of the excluded artist. |

### Quality Definition

Per-quality-tier size limits managed by Lidarr.

Quality definitions control the minimum and maximum acceptable file sizes
(in MB) for each quality tier. Only the size fields and `title` are
user-configurable; `quality` and `weight` are server-assigned and read-only.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `title` | string | yes |  | Display name for this quality tier; used as the natural key to match against live entries. |
| `min_size` | number | no |  | Minimum acceptable size in MB for releases of this quality; `null` = no minimum. |
| `max_size` | number | no |  | Maximum acceptable size in MB for releases of this quality; `null` = no maximum. |
| `preferred_size` | number | no |  | Preferred size in MB for releases of this quality; used for scoring when multiple options exist. |

### Download Client

A Lidarr download client (usenet or torrent).

Composes the shared provider envelope (id, name, tags, read-only metadata)
with a per-implementation typed fields-blob and the envelope-level flags
that apply to every client type.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable` | boolean | yes |  | Whether this download client is active. |
| `protocol` | [`download_protocol`](#download-protocol) | yes |  | Download protocol used by this client (torrent or usenet). |
| `priority` | integer | no | `1` | Client priority relative to other configured download clients. |
| `remove_completed_downloads` | boolean | no | `true` | Remove downloads from the client once Lidarr has imported them. |
| `remove_failed_downloads` | boolean | yes |  | Remove downloads from the client if they fail to complete. |

Set `implementation` to one of: [`Aria2`](#download-client-aria2) / [`Deluge`](#download-client-deluge) / [`Flood`](#download-client-flood) / [`Hadouken`](#download-client-hadouken) / [`Nzbget`](#download-client-nzbget) / [`Nzbvortex`](#download-client-nzbvortex) / [`Pneumatic`](#download-client-pneumatic) / [`QBittorrent`](#download-client-qbittorrent) / [`RTorrent`](#download-client-rtorrent) / [`Sabnzbd`](#download-client-sabnzbd) / [`TorrentBlackhole`](#download-client-torrentblackhole) / [`TorrentDownloadStation`](#download-client-torrentdownloadstation) / [`Transmission`](#download-client-transmission) / [`UsenetBlackhole`](#download-client-usenetblackhole) / [`UsenetDownloadStation`](#download-client-usenetdownloadstation) / [`UTorrent`](#download-client-utorrent) / [`Vuze`](#download-client-vuze).

#### Download Client: Aria2

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Aria2 RPC server. |
| `port` | integer | no |  | TCP port the Aria2 RPC server listens on. |
| `secret_token` | secret string | no |  | Secret token for authenticating with the Aria2 RPC interface. Credential — redacted in plan output. |
| `rpc_path` | string | no |  | Path to the Aria2 JSON-RPC endpoint (default: `/rpc`). |
| `use_ssl` | boolean | no |  | Connect to the Aria2 RPC server over HTTPS. |

#### Download Client: Deluge

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Deluge daemon. |
| `port` | integer | no |  | TCP port the Deluge web UI listens on. |
| `password` | secret string | no |  | Password for authenticating with the Deluge web UI. Credential — redacted in plan output. |
| `music_category` | string | no |  | Category (label) assigned to music downloads in Deluge. |
| `music_imported_category` | string | no |  | Category Deluge moves completed downloads to after Lidarr imports them. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `url_base` | string | no |  | URL base path if Deluge is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Deluge in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Deluge over HTTPS. |

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
| `field_tags` | array of string | no |  | Tags applied to the torrent in Flood (string labels, not Lidarr tag ids). Note: wire name is `tags`; the Lidarr API uses a field exception mapping. |
| `additional_tags` | array of integer | no |  | Additional Lidarr-managed metadata tags appended to the torrent (integer codes). |
| `post_import_tags` | array of string | no |  | Tags applied to the torrent in Flood after Lidarr imports it. |

#### Download Client: Hadouken

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Hadouken server. |
| `port` | integer | no |  | TCP port the Hadouken web UI listens on. |
| `username` | string | no |  | Username for authenticating with Hadouken. |
| `password` | secret string | no |  | Password for authenticating with Hadouken. Credential — redacted in plan output. |
| `category` | string | no |  | Category assigned to torrents in Hadouken. |
| `url_base` | string | no |  | URL base path if Hadouken is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to Hadouken over HTTPS. |

#### Download Client: Nzbget

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBGet server. |
| `port` | integer | no |  | TCP port the NZBGet web UI listens on. |
| `username` | string | no |  | Username for authenticating with NZBGet. |
| `password` | secret string | no |  | Password for authenticating with NZBGet. Credential — redacted in plan output. |
| `music_category` | string | no |  | Category assigned to music downloads in NZBGet. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `url_base` | string | no |  | URL base path if NZBGet is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add downloads to NZBGet in a paused state. |
| `use_ssl` | boolean | no |  | Connect to NZBGet over HTTPS. |

#### Download Client: Nzbvortex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBVortex server. |
| `port` | integer | no |  | TCP port the NZBVortex server listens on. |
| `api_key` | secret string | no |  | API key used to authenticate with NZBVortex. Credential — redacted in plan output. |
| `music_category` | string | no |  | Category assigned to music downloads in NZBVortex. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `url_base` | string | no |  | URL base path if NZBVortex is hosted behind a reverse proxy. |

#### Download Client: Pneumatic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Lidarr drops NZB files into for Pneumatic to pick up. |
| `strm_folder` | string | no |  | Folder Pneumatic writes `.strm` stream files to. |

#### Download Client: QBittorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the qBittorrent server. |
| `port` | integer | no |  | TCP port the qBittorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with qBittorrent. |
| `password` | secret string | no |  | Password for authenticating with qBittorrent. Credential — redacted in plan output. |
| `music_category` | string | no |  | Category assigned to music downloads in qBittorrent. |
| `music_imported_category` | string | no |  | Category qBittorrent moves completed downloads to after Lidarr imports them. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `initial_state` | integer | no |  | Initial torrent state. 0 = Start, 1 = ForceStart, 2 = Pause. |
| `url_base` | string | no |  | URL base path if qBittorrent is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to qBittorrent over HTTPS. |
| `sequential_order` | boolean | no |  | Download pieces in sequential order to enable early playback. |
| `first_and_last` | boolean | no |  | Prioritise downloading the first and last pieces of each file first. |

#### Download Client: RTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the rTorrent SCGI/HTTP endpoint. |
| `port` | integer | no |  | TCP port the rTorrent SCGI or HTTP interface listens on. |
| `username` | string | no |  | Username for authenticating with rTorrent (used when fronted by a web server). |
| `password` | secret string | no |  | Password for authenticating with rTorrent (used when fronted by a web server). Credential — redacted in plan output. |
| `music_category` | string | no |  | Label assigned to music torrents in rTorrent. |
| `music_directory` | string | no |  | Directory rTorrent saves music downloads to. |
| `music_imported_category` | string | no |  | Label rTorrent moves completed downloads to after Lidarr imports them. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
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
| `music_category` | string | no |  | Category assigned to music downloads in SABnzbd. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `url_base` | string | no |  | URL base path if SABnzbd is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to SABnzbd over HTTPS. |

#### Download Client: TorrentBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `torrent_folder` | string | no |  | Folder Lidarr drops `.torrent` files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Lidarr watches for completed downloads from the external client. |
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
| `music_category` | string | no |  | Shared folder or category assigned to music downloads. |
| `music_directory` | string | no |  | Directory Download Station saves music downloads to. |
| `use_ssl` | boolean | no |  | Connect to Synology DSM over HTTPS. |

#### Download Client: Transmission

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Transmission server. |
| `port` | integer | no |  | TCP port the Transmission RPC interface listens on. |
| `username` | string | no |  | Username for authenticating with Transmission. |
| `password` | secret string | no |  | Password for authenticating with Transmission. Credential — redacted in plan output. |
| `music_category` | string | no |  | Category (label) assigned to music downloads in Transmission. |
| `music_directory` | string | no |  | Directory Transmission saves music downloads to. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `url_base` | string | no |  | URL base path if Transmission is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Transmission in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Transmission over HTTPS. |

#### Download Client: UsenetBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Lidarr drops NZB files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Lidarr watches for completed downloads from the external client. |

#### Download Client: UsenetDownloadStation

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Synology NAS running Download Station. |
| `port` | integer | no |  | TCP port the Synology DSM web interface listens on. |
| `username` | string | no |  | Username for authenticating with Synology DSM. |
| `password` | secret string | no |  | Password for authenticating with Synology DSM. Credential — redacted in plan output. |
| `music_category` | string | no |  | Shared folder or category assigned to music downloads. |
| `music_directory` | string | no |  | Directory Download Station saves music downloads to. |
| `use_ssl` | boolean | no |  | Connect to Synology DSM over HTTPS. |

#### Download Client: UTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the uTorrent web UI. |
| `port` | integer | no |  | TCP port the uTorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with uTorrent. |
| `password` | secret string | no |  | Password for authenticating with uTorrent. Credential — redacted in plan output. |
| `music_category` | string | no |  | Category assigned to music downloads in uTorrent. |
| `music_imported_category` | string | no |  | Category uTorrent moves completed downloads to after Lidarr imports them. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `initial_state` | integer | no |  | Initial torrent state. 0 = Start, 1 = ForceStart, 2 = Pause, 3 = Stop. Note: the API field name is intentionally misspelled as "intialState". |
| `url_base` | string | no |  | URL base path if uTorrent is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to uTorrent over HTTPS. |

#### Download Client: Vuze

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Vuze remote UI server. |
| `port` | integer | no |  | TCP port the Vuze remote interface listens on. |
| `username` | string | no |  | Username for authenticating with Vuze. |
| `password` | secret string | no |  | Password for authenticating with Vuze. Credential — redacted in plan output. |
| `music_category` | string | no |  | Category (label) assigned to music downloads in Vuze. |
| `music_directory` | string | no |  | Directory Vuze saves music downloads to. |
| `recent_music_priority` | integer | no |  | Priority for releases added in the last 14 days. |
| `older_music_priority` | integer | no |  | Priority for releases older than 14 days. |
| `url_base` | string | no |  | URL base path if Vuze is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Vuze in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Vuze over HTTPS. |

### Indexer

Indexer definition — connects Lidarr to a usenet or torrent search source.

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

Set `implementation` to one of: [`FileList`](#indexer-filelist) / [`Gazelle`](#indexer-gazelle) / [`Headphones`](#indexer-headphones) / [`IPTorrents`](#indexer-iptorrents) / [`Newznab`](#indexer-newznab) / [`Nyaa`](#indexer-nyaa) / [`Redacted`](#indexer-redacted) / [`TorrentRssIndexer`](#indexer-torrentrss) / [`Torrentleech`](#indexer-torrentleech) / [`Torznab`](#indexer-torznab).

#### Indexer: FileList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the FileList tracker (optional; has a built-in default). |
| `username` | string | yes |  | FileList account username. |
| `passkey` | secret string | yes |  | FileList passkey for API authentication. Credential — redacted in plan output. |
| `categories` | array of integer | no |  | Music category IDs to include in searches. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

#### Indexer: Gazelle

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the Gazelle tracker. |
| `username` | string | yes |  | Tracker account username. |
| `password` | secret string | yes |  | Tracker account password. Credential — redacted in plan output. |
| `use_freeleech_token` | boolean | no |  | Use freeleech tokens automatically when grabbing. |
| `early_release_limit` | integer | no |  | Number of days before release date to start monitoring. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `discography_seed_time` | integer | no |  | Minimum seeding time in minutes for a full discography download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

#### Indexer: Headphones

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | yes |  | Headphones account username. |
| `password` | secret string | yes |  | Headphones account password. Credential — redacted in plan output. |
| `categories` | array of integer | no |  | Usenet category IDs to include in searches. |
| `early_release_limit` | integer | no |  | Number of days before release date to start monitoring. |

#### Indexer: IpTorrents

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | RSS feed URL including the user's passkey (provided by IPTorrents). |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

#### Indexer: Newznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the Newznab indexer. |
| `api_path` | string | no |  | URL path to the Newznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Newznab indexer. Credential — redacted in plan output. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `categories` | array of integer | no |  | Newznab category IDs to include in searches. |

#### Indexer: Nyaa

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Nyaa indexer. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

#### Indexer: Redacted

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Redacted API key for authentication. Credential — redacted in plan output. |
| `use_freeleech_token` | boolean | no |  | Use freeleech tokens automatically when grabbing. |
| `early_release_limit` | integer | no |  | Number of days before release date to start monitoring. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `discography_seed_time` | integer | no |  | Minimum seeding time in minutes for a full discography download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

#### Indexer: TorrentRss

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | URL of the torrent RSS feed. |
| `cookie` | string | no |  | Session cookie sent with RSS requests for authenticated feeds. |
| `allow_zero_size` | boolean | no |  | Allow releases that report a size of zero bytes. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

#### Indexer: TorrentLeech

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the TorrentLeech tracker (optional; has a built-in default). |
| `api_key` | secret string | yes |  | TorrentLeech API key for authentication. Credential — redacted in plan output. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `discography_seed_time` | integer | no |  | Minimum seeding time in minutes for a full discography download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

#### Indexer: Torznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Torznab indexer. |
| `api_path` | string | no |  | URL path to the Torznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Torznab indexer. Credential — redacted in plan output. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `categories` | array of integer | no |  | Torznab category IDs to include in searches. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Lidarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Lidarr must reach before stopping seeding. |

### Metadata

Metadata consumer — instructs Lidarr to write sidecar metadata files and
artwork alongside downloaded media using a specific plugin.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable` | boolean | yes |  | Whether this metadata consumer is active. |

Set `implementation` to one of: [`XbmcMetadata`](#metadata-kodi) / [`RoksboxMetadata`](#metadata-roksbox) / [`WdtvMetadata`](#metadata-wdtv).

#### Metadata: Kodi

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `artist_metadata` | boolean | no |  | Write artist-level metadata files. |
| `album_metadata` | boolean | no |  | Write album-level metadata files. |
| `artist_images` | boolean | no |  | Download and store artist-level artwork. |
| `album_images` | boolean | no |  | Download and store album-level artwork. |

#### Metadata: Roksbox

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `artist_images` | boolean | no |  | Download and store artist-level artwork. |
| `album_images` | boolean | no |  | Download and store album-level artwork. |
| `track_metadata` | boolean | no |  | Write track-level metadata files. |

#### Metadata: Wdtv

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `track_metadata` | boolean | no |  | Write track-level metadata files. |

### Notification

A Lidarr notification connection — routes artist/album/track events to external services.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `on_grab` | boolean | yes |  | Fire notification when a release is grabbed for download. |
| `on_release_import` | boolean | yes |  | Fire notification when a release is imported after download. |
| `on_upgrade` | boolean | yes |  | Fire notification when a file is upgraded to a higher-quality version. |
| `on_rename` | boolean | yes |  | Fire notification when files are renamed. |
| `on_artist_add` | boolean | yes |  | Fire notification when an artist is added to the Lidarr library. |
| `on_artist_delete` | boolean | yes |  | Fire notification when an artist is deleted from the library. |
| `on_album_delete` | boolean | yes |  | Fire notification when an album is deleted from the library. |
| `on_health_issue` | boolean | yes |  | Fire notification when a health-check issue is detected. |
| `on_health_restored` | boolean | yes |  | Fire notification when a previously detected health-check issue is resolved. |
| `on_download_failure` | boolean | yes |  | Fire notification when a download fails. |
| `on_import_failure` | boolean | yes |  | Fire notification when an import fails. |
| `on_track_retag` | boolean | yes |  | Fire notification when a track file is retagged. |
| `on_application_update` | boolean | yes |  | Fire notification when a Lidarr application update is available. |
| `include_health_warnings` | boolean | yes |  | Include health warnings (not just errors) in health-issue notifications. |

Set `implementation` to one of: [`Apprise`](#notification-apprise) / [`CustomScript`](#notification-customscript) / [`Discord`](#notification-discord) / [`Email`](#notification-email) / [`MediaBrowser`](#notification-emby) / [`Gotify`](#notification-gotify) / [`Join`](#notification-join) / [`Xbmc`](#notification-kodi) / [`Mailgun`](#notification-mailgun) / [`Notifiarr`](#notification-notifiarr) / [`Ntfy`](#notification-ntfy) / [`PlexServer`](#notification-plex) / [`Prowl`](#notification-prowl) / [`PushBullet`](#notification-pushbullet) / [`Pushover`](#notification-pushover) / [`Sendgrid`](#notification-sendgrid) / [`Signal`](#notification-signal) / [`Simplepush`](#notification-simplepush) / [`Slack`](#notification-slack) / [`Subsonic`](#notification-subsonic) / [`SynologyIndexer`](#notification-synologyindexer) / [`Telegram`](#notification-telegram) / [`Twitter`](#notification-twitter) / [`Webhook`](#notification-webhook).

#### Notification: Apprise

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | yes |  | Base URL of the Apprise API server. |
| `stateless_urls` | string | no |  | Comma-separated stateless Apprise notification URLs (e.g. `slack://…`). |
| `notification_type` | integer | no |  | Notification type/category identifier sent to Apprise. |
| `auth_username` | string | no |  | HTTP basic-auth username for the Apprise server. |
| `auth_password` | secret string | no |  | HTTP basic-auth password for the Apprise server. Credential — redacted in plan output. |
| `configuration_key` | secret string | no |  | Apprise persistent-store configuration key. Credential — redacted in plan output. |
| `field_tags` | array of string | no |  | Tag filters applied to the Apprise notification dispatch. |

#### Notification: CustomScript

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `path` | string | yes |  | Absolute path to the script to execute on notification events. |
| `arguments` | string | no |  | Arguments passed to the script on invocation. |

#### Notification: Discord

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `web_hook_url` | string | yes |  | Discord incoming webhook URL. |
| `username` | string | no |  | Display name override for the webhook bot. |
| `avatar` | string | no |  | Avatar image URL for the webhook bot. |
| `author` | string | no |  | Author name shown in the Discord embed header. |
| `grab_fields` | array of integer | no |  | Field indices included in grab-event notification embeds. |
| `import_fields` | array of integer | no |  | Field indices included in import-event notification embeds. |

#### Notification: Email

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | yes |  | SMTP server hostname or IP address. |
| `port` | integer | yes |  | SMTP server port number. |
| `require_encryption` | boolean | yes |  | Require TLS encryption for the SMTP connection. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `username` | string | no |  | SMTP authentication username. |
| `password` | secret string | no |  | SMTP authentication password. Credential — redacted in plan output. |
| `to` | array of string | no |  | Primary recipient email addresses. |
| `cc` | array of string | no |  | Carbon-copy recipient email addresses. |
| `bcc` | array of string | no |  | Blind carbon-copy recipient email addresses. |

#### Notification: Emby

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Emby server hostname or IP address. |
| `api_key` | secret string | yes |  | Emby API key for authentication. Credential — redacted in plan output. |
| `port` | integer | no |  | Emby server HTTP port. |
| `use_ssl` | boolean | no |  | Connect to Emby over HTTPS. |
| `notify` | boolean | no |  | Send an on-screen notification to Emby users on events. |
| `update_library` | boolean | no |  | Trigger an Emby library refresh after a track is imported. |

#### Notification: Gotify

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | yes |  | Gotify server URL. |
| `app_token` | secret string | yes |  | Gotify application token. Credential — redacted in plan output. |
| `priority` | integer | no |  | Notification priority level. |

#### Notification: Join

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Join API key. Credential — redacted in plan output. |
| `device_names` | string | no |  | Target device names to deliver the notification to. |
| `priority` | integer | no |  | Notification priority level. |

#### Notification: Kodi

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Kodi hostname or IP address. |
| `port` | integer | no |  | Kodi JSON-RPC HTTP port. |
| `username` | string | no |  | Kodi authentication username. |
| `password` | secret string | no |  | Kodi authentication password. Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to Kodi over HTTPS. |
| `notify` | boolean | no |  | Display an on-screen notification in Kodi on events. |
| `display_time` | integer | no |  | Duration in milliseconds to display the on-screen notification. |
| `update_library` | boolean | no |  | Trigger a Kodi music library update after a track is imported. |
| `clean_library` | boolean | no |  | Trigger a Kodi music library clean after a track file is deleted. |
| `always_update` | boolean | no |  | Always update the library on every event, not just import events. |

#### Notification: Mailgun

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Mailgun API key. Credential — redacted in plan output. |
| `sender_domain` | string | yes |  | Mailgun sending domain. |
| `from` | string | yes |  | Sender email address. |
| `recipients` | array of string | no |  | Recipient email addresses. |
| `use_eu_endpoint` | boolean | no |  | Use the Mailgun EU (European) API endpoint instead of the US endpoint. |

#### Notification: Notifiarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Notifiarr API key. Credential — redacted in plan output. |

#### Notification: Ntfy

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | yes |  | ntfy server URL (default: https://ntfy.sh). |
| `username` | string | no |  | ntfy authentication username. |
| `password` | secret string | no |  | ntfy authentication password. Credential — redacted in plan output. |
| `access_token` | secret string | no |  | ntfy access token (alternative to username/password). Credential — redacted in plan output. |
| `priority` | integer | no |  | Notification priority level (1 = min … 5 = max). |
| `click_url` | string | no |  | URL opened when the notification is clicked. |
| `topics` | array of string | no |  | ntfy topics to publish to. |
| `field_tags` | array of string | no |  | Tag emojis attached to the notification. |

#### Notification: Plex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Plex server hostname or IP address. |
| `port` | integer | no |  | Plex server port. |
| `auth_token` | secret string | yes |  | Plex authentication token (X-Plex-Token). Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to the Plex server over HTTPS. |
| `update_library` | boolean | no |  | Trigger a Plex music library refresh after a track is imported. |

#### Notification: Prowl

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Prowl API key. Credential — redacted in plan output. |
| `priority` | integer | no |  | Notification priority (-2 = very low … 2 = emergency). |

#### Notification: Pushbullet

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Pushbullet API key. Credential — redacted in plan output. |
| `sender_id` | string | no |  | Optional Pushbullet sender identity (device or channel). |
| `device_ids` | array of string | no |  | Target device IDs to send the notification to. |
| `channel_tags` | array of string | no |  | Pushbullet channel tags to publish the notification to. |

#### Notification: Pushover

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Pushover application API token. Credential — redacted in plan output. |
| `user_key` | secret string | yes |  | Pushover user or group key identifying the recipient. Credential — redacted in plan output. |
| `priority` | integer | no |  | Notification priority (-2 = lowest … 2 = emergency). |
| `sound` | string | no |  | Notification sound name played on the device. |
| `devices` | array of string | no |  | Target device names; leave empty to send to all registered devices. |
| `retry` | integer | no |  | Retry interval in seconds for emergency-priority notifications. |
| `expire` | integer | no |  | Expiration time in seconds after which emergency retries stop. |

#### Notification: Sendgrid

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | SendGrid API key. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address. |
| `recipients` | array of string | no |  | Recipient email addresses. |

#### Notification: Signal

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Hostname or IP address of the signal-cli REST API host. |
| `port` | integer | no |  | HTTP port of the signal-cli REST API. |
| `sender_number` | secret string | yes |  | Phone number registered in signal-cli that sends messages. Credential — redacted in plan output. |
| `receiver_id` | string | yes |  | Phone number or group ID that receives the notification messages. |
| `use_ssl` | boolean | no |  | Connect to the signal-cli REST API over HTTPS. |
| `auth_username` | string | no |  | HTTP basic-auth username for the signal-cli REST API. |
| `auth_password` | secret string | no |  | HTTP basic-auth password for the signal-cli REST API. Credential — redacted in plan output. |

#### Notification: Simplepush

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `key` | secret string | yes |  | Simplepush key (device identifier). Credential — redacted in plan output. |
| `event` | string | no |  | Custom event name for filtering notifications in the Simplepush app. |

#### Notification: Slack

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `web_hook_url` | string | yes |  | Slack incoming webhook URL. |
| `username` | string | yes |  | Display name for the webhook bot. |
| `icon` | string | no |  | Emoji name or image URL to use as the bot's icon (e.g. `:ghost:`). |
| `channel` | string | no |  | Slack channel to post to, overriding the webhook's default channel. |

#### Notification: Subsonic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Subsonic server hostname or IP address. |
| `port` | integer | no |  | Subsonic server port. |
| `username` | string | yes |  | Subsonic authentication username. |
| `password` | secret string | yes |  | Subsonic authentication password. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path for the Subsonic server (e.g. `/subsonic`). |
| `use_ssl` | boolean | no |  | Connect to the Subsonic server over HTTPS. |
| `notify` | boolean | no |  | Send an on-screen notification to the Subsonic server on events. |
| `update_library` | boolean | no |  | Trigger a Subsonic library update after a track is imported. |

#### Notification: SynologyIndexer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `update_library` | boolean | no |  | Trigger a Synology library update after a track is imported. |

#### Notification: Telegram

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bot_token` | secret string | yes |  | Telegram bot token issued by BotFather. Credential — redacted in plan output. |
| `chat_id` | string | yes |  | Target chat, group, or channel ID to send messages to. |
| `send_silently` | boolean | no |  | Send the notification silently (no sound or alert on the recipient's device). |

#### Notification: Twitter

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `consumer_key` | secret string | yes |  | Twitter application consumer key (API key). Credential — redacted in plan output. |
| `consumer_secret` | secret string | yes |  | Twitter application consumer secret (API secret). Credential — redacted in plan output. |
| `access_token` | secret string | yes |  | Twitter user OAuth access token. Credential — redacted in plan output. |
| `access_token_secret` | secret string | yes |  | Twitter user OAuth access token secret. Credential — redacted in plan output. |
| `mention` | string | no |  | Twitter username to mention in the notification tweet. |
| `direct_message` | boolean | no |  | Send the notification as a direct message rather than a public tweet. |

#### Notification: Webhook

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `url` | string | yes |  | Webhook endpoint URL that receives the HTTP request. |
| `method` | integer | yes |  | HTTP method to use: 1 = POST, 2 = PUT. |
| `username` | string | no |  | HTTP basic-auth username sent with the request. |
| `password` | secret string | no |  | HTTP basic-auth password sent with the request. Credential — redacted in plan output. |

### Import List

An import list syncs artists and albums from an external source into Lidarr's library.

Each import list pairs a shared envelope (identity, monitoring preferences,
root folder, quality profile, metadata profile) with a typed per-implementation
settings blob (the `config` field).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable_automatic_add` | boolean | yes |  | Whether Lidarr automatically adds artists found on this list. |
| `should_monitor_existing` | boolean | yes |  | Whether to monitor items already present in Lidarr's library when the list is refreshed. |
| `should_search` | boolean | yes |  | Whether to search for missing tracks of artists added by this list. |
| `should_monitor` | string | no |  | What to monitor when an artist is added from this list (`"none"`, `"specificAlbum"`, or `"entireArtist"`). |
| `monitor_new_items` | string | no |  | How newly released albums are monitored after the artist is added (`"all"`, `"none"`, or `"new"`). |
| `root_folder_path` | string | no |  | Root folder where artists added by this list are placed. |
| `quality_profile_id` | integer | yes |  | Quality profile assigned to artists added by this list. References a [`quality_profile`](#quality-profile) by name (`${ref.quality_profile.<key>}`). |
| `metadata_profile_id` | integer | yes |  | Metadata profile assigned to artists added by this list. References a [`metadata_profile`](#metadata-profile) by name (`${ref.metadata_profile.<key>}`). |
| `list_order` | integer | yes |  | Display sort order of this list in the UI. |

Set `implementation` to one of: [`HeadphonesImport`](#import-list-headphones) / [`LastFmTag`](#import-list-lastfmtag) / [`LastFMUser`](#import-list-lastfmuser) / [`LidarrImport`](#import-list-lidarrimport) / [`LidarrLists`](#import-list-lidarrlists) / [`MusicBrainzSeries`](#import-list-musicbrainz) / [`SpotifySavedAlbums`](#import-list-spotifyalbums) / [`SpotifyFollowedArtists`](#import-list-spotifyartists) / [`SpotifyPlaylist`](#import-list-spotifyplaylists).

#### Import List: Headphones

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the Headphones instance (e.g. `"http://headphones:8181"`). |
| `api_key` | secret string | no |  | API key for authenticating with the Headphones instance. Credential — redacted in plan output. |

#### Import List: LastFmTag

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tag_id` | string | no |  | Last.fm tag identifier to pull artists from. |
| `count` | integer | no |  | Number of artists to pull from the tag list. |

#### Import List: LastFmUser

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `user_id` | string | no |  | Last.fm username whose library to import. |
| `count` | integer | no |  | Number of artists to pull from the user's library. |

#### Import List: LidarrImport

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the source Lidarr instance (e.g. `"http://lidarr:8686"`). |
| `api_key` | secret string | no |  | API key for authenticating with the source Lidarr instance. Credential — redacted in plan output. |
| `profile_ids` | array of integer | no |  | Quality profile IDs to filter by on the source Lidarr instance. |
| `tag_ids` | array of integer | no |  | Tag IDs to filter by on the source Lidarr instance. |

#### Import List: LidarrLists

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_id` | string | no |  | Identifier of the Lidarr Lists list to import from. |

#### Import List: MusicBrainz

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `series_id` | string | no |  | MusicBrainz series MBID to import artists from. |

#### Import List: SpotifyAlbums

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for the Spotify user session. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | Expiry timestamp of the current access token. |

#### Import List: SpotifyArtists

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for the Spotify user session. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | Expiry timestamp of the current access token. |

#### Import List: SpotifyPlaylists

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for the Spotify user session. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | Expiry timestamp of the current access token. |
| `playlist_ids` | array of string | no |  | Spotify playlist IDs to import artists from. |

### Media Management

`/api/v1/config/mediamanagement` — file handling and media management settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `auto_unmonitor_previously_downloaded_tracks` | boolean | yes |  | Automatically unmonitors a track after its file has been downloaded. |
| `recycle_bin` | string | no |  | Path to the recycle bin folder for deleted track files; empty disables the recycle bin. |
| `recycle_bin_cleanup_days` | integer | no | `7` | Number of days before files in the recycle bin are permanently deleted; 0 disables automatic cleanup. |
| `download_propers_and_repacks` | string | no | `doNotPrefer` | Whether to download proper/repack releases: `preferAndUpgrade`, `doNotUpgrade`, or `doNotPrefer`. |
| `create_empty_artist_folders` | boolean | yes |  | Creates a folder for an artist even before any file has been downloaded. |
| `delete_empty_folders` | boolean | yes |  | Removes empty artist/album folders after a file is deleted or moved. |
| `file_date` | string | no | `none` | Sets the file modification date to the album release date: `none` or `albumReleaseDate`. |
| `watch_library_for_changes` | boolean | yes |  | Re-scans the artist folder for changes on disk: `always`, `afterManual`, or `never`. |
| `rescan_after_refresh` | string | no | `always` | When to rescan the artist folder after a library refresh: `always`, `afterManual`, or `never`. |
| `allow_fingerprinting` | string | no | `never` | Controls audio fingerprinting for track matching: `never`, `newFiles`, or `allFiles`. |
| `set_permissions_linux` | boolean | yes |  | Sets file and folder permissions on imported files (Linux/macOS only). |
| `chmod_folder` | string | no |  | Octal permission bits applied to imported track folders (e.g. `755`); requires `set_permissions_linux`. |
| `chown_group` | string | no |  | Group name or GID to chown imported files and folders to; requires `set_permissions_linux`. |
| `skip_free_space_check_when_importing` | boolean | yes |  | Skips the available disk space check before importing a track file. |
| `minimum_free_space_when_importing` | integer | no | `100` | Minimum free disk space in MB required on the destination before Lidarr will import. |
| `copy_using_hardlinks` | boolean | no | `true` | Uses hardlinks instead of copying when source and destination are on the same filesystem. |
| `enable_media_info` | boolean | no | `true` | Reads and stores media info (codec, bitrate, audio channels) for imported files. |
| `use_script_import` | boolean | yes |  | Delegates file import handling to an external script instead of the built-in importer. |
| `script_import_path` | string | no |  | Absolute path to the script used for custom imports; required when `use_script_import` is true. |
| `import_extra_files` | boolean | yes |  | Imports extra files (artwork, lyrics, etc.) alongside the track file. |
| `extra_file_extensions` | string | no |  | Comma-separated list of file extensions to import alongside the track file (e.g. `jpg,png,lrc`). |

### Naming

`/api/v1/config/naming` — track file and artist folder naming configuration.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `rename_tracks` | boolean | yes |  | Renames existing track files to match the configured naming format on import or refresh. |
| `replace_illegal_characters` | boolean | no | `true` | Replaces characters that are illegal on common filesystems in file and folder names. |
| `colon_replacement_format` | integer | yes |  | How to handle colons in artist/album/track titles; integer code (0 = delete, 1 = dash, 2 = space dash, 3 = space dash space, 4 = smart). |
| `standard_track_format` | string | no |  | Naming template for standard (single-disc) track files; uses Lidarr naming tokens. |
| `multi_disc_track_format` | string | no |  | Naming template for multi-disc track files; uses Lidarr naming tokens. |
| `artist_folder_format` | string | no |  | Naming template for artist root folders; uses Lidarr naming tokens. |
| `include_artist_name` | boolean | yes |  | Includes the artist name in the auto-generated folder name components. |
| `include_album_title` | boolean | yes |  | Includes the album title in the auto-generated folder name components. |
| `include_quality` | boolean | yes |  | Includes the quality profile name in the auto-generated folder name components. |
| `replace_spaces` | boolean | yes |  | Replaces spaces with the configured separator character in generated names. |
| `separator` | string | no |  | Separator character used between tokens when `replace_spaces` is enabled (e.g. `.` or `_`). |
| `number_style` | string | no |  | Style for track number formatting (e.g. `1` for plain, `01` for zero-padded). |

### Ui Config

`/api/v1/config/ui` — UI display and localisation settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `first_day_of_week` | integer | yes |  | Day the calendar week starts on: 0 = Sunday, 1 = Monday. |
| `calendar_week_column_header` | string | no |  | Format string for the column header in the calendar week view (e.g. `ddd M/D`). |
| `short_date_format` | string | no |  | Short date format string used throughout the UI (e.g. `MMM D YYYY`). |
| `long_date_format` | string | no |  | Long date format string used in detail views (e.g. `dddd, MMMM D YYYY`). |
| `time_format` | string | no |  | Time format string used in the UI: e.g. `h(:mm)a` (12-hour) or `HH:mm` (24-hour). |
| `show_relative_dates` | boolean | yes |  | Displays dates as relative time (e.g. "2 days ago") rather than absolute dates. |
| `enable_color_impaired_mode` | boolean | yes |  | Enables a colour-blind-friendly UI mode with adjusted colour palettes. |
| `ui_language` | integer | yes |  | Language ID for the Lidarr UI interface. |
| `expand_album_by_default` | boolean | yes |  | Expands albums by default in the artist detail view. |
| `expand_single_by_default` | boolean | yes |  | Expands singles by default in the artist detail view. |
| `expand_ep_by_default` | boolean | yes |  | Expands EPs by default in the artist detail view. |
| `expand_broadcast_by_default` | boolean | yes |  | Expands broadcast albums by default in the artist detail view. |
| `expand_other_by_default` | boolean | yes |  | Expands other release types by default in the artist detail view. |
| `theme` | string | no |  | UI colour theme name (e.g. `dark`, `light`, `auto`). |

### Indexer Config

`/api/v1/config/indexer` — global indexer and RSS sync settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `minimum_age` | integer | yes |  | Minimum age in minutes a Usenet release must be before Lidarr will grab it. |
| `maximum_size` | integer | yes |  | Maximum release size in MB that Lidarr will grab; 0 = unlimited. |
| `retention` | integer | yes |  | Usenet retention period in days; 0 = unlimited. |
| `rss_sync_interval` | integer | no | `60` | Interval in minutes between RSS feed syncs; 0 = disable RSS sync. |

### Download Client Config

`/api/v1/config/downloadclient` — download client handling settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `download_client_working_folders` | string | no |  | Pipe-separated list of category or folder names that download clients use for in-progress downloads (e.g. `_UNPACK_|_FAILED_`). |
| `enable_completed_download_handling` | boolean | no | `true` | Automatically imports completed downloads from the download client. |
| `auto_redownload_failed` | boolean | no | `true` | Automatically searches for a replacement release when a download fails. |
| `auto_redownload_failed_from_interactive_search` | boolean | no | `true` | Automatically re-downloads a failed release that was found via interactive search. |

### Host Config

`/api/v1/config/host` — Lidarr host, network, authentication, proxy, and backup settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bind_address` | string | no |  | IP address or hostname Lidarr binds to; `*` binds to all interfaces. |
| `port` | integer | yes |  | HTTP port Lidarr listens on. |
| `ssl_port` | integer | yes |  | HTTPS port Lidarr listens on when SSL is enabled. |
| `enable_ssl` | boolean | yes |  | Enables HTTPS/TLS for the Lidarr web UI. |
| `launch_browser` | boolean | yes |  | Opens the Lidarr web UI in the default browser on startup. |
| `authentication_method` | string | no | `none` | Authentication method for the Lidarr web UI: `none`, `basic`, `forms`, or `external`. |
| `authentication_required` | string | no | `enabled` | Whether authentication is required: `enabled` or `disabledForLocalAddresses`. |
| `analytics_enabled` | boolean | yes |  | Sends anonymised usage and error data to the Lidarr team. |
| `username` | string | no |  | Username for basic or forms authentication. |
| `password` | secret string | no |  | Password for basic or forms authentication. Credential — redacted in plan output. |
| `password_confirmation` | secret string | no |  | Password confirmation field; must match `password` when changing credentials. Credential — redacted in plan output. |
| `log_level` | string | no |  | Log verbosity level (e.g. `info`, `debug`, `trace`). |
| `log_size_limit` | integer | yes |  | Maximum size in MB for each log file before it is rotated. |
| `console_log_level` | string | no |  | Log level for console output; overrides `log_level` for stdout. |
| `branch` | string | no |  | Update channel or branch Lidarr checks for updates (e.g. `main`, `develop`). |
| `api_key` | secret string | no |  | Lidarr API key used to authenticate API requests. Credential — redacted in plan output. |
| `ssl_cert_path` | string | no |  | Absolute path to the SSL certificate file (PEM/PFX). |
| `ssl_cert_password` | secret string | no |  | Password for the SSL certificate if it is password-protected. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path for reverse-proxy deployments (e.g. `/lidarr`). |
| `instance_name` | string | no |  | Display name for this Lidarr instance shown in the browser title and notifications. |
| `application_url` | string | no |  | Externally reachable URL for this instance, used in notifications. |
| `update_automatically` | boolean | yes |  | Allows Lidarr to update itself automatically when a new version is available. |
| `update_mechanism` | string | no | `docker` | How Lidarr applies updates: `builtIn`, `script`, `external`, `apt`, or `docker`. |
| `update_script_path` | string | no |  | Absolute path to the update script; required when `update_mechanism` is `script`. |
| `proxy_enabled` | boolean | yes |  | Routes Lidarr's outbound HTTP traffic through a proxy server. |
| `proxy_type` | string | no | `http` | Proxy protocol: `http`, `socks4`, or `socks5`. |
| `proxy_hostname` | string | no |  | Hostname or IP address of the proxy server. |
| `proxy_port` | integer | yes |  | Port of the proxy server. |
| `proxy_username` | string | no |  | Username for proxy authentication. |
| `proxy_password` | secret string | no |  | Password for proxy authentication. Credential — redacted in plan output. |
| `proxy_bypass_filter` | string | no |  | Comma-separated list of hosts or IP ranges that bypass the proxy. |
| `proxy_bypass_local_addresses` | boolean | yes |  | Bypasses the proxy for connections to local/private addresses. |
| `certificate_validation` | string | no | `enabled` | TLS certificate validation mode: `enabled`, `disabledForLocalAddresses`, or `disabled`. |
| `backup_folder` | string | no |  | Folder path where Lidarr stores automatic database backups. |
| `backup_interval` | integer | yes |  | Interval in days between automatic backups. |
| `backup_retention` | integer | yes |  | Number of days to retain automatic backups before they are deleted. |
| `trust_cgnat_ip_addresses` | boolean | yes |  | Trusts Carrier-Grade NAT (CGNAT) IP address ranges for source IP determination. |

### Metadata Provider Config

`/api/v1/config/metadataprovider` — music metadata source and audio tag write settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `metadata_source` | string | no |  | The metadata source Lidarr uses to look up artist and album information (e.g. `lastfm`). |
| `write_audio_tags` | string | no | `no` | Controls when Lidarr writes audio tags to imported track files: `no`, `newFiles`, `allFiles`, or `sync`. |
| `scrub_audio_tags` | boolean | yes |  | Removes embedded audio tags that are not managed by Lidarr from imported track files. |
| `embed_cover_art` | boolean | yes |  | Embeds cover art into imported track files as an ID3/APE tag. |

## Types

### Quality Profile Quality Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id (leaf items only; absent on group items). |
| `name` | string | no |  | Group or quality tier label displayed in the UI, e.g. `"Lossless"`. |
| `quality` | [`quality`](#quality) | no |  | Quality definition for leaf items; `None` for group items. |
| `items` | array of [`quality_profile_quality_item`](#quality-profile-quality-item) | no |  | Nested group members — empty for leaf items. |
| `allowed` | boolean | yes |  | When `true`, Lidarr will accept releases at this quality tier. |

### Profile Format Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id. |
| `format` | integer | yes |  | Id of the custom format being scored; resolved from `${ref.custom_format.<name>}` at apply. References a [`custom_format`](#custom-format) by name (`${ref.custom_format.<key>}`). |
| `name` | string | no |  | Custom-format name, mirrored from the format definition. |
| `score` | integer | no |  | Points awarded to a release matching this format; negative values penalise. |

### Primary Album Type Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id. |
| `album_type` | [`primary_album_type`](#primary-album-type) | no |  | The primary album type described by this item. |
| `allowed` | boolean | yes |  | When `true`, albums of this type are monitored. |

### Secondary Album Type Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id. |
| `album_type` | [`secondary_album_type`](#secondary-album-type) | no |  | The secondary album type described by this item. |
| `allowed` | boolean | yes |  | When `true`, albums with this secondary classification are monitored. |

### Release Status Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id. |
| `release_status` | [`release_status`](#release-status) | no |  | The release status described by this item. |
| `allowed` | boolean | yes |  | When `true`, releases with this status are monitored. |

### Monitor Types

Allowed values: `all` / `future` / `missing` / `existing` / `latest` / `first` / `none`.

### New Item Monitor Types

Allowed values: `all` / `none` / `new`.

### Download Protocol

Allowed values: `usenet` / `torrent`.

### Quality

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned quality definition id. |
| `name` | string | no |  | Quality tier label, e.g. `"FLAC"` or `"MP3-320"`. |

### Primary Album Type

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned album type id. |
| `name` | string | no |  | Album type label, e.g. `"Album"`, `"Single"`, `"EP"`. |

### Secondary Album Type

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned secondary album type id. |
| `name` | string | no |  | Secondary album type label, e.g. `"Studio"`, `"Live"`, `"Remix"`. |

### Release Status

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned release status id. |
| `name` | string | no |  | Release status label, e.g. `"Official"`, `"Bootleg"`, `"Promotional"`. |

