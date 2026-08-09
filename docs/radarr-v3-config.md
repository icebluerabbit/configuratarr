# Radarr v3 Configuration

Radarr v3 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Tag

A label applied to movies, indexers, download clients, etc.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the name referenced in `${ref.tag.<label>}`. |

### Quality Profile

Named quality profile — ordered quality ladder with format-score gates.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.quality_profile.<name>}`. |
| `upgrade_allowed` | boolean | yes |  | When `true`, Radarr will seek a better-quality release after the initial download. |
| `cutoff` | integer | yes |  | Id of the cutoff quality; Radarr will not seek upgrades past this point. |
| `items` | array of [`quality_profile_item`](#quality-profile-item) | no |  | Ordered quality ladder — all quality tiers and groups this profile considers. |
| `min_format_score` | integer | yes |  | Minimum aggregate custom-format score a release must reach to be grabbed. |
| `cutoff_format_score` | integer | yes |  | Minimum format score that satisfies the upgrade cutoff. |
| `min_upgrade_format_score` | integer | yes |  | Minimum improvement in custom-format score required to trigger an upgrade. |
| `format_items` | array of [`profile_format_item`](#profile-format-item) | no |  | Custom-format score contributions attached to this profile. |
| `language` | [`language`](#language) | no |  | Language requirement for grabbed releases. |

### Custom Format

A custom format — a named collection of specification conditions Radarr uses
to score releases. The score influences download decisions via quality profiles.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.custom_format.<name>}`. |
| `include_custom_format_when_renaming` | boolean | no |  | When true, the format name is included in Radarr's file rename template. |
| `specifications` | array of any | no |  | Specification conditions, each a provider-shaped object (`implementation` + `fields[]`). Raw JSON — see the module docs. |

### Download Client

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enable` | boolean | yes |  | Whether this download client is active. |
| `protocol` | [`download_protocol`](#download-protocol) | yes |  | Download protocol used by this client (torrent or usenet). |
| `priority` | integer | no | `1` | Client priority relative to other configured download clients. |
| `remove_completed_downloads` | boolean | no | `true` | Remove downloads from the client once Radarr has imported them. |
| `remove_failed_downloads` | boolean | yes |  | Remove downloads from the client if they fail to complete. |

Set `implementation` to one of: [`Aria2`](#download-client-aria2) / [`Deluge`](#download-client-deluge) / [`Flood`](#download-client-flood) / [`TorrentFreeboxDownload`](#download-client-freebox) / [`Hadouken`](#download-client-hadouken) / [`Nzbget`](#download-client-nzbget) / [`Nzbvortex`](#download-client-nzbvortex) / [`Pneumatic`](#download-client-pneumatic) / [`QBittorrent`](#download-client-qbittorrent) / [`RTorrent`](#download-client-rtorrent) / [`Sabnzbd`](#download-client-sabnzbd) / [`TorrentBlackhole`](#download-client-torrentblackhole) / [`TorrentDownloadStation`](#download-client-torrentdownloadstation) / [`Transmission`](#download-client-transmission) / [`UsenetBlackhole`](#download-client-usenetblackhole) / [`UsenetDownloadStation`](#download-client-usenetdownloadstation) / [`UTorrent`](#download-client-utorrent) / [`Vuze`](#download-client-vuze).

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
| `port` | integer | no |  | TCP port the Deluge daemon listens on. |
| `password` | secret string | no |  | Password for authenticating with the Deluge daemon. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if Deluge is hosted behind a reverse proxy. |
| `movie_category` | string | no |  | Label assigned to movie downloads in Deluge. |
| `movie_imported_category` | string | no |  | Label the client moves completed downloads to after Radarr imports them. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
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
| `field_tags` | array of string | no |  | Tags applied to the torrent in Flood (string labels, not Radarr tag ids) |
| `additional_tags` | array of integer | no |  | Additional Radarr-managed metadata tags appended to the torrent (integer codes). |
| `post_import_tags` | array of string | no |  | Tags applied to the torrent in Flood after Radarr imports it. |

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
| `category` | string | no |  | Category assigned to movie torrents in Hadouken. |
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

#### Download Client: Nzbvortex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBVortex server. |
| `port` | integer | no |  | TCP port the NZBVortex server listens on. |
| `api_key` | secret string | no |  | API key used to authenticate with NZBVortex. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if NZBVortex is hosted behind a reverse proxy. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |

#### Download Client: Pneumatic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Radarr drops NZB files into for Pneumatic to pick up. |
| `strm_folder` | string | no |  | Folder Pneumatic writes `.strm` stream files to. |

#### Download Client: QBittorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the qBittorrent server. |
| `port` | integer | no |  | TCP port the qBittorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with qBittorrent. |
| `password` | secret string | no |  | Password for authenticating with qBittorrent. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category assigned to movie downloads in qBittorrent. |
| `movie_imported_category` | string | no |  | Category the client moves completed downloads to after Radarr imports them. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `initial_state` | integer | no |  | 0 = Start, 1 = ForceStart, 2 = Pause |
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
| `movie_category` | string | no |  | Label assigned to movie torrents in rTorrent. |
| `movie_directory` | string | no |  | Directory rTorrent saves movie downloads to. |
| `movie_imported_category` | string | no |  | Label the client moves completed downloads to after Radarr imports them. |
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
| `torrent_folder` | string | no |  | Folder Radarr drops `.torrent` files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Radarr watches for completed downloads from the external client. |
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

#### Download Client: Transmission

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Transmission server. |
| `port` | integer | no |  | TCP port the Transmission RPC interface listens on. |
| `username` | string | no |  | Username for authenticating with Transmission. |
| `password` | secret string | no |  | Password for authenticating with Transmission. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category (label) assigned to movie downloads in Transmission. |
| `movie_directory` | string | no |  | Directory Transmission saves movie downloads to. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `url_base` | string | no |  | URL base path if Transmission is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Transmission in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Transmission over HTTPS. |

#### Download Client: UsenetBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Folder Radarr drops NZB files into for an external client to pick up. |
| `watch_folder` | string | no |  | Folder Radarr watches for completed downloads from the external client. |

#### Download Client: UsenetDownloadStation

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Synology NAS running Download Station. |
| `port` | integer | no |  | TCP port the Synology DSM web interface listens on. |
| `username` | string | no |  | Username for authenticating with Synology DSM. |
| `password` | secret string | no |  | Password for authenticating with Synology DSM. Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Connect to Synology DSM over HTTPS. |

#### Download Client: UTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the uTorrent web UI. |
| `port` | integer | no |  | TCP port the uTorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with uTorrent. |
| `password` | secret string | no |  | Password for authenticating with uTorrent. Credential — redacted in plan output. |
| `movie_category` | string | no |  | Category assigned to movie downloads in uTorrent. |
| `movie_imported_category` | string | no |  | Category the client moves completed downloads to after Radarr imports them. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `initial_state` | integer | no |  | Initial state. 0 = Start, 1 = ForceStart, 2 = Pause, 3 = Stop Note: the Terraform provider uses the intentional typo "intialState" (missing 'i') as the API field name for uTorrent. |
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
| `movie_directory` | string | no |  | Directory Vuze saves movie downloads to. |
| `recent_movie_priority` | integer | no |  | Priority for movies released in the last 14 days. |
| `older_movie_priority` | integer | no |  | Priority for movies released more than 14 days ago. |
| `url_base` | string | no |  | URL base path if Vuze is hosted behind a reverse proxy. |
| `add_paused` | boolean | no |  | Add torrents to Vuze in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Vuze over HTTPS. |

### Indexer

Indexer definition — connects Radarr to a usenet or torrent search source.

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

Set `implementation` to one of: [`FileList`](#indexer-filelist) / [`HDBits`](#indexer-hdbits) / [`IPTorrents`](#indexer-iptorrents) / [`Newznab`](#indexer-newznab) / [`Nyaa`](#indexer-nyaa) / [`PassThePopcorn`](#indexer-passthepopcorn) / [`TorrentPotato`](#indexer-torrentpotato) / [`TorrentRssIndexer`](#indexer-torrentrss) / [`Torznab`](#indexer-torznab).

#### Indexer: FileList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the FileList tracker. |
| `username` | string | yes |  | FileList account username. |
| `passkey` | secret string | no |  | FileList account passkey used for API authentication. Credential — redacted in plan output. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `categories` | array of integer | no |  | FileList category IDs to include in searches. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

#### Indexer: HdBits

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the HDBits tracker. |
| `username` | string | yes |  | HDBits account username. |
| `api_key` | secret string | no |  | HDBits API key for authentication. Credential — redacted in plan output. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `categories` | array of integer | no |  | HDBits category IDs to include in searches. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |
| `codecs` | array of integer | no |  | HDBits codec filter IDs; empty means no codec restriction. |
| `mediums` | array of integer | no |  | HDBits medium (source) filter IDs; empty means no medium restriction. |

#### Indexer: IpTorrents

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | RSS feed URL including the user's passkey (provided by IPTorrents). |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

#### Indexer: Newznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Newznab indexer. |
| `api_path` | string | no |  | URL path to the Newznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Newznab indexer. Credential — redacted in plan output. |
| `remove_year` | boolean | yes |  | Strip the release year from search queries before sending them to the indexer. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `categories` | array of integer | no |  | Newznab category IDs to include in searches. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |

#### Indexer: Nyaa

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Nyaa indexer. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

#### Indexer: PassThePopcorn

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the PassThePopcorn tracker. |
| `api_user` | string | no |  | PTP API username used alongside the API key. |
| `api_key` | secret string | no |  | PTP API key for authentication. Credential — redacted in plan output. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

#### Indexer: TorrentPotato

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the TorrentPotato-compatible indexer. |
| `user` | string | no |  | Username for TorrentPotato authentication. |
| `passkey` | secret string | no |  | Passkey for TorrentPotato authentication. Credential — redacted in plan output. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

#### Indexer: TorrentRss

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | URL of the torrent RSS feed. |
| `cookie` | secret string | no |  | Session cookie sent with RSS requests for authenticated feeds. Credential — redacted in plan output. |
| `allow_zero_size` | boolean | yes |  | Allow releases that report a size of zero bytes. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

#### Indexer: Torznab

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | yes |  | Base URL of the Torznab indexer. |
| `api_path` | string | no |  | URL path to the Torznab API endpoint, appended to base_url. |
| `api_key` | secret string | no |  | API key for authenticating requests to the Torznab indexer. Credential — redacted in plan output. |
| `remove_year` | boolean | yes |  | Strip the release year from search queries before sending them to the indexer. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders a torrent must have to be grabbed. |
| `seed_time` | integer | no |  | Minimum seeding time in minutes Radarr must seed after download. |
| `seed_ratio` | number | no |  | Minimum seed ratio Radarr must reach before stopping seeding. |
| `additional_parameters` | string | no |  | Extra query string parameters appended verbatim to every API request. |
| `categories` | array of integer | no |  | Torznab category IDs to include in searches. |
| `multi_languages` | array of integer | no |  | Language IDs to treat as multi-language releases. |
| `required_flags` | array of integer | no |  | Tracker-specific flag IDs that a release must carry to be grabbed. |

### Notification

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `on_grab` | boolean | yes |  | Fire notification when a release is grabbed for download. |
| `on_download` | boolean | yes |  | Fire notification when a movie file is imported after download. |
| `on_upgrade` | boolean | yes |  | Fire notification when a file is upgraded to a higher-quality version. |
| `on_rename` | boolean | yes |  | Fire notification when a movie file is renamed. |
| `on_movie_added` | boolean | yes |  | Fire notification when a movie is added to the Radarr library. |
| `on_movie_delete` | boolean | yes |  | Fire notification when a movie is deleted from the library. |
| `on_movie_file_delete` | boolean | yes |  | Fire notification when a movie file is deleted. |
| `on_movie_file_delete_for_upgrade` | boolean | yes |  | Fire notification when a file is deleted to make room for an upgrade. |
| `on_health_issue` | boolean | yes |  | Fire notification when a health-check issue is detected. |
| `on_health_restored` | boolean | yes |  | Fire notification when a previously detected health-check issue is resolved. |
| `on_application_update` | boolean | yes |  | Fire notification when a Radarr application update is available. |
| `on_manual_interaction_required` | boolean | yes |  | Fire notification when a download requires manual interaction. |
| `include_health_warnings` | boolean | yes |  | Include health warnings (not just errors) in health-issue notifications. |

Set `implementation` to one of: [`Apprise`](#notification-apprise) / [`CustomScript`](#notification-customscript) / [`Discord`](#notification-discord) / [`Email`](#notification-email) / [`MediaBrowser`](#notification-emby) / [`Gotify`](#notification-gotify) / [`Join`](#notification-join) / [`Xbmc`](#notification-kodi) / [`Mailgun`](#notification-mailgun) / [`Notifiarr`](#notification-notifiarr) / [`Ntfy`](#notification-ntfy) / [`PlexServer`](#notification-plex) / [`Prowl`](#notification-prowl) / [`PushBullet`](#notification-pushbullet) / [`Pushover`](#notification-pushover) / [`Sendgrid`](#notification-sendgrid) / [`Simplepush`](#notification-simplepush) / [`Slack`](#notification-slack) / [`SynologyIndexer`](#notification-synologyindexer) / [`Telegram`](#notification-telegram) / [`Trakt`](#notification-trakt) / [`Twitter`](#notification-twitter) / [`Webhook`](#notification-webhook).

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
| `path` | string | yes |  | Absolute filesystem path to the script to execute. |
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
| `update_library` | boolean | no |  | Trigger an Emby library refresh after a movie is imported. |

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
| `update_library` | boolean | no |  | Trigger a Kodi video library update after a movie is imported. |
| `clean_library` | boolean | no |  | Trigger a Kodi video library clean after a movie is deleted. |
| `always_update` | boolean | no |  | Always update the library on every event, not just import events. |

#### Notification: Mailgun

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Mailgun API key for authentication. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `sender_domain` | string | yes |  | Mailgun sending domain registered in your account. |
| `use_eu_endpoint` | boolean | no |  | Use the EU Mailgun API endpoint instead of the US endpoint. |
| `recipients` | array of string | no |  | Recipient email addresses. |

#### Notification: Notifiarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Notifiarr API key for authentication. Credential — redacted in plan output. |

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
| `update_library` | boolean | no |  | Trigger a Plex library section refresh after a movie is imported. |

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
| `update_library` | boolean | no |  | Trigger a Synology media library update after a movie is imported. |

#### Notification: Telegram

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bot_token` | secret string | yes |  | Telegram bot token issued by BotFather. Credential — redacted in plan output. |
| `chat_id` | string | yes |  | Target chat, group, or channel ID to send messages to. |
| `topic_id` | string | no |  | Topic (message thread) ID for supergroup forums. |
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

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `enabled` | boolean | yes |  | Whether the import list is active and will be processed. |
| `enable_auto` | boolean | yes |  | Whether movies from this list are automatically added to Radarr. |
| `monitor` | string | no |  | Monitoring strategy applied to added movies (e.g. `"movieOnly"`, `"movieAndCollection"`). |
| `root_folder_path` | string | no |  | Root folder where movies added by this list are placed. |
| `quality_profile_id` | integer | yes |  | Quality profile assigned to movies added by this list. References a [`quality_profile`](#quality-profile) by name (`${ref.quality_profile.<key>}`). |
| `search_on_add` | boolean | yes |  | Whether to trigger an immediate search when a movie is added. |
| `minimum_availability` | string | no |  | Minimum availability status before a movie is considered (e.g. `"announced"`, `"inCinemas"`, `"released"`). |
| `list_order` | integer | yes |  | Display sort order of this list in the UI. |

Set `implementation` to one of: [`CouchPotatoImport`](#import-list-couchpotato) / [`RadarrListImport`](#import-list-custom) / [`IMDbListImport`](#import-list-imdb) / [`PlexImport`](#import-list-plex) / [`RadarrImport`](#import-list-radarr) / [`RSSImport`](#import-list-rss) / [`StevenLuImport`](#import-list-stevenlu) / [`Stevenlu2Import`](#import-list-stevenlu2) / [`TMDbCompanyImport`](#import-list-tmdbcompany) / [`TMDbKeywordImport`](#import-list-tmdbkeyword) / [`TMDbListImport`](#import-list-tmdblist) / [`TMDbPersonImport`](#import-list-tmdbperson) / [`TMDbPopularImport`](#import-list-tmdbpopular) / [`TMDbUserImport`](#import-list-tmdbuser) / [`TraktListImport`](#import-list-traktlist) / [`TraktPopularImport`](#import-list-traktpopular) / [`TraktUserImport`](#import-list-traktuser).

#### Import List: CouchPotato

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `link` | string | no |  | Host or IP address of the CouchPotato instance. |
| `port` | integer | no |  | Port number the CouchPotato instance listens on. |
| `url_base` | string | no |  | URL path prefix if CouchPotato is served under a sub-path. |
| `api_key` | secret string | no |  | API key for authenticating with the CouchPotato instance. Credential — redacted in plan output. |
| `only_active` | boolean | no |  | When true, imports only movies in an active (wanted) state. |

#### Import List: Custom

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `url` | string | no |  | URL of the custom Radarr-compatible list endpoint. |

#### Import List: Imdb

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_id` | string | no |  | IMDb list identifier (e.g. `"ls012345678"`). |

#### Import List: Plex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | Plex authentication token granting access to the user's watchlist. Credential — redacted in plan output. |

#### Import List: Radarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Base URL of the source Radarr instance. |
| `api_key` | secret string | no |  | API key for authenticating with the source Radarr instance. Credential — redacted in plan output. |
| `profile_ids` | array of integer | no |  | Profile IDs to import from — stored as a JSON array of ints. |
| `tag_ids` | array of integer | no |  | Tag IDs to filter by — stored as a JSON array of ints. |

#### Import List: Rss

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `link` | string | no |  | URL of the RSS feed to import movies from. |

#### Import List: StevenLu

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `link` | string | no |  | URL of the StevenLu popular movies list feed. |

#### Import List: StevenLu2

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `source` | integer | no |  | Source list selector for the StevenLu2 feed (integer enum). |
| `min_score` | integer | no |  | Minimum score threshold for a movie to be included. |

#### Import List: TmdbCompany

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `company_id` | string | no |  | TMDb company identifier whose productions are imported. |

#### Import List: TmdbKeyword

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `keyword_id` | string | no |  | TMDb keyword identifier used to filter movies. |

#### Import List: TmdbList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `list_id` | string | no |  | TMDb list identifier to import movies from. |

#### Import List: TmdbPerson

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `person_id` | string | no |  | TMDb person identifier whose associated movies are imported. |
| `person_cast` | boolean | no |  | Include movies where the person appears as a cast member. |
| `person_cast_director` | boolean | no |  | Include movies where the person is credited as director. |
| `person_cast_producer` | boolean | no |  | Include movies where the person is credited as producer. |
| `person_cast_sound` | boolean | no |  | Include movies where the person has a sound department credit. |
| `person_cast_writing` | boolean | no |  | Include movies where the person is credited as a writer. |

#### Import List: TmdbPopular

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tmdb_list_type` | integer | no |  | Category of popular movies to import (integer enum; e.g. popular, top-rated, upcoming). |
| `language_code` | integer | no |  | Language filter for returned movies (integer enum corresponding to a language code). |
| `min_vote_average` | string | no |  | filterCriteria.minVoteAverage — minimum vote average filter. |
| `min_votes` | string | no |  | filterCriteria.minVotes — minimum vote count filter. |
| `certification` | string | no |  | filterCriteria.certification — certification filter (e.g. "PG-13"). |
| `include_genre_ids` | string | no |  | filterCriteria.includeGenreIds — genre IDs to include. |
| `exclude_genre_ids` | string | no |  | filterCriteria.excludeGenreIds — genre IDs to exclude. |

#### Import List: TmdbUser

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `account_id` | string | no |  | TMDb account identifier for the target user. |
| `access_token` | secret string | no |  | TMDb v4 read access token for the user's account. Credential — redacted in plan output. |
| `user_list_type` | integer | no |  | Type of user list to import (integer enum; e.g. favorites, watchlist, rated). |

#### Import List: TraktList

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for authenticating with the Trakt API. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 expiry timestamp for the access token. |
| `auth_user` | secret string | no |  | Trakt username associated with the OAuth credentials. Credential — redacted in plan output. |
| `username` | string | no |  | Trakt username whose list is being imported. |
| `listname` | string | no |  | Slug name of the Trakt list to import. |
| `limit` | integer | no |  | Maximum number of movies to import from the list. |

#### Import List: TraktPopular

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for authenticating with the Trakt API. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 expiry timestamp for the access token. |
| `auth_user` | secret string | no |  | Trakt username associated with the OAuth credentials. Credential — redacted in plan output. |
| `trakt_list_type` | integer | no |  | Category of popular movies to import (integer enum; e.g. popular, trending, anticipated). |
| `limit` | integer | no |  | Maximum number of movies to import. |
| `certification` | string | no |  | Content certification filter (e.g. `"pg-13"`). |
| `genres` | string | no |  | Comma-separated list of genre slugs to include. |
| `years` | string | no |  | Year or year range filter (e.g. `"2010"` or `"2010-2020"`). |
| `rating` | string | no |  | Score range filter (e.g. `"70-100"`). |
| `trakt_additional_parameters` | string | no |  | Extra query parameters appended to the Trakt API request. |

#### Import List: TraktUser

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `access_token` | secret string | no |  | OAuth access token for authenticating with the Trakt API. Credential — redacted in plan output. |
| `refresh_token` | secret string | no |  | OAuth refresh token used to renew the access token. Credential — redacted in plan output. |
| `expires` | string | no |  | ISO 8601 expiry timestamp for the access token. |
| `auth_user` | secret string | no |  | Trakt username associated with the OAuth credentials. Credential — redacted in plan output. |
| `username` | string | no |  | Trakt username whose list is being imported. |
| `trakt_list_type` | integer | no |  | User list type to import (integer enum; e.g. watchlist, watched, collection). |
| `limit` | integer | no |  | Maximum number of movies to import. |

### Root Folder

A root folder Radarr watches for movies.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `path` | string | yes |  | Natural key — the absolute filesystem path. |

### Import List Exclusion

A movie excluded from all import lists, keyed by its TMDB id.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tmdb_id` | integer | yes |  | Natural key — TMDB movie id, uniquely identifies the excluded movie. |
| `movie_title` | string | no |  | Title of the excluded movie, stored for display purposes. |
| `movie_year` | integer | yes |  | Release year of the excluded movie. |

### Auto Tag

Automatic tagging rule — applies tags to movies matching its specifications.

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
| `minimum_free_space_when_importing` | integer | no | `100` | Minimum free disk space in MB required on the destination before Radarr will import. |
| `copy_using_hardlinks` | boolean | no | `true` | Uses hardlinks instead of copying when source and destination are on the same filesystem. |
| `use_script_import` | boolean | yes |  | Delegates file import handling to an external script instead of the built-in importer. |
| `script_import_path` | string | no |  | Absolute path to the script used for custom imports; required when `use_script_import` is true. |
| `import_extra_files` | boolean | yes |  | Imports extra files (subtitles, NFO, etc.) alongside the movie file. |
| `extra_file_extensions` | string | no |  | Comma-separated list of file extensions to import alongside the movie file (e.g. `srt,nfo`). |
| `enable_media_info` | boolean | no | `true` | Reads and stores media info (codec, resolution, audio channels) for imported files. |

### Naming

`/api/v3/config/naming` — movie file and folder naming configuration.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `rename_movies` | boolean | yes |  | Renames existing movie files to match the configured naming format on import or refresh. |
| `replace_illegal_characters` | boolean | no | `true` | Replaces characters that are illegal on common filesystems in file and folder names. |
| `colon_replacement_format` | string | no | `delete` | How to handle colons in movie titles: `delete`, `dash`, `spaceDash`, `spaceDashSpace`, or `smart`. |
| `standard_movie_format` | string | no |  | Naming template string for movie files; uses Radarr naming tokens (e.g. `{Movie Title}`). |
| `movie_folder_format` | string | no |  | Naming template string for movie folders; uses Radarr naming tokens. |

### Metadata Config

`/api/v3/config/metadata` — certification country for metadata lookups.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `certification_country` | string | no |  | ISO country code used to select the content rating system (e.g. `us` for MPAA, `gb` for BBFC). |

### Ui Config

`/api/v3/config/ui` — UI display and localisation settings.

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
| `movie_info_language` | integer | yes |  | Language ID for displaying movie titles and metadata in the UI. |
| `ui_language` | integer | yes |  | Language ID for the Radarr UI interface itself. |
| `theme` | string | no |  | UI colour theme name (e.g. `dark`, `light`, `auto`). |

### Indexer Config

`/api/v3/config/indexer` — global indexer and RSS sync settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `minimum_age` | integer | yes |  | Minimum age in minutes a Usenet release must be before Radarr will grab it. |
| `maximum_size` | integer | yes |  | Maximum release size in MB that Radarr will grab; 0 = unlimited. |
| `retention` | integer | yes |  | Usenet retention period in days; 0 = unlimited. |
| `rss_sync_interval` | integer | no | `60` | Interval in minutes between RSS feed syncs; 0 = disable RSS sync. |
| `prefer_indexer_flags` | boolean | yes |  | Prefers releases flagged by indexers (e.g. freeleech on torrents) when scoring candidates. |
| `availability_delay` | integer | yes |  | Number of days before (`-`) or after (`+`) a movie's availability date to start searching. |
| `allow_hardcoded_subs` | boolean | yes |  | Allows grabbing releases that contain hardcoded (burned-in) subtitles. |
| `whitelisted_hardcoded_subs` | string | no |  | Comma-separated list of subtitle language codes whose hardcoded releases are permitted. |

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

## Types

### Quality Profile Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | integer | no |  |  |
| `name` | string | no |  | Group or quality tier label displayed in the UI, e.g. `"HD Bluray"`. |
| `quality` | [`quality`](#quality) | no |  | Quality definition for leaf items; `None` for group items. |
| `items` | array of [`quality_profile_item`](#quality-profile-item) | no |  | Nested group members — empty for leaf items. |
| `allowed` | boolean | yes |  | When `true`, Radarr will accept releases at this quality tier. |

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
| `name` | string | no |  | Quality tier name, e.g. `"Bluray-2160p"`. |
| `source` | string | no |  | Source medium string, e.g. `"bluray"`, `"webdl"`. |
| `resolution` | integer | no |  | Vertical pixel resolution for this quality tier, e.g. `2160`. |
| `modifier` | string | no |  | Modifier string, e.g. `"none"`, `"remux"`. |

