# Sonarr v3 Configuration

Sonarr v3 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Tag

A label applied to series, indexers, download clients, notifications, etc.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the name referenced in `${ref.tag.<label>}`. |

### Custom Format

A custom format — a named collection of specification conditions Sonarr uses
to score releases. The score influences download decisions via quality profiles.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.custom_format.<name>}`. |
| `include_custom_format_when_renaming` | boolean | no |  | When true, the format name is included in Sonarr's file rename template. |
| `specifications` | array of any | no |  | Specification conditions, each a provider-shaped object (`implementation` + `fields[]`). Raw JSON — see the module docs. |

### Custom Filter

A saved custom filter for a Sonarr UI page.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the user-visible label for this filter. |
| `filter_type` | string | no |  | The UI page context this filter applies to (e.g. `SeriesIndex`, `EpisodeFile`). Wire name is `type` (a Rust keyword). |
| `filters` | array of any | no |  | Filter conditions, each a raw object with `key`, `value`, and `type`. Raw JSON — the condition shape is not described in the static spec. |

### Quality Profile

Named quality profile — ordered quality ladder with format-score gates.

Sonarr evaluates profiles top-to-bottom: the first `allowed` quality (or
group) that a release matches determines whether it is grabbed and whether an
upgrade is triggered. Custom-format scores further filter grabs and
upgrades via `min_format_score` and `cutoff_format_score`.

Unlike Radarr, Sonarr quality profiles do **not** embed a language
constraint — language preference is managed separately via `LanguageProfile`.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.quality_profile.<name>}`. |
| `upgrade_allowed` | boolean | yes |  | When `true`, Sonarr will seek a better-quality release after the initial download. |
| `cutoff` | integer | yes |  | Id of the cutoff quality tier; Sonarr will not seek upgrades past this point. |
| `items` | array of [`quality_profile_item`](#quality-profile-item) | no |  | Ordered quality ladder — all quality tiers and groups this profile considers. |
| `min_format_score` | integer | yes |  | Minimum aggregate custom-format score a release must reach to be grabbed. |
| `cutoff_format_score` | integer | yes |  | Minimum format score that satisfies the upgrade cutoff. |
| `min_upgrade_format_score` | integer | yes |  | Minimum improvement in custom-format score required to trigger an upgrade. |
| `format_items` | array of [`profile_format_item`](#profile-format-item) | no |  | Custom-format score contributions attached to this profile. |

### Language Profile

Language profile — ordered list of acceptable languages with an upgrade cutoff.

Sonarr assigns one language profile to each series. The profile controls:
- which languages are acceptable (`languages` list with `allowed: true`),
- the upgrade cutoff (`cutoff`) — Sonarr stops seeking upgrades once it has
a release at or above this language.
- whether Sonarr will upgrade from a lower-ranked language (`upgrade_allowed`).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.language_profile.<name>}`. |
| `upgrade_allowed` | boolean | yes |  | When `true`, Sonarr will seek a release in a higher-preference language after an initial download. |
| `cutoff` | [`language`](#language) | no |  | Language at which Sonarr stops seeking upgrades. Must be an `allowed` entry in the `languages` list. |
| `languages` | array of [`language_profile_item`](#language-profile-item) | no |  | Ordered list of languages and their acceptance state; preference decreases toward the end of the list. |

### Release Profile

Release profile — term-based acceptance and rejection filter for grabbed releases.

When `enabled`, Sonarr checks every candidate release title against the
`required` and `ignored` term lists before deciding to grab it:
- `required`: at least one term must appear in the release title.
- `ignored`: none of the terms may appear in the release title.

Setting `indexer_id` to `0` (the default) applies the profile to releases
from all indexers; a non-zero value restricts it to a specific indexer.
Tag-scoped profiles (via `tags`) apply only to series that carry one of
the listed tags; an empty `tags` list means the profile applies globally.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.release_profile.<name>}`. |
| `enabled` | boolean | yes |  | When `false`, this profile is saved but not applied to any grabs. |
| `required` | any | no |  | Terms that must appear in a release title for it to be accepted. Untyped in the spec; in practice an array of term strings. `None` means no required-term constraint. |
| `ignored` | any | no |  | Terms that must **not** appear in a release title; releases containing any of these terms are rejected. Untyped in the spec; in practice an array of term strings. `None` means no ignored-term constraint. |
| `indexer_id` | integer | no | `0` | Id of the indexer this profile is restricted to; `0` means all indexers. |
| `tags` | array of integer | no |  | Series tag ids this profile applies to; resolved from `${ref.tag.<label>}` at apply. An empty list means the profile is applied globally to all series. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |

### Auto Tag

Automatic tagging rule — applies tags to series matching its specifications.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — the rule name referenced in `${ref.auto_tag.<name>}`. |
| `remove_tags_automatically` | boolean | yes |  | When `true`, tags added by this rule are removed if the series no longer matches its specifications. |
| `tags` | array of integer | no |  | Tag ids applied when the specifications match. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `specifications` | array of any | no |  | Specification conditions (dynamic fields blob — stored as opaque JSON). |

### Remote Path Mapping

A remote-to-local path mapping for a download client host.

Sonarr uses these to translate paths returned by download clients that run
on a different host (or container) where filesystem paths differ.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Hostname or IP of the download client that uses the remote path. |
| `remote_path` | string | yes |  | Natural key — the path as the remote download client reports it. |
| `local_path` | string | yes |  | The local filesystem path that corresponds to `remote_path`. |

### Root Folder

A root folder Sonarr watches for series.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `path` | string | yes |  | Natural key — the absolute filesystem path Sonarr watches. |

### Import List Exclusion

A series excluded from Sonarr import list processing.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tvdb_id` | integer | yes |  | Natural key — the TVDB series ID being excluded. Referenced as `${ref.import_list_exclusion.<tvdb_id>}`. |
| `title` | string | no |  | Display title of the excluded series. |

### Download Client

A Sonarr download client (usenet or torrent).

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
| `remove_completed_downloads` | boolean | no | `true` | Remove downloads from the client once Sonarr has imported them. |
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
| `tv_category` | string | no |  | Category (label) assigned to TV series downloads in Deluge. |
| `tv_imported_category` | string | no |  | Category Deluge moves completed downloads to after Sonarr imports them. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
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
| `start_on_add` | boolean | no |  | Start torrents immediately when added (instead of adding paused). |
| `use_ssl` | boolean | no |  | Connect to Flood over HTTPS. |
| `field_tags` | array of string | no |  | Tags applied to the torrent in Flood (string labels, not Sonarr tag ids). |
| `additional_tags` | array of integer | no |  | Additional Sonarr-managed metadata tags appended to the torrent (integer codes). |
| `post_import_tags` | array of string | no |  | Tags applied to the torrent in Flood after Sonarr imports it. |

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
| `tv_category` | string | no |  | Category assigned to TV series downloads in NZBGet. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
| `url_base` | string | no |  | URL base path if NZBGet is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add downloads to NZBGet in a paused state. |
| `use_ssl` | boolean | no |  | Connect to NZBGet over HTTPS. |

#### Download Client: Nzbvortex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBVortex server. |
| `port` | integer | no |  | TCP port the NZBVortex server listens on. |
| `api_key` | secret string | no |  | API key used to authenticate with NZBVortex. Credential — redacted in plan output. |
| `tv_category` | string | no |  | Category assigned to TV series downloads in NZBVortex. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
| `url_base` | string | no |  | URL base path if NZBVortex is hosted behind a reverse proxy. |

#### Download Client: Pneumatic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Sonarr drops NZB files into for Pneumatic to pick up. |
| `strm_folder` | string | no |  | Folder Pneumatic writes `.strm` stream files to. |

#### Download Client: QBittorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the qBittorrent server. |
| `port` | integer | no |  | TCP port the qBittorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with qBittorrent. |
| `password` | secret string | no |  | Password for authenticating with qBittorrent. Credential — redacted in plan output. |
| `tv_category` | string | no |  | Category assigned to TV series downloads in qBittorrent. |
| `tv_imported_category` | string | no |  | Category qBittorrent moves completed downloads to after Sonarr imports them. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
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
| `tv_category` | string | no |  | Label assigned to TV series torrents in rTorrent. |
| `tv_directory` | string | no |  | Directory rTorrent saves TV series downloads to. |
| `tv_imported_category` | string | no |  | Label rTorrent moves completed downloads to after Sonarr imports them. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
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
| `tv_category` | string | no |  | Category assigned to TV series downloads in SABnzbd. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
| `url_base` | string | no |  | URL base path if SABnzbd is hosted behind a reverse proxy. |
| `use_ssl` | boolean | no |  | Connect to SABnzbd over HTTPS. |

#### Download Client: TorrentBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `torrent_folder` | string | no |  | Folder Sonarr drops `.torrent` files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Sonarr watches for completed downloads from the external client. |
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
| `tv_category` | string | no |  | Shared folder or category assigned to TV series downloads. |
| `tv_directory` | string | no |  | Directory Download Station saves TV series downloads to. |
| `use_ssl` | boolean | no |  | Connect to Synology DSM over HTTPS. |

#### Download Client: Transmission

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Transmission server. |
| `port` | integer | no |  | TCP port the Transmission RPC interface listens on. |
| `username` | string | no |  | Username for authenticating with Transmission. |
| `password` | secret string | no |  | Password for authenticating with Transmission. Credential — redacted in plan output. |
| `tv_category` | string | no |  | Category (label) assigned to TV series downloads in Transmission. |
| `tv_directory` | string | no |  | Directory Transmission saves TV series downloads to. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
| `url_base` | string | no |  | URL base path if Transmission is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Transmission in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Transmission over HTTPS. |

#### Download Client: UsenetBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Sonarr drops NZB files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Sonarr watches for completed downloads from the external client. |

#### Download Client: UsenetDownloadStation

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Synology NAS running Download Station. |
| `port` | integer | no |  | TCP port the Synology DSM web interface listens on. |
| `username` | string | no |  | Username for authenticating with Synology DSM. |
| `password` | secret string | no |  | Password for authenticating with Synology DSM. Credential — redacted in plan output. |
| `tv_category` | string | no |  | Shared folder or category assigned to TV series downloads. |
| `tv_directory` | string | no |  | Directory Download Station saves TV series downloads to. |
| `use_ssl` | boolean | no |  | Connect to Synology DSM over HTTPS. |

#### Download Client: UTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the uTorrent web UI. |
| `port` | integer | no |  | TCP port the uTorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with uTorrent. |
| `password` | secret string | no |  | Password for authenticating with uTorrent. Credential — redacted in plan output. |
| `tv_category` | string | no |  | Category assigned to TV series downloads in uTorrent. |
| `tv_imported_category` | string | no |  | Category uTorrent moves completed downloads to after Sonarr imports them. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
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
| `tv_category` | string | no |  | Category (label) assigned to TV series downloads in Vuze. |
| `tv_directory` | string | no |  | Directory Vuze saves TV series downloads to. |
| `recent_tv_priority` | integer | no |  | Priority for episodes aired in the last 14 days. |
| `older_tv_priority` | integer | no |  | Priority for episodes aired more than 14 days ago. |
| `url_base` | string | no |  | URL base path if Vuze is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Vuze in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Vuze over HTTPS. |

### Indexer

Indexer definition — connects Sonarr to a usenet or torrent search source.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable_rss` | boolean | yes |  | Whether to include this indexer in RSS sync feeds. |
| `enable_automatic_search` | boolean | yes |  | Whether to use this indexer for automatic (monitored) searches. |
| `enable_interactive_search` | boolean | yes |  | Whether to use this indexer for interactive (manual) searches. |
| `protocol` | [`download_protocol`](#download-protocol) | yes |  | Transport protocol used by this indexer (usenet or torrent). |
| `priority` | integer | no | `25` | Indexer priority; lower values are preferred when multiple indexers match a grab. |
| `season_search_maximum_single_episode_age` | integer | no |  | Maximum age in days for single-episode season search results. |
| `download_client_id` | integer | no |  | Download client to use exclusively for grabs from this indexer; absent means use the default. References a [`download_client`](#download-client) by name (`${ref.download_client.<key>}`). |

Set `implementation` to one of: [`BroadcastheNet`](#indexer-broadcasthenet) / [`Fanzub`](#indexer-fanzub) / [`FileList`](#indexer-filelist) / [`HDBits`](#indexer-hdbits) / [`IPTorrents`](#indexer-iptorrents) / [`Newznab`](#indexer-newznab) / [`Nyaa`](#indexer-nyaa) / [`TorrentRssIndexer`](#indexer-torrentrss) / [`Torrentleech`](#indexer-torrentleech) / [`Torznab`](#indexer-torznab).

#### Indexer: BroadcastheNet

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the BroadcastheNet tracker API. |
| `api_key` | secret string | yes |  | API key for authenticating requests to BroadcastheNet. Credential — redacted in plan output. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

#### Indexer: Fanzub

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the Fanzub indexer (defaults to the public instance if absent). |
| `anime_standard_format_search` | boolean | no |  | Search anime releases using the standard Sonarr title format. |

#### Indexer: FileList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the FileList tracker (optional; has a built-in default). |
| `username` | string | yes |  | FileList account username. |
| `passkey` | secret string | yes |  | FileList passkey for API authentication. Credential — redacted in plan output. |
| `categories` | array of integer | no |  | TV category IDs to include in searches. |
| `anime_categories` | array of integer | no |  | Anime category IDs to include in searches. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

#### Indexer: HdBits

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the HDBits tracker (optional; has a built-in default). |
| `username` | string | yes |  | HDBits account username. |
| `api_key` | secret string | yes |  | HDBits API key for authentication. Credential — redacted in plan output. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

#### Indexer: IpTorrents

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | RSS feed URL including the user's passkey (provided by IPTorrents). |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

#### Indexer: Newznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the Newznab indexer. |
| `api_path` | string | no |  | URL path to the Newznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Newznab indexer. Credential — redacted in plan output. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `anime_standard_format_search` | boolean | no |  | Search anime releases using the standard Sonarr title format. |
| `categories` | array of integer | no |  | Newznab category IDs to include in searches. |
| `anime_categories` | array of integer | no |  | Anime category IDs to include in searches. |

#### Indexer: Nyaa

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Nyaa indexer. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `anime_standard_format_search` | boolean | no |  | Search anime releases using the standard Sonarr title format. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

#### Indexer: TorrentRss

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | URL of the torrent RSS feed. |
| `cookie` | string | no |  | Session cookie sent with RSS requests for authenticated feeds. |
| `allow_zero_size` | boolean | no |  | Allow releases that report a size of zero bytes. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

#### Indexer: TorrentLeech

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the TorrentLeech tracker (optional; has a built-in default). |
| `api_key` | secret string | yes |  | TorrentLeech API key for authentication. Credential — redacted in plan output. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

#### Indexer: Torznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Torznab indexer. |
| `api_path` | string | no |  | URL path to the Torznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Torznab indexer. Credential — redacted in plan output. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `anime_standard_format_search` | boolean | no |  | Search anime releases using the standard Sonarr title format. |
| `categories` | array of integer | no |  | Torznab category IDs to include in searches. |
| `anime_categories` | array of integer | no |  | Anime category IDs to include in searches. |
| `minimum_seeders` | integer | no |  | Minimum number of seeders a torrent must have to be grabbed. |
| `season_pack_seed_time` | integer | no |  | Minimum seeding time in minutes after a season-pack download. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Sonarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Sonarr must reach before stopping seeding. |

### Metadata

Metadata consumer — instructs Sonarr to write sidecar metadata files and
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
| `series_metadata` | boolean | no |  | Write series-level NFO metadata files. |
| `series_metadata_url` | boolean | no |  | Include series metadata URLs inside NFO files. |
| `series_images` | boolean | no |  | Download and store series-level artwork. |
| `season_images` | boolean | no |  | Download and store season-level artwork. |
| `episode_metadata` | boolean | no |  | Write episode-level NFO metadata files. |
| `episode_images` | boolean | no |  | Download and store episode-level artwork (thumbnails). |

#### Metadata: Roksbox

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `series_images` | boolean | no |  | Download and store series-level artwork. |
| `season_images` | boolean | no |  | Download and store season-level artwork. |
| `episode_metadata` | boolean | no |  | Write episode-level metadata files. |
| `episode_images` | boolean | no |  | Download and store episode-level artwork (thumbnails). |

#### Metadata: Wdtv

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `series_images` | boolean | no |  | Download and store series-level artwork. |
| `season_images` | boolean | no |  | Download and store season-level artwork. |
| `episode_metadata` | boolean | no |  | Write episode-level metadata files. |
| `episode_images` | boolean | no |  | Download and store episode-level artwork (thumbnails). |

### Notification

A Sonarr notification connection — routes series/episode events to external services.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `on_grab` | boolean | yes |  | Fire notification when a release is grabbed for download. |
| `on_download` | boolean | yes |  | Fire notification when an episode file is imported after download. |
| `on_upgrade` | boolean | yes |  | Fire notification when a file is upgraded to a higher-quality version. |
| `on_import_complete` | boolean | yes |  | Fire notification when an import is completed (post-download processing). |
| `on_rename` | boolean | yes |  | Fire notification when episode files are renamed. |
| `on_series_add` | boolean | yes |  | Fire notification when a series is added to the Sonarr library. |
| `on_series_delete` | boolean | yes |  | Fire notification when a series is deleted from the library. |
| `on_episode_file_delete` | boolean | yes |  | Fire notification when an episode file is deleted. |
| `on_episode_file_delete_for_upgrade` | boolean | yes |  | Fire notification when an episode file is deleted to make room for an upgrade. |
| `on_health_issue` | boolean | yes |  | Fire notification when a health-check issue is detected. |
| `include_health_warnings` | boolean | yes |  | Include health warnings (not just errors) in health-issue notifications. |
| `on_health_restored` | boolean | yes |  | Fire notification when a previously detected health-check issue is resolved. |
| `on_application_update` | boolean | yes |  | Fire notification when a Sonarr application update is available. |
| `on_manual_interaction_required` | boolean | yes |  | Fire notification when a download requires manual interaction. |

Set `implementation` to one of: [`Apprise`](#notification-apprise) / [`CustomScript`](#notification-customscript) / [`Discord`](#notification-discord) / [`Email`](#notification-email) / [`MediaBrowser`](#notification-emby) / [`Gotify`](#notification-gotify) / [`Join`](#notification-join) / [`Xbmc`](#notification-kodi) / [`Mailgun`](#notification-mailgun) / [`Ntfy`](#notification-ntfy) / [`PlexServer`](#notification-plex) / [`Prowl`](#notification-prowl) / [`PushBullet`](#notification-pushbullet) / [`Pushover`](#notification-pushover) / [`Sendgrid`](#notification-sendgrid) / [`Signal`](#notification-signal) / [`Simplepush`](#notification-simplepush) / [`Slack`](#notification-slack) / [`SynologyIndexer`](#notification-synologyindexer) / [`Telegram`](#notification-telegram) / [`Trakt`](#notification-trakt) / [`Twitter`](#notification-twitter) / [`Webhook`](#notification-webhook).

#### Notification: Apprise

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | yes |  | Base URL of the Apprise API server. |
| `stateless_urls` | string | no |  | Comma-separated stateless Apprise notification URLs (e.g. `slack://…`). |
| `notification_type` | integer | no |  | Notification type/category identifier sent to Apprise. |
| `auth_username` | string | no |  | HTTP basic-auth username for the Apprise server. |
| `auth_password` | secret string | no |  | HTTP basic-auth password for the Apprise server. Credential — redacted in plan output. |
| `configuration_key` | string | no |  | Apprise persistent-store configuration key. |
| `field_tags` | array of string | no |  | Tag filters applied to the Apprise notification dispatch. |

#### Notification: CustomScript

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `path` | string | yes |  | Absolute filesystem path to the script to execute on notification events. |
| `arguments` | string | no |  | Additional arguments passed to the script on invocation. |

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
| `use_encryption` | integer | yes |  | Encryption mode: 0 = none, 1 = SSL/TLS, 2 = STARTTLS. |
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
| `port` | integer | yes |  | Emby server HTTP port. |
| `api_key` | secret string | yes |  | Emby API key for authentication. Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to Emby over HTTPS. |
| `notify` | boolean | no |  | Send an on-screen notification to Emby users on events. |
| `update_library` | boolean | no |  | Trigger an Emby library refresh after an episode is imported. |

#### Notification: Gotify

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | yes |  | Gotify server URL (e.g. `http://gotify.example.com`). |
| `app_token` | secret string | yes |  | Gotify application token used to publish messages. Credential — redacted in plan output. |
| `priority` | integer | no |  | Message priority level sent with each notification. |

#### Notification: Join

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Join API key for authentication. Credential — redacted in plan output. |
| `device_names` | string | no |  | Comma-separated target device names; leave empty to send to all devices. |
| `priority` | integer | no |  | Notification priority level. |

#### Notification: Kodi

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Kodi hostname or IP address. |
| `port` | integer | yes |  | Kodi JSON-RPC HTTP port. |
| `username` | string | no |  | Kodi authentication username. |
| `password` | secret string | no |  | Kodi authentication password. Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to Kodi over HTTPS. |
| `notify` | boolean | no |  | Display an on-screen notification in Kodi on events. |
| `display_time` | integer | no |  | Duration in milliseconds to display the on-screen notification. |
| `update_library` | boolean | no |  | Trigger a Kodi video library update after an episode is imported. |
| `clean_library` | boolean | no |  | Trigger a Kodi video library clean after an episode file is deleted. |
| `always_update` | boolean | no |  | Always update the library on every event, not just import events. |

#### Notification: Mailgun

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Mailgun API key for authentication. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `sender_domain` | string | yes |  | Mailgun sending domain registered in your account. |
| `use_eu_endpoint` | boolean | no |  | Use the EU Mailgun API endpoint instead of the US endpoint. |
| `recipients` | array of string | no |  | Recipient email addresses. |

#### Notification: Ntfy

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | yes |  | Base URL of the ntfy server (e.g. `https://ntfy.sh`). |
| `topics` | array of string | no |  | ntfy topic names to publish notifications to. |
| `priority` | integer | no |  | Message priority level (1 = min … 5 = max). |
| `username` | string | no |  | HTTP basic-auth username for the ntfy server. |
| `password` | secret string | no |  | HTTP basic-auth password for the ntfy server. Credential — redacted in plan output. |
| `access_token` | secret string | no |  | Bearer access token for ntfy authentication (alternative to username/password). Credential — redacted in plan output. |
| `click_url` | string | no |  | URL opened when the notification is tapped by the user. |
| `field_tags` | array of string | no |  | ntfy message tags applied to the notification (emoji shortcodes accepted). |

#### Notification: Plex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Plex Media Server hostname or IP address. |
| `port` | integer | yes |  | Plex Media Server HTTP port. |
| `auth_token` | secret string | yes |  | Plex authentication token (X-Plex-Token). Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to Plex over HTTPS. |
| `update_library` | boolean | no |  | Trigger a Plex library section refresh after an episode is imported. |

#### Notification: Prowl

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Prowl API key for authentication. Credential — redacted in plan output. |
| `priority` | integer | no |  | Notification priority level (-2 = very low … 2 = emergency). |

#### Notification: Pushbullet

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | PushBullet API key for authentication. Credential — redacted in plan output. |
| `sender_id` | string | no |  | Sender device identifier shown as the push source. |
| `device_ids` | array of string | no |  | Target device identifiers to receive the push notification. |
| `channel_tags` | array of string | no |  | PushBullet channel tags to publish the notification to. |

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
| `api_key` | secret string | yes |  | SendGrid API key for authentication. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `recipients` | array of string | no |  | Recipient email addresses. |

#### Notification: Signal

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | yes |  | Hostname or IP address of the signal-cli REST API host. |
| `port` | integer | no |  | HTTP port of the signal-cli REST API. |
| `sender_number` | string | yes |  | Phone number registered in signal-cli that sends messages. |
| `receiver_id` | string | yes |  | Phone number or group ID that receives the notification messages. |
| `use_ssl` | boolean | no |  | Connect to the signal-cli REST API over HTTPS. |
| `auth_username` | string | no |  | HTTP basic-auth username for the signal-cli REST API. |
| `auth_password` | secret string | no |  | HTTP basic-auth password for the signal-cli REST API. Credential — redacted in plan output. |

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

#### Notification: SynologyIndexer

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `update_library` | boolean | no |  | Trigger a Synology media library update after an episode is imported. |

#### Notification: Telegram

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bot_token` | secret string | yes |  | Telegram bot token issued by BotFather. Credential — redacted in plan output. |
| `chat_id` | string | yes |  | Target chat, group, or channel ID to send messages to. |
| `send_silently` | boolean | no |  | Send the notification silently (no sound or alert on the recipient's device). |

#### Notification: Trakt

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | yes |  | Trakt OAuth access token. Credential — redacted in plan output. |
| `refresh_token` | secret string | yes |  | Trakt OAuth refresh token used to obtain a new access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 timestamp at which the access token expires. |
| `auth_user` | string | no |  | Trakt username associated with the authenticated account. |

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

An import list syncs series from an external source into Sonarr's library.

Each import list pairs a shared envelope (identity, monitoring preferences,
root folder, quality profile, series type) with a typed per-implementation
settings blob (the `config` field).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable_automatic_add` | boolean | yes |  | Whether Sonarr automatically adds series found on this list. |
| `search_for_missing_episodes` | boolean | yes |  | Whether Sonarr searches for missing episodes of added series. |
| `should_monitor` | string | no |  | Episode monitoring strategy applied when series are added (e.g. `"all"`, `"future"`, `"missing"`, `"pilot"`, `"firstSeason"`). |
| `monitor_new_items` | string | no |  | How newly released episodes are monitored after the series is added (`"all"` or `"none"`). |
| `root_folder_path` | string | no |  | Root folder where series added by this list are placed. |
| `quality_profile_id` | integer | yes |  | Quality profile assigned to series added by this list. References a [`quality_profile`](#quality-profile) by name (`${ref.quality_profile.<key>}`). |
| `series_type` | string | no |  | Series type for added series (`"standard"`, `"daily"`, or `"anime"`). |
| `season_folder` | boolean | yes |  | Whether to organise episodes into per-season subfolders. |
| `list_order` | integer | yes |  | Display sort order of this list in the UI. |

Set `implementation` to one of: [`CustomImport`](#import-list-custom) / [`ImdbListImport`](#import-list-imdb) / [`PlexImport`](#import-list-plex) / [`PlexRssImport`](#import-list-plexrss) / [`SimklUserImport`](#import-list-simkluser) / [`SonarrImport`](#import-list-sonarr) / [`TraktListImport`](#import-list-traktlist) / [`TraktPopularImport`](#import-list-traktpopular) / [`TraktUserImport`](#import-list-traktuser).

#### Import List: Custom

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the custom list endpoint. |

#### Import List: Imdb

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_id` | string | no |  | IMDb list identifier (e.g. `"ls012345678"`). |

#### Import List: Plex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for the Plex account whose watchlist is imported. Credential — redacted in plan output. |

#### Import List: PlexRss

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `url` | string | no |  | Full URL of the Plex RSS feed to import from. |

#### Import List: SimklUser

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_type` | integer | no |  | Integer identifying which Simkl list to import: `0` = Watching, `1` = Plan to Watch, `2` = Hold, `3` = Completed, `4` = Dropped. |
| `access_token` | secret string | no |  | OAuth access token for the Simkl account. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `auth_user` | string | no |  | Simkl username linked to the OAuth credentials. |
| `expires` | string | no |  | ISO 8601 expiry timestamp for the access token. |

#### Import List: Sonarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the source Sonarr instance (e.g. `"http://sonarr:8989"`). |
| `api_key` | secret string | no |  | API key for authenticating with the source Sonarr instance. Credential — redacted in plan output. |
| `profile_ids` | array of integer | no |  | Quality profile IDs to filter by on the source Sonarr instance. |
| `language_profile_ids` | array of integer | no |  | Language profile IDs to filter by on the source Sonarr instance. |
| `tag_ids` | array of integer | no |  | Tag IDs to filter by on the source Sonarr instance. |

#### Import List: TraktList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for authenticating with the Trakt API. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 expiry timestamp for the access token. |
| `auth_user` | string | no |  | Trakt username associated with the OAuth credentials. |
| `username` | string | no |  | Trakt username whose list is being imported. |
| `listname` | string | no |  | Slug name of the Trakt list to import. |
| `limit` | integer | no |  | Maximum number of series to import from the list. |
| `trakt_additional_parameters` | string | no |  | Extra query parameters appended to the Trakt API request. |

#### Import List: TraktPopular

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for authenticating with the Trakt API. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 expiry timestamp for the access token. |
| `auth_user` | string | no |  | Trakt username associated with the OAuth credentials. |
| `trakt_list_type` | integer | no |  | Category of popular series to import (integer enum): `0` = Trending, `1` = Popular, `2` = Anticipated, `3`–`6` = TopWatched*, `7`–`10` = Recommended*. |
| `limit` | integer | no |  | Maximum number of series to import. |
| `genres` | string | no |  | Comma-separated list of genre slugs to include (e.g. `"drama,comedy"`). |
| `years` | string | no |  | Year or year range filter (e.g. `"2020"` or `"2015-2020"`). |
| `rating` | string | no |  | Score range filter (e.g. `"70-100"`). |
| `trakt_additional_parameters` | string | no |  | Extra query parameters appended to the Trakt API request. |

#### Import List: TraktUser

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for authenticating with the Trakt API. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 expiry timestamp for the access token. |
| `auth_user` | string | no |  | Trakt username associated with the OAuth credentials. |
| `username` | string | no |  | Trakt username whose list is being imported. |
| `trakt_list_type` | integer | no |  | Which Trakt user list to import (integer enum): `0` = WatchList, `1` = WatchedList, `2` = CollectionList. |
| `limit` | integer | no |  | Maximum number of series to import. |
| `trakt_additional_parameters` | string | no |  | Extra query parameters appended to the Trakt API request. |

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

### Media Management

`/api/v3/config/mediamanagement` — file handling and media management settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `auto_unmonitor_previously_downloaded_episodes` | boolean | yes |  | Automatically unmonitors an episode after its file has been downloaded. |
| `recycle_bin` | string | no |  | Path to the recycle bin folder for deleted episode files; empty disables the recycle bin. |
| `recycle_bin_cleanup_days` | integer | no | `7` | Number of days before files in the recycle bin are permanently deleted; 0 disables automatic cleanup. |
| `download_propers_and_repacks` | string | no | `doNotPrefer` | Whether to download proper/repack releases: `preferAndUpgrade`, `doNotUpgrade`, or `doNotPrefer`. |
| `create_empty_series_folders` | boolean | yes |  | Creates a folder for a series even before its file has been downloaded. |
| `delete_empty_folders` | boolean | yes |  | Removes empty series/season folders after a file is deleted or moved. |
| `file_date` | string | no | `none` | Sets the file modification date to the episode air date: `none`, `localAirDate`, or `utcAirDate`. |
| `rescan_after_refresh` | string | no | `always` | When to rescan the series folder after a library refresh: `always`, `afterManual`, or `never`. |
| `set_permissions_linux` | boolean | yes |  | Sets file and folder permissions on imported files (Linux/macOS only). |
| `chmod_folder` | string | no |  | Octal permission bits applied to imported episode folders (e.g. `755`); requires `set_permissions_linux`. |
| `chown_group` | string | no |  | Group name or GID to chown imported files and folders to; requires `set_permissions_linux`. |
| `episode_title_required` | string | no | `always` | When an episode title is required to import: `always`, `bulkSeasonReleases`, or `never`. |
| `skip_free_space_check_when_importing` | boolean | yes |  | Skips the available disk space check before importing an episode file. |
| `minimum_free_space_when_importing` | integer | no | `100` | Minimum free disk space in MB required on the destination before Sonarr will import. |
| `copy_using_hardlinks` | boolean | no | `true` | Uses hardlinks instead of copying when source and destination are on the same filesystem. |
| `use_script_import` | boolean | yes |  | Delegates file import handling to an external script instead of the built-in importer. |
| `script_import_path` | string | no |  | Absolute path to the script used for custom imports; required when `use_script_import` is true. |
| `import_extra_files` | boolean | yes |  | Imports extra files (subtitles, NFO, etc.) alongside the episode file. |
| `extra_file_extensions` | string | no |  | Comma-separated list of file extensions to import alongside the episode file (e.g. `srt,nfo`). |
| `enable_media_info` | boolean | no | `true` | Reads and stores media info (codec, resolution, audio channels) for imported files. |

### Naming

`/api/v3/config/naming` — episode file and folder naming configuration.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `rename_episodes` | boolean | yes |  | Renames existing episode files to match the configured naming format on import or refresh. |
| `replace_illegal_characters` | boolean | no | `true` | Replaces characters that are illegal on common filesystems in file and folder names. |
| `colon_replacement_format` | integer | yes |  | How to handle colons in series/episode titles; integer code (0 = delete, 1 = dash, 2 = space dash, 3 = space dash space, 4 = smart). |
| `custom_colon_replacement_format` | string | no |  | Custom colon replacement string; used when `colon_replacement_format` is set to a custom mode. |
| `multi_episode_style` | integer | yes |  | Style for multi-episode file naming; integer code (0 = extend, 1 = duplicate, 2 = repeat, 3 = scene, 4 = range, 5 = prefixed range). |
| `standard_episode_format` | string | no |  | Naming template for standard (non-daily, non-anime) episode files; uses Sonarr naming tokens. |
| `daily_episode_format` | string | no |  | Naming template for daily (date-based) episode files; uses Sonarr naming tokens. |
| `anime_episode_format` | string | no |  | Naming template for anime episode files; uses Sonarr naming tokens. |
| `series_folder_format` | string | no |  | Naming template for series root folders; uses Sonarr naming tokens. |
| `season_folder_format` | string | no |  | Naming template for season subfolders; uses Sonarr naming tokens. |
| `specials_folder_format` | string | no |  | Naming template for the Specials season subfolder; uses Sonarr naming tokens. |

### Ui Config

`/api/v3/config/ui` — UI display and localisation settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `first_day_of_week` | integer | yes |  | Day the calendar week starts on: 0 = Sunday, 1 = Monday. |
| `calendar_week_column_header` | string | no |  | Format string for the column header in the calendar week view (e.g. `ddd M/D`). |
| `short_date_format` | string | no |  | Short date format string used throughout the UI (e.g. `MMM D YYYY`). |
| `long_date_format` | string | no |  | Long date format string used in detail views (e.g. `dddd, MMMM D YYYY`). |
| `time_format` | string | no |  | Time format string used in the UI: e.g. `h(:mm)a` (12-hour) or `HH:mm` (24-hour). |
| `show_relative_dates` | boolean | yes |  | Displays dates as relative time (e.g. "2 days ago") rather than absolute dates. |
| `enable_color_impaired_mode` | boolean | yes |  | Enables a colour-blind-friendly UI mode with adjusted colour palettes. |
| `theme` | string | no |  | UI colour theme name (e.g. `dark`, `light`, `auto`). |
| `ui_language` | integer | yes |  | Language ID for the Sonarr UI interface. |

### Indexer Config

`/api/v3/config/indexer` — global indexer and RSS sync settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `minimum_age` | integer | yes |  | Minimum age in minutes a Usenet release must be before Sonarr will grab it. |
| `retention` | integer | yes |  | Usenet retention period in days; 0 = unlimited. |
| `maximum_size` | integer | yes |  | Maximum release size in MB that Sonarr will grab; 0 = unlimited. |
| `rss_sync_interval` | integer | no | `60` | Interval in minutes between RSS feed syncs; 0 = disable RSS sync. |

### Download Client Config

`/api/v3/config/downloadclient` — download client handling settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `download_client_working_folders` | string | no |  | Pipe-separated list of category or folder names that download clients use for in-progress downloads (e.g. `_UNPACK_|_FAILED_`). |
| `enable_completed_download_handling` | boolean | no | `true` | Automatically imports completed downloads from the download client. |
| `auto_redownload_failed` | boolean | no | `true` | Automatically searches for a replacement release when a download fails. |
| `auto_redownload_failed_from_interactive_search` | boolean | no | `true` | Automatically re-downloads a failed release that was found via interactive search. |

### Import List Config

`/api/v3/config/importlist` — import list sync level configuration.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_sync_level` | string | no |  | Action taken when a series is removed from all import lists: `disabled`, `logOnly`, `keepAndUnmonitor`, or `keepAndTag`. |
| `list_sync_tag` | integer | no |  | Tag applied to series when `list_sync_level` is `keepAndTag`; resolved from `${ref.tag.<label>}`. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |

### Host Config

`/api/v3/config/host` — Sonarr host, network, authentication, proxy, and backup settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bind_address` | string | no |  | IP address or hostname Sonarr binds to; `*` binds to all interfaces. |
| `port` | integer | yes |  | HTTP port Sonarr listens on. |
| `ssl_port` | integer | yes |  | HTTPS port Sonarr listens on when SSL is enabled. |
| `enable_ssl` | boolean | yes |  | Enables HTTPS/TLS for the Sonarr web UI. |
| `launch_browser` | boolean | yes |  | Opens the Sonarr web UI in the default browser on startup. |
| `authentication_method` | string | no | `none` | Authentication method for the Sonarr web UI: `none`, `basic`, `forms`, or `external`. |
| `authentication_required` | string | no | `enabled` | Whether authentication is required: `enabled` or `disabledForLocalAddresses`. |
| `analytics_enabled` | boolean | yes |  | Sends anonymised usage and error data to the Sonarr team. |
| `username` | string | no |  | Username for basic or forms authentication. |
| `password` | secret string | no |  | Password for basic or forms authentication. Credential — redacted in plan output. |
| `password_confirmation` | secret string | no |  | Password confirmation field; must match `password` when changing credentials. Credential — redacted in plan output. |
| `log_level` | string | no |  | Log verbosity level (e.g. `info`, `debug`, `trace`). |
| `log_size_limit` | integer | yes |  | Maximum size in MB for each log file before it is rotated. |
| `console_log_level` | string | no |  | Log level for console output; overrides `log_level` for stdout. |
| `branch` | string | no |  | Update channel or branch Sonarr checks for updates (e.g. `main`, `develop`). |
| `api_key` | secret string | no |  | Sonarr API key used to authenticate API requests. Credential — redacted in plan output. |
| `ssl_cert_path` | string | no |  | Absolute path to the SSL certificate file (PEM/PFX). |
| `ssl_cert_password` | secret string | no |  | Password for the SSL certificate if it is password-protected. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path for reverse-proxy deployments (e.g. `/sonarr`). |
| `instance_name` | string | no |  | Display name for this Sonarr instance shown in the browser title and notifications. |
| `application_url` | string | no |  | Externally reachable URL for this instance, used in notifications. |
| `update_automatically` | boolean | yes |  | Allows Sonarr to update itself automatically when a new version is available. |
| `update_mechanism` | string | no | `docker` | How Sonarr applies updates: `builtIn`, `script`, `external`, `apt`, or `docker`. |
| `update_script_path` | string | no |  | Absolute path to the update script; required when `update_mechanism` is `script`. |
| `proxy_enabled` | boolean | yes |  | Routes Sonarr's outbound HTTP traffic through a proxy server. |
| `proxy_type` | string | no | `http` | Proxy protocol: `http`, `socks4`, or `socks5`. |
| `proxy_hostname` | string | no |  | Hostname or IP address of the proxy server. |
| `proxy_port` | integer | yes |  | Port of the proxy server. |
| `proxy_username` | string | no |  | Username for proxy authentication. |
| `proxy_password` | secret string | no |  | Password for proxy authentication. Credential — redacted in plan output. |
| `proxy_bypass_filter` | string | no |  | Comma-separated list of hosts or IP ranges that bypass the proxy. |
| `proxy_bypass_local_addresses` | boolean | yes |  | Bypasses the proxy for connections to local/private addresses. |
| `certificate_validation` | string | no | `enabled` | TLS certificate validation mode: `enabled`, `disabledForLocalAddresses`, or `disabled`. |
| `backup_folder` | string | no |  | Folder path where Sonarr stores automatic database backups. |
| `backup_interval` | integer | yes |  | Interval in days between automatic backups. |
| `backup_retention` | integer | yes |  | Number of days to retain automatic backups before they are deleted. |
| `trust_cgnat_ip_addresses` | boolean | yes |  | Trusts Carrier-Grade NAT (CGNAT) IP address ranges for source IP determination. |

## Types

### Quality Profile Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id (leaf items only; absent on group items). |
| `name` | string | no |  | Group or quality tier label displayed in the UI, e.g. `"WEB 1080p"`. |
| `quality` | [`quality`](#quality) | no |  | Quality definition for leaf items; `None` for group items. |
| `items` | array of [`quality_profile_item`](#quality-profile-item) | no |  | Nested group members — empty for leaf items. |
| `allowed` | boolean | yes |  | When `true`, Sonarr will accept releases at this quality tier. |
| `min_size` | number | no |  | Minimum acceptable file size in megabytes; `None` uses the global default. |
| `max_size` | number | no |  | Maximum acceptable file size in megabytes; `None` uses the global default. |
| `preferred_size` | number | no |  | Preferred file size in megabytes; `None` uses the global default. |

### Profile Format Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id. |
| `format` | integer | yes |  | Id of the custom format being scored; resolved from `${ref.custom_format.<name>}` at apply. References a [`custom_format`](#custom-format) by name (`${ref.custom_format.<key>}`). |
| `name` | string | no |  | Custom-format name, mirrored from the format definition. |
| `score` | integer | no |  | Points awarded to a release matching this format; negative values penalise. |

### Language

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned language id (e.g. `1` for English, `2` for French). |
| `name` | string | no |  | Language name, e.g. `"English"`. |

### Language Profile Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned item id. |
| `language` | [`language`](#language) | no |  | The language this entry represents. |
| `allowed` | boolean | yes |  | When `true`, Sonarr will accept releases in this language. |

### Download Protocol

Allowed values: `usenet` / `torrent`.

### Quality

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  | Server-assigned quality definition id. |
| `name` | string | no |  | Quality tier label, e.g. `"HDTV-1080p"`. |
| `source` | [`quality_source`](#quality-source) | no |  | Acquisition source medium. |
| `resolution` | integer | no |  | Vertical pixel resolution, e.g. `1080`. |

### Quality Source

Allowed values: `television` / `televisionRaw` / `web` / `webRip` / `dvd` / `bluray` / `blurayRaw`.

