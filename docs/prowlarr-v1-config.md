# Prowlarr v1 Configuration

Prowlarr v1 — desired-state config for one instance.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Tag

A label applied to indexers, applications, download clients, and other resources
to enable grouping and filtering.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the name referenced in `${ref.tag.<label>}`. |

### App Profile

Application sync profile — controls how Prowlarr syncs indexers to connected
applications (Sonarr, Radarr, etc.).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — the display name for this application profile. |
| `enable_rss` | boolean | yes |  | Enables RSS feed syncing for indexers using this profile. |
| `enable_automatic_search` | boolean | yes |  | Enables automatic (monitored) search syncing for indexers using this profile. |
| `enable_interactive_search` | boolean | yes |  | Enables interactive (manual) search syncing for indexers using this profile. |
| `minimum_seeders` | integer | yes |  | Minimum number of seeders required when syncing torrent indexers. |

### Custom Filter

A saved custom filter for a Prowlarr UI page.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `label` | string | yes |  | Natural key — the user-visible label for this filter. |
| `filter_type` | string | no |  | The UI page context this filter applies to. Wire name is `type` (a Rust keyword). |
| `filters` | array of any | no |  | Filter conditions, each a raw object with `key`, `value`, and `type`. Raw JSON — the condition shape is not described in the static spec. |

### Indexer

Indexer definition — connects Prowlarr to a usenet or torrent search source.

Create/update use `?forceSave=true`: Prowlarr otherwise runs a live
connectivity test against the indexer on save and rejects (HTTP 400) when the
site is unreachable *from the host running configuratarr* — but the tracker
may only be reachable through Prowlarr's own proxy/FlareSolverr, or be
transiently down. A declarative sync must converge to the desired config
regardless; Prowlarr still surfaces the failing health check afterward.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `implementation` | string | yes |  | Provider implementation class name, e.g. `"Transmission"`. |
| `config_contract` | string | no |  | Configuration contract class name that governs the fields schema. |
| `fields` | any | yes |  | Open provider settings authored as a `name: value` map. On the wire this becomes the *arr `fields: [{name, value}]` blob (via `#[fields_map]`); `${env}`/`${ref}` resolve inside the values, and the whole map is redacted (rendered `Complex`) in plan output. |
| `enable` | boolean | yes |  | Whether this indexer is active and included in searches. |
| `redirect` | boolean | yes |  | Whether Prowlarr should follow redirects when querying this indexer. |
| `priority` | integer | no | `25` | Indexer priority; lower values are preferred when multiple indexers match. |
| `protocol` | [`download_protocol`](#download-protocol) | yes |  | Transport protocol used by this indexer. |
| `privacy` | [`indexer_privacy`](#indexer-privacy) | yes |  | Privacy level of this indexer (public, semi-private, or private). |
| `language` | string | no |  | Language of content indexed by this indexer. |
| `app_profile_id` | integer | no |  | Application profile controlling how this indexer is synced to applications. References a [`app_profile`](#app-profile) by name (`${ref.app_profile.<key>}`). |
| `download_client_id` | integer | no |  | Download client to use exclusively for grabs from this indexer; absent means use the default client. References a [`download_client`](#download-client) by name (`${ref.download_client.<key>}`). |

### Indexer Proxy

A Prowlarr indexer proxy — routes requests for specific indexers through an
intermediate proxy (FlareSolverr, HTTP CONNECT, SOCKS4, or SOCKS5) to
bypass bot-protection or access geo-restricted indexers.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `on_health_issue` | boolean | yes |  | Fire notification/action when a health-check issue is detected. |
| `include_health_warnings` | boolean | yes |  | Include health warnings (not just errors) in health notifications. |

Set `implementation` to one of: [`Flaresolverr`](#indexer-proxy-flaresolverr) / [`HTTP`](#indexer-proxy-http) / [`Socks4`](#indexer-proxy-socks4) / [`Socks5`](#indexer-proxy-socks5).

#### Indexer Proxy: FlareSolverr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | URL of the FlareSolverr instance, e.g. `http://localhost:8191`. |
| `request_timeout` | integer | no |  | Request timeout in seconds sent to the FlareSolverr session. |

#### Indexer Proxy: Http

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the HTTP proxy server. |
| `port` | integer | no |  | TCP port the HTTP proxy listens on. |
| `username` | string | no |  | Username for authenticating with the HTTP proxy. |
| `password` | secret string | no |  | Password for authenticating with the HTTP proxy. Credential — redacted in plan output. |

#### Indexer Proxy: Socks4

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the SOCKS4 proxy server. |
| `port` | integer | no |  | TCP port the SOCKS4 proxy listens on. |
| `username` | string | no |  | Username for authenticating with the SOCKS4 proxy. |
| `password` | secret string | no |  | Password for authenticating with the SOCKS4 proxy. Credential — redacted in plan output. |

#### Indexer Proxy: Socks5

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the SOCKS5 proxy server. |
| `port` | integer | no |  | TCP port the SOCKS5 proxy listens on. |
| `username` | string | no |  | Username for authenticating with the SOCKS5 proxy. |
| `password` | secret string | no |  | Password for authenticating with the SOCKS5 proxy. Credential — redacted in plan output. |

### Download Client

A Prowlarr download client (usenet or torrent).

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
| `categories` | array of [`download_client_category`](#download-client-category) | no |  | Category mappings between Prowlarr categories and client categories. |

Set `implementation` to one of: [`Aria2`](#download-client-aria2) / [`Deluge`](#download-client-deluge) / [`Flood`](#download-client-flood) / [`TorrentFreeboxDownload`](#download-client-freebox) / [`Hadouken`](#download-client-hadouken) / [`Nzbget`](#download-client-nzbget) / [`NzbVortex`](#download-client-nzbvortex) / [`Pneumatic`](#download-client-pneumatic) / [`QBittorrent`](#download-client-qbittorrent) / [`RTorrent`](#download-client-rtorrent) / [`Sabnzbd`](#download-client-sabnzbd) / [`TorrentBlackhole`](#download-client-torrentblackhole) / [`TorrentDownloadStation`](#download-client-torrentdownloadstation) / [`Transmission`](#download-client-transmission) / [`UsenetBlackhole`](#download-client-usenetblackhole) / [`UsenetDownloadStation`](#download-client-usenetdownloadstation) / [`UTorrent`](#download-client-utorrent) / [`Vuze`](#download-client-vuze).

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
| `host` | string | no |  | Hostname or IP address of the Deluge server. |
| `port` | integer | no |  | TCP port the Deluge web UI listens on. |
| `password` | secret string | no |  | Password for authenticating with Deluge. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if Deluge is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in Deluge. |
| `item_priority` | integer | no |  | Item priority when adding to Deluge. `0` = Last, `1` = First. |
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
| `field_tags` | array of string | no |  | Tags applied to the torrent in Flood (string labels, not Prowlarr tag ids). |
| `additional_tags` | array of integer | no |  | Additional Prowlarr-managed metadata tags appended to the torrent (integer codes). |

#### Download Client: Freebox

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Freebox server. |
| `port` | integer | no |  | TCP port the Freebox API listens on. |
| `api_url` | string | no |  | URL of the Freebox API endpoint. |
| `app_id` | string | no |  | Application identifier registered with the Freebox OS. |
| `app_token` | secret string | no |  | Application token for authenticating with the Freebox OS. Credential — redacted in plan output. |
| `category` | string | no |  | Category label assigned to downloads on the Freebox. |
| `destination_directory` | string | no |  | Destination directory for completed downloads on the Freebox. |
| `item_priority` | integer | no |  | Item priority when adding to the Freebox. `0` = Last, `1` = First. |
| `add_paused` | boolean | no |  | Add torrents to the Freebox in a paused state. |
| `use_ssl` | boolean | no |  | Connect to the Freebox over HTTPS. |

#### Download Client: Hadouken

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Hadouken server. |
| `port` | integer | no |  | TCP port the Hadouken web UI listens on. |
| `username` | string | no |  | Username for authenticating with Hadouken. |
| `password` | secret string | no |  | Password for authenticating with Hadouken. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if Hadouken is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in Hadouken. |
| `use_ssl` | boolean | no |  | Connect to Hadouken over HTTPS. |

#### Download Client: Nzbget

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBGet server. |
| `port` | integer | no |  | TCP port the NZBGet web UI listens on. |
| `username` | string | no |  | Username for authenticating with NZBGet. |
| `password` | secret string | no |  | Password for authenticating with NZBGet. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if NZBGet is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in NZBGet. |
| `item_priority` | integer | no |  | Item priority when adding to NZBGet. `0` = Last, `1` = First. |
| `add_paused` | boolean | no |  | Add NZBs to NZBGet in a paused state. |
| `use_ssl` | boolean | no |  | Connect to NZBGet over HTTPS. |

#### Download Client: NzbVortex

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the NZBVortex server. |
| `port` | integer | no |  | TCP port the NZBVortex server listens on. |
| `api_key` | secret string | no |  | API key used to authenticate with NZBVortex. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if NZBVortex is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in NZBVortex. |
| `item_priority` | integer | no |  | Item priority when adding to NZBVortex. `0` = Last, `1` = First. |

#### Download Client: Pneumatic

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Directory to drop NZB files into for processing. |
| `strm_folder` | string | no |  | Directory for STRM files generated by Pneumatic. |

#### Download Client: QBittorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the qBittorrent server. |
| `port` | integer | no |  | TCP port the qBittorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with qBittorrent. |
| `password` | secret string | no |  | Password for authenticating with qBittorrent. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if qBittorrent is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in qBittorrent. |
| `item_priority` | integer | no |  | Item priority when adding to qBittorrent. `0` = Last, `1` = First. |
| `initial_state` | integer | no |  | Initial torrent state. `0` = Start, `1` = ForceStart, `2` = Pause. |
| `use_ssl` | boolean | no |  | Connect to qBittorrent over HTTPS. |

#### Download Client: RTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the rTorrent SCGI/HTTP endpoint. |
| `port` | integer | no |  | TCP port the rTorrent SCGI or HTTP interface listens on. |
| `username` | string | no |  | Username for authenticating with rTorrent (used when fronted by a web server). |
| `password` | secret string | no |  | Password for authenticating with rTorrent (used when fronted by a web server). Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if rTorrent is hosted behind a reverse proxy. |
| `category` | string | no |  | Label assigned to torrents in rTorrent. |
| `directory` | string | no |  | Directory rTorrent saves downloads to. |
| `item_priority` | integer | no |  | Item priority when adding to rTorrent. `0` = Last, `1` = First. |
| `add_stopped` | boolean | no |  | Add torrents to rTorrent in a stopped state rather than starting immediately. |
| `use_ssl` | boolean | no |  | Connect to rTorrent over HTTPS. |

#### Download Client: Sabnzbd

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the SABnzbd server. |
| `port` | integer | no |  | TCP port the SABnzbd web UI listens on. |
| `api_key` | secret string | no |  | SABnzbd API key used as an alternative to username/password auth. Credential — redacted in plan output. |
| `username` | string | no |  | Username for authenticating with SABnzbd. |
| `password` | secret string | no |  | Password for authenticating with SABnzbd. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if SABnzbd is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in SABnzbd. |
| `item_priority` | integer | no |  | Item priority when adding to SABnzbd. `0` = Last, `1` = First. |
| `use_ssl` | boolean | no |  | Connect to SABnzbd over HTTPS. |

#### Download Client: TorrentBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `torrent_folder` | string | no |  | Directory to drop `.torrent` files into for processing. |
| `magnet_file_extension` | string | no |  | File extension used for saved magnet link files (default: `.magnet`). |
| `save_magnet_files` | boolean | no |  | Save magnet links to files instead of launching them directly. |

#### Download Client: TorrentDownloadStation

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Synology NAS running Download Station. |
| `port` | integer | no |  | TCP port Download Station listens on. |
| `username` | string | no |  | Username for authenticating with Download Station. |
| `password` | secret string | no |  | Password for authenticating with Download Station. Credential — redacted in plan output. |
| `category` | string | no |  | Category label assigned to downloads in Download Station. |
| `station_directory` | string | no |  | Destination directory for completed downloads on the NAS.  Note: the Prowlarr API uses the wire key `tvDirectory` for this field for historical reasons. |
| `use_ssl` | boolean | no |  | Connect to Download Station over HTTPS. |

#### Download Client: Transmission

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Transmission server. |
| `port` | integer | no |  | TCP port the Transmission RPC interface listens on. |
| `username` | string | no |  | Username for authenticating with Transmission. |
| `password` | secret string | no |  | Password for authenticating with Transmission. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if Transmission is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in Transmission. |
| `directory` | string | no |  | Directory Transmission saves downloads to. |
| `item_priority` | integer | no |  | Item priority when adding to Transmission. `0` = Last, `1` = First. |
| `add_paused` | boolean | no |  | Add torrents to Transmission in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Transmission over HTTPS. |

#### Download Client: UsenetBlackhole

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `nzb_folder` | string | no |  | Directory to drop NZB files into for processing. |

#### Download Client: UsenetDownloadStation

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Synology NAS running Download Station. |
| `port` | integer | no |  | TCP port Download Station listens on. |
| `username` | string | no |  | Username for authenticating with Download Station. |
| `password` | secret string | no |  | Password for authenticating with Download Station. Credential — redacted in plan output. |
| `category` | string | no |  | Category label assigned to downloads in Download Station. |
| `station_directory` | string | no |  | Destination directory for completed downloads on the NAS.  Note: the Prowlarr API uses the wire key `tvDirectory` for this field for historical reasons. |
| `use_ssl` | boolean | no |  | Connect to Download Station over HTTPS. |

#### Download Client: UTorrent

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the µTorrent server. |
| `port` | integer | no |  | TCP port the µTorrent web UI listens on. |
| `username` | string | no |  | Username for authenticating with µTorrent. |
| `password` | secret string | no |  | Password for authenticating with µTorrent. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if µTorrent is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in µTorrent. |
| `item_priority` | integer | no |  | Item priority when adding to µTorrent. `0` = Last, `1` = First. |
| `initial_state` | integer | no |  | Initial state on add. `0` = Start, `1` = ForceStart, `2` = Pause, `3` = Stop.  Note: the Prowlarr API wire key is `intialState` (intentional upstream typo). |
| `use_ssl` | boolean | no |  | Connect to µTorrent over HTTPS. |

#### Download Client: Vuze

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `host` | string | no |  | Hostname or IP address of the Vuze server. |
| `port` | integer | no |  | TCP port the Vuze RPC interface listens on. |
| `username` | string | no |  | Username for authenticating with Vuze. |
| `password` | secret string | no |  | Password for authenticating with Vuze. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path if Vuze is hosted behind a reverse proxy. |
| `category` | string | no |  | Category label assigned to downloads in Vuze. |
| `directory` | string | no |  | Directory Vuze saves downloads to. |
| `item_priority` | integer | no |  | Item priority when adding to Vuze. `0` = Last, `1` = First. |
| `add_paused` | boolean | no |  | Add torrents to Vuze in a paused state. |
| `use_ssl` | boolean | no |  | Connect to Vuze over HTTPS. |

### Application

Prowlarr application — connects Prowlarr to an *arr app or other media manager
so that Prowlarr can push indexer definitions to it.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `sync_level` | [`application_sync_level`](#application-sync-level) | yes |  | How Prowlarr syncs indexers to this application. |

Set `implementation` to one of: [`LazyLibrarian`](#application-lazylibrarian) / [`Lidarr`](#application-lidarr) / [`Mylar`](#application-mylar) / [`Radarr`](#application-radarr) / [`Readarr`](#application-readarr) / [`Sonarr`](#application-sonarr) / [`Whisparr`](#application-whisparr).

#### Application: LazyLibrarian

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `prowlarr_url` | string | no |  | Prowlarr server URL that LazyLibrarian uses to pull indexers. |
| `base_url` | string | no |  | LazyLibrarian base URL. |
| `api_key` | secret string | no |  | LazyLibrarian API key for authenticating Prowlarr's push requests. Credential — redacted in plan output. |
| `sync_categories` | array of integer | no |  | Newznab category IDs to sync to LazyLibrarian. |

#### Application: Lidarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `prowlarr_url` | string | no |  | Prowlarr server URL that Lidarr uses to pull indexers. |
| `base_url` | string | no |  | Lidarr base URL. |
| `api_key` | secret string | no |  | Lidarr API key for authenticating Prowlarr's push requests. Credential — redacted in plan output. |
| `sync_categories` | array of integer | no |  | Newznab category IDs to sync to Lidarr. |

#### Application: Mylar

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `prowlarr_url` | string | no |  | Prowlarr server URL that Mylar uses to pull indexers. |
| `base_url` | string | no |  | Mylar base URL. |
| `api_key` | secret string | no |  | Mylar API key for authenticating Prowlarr's push requests. Credential — redacted in plan output. |
| `sync_categories` | array of integer | no |  | Newznab category IDs to sync to Mylar. |

#### Application: Radarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `prowlarr_url` | string | no |  | Prowlarr server URL that Radarr uses to pull indexers. |
| `base_url` | string | no |  | Radarr base URL. |
| `api_key` | secret string | no |  | Radarr API key for authenticating Prowlarr's push requests. Credential — redacted in plan output. |
| `sync_categories` | array of integer | no |  | Newznab category IDs to sync to Radarr. |

#### Application: Readarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `prowlarr_url` | string | no |  | Prowlarr server URL that Readarr uses to pull indexers. |
| `base_url` | string | no |  | Readarr base URL. |
| `api_key` | secret string | no |  | Readarr API key for authenticating Prowlarr's push requests. Credential — redacted in plan output. |
| `sync_categories` | array of integer | no |  | Newznab category IDs to sync to Readarr. |

#### Application: Sonarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `prowlarr_url` | string | no |  | Prowlarr server URL that Sonarr uses to pull indexers. |
| `base_url` | string | no |  | Sonarr base URL. |
| `api_key` | secret string | no |  | Sonarr API key for authenticating Prowlarr's push requests. Credential — redacted in plan output. |
| `sync_categories` | array of integer | no |  | Standard Newznab category IDs to sync to Sonarr. |
| `anime_sync_categories` | array of integer | no |  | Anime-specific Newznab category IDs to sync to Sonarr. |

#### Application: Whisparr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `prowlarr_url` | string | no |  | Prowlarr server URL that Whisparr uses to pull indexers. |
| `base_url` | string | no |  | Whisparr base URL. |
| `api_key` | secret string | no |  | Whisparr API key for authenticating Prowlarr's push requests. Credential — redacted in plan output. |
| `sync_categories` | array of integer | no |  | Newznab category IDs to sync to Whisparr. |

### Notification

A Prowlarr notification connection — routes indexer/health/update events to external services.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Provider instance name — the resource's natural key. |
| `tags` | array of integer | no |  | Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply. References a [`tag`](#tag) by name (`${ref.tag.<key>}`). |
| `on_grab` | boolean | yes |  | Fire notification when a release is grabbed for download. |
| `on_health_issue` | boolean | yes |  | Fire notification when a health-check issue is detected. |
| `on_health_restored` | boolean | yes |  | Fire notification when a previously detected health-check issue is resolved. |
| `on_application_update` | boolean | yes |  | Fire notification when a Prowlarr application update is available. |
| `include_manual_grabs` | boolean | yes |  | Include grabs triggered manually (not via an automated search). |
| `include_health_warnings` | boolean | yes |  | Include health warnings (not just errors) in health-issue notifications. |

Set `implementation` to one of: [`Apprise`](#notification-apprise) / [`CustomScript`](#notification-customscript) / [`Discord`](#notification-discord) / [`Email`](#notification-email) / [`Gotify`](#notification-gotify) / [`Join`](#notification-join) / [`Mailgun`](#notification-mailgun) / [`Notifiarr`](#notification-notifiarr) / [`Ntfy`](#notification-ntfy) / [`Prowl`](#notification-prowl) / [`PushBullet`](#notification-pushbullet) / [`Pushover`](#notification-pushover) / [`Sendgrid`](#notification-sendgrid) / [`Signal`](#notification-signal) / [`Simplepush`](#notification-simplepush) / [`Slack`](#notification-slack) / [`Telegram`](#notification-telegram) / [`Twitter`](#notification-twitter) / [`Webhook`](#notification-webhook).

#### Notification: Apprise

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | no |  | Base URL of the Apprise API server. |
| `stateless_urls` | string | no |  | Comma-separated stateless Apprise notification URLs (e.g. `slack://…`). |
| `notification_type` | integer | no |  | Notification type/category identifier sent to Apprise. |
| `auth_username` | string | no |  | HTTP basic-auth username for the Apprise server. |
| `auth_password` | secret string | no |  | HTTP basic-auth password for the Apprise server. Credential — redacted in plan output. |
| `configuration_key` | secret string | no |  | Apprise persistent-store configuration key. Credential — redacted in plan output. |
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

#### Notification: Email

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | yes |  | SMTP server hostname or IP address. |
| `port` | integer | no |  | SMTP server port number. |
| `use_encryption` | integer | no |  | Encryption mode: 0 = none, 1 = SSL/TLS, 2 = STARTTLS. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `username` | string | no |  | SMTP authentication username. |
| `password` | secret string | no |  | SMTP authentication password. Credential — redacted in plan output. |
| `to` | array of string | no |  | Primary recipient email addresses. |
| `cc` | array of string | no |  | Carbon-copy recipient email addresses. |
| `bcc` | array of string | no |  | Blind carbon-copy recipient email addresses. |

#### Notification: Gotify

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server` | string | yes |  | Gotify server URL (e.g. `http://gotify.example.com`). |
| `app_token` | secret string | yes |  | Gotify application token used to publish messages. Credential — redacted in plan output. |
| `priority` | integer | no |  | Message priority level sent with each notification. |

#### Notification: Join

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | no |  | Join API key for authentication. Credential — redacted in plan output. |
| `device_names` | string | no |  | Comma-separated target device names; leave empty to send to all devices. |
| `priority` | integer | no |  | Notification priority level. |

#### Notification: Mailgun

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | no |  | Mailgun API key for authentication. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address shown in the From header. |
| `sender_domain` | string | no |  | Mailgun sending domain registered in your account. |
| `use_eu_endpoint` | boolean | no |  | Use the EU Mailgun API endpoint instead of the US endpoint. |
| `recipients` | array of string | no |  | Recipient email addresses. |

#### Notification: Notifiarr

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `api_key` | secret string | yes |  | Notifiarr API key for authentication. Credential — redacted in plan output. |

#### Notification: Ntfy

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `server_url` | string | no |  | Base URL of the ntfy server (e.g. `https://ntfy.sh`). |
| `topics` | array of string | no |  | ntfy topic names to publish notifications to. |
| `priority` | integer | no |  | Message priority level (1 = min … 5 = max). |
| `username` | string | no |  | HTTP basic-auth username for the ntfy server. |
| `password` | secret string | no |  | HTTP basic-auth password for the ntfy server. Credential — redacted in plan output. |
| `access_token` | secret string | no |  | Bearer access token for ntfy authentication (alternative to username/password). Credential — redacted in plan output. |
| `click_url` | string | no |  | URL opened when the notification is tapped by the user. |
| `field_tags` | array of string | no |  | ntfy message tags applied to the notification (emoji shortcodes accepted). |

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
| `api_key` | secret string | no |  | SendGrid API key for authentication. Credential — redacted in plan output. |
| `from` | string | yes |  | Sender email address shown in the From header. |
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
| `key` | secret string | yes |  | Simplepush API key identifying the recipient device. Credential — redacted in plan output. |
| `event` | string | no |  | Custom event name for categorizing notifications in Simplepush. |

#### Notification: Slack

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `web_hook_url` | string | yes |  | Slack incoming webhook URL. |
| `username` | string | yes |  | Display name for the webhook bot. |
| `icon` | string | no |  | Emoji name or image URL to use as the bot's icon (e.g. `:ghost:`). |
| `channel` | string | no |  | Slack channel to post to, overriding the webhook's default channel. |

#### Notification: Telegram

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bot_token` | secret string | yes |  | Telegram bot token issued by BotFather. Credential — redacted in plan output. |
| `chat_id` | string | yes |  | Target chat, group, or channel ID to send messages to. |
| `topic_id` | string | no |  | Forum topic ID to post into (for supergroups with topics enabled). |
| `send_silently` | boolean | no |  | Send the notification silently (no sound or alert on the recipient's device). |

#### Notification: Twitter

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `consumer_key` | secret string | yes |  | Twitter application consumer key (API key). Credential — redacted in plan output. |
| `consumer_secret` | secret string | yes |  | Twitter application consumer secret (API secret). Credential — redacted in plan output. |
| `access_token` | secret string | yes |  | Twitter user OAuth access token. Credential — redacted in plan output. |
| `access_token_secret` | secret string | yes |  | Twitter user OAuth access token secret. Credential — redacted in plan output. |
| `mention` | string | yes |  | Twitter username to mention in the notification tweet. |
| `direct_message` | boolean | no |  | Send the notification as a direct message rather than a public tweet. |

#### Notification: Webhook

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `url` | string | yes |  | Webhook endpoint URL that receives the HTTP request. |
| `method` | integer | yes |  | HTTP method to use: 1 = POST, 2 = PUT. |
| `username` | string | no |  | HTTP basic-auth username sent with the request. |
| `password` | secret string | no |  | HTTP basic-auth password sent with the request. Credential — redacted in plan output. |

### Host Config

`/api/v1/config/host` — Prowlarr host, network, authentication, proxy, and backup settings.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `bind_address` | string | no |  | IP address or hostname Prowlarr binds to; `*` binds to all interfaces. |
| `port` | integer | yes |  | HTTP port Prowlarr listens on. |
| `ssl_port` | integer | yes |  | HTTPS port Prowlarr listens on when SSL is enabled. |
| `enable_ssl` | boolean | yes |  | Enables HTTPS/TLS for the Prowlarr web UI. |
| `launch_browser` | boolean | yes |  | Opens the Prowlarr web UI in the default browser on startup. |
| `authentication_method` | [`authentication_type`](#authentication-type) | no |  | Authentication method for the Prowlarr web UI. |
| `authentication_required` | [`authentication_required_type`](#authentication-required-type) | no |  | Whether authentication is required for local network addresses. |
| `analytics_enabled` | boolean | yes |  | Sends anonymised usage and error data to the Prowlarr team. |
| `username` | string | no |  | Username for basic or forms authentication. |
| `password` | secret string | no |  | Password for basic or forms authentication. Credential — redacted in plan output. |
| `password_confirmation` | secret string | no |  | Password confirmation field; must match `password` when changing credentials. Credential — redacted in plan output. |
| `log_level` | string | no |  | Log verbosity level (e.g. `info`, `debug`, `trace`). |
| `log_size_limit` | integer | yes |  | Maximum size in MB for each log file before it is rotated. |
| `console_log_level` | string | no |  | Log level for console output; overrides `log_level` for stdout. |
| `branch` | string | no |  | Update channel or branch Prowlarr checks for updates (e.g. `main`, `develop`). |
| `api_key` | secret string | no |  | Prowlarr API key used to authenticate API requests. Credential — redacted in plan output. |
| `ssl_cert_path` | string | no |  | Absolute path to the SSL certificate file (PEM/PFX). |
| `ssl_cert_password` | secret string | no |  | Password for the SSL certificate if it is password-protected. Credential — redacted in plan output. |
| `url_base` | string | no |  | URL base path for reverse-proxy deployments (e.g. `/prowlarr`). |
| `instance_name` | string | no |  | Display name for this Prowlarr instance shown in the browser title and notifications. |
| `application_url` | string | no |  | Externally reachable URL for this instance, used in notifications. |
| `update_automatically` | boolean | yes |  | Allows Prowlarr to update itself automatically when a new version is available. |
| `update_mechanism` | [`update_mechanism`](#update-mechanism) | no |  | How Prowlarr applies updates. |
| `update_script_path` | string | no |  | Absolute path to the update script; required when `update_mechanism` is `Script`. |
| `proxy_enabled` | boolean | yes |  | Routes Prowlarr's outbound HTTP traffic through a proxy server. |
| `proxy_type` | [`proxy_type`](#proxy-type) | no |  | Proxy protocol used for outbound connections. |
| `proxy_hostname` | string | no |  | Hostname or IP address of the proxy server. |
| `proxy_port` | integer | yes |  | Port of the proxy server. |
| `proxy_username` | string | no |  | Username for proxy authentication. |
| `proxy_password` | secret string | no |  | Password for proxy authentication. Credential — redacted in plan output. |
| `proxy_bypass_filter` | string | no |  | Comma-separated list of hosts or IP ranges that bypass the proxy. |
| `proxy_bypass_local_addresses` | boolean | yes |  | Bypasses the proxy for connections to local/private addresses. |
| `certificate_validation` | [`certificate_validation_type`](#certificate-validation-type) | no |  | TLS certificate validation mode for outbound connections. |
| `backup_folder` | string | no |  | Folder path where Prowlarr stores automatic database backups. |
| `backup_interval` | integer | yes |  | Interval in days between automatic backups. |
| `backup_retention` | integer | yes |  | Number of days to retain automatic backups before they are deleted. |
| `history_cleanup_days` | integer | yes |  | Number of days to retain indexer search history entries. |
| `trust_cgnat_ip_addresses` | boolean | yes |  | Trusts Carrier-Grade NAT (CGNAT) IP address ranges for source IP determination. |

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
| `ui_language` | string | no |  | Language identifier for the Prowlarr UI interface (e.g. a locale tag or name string). |
| `theme` | string | no |  | UI colour theme name (e.g. `dark`, `light`, `auto`). |

### Download Client Config

`/api/v1/config/downloadclient` — download client handling settings.

Prowlarr's download client config exposes only the server-assigned `id`;
there are no user-configurable fields at this endpoint.

_No user-configurable fields._

### Development Config

`/api/v1/config/development` — developer and debugging configuration.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `console_log_level` | string | no |  | Log verbosity level for console output (e.g. `info`, `debug`, `trace`). |
| `log_sql` | boolean | yes |  | Logs SQL queries executed against the database when enabled. |
| `log_indexer_response` | boolean | yes |  | Logs raw HTTP responses from indexers for debugging. |
| `log_rotate` | integer | yes |  | Number of log files to retain before rotation discards the oldest. |
| `filter_sentry_events` | boolean | yes |  | Filters events sent to Sentry error tracking; reduces noise in reports. |

## Types

### Download Protocol

Allowed values: `usenet` / `torrent`.

### Indexer Privacy

Allowed values: `public` / `semiPrivate` / `private`.

### Download Client Category

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `client_category` | string | no |  | Download client category name, e.g. `"Movies"`. |
| `categories` | array of integer | no |  | Prowlarr category ids mapped to this client category. |

### Application Sync Level

Allowed values: `disabled` / `addOnly` / `fullSync`.

### Authentication Type

Allowed values: `none` / `basic` / `forms` / `external`.

### Authentication Required Type

Allowed values: `enabled` / `disabledForLocalAddresses`.

### Update Mechanism

Allowed values: `builtIn` / `script` / `external` / `apt` / `docker`.

### Proxy Type

Allowed values: `http` / `socks4` / `socks5`.

### Certificate Validation Type

Allowed values: `enabled` / `disabledForLocalAddresses` / `disabled`.

