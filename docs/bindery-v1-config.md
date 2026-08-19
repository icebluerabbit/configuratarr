# Bindery v1 Configuration

Bindery v1 — desired-state config for one instance.

Auth is the `X-Api-Key` header (note the casing: Komga spells the same idea
`X-API-Key`). The key is the instance key shown in Bindery's settings, not a
per-user token.

The health check is `/api/v1/system/status` — authenticated, so a green
response proves credentials too. `/api/v1/health` is deliberately not used:
it answers a constant `{"status":"ok"}` without checking auth.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Root Folder

A root folder Bindery watches for content.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `path` | string | yes |  | Natural key — the absolute path on the Bindery server (or container) filesystem. Must already exist, be accessible, and be a directory. |

### Quality Profile

Named quality profile — ordered file-format preference list with an
upgrade cutoff.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.quality_profile.<name>}`. Trimmed server-side; must be non-empty and unique across **all** profiles (409 on collision). |
| `upgrade_allowed` | boolean | no | `false` | Whether an existing file may be replaced by a better-ranked one. |
| `cutoff` | string | yes |  | Format at which upgrading stops. **Trimmed and lower-cased server-side** — write it lower-case or every read-back will show a diff and the config will churn forever. Must be non-empty and must equal the `quality` of one of `items` with `allowed: true`. |
| `items` | array of [`quality_item`](#quality-item) | no |  | Ordered preference list of formats. Must contain at least one entry and at least one entry with `allowed: true`; duplicate (lower-cased) qualities are rejected server-side. |

### Metadata Profile

Named metadata profile controlling which discovered books get added.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.metadata_profile.<name>}`. Required on create (empty is rejected); NOT re-validated on update. Not unique — no server-side collision check. |
| `min_popularity` | integer | no | `0` | Minimum popularity score a book must reach to be added. `0` disables the filter. |
| `min_pages` | integer | no | `0` | Minimum page count a book must reach to be added. `0` disables the filter. |
| `skip_missing_date` | boolean | no | `false` | Skip books with no release date. |
| `skip_missing_isbn` | boolean | no | `false` | Skip books with no ISBN. |
| `skip_part_books` | boolean | no | `false` | Skip books that are parts/volumes of a larger work. |
| `allowed_languages` | string | no | `eng` | Language filter as a raw string, matched verbatim rather than as a list. On create, an empty value is replaced server-side with `eng`; that fallback is **not** applied on update, so sending an empty string there clears the filter. This codec always emits the field (its default is `eng`), so an omitted config value never triggers the update-time gap. |
| `unknown_language_behavior` | [`unknown_language_behavior`](#unknown-language-behavior) | no |  | What to do when the metadata source reports no language for a book while `allowed_languages` is non-empty: `pass` imports it anyway, `fail` skips it. Coerced server-side on both create and update — any value other than exactly `fail` becomes `pass`, so an omitted or unrecognised value is never rejected, it silently falls back to `pass` (the API's own default). See [`UnknownLanguageBehavior`]. |

### Custom Format

A named set of release-matching conditions used to score/filter releases.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Natural key — referenced in `${ref.custom_format.<name>}`. Required on create (empty is rejected); NOT re-validated on update, so a `PUT` omitting it would store an empty string — this codec always emits it. Not unique — no server-side collision check. |
| `conditions` | array of [`custom_condition`](#custom-condition) | no |  | Conditions that must match for the format to apply. The API normalises a `null`/omitted value to `[]` on both create and update, so a plain always-emitted `Vec` here is correct — never distinguish "omitted" from "empty" for this field. Fully replaced on update; there is no per-condition endpoint. |

### Delay Profile

A delay profile — minutes to hold a release per protocol before grabbing
it, so a better release has time to appear.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `usenet_delay` | integer | no | `0` | Minutes to hold a usenet release before grabbing it. `0` = grab immediately. Not validated — negative values are accepted and stored. |
| `torrent_delay` | integer | no | `0` | Minutes to hold a torrent release before grabbing it. `0` = grab immediately. Not validated — negative values are accepted and stored. |
| `preferred_protocol` | string | no |  | Protocol preferred when both are available (`usenet` or `torrent`, not enforced server-side). Omitted or empty resolves to `usenet`. |
| `enable_usenet` | boolean | no | `false` | Whether usenet releases are eligible under this profile. |
| `enable_torrent` | boolean | no | `false` | Whether torrent releases are eligible under this profile. |
| `order` | integer | no | `0` | Evaluation order; `LIST` returns profiles sorted by this ascending. Not deduplicated — several profiles may share an order. |

### Indexer

A newznab/torznab indexer (`models.Indexer`).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — natural key. Required (non-empty) on create. |
| `indexer_type` | string | no | `newznab` | Indexer protocol family. Free-form (not an enforced enum on this API, unlike `DownloadClient.type`). Defaults to `newznab` when omitted or empty on create. |
| `url` | string | yes |  | Base URL of the indexer's newznab/torznab API. Required on create. Validated by the server's LAN+loopback outbound policy; a rejected URL yields a 400. |
| `api_key` | secret string | no |  | Indexer API key. SENSITIVE — Bindery stores it in plaintext and echoes it back on every read, so the whole `/api/v1/indexer` subtree is admin-only. It is stripped from search-result URLs, so interactive search doesn't leak it. Credential — redacted in plan output. |
| `categories` | array of integer | no |  | Newznab category ids to query. Omit the key to keep the server's book/audiobook defaults (`[7000, 7020, 3030]`); sending an empty list also resets it to those defaults. |
| `include_parent_categories` | boolean | no |  | Whether parent newznab categories are implicitly included alongside the configured `categories`. |
| `priority` | integer | no |  | Ranking priority — higher wins when multiple indexers return the same release. |
| `enabled` | boolean | no |  | Whether this indexer is used. A disabled indexer stays configured but is skipped for searches and RSS. |
| `supports_search` | boolean | no |  | Whether this indexer answers interactive/automatic search queries. An indexer with this off is used for RSS only. |
| `prowlarr_instance_id` | integer | no |  | Set when this indexer was synced down from a Prowlarr instance; absent for manually created indexers. Resolved from `${ref.prowlarr_instance.<name>}` at apply. References a [`prowlarr_instance`](#prowlarr-instance) by name (`${ref.prowlarr_instance.<key>}`). |
| `prowlarr_indexer_id` | integer | no |  | This indexer's id on the Prowlarr side (not a local reference — an opaque id belonging to the remote Prowlarr instance). |
| `seed_ratio` | number | no |  | Per-indexer seed-ratio override for grabbed torrents. Absent/null means no override; `-1` is the unlimited sentinel. |
| `freeleech_only` | boolean | no |  | Restrict automatic grabs to freeleech releases only. Non-freeleech releases are held for manual approval rather than hidden; interactive search is unaffected. |

### Prowlarr Instance

Connection config for a Prowlarr server (`models.ProwlarrInstance`).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — natural key. Unlike the other three resources in this family, create does *not* require this: the server defaults it to `Prowlarr` when empty. Modeled the same as the others' natural key regardless, since a config always names the instance it's declaring. |
| `url` | string | yes |  | Base URL of the Prowlarr server. Required on create; validated by the server's LAN+loopback outbound policy. |
| `api_key` | secret string | no |  | Prowlarr API key. SENSITIVE — Bindery stores it in plaintext and echoes it back on every read, so the whole `/api/v1/prowlarr` subtree is admin-only. Changing it also rewrites the stored key of every indexer synced from this instance. Credential — redacted in plan output. |
| `sync_on_startup` | boolean | no |  | Trigger an indexer sync from this Prowlarr instance whenever Bindery starts up. |
| `enabled` | boolean | no |  | Whether this instance is used. A disabled instance stays configured but is skipped when indexers are synced. |

### Download Client

A download client (`models.DownloadClient`).

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — natural key. Required (non-empty) on create. |
| `client_type` | [`download_client_type`](#download-client-type) | no |  | Client implementation. Omitted, the server defaults to `sabnzbd` (see [`DownloadClientType`]). |
| `host` | string | yes |  | Bare hostname or IP. Required (non-empty) on create. A leading `http://`/`https://` is stripped server-side — the scheme is instead derived from `use_ssl` — so writing one here churns forever as the live value reads back without it. |
| `port` | integer | no | `8080` | Defaults to `8080` when `0`/omitted on create. |
| `api_key` | secret string | no |  | API key for clients that authenticate with one (SABnzbd, NZBGet). SENSITIVE — Bindery stores this in plaintext and echoes it back verbatim on every read (List/Get) and in the Create/Update response body; there is no write-only masking, which is why this whole route subtree is admin-gated. It still round-trips exactly, so a `crud` diff converges despite the plaintext echo. Credential — redacted in plan output. |
| `use_ssl` | boolean | no |  | Selects `https` vs `http` for the outbound URL. |
| `url_base` | string | no |  | Path prefix if the client is served under a subpath. |
| `category` | string | no | `books` | Category/label applied to ebook downloads. Defaults to `books` when empty on create. |
| `category_audiobook` | string | no |  | Category/label for audiobook downloads; falls back to `category` when empty. |
| `path_remap` | string | no |  | Per-client remap applied to the completed-download path before Bindery stats it; falls back to the global `BINDERY_DOWNLOAD_PATH_REMAP` when unset. |
| `priority` | integer | no |  | Client priority relative to other configured download clients. |
| `enabled` | boolean | no |  | Whether this download client is active. |
| `username` | secret string | no |  | Username for credential-authenticating clients (qBittorrent, Transmission). SENSITIVE — plaintext, echoed back verbatim; see `api_key` for the full rationale. Credential — redacted in plan output. |
| `password` | secret string | no |  | Password for credential-authenticating clients. SENSITIVE — plaintext, echoed back verbatim; see `api_key` for the full rationale. Credential — redacted in plan output. |

### Notification

A webhook notification connection (`models.Notification`).

Bindery has exactly one delivery implementation — an outbound JSON HTTP
request — so `type` is a stored label rather than a dispatch
discriminator; per-target behavior is derived from `url`/`topic`/`method`
instead.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — natural key. Required on create. |
| `notification_type` | string | no | `webhook` | Connection kind label. **Not validated and not branched on anywhere server-side** — dispatch is purely on `url`/`topic`/`method`. The web UI hard-codes `webhook` for every connection it creates, so that's the only value seen in practice; defaults to it here too. |
| `url` | string | yes |  | Webhook endpoint. Required on create. Validated against the outbound SSRF policy on create, on update when non-empty, and again at send time including every redirect hop. |
| `method` | string | no | `POST` | HTTP method for the outbound request; upper-cased at send time and defaulted to `POST` when empty. The UI offers POST, PUT and GET. |
| `headers` | string | no |  | Extra request headers as a **JSON-encoded string** holding a flat object of string values, e.g. `{"Authorization": "Bearer ..."}`. Empty or `{}` sends none; a value that fails to parse is silently ignored at send time. Typically carries credentials (hence this whole surface being admin-only) but the spec doesn't mark it `x-sensitive` the way `apiKey`/`username`/`password` are elsewhere in this service, so it's modeled as a plain string rather than `SecretValue`. |
| `topic` | string | no |  | ntfy topic. When set, Bindery POSTs to the URL's server root with a `topic` field in the JSON body instead of POSTing to the topic URL directly, so ntfy renders the payload natively. |
| `on_grab` | boolean | no |  | Fire on the `grabbed` event. |
| `on_import` | boolean | no |  | Fire on the `bookImported` event. |
| `on_upgrade` | boolean | no |  | Fire on the `upgrade` event. |
| `on_failure` | boolean | no |  | Fire on the `downloadFailed` event. |
| `on_health` | boolean | no |  | Fire on the `health` event. |
| `enabled` | boolean | no |  | Disabled connections are skipped by the dispatcher. The test route ignores this flag. |

### Import List

`/api/v1/importlist` — a configured import list.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Display name — its identity (`${ref.import_list.<name>}`). |
| `list_type` | string | no | `csv` | Provider kind. Not validated server-side, but only `hardcover` has a syncer wired; an empty value is defaulted to `csv` on create (a no-op list with no syncer). |
| `url` | string | no |  | Source locator. For `hardcover` lists this is the list slug (matched against the account's Hardcover lists), not an absolute URL. |
| `api_key` | secret string | no |  | Provider credential (e.g. the Hardcover API token). `writeOnly` on the API — every response blanks it; read `api_key_configured` instead. See the module doc for why this makes rotation undetectable. Credential — redacted in plan output. |
| `account` | string | no |  | Provider-side account identity the list belongs to (e.g. the Hardcover username the token was loaded with). Settable on create; **not patchable** via the update route — excluded from [`in_sync`] so drift here is never (falsely) flagged as fixable. |
| `root_folder_id` | integer | no |  | Root folder assigned to content synced from this list. References a [`root_folder`](#root-folder) by name (`${ref.root_folder.<key>}`). |
| `quality_profile_id` | integer | no |  | Quality profile stamped on authors created by this list's sync. References a [`quality_profile`](#quality-profile) by name (`${ref.quality_profile.<key>}`). |
| `owner_user_id` | integer | no |  | Bindery user who owns books/authors synced from this list. `None` (absent) means "leave as-is" here — see the module doc; there is no way to declare "clear to global" through this field. References a [`user`](#user) by name (`${ref.user.<key>}`). |
| `media_type` | string | no |  | Pins the format synced books are created as: `ebook`, `audiobook`, or `both`. Empty/absent keeps the format derived from the source. |
| `monitor_new` | boolean | no |  | Whether newly-synced works are monitored. |
| `auto_add` | boolean | no |  | Whether works from this list are automatically added. |
| `enabled` | boolean | no |  | Whether the list is active; disabled lists are skipped by the scheduler and reject a manual sync. |

### Import List Exclusion

`/api/v1/importlistexclusion` — a blocked work.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `foreign_id` | string | yes |  | Natural key — the provider id of the excluded work. Required on create. |
| `title` | string | no |  | Display-only title of the excluded work. |
| `author_name` | string | no |  | Display-only author name of the excluded work. |

### User

`/api/v1/auth/users` — a local account.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `username` | string | yes |  | Login name — the account's identity. |
| `role` | [`user_role`](#user-role) | yes |  | The account's role. |
| `password` | secret string | no |  | Initial password, used only when creating this user (`POST /api/v1/auth/users`). Required to create one — declaring a user that doesn't exist yet without a password is an error. Ignored for an already-existing user; see `reset_password` to change one. Credential — redacted in plan output. |
| `reset_password` | secret string | no |  | Opt-in password reset for an existing user. When set, **every apply** PUTs this value to `.../reset-password` — remove the field from the config once the reset has taken effect, or it keeps firing. Credential — redacted in plan output. |

### Oidc Provider

`/api/v1/auth/oidc/providers` — one OIDC identity provider. `case = snake`
— the API's JSON keys (`client_id`, `allowed_groups`, …) are the snake
field names verbatim, not camelCase.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `id` | string | yes |  | Stable provider id used in the login/callback paths (`^[a-z0-9_-]{1,32}$`). |
| `name` | string | no |  | Display name rendered on the login page. |
| `issuer` | string | yes |  | OIDC issuer URL — discovery is fetched from `<issuer>/.well-known/openid-configuration`. |
| `client_id` | string | yes |  | OAuth2 client id. |
| `client_secret` | secret string | no |  | OAuth2 client secret. Write-only, never read back. The reconcile hook sends this verbatim only for a provider id that isn't live yet; for one that already exists it always sends `""` instead, to preserve the stored secret regardless of what's declared here. Credential — redacted in plan output. |
| `scopes` | array of string | no |  | Requested scopes. Empty falls back to the server's `openid profile email` default. |
| `allowed_groups` | array of string | no |  | When non-empty, a login is admitted only if the user's group claim intersects this list. |

### Setting

Bindery settings — every validated key (all optional: present in config =
manage that key, absent = leave it alone). See the module docs for the
secret-key and `prune` caveats.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `hardcover_api_token` | secret string | no |  | `hardcover.api_token` — Hardcover API token. One of the two secret keys this endpoint allows writing (rejected 403 for every other secret); validated to reject control characters. **Never reads back** — GET 404s for a secret key exactly as for an unset one — so a configured value always reports `Updated`, every apply. Credential — redacted in plan output. |
| `hardcover_enhanced_series_enabled` | boolean | no |  | `hardcover.enhanced_series_enabled` — case-insensitive `true`/`false`. |
| `hardcover_sync_interval` | string | no |  | `hardcover.sync_interval` — Go duration string, must be within `[1h, 168h]`. Takes effect only after a restart. |
| `abs_enabled` | boolean | no |  | `abs.enabled` — case-insensitive `true`/`false`. |
| `abs_base_url` | string | no |  | `abs.base_url` — must pass `abs.ValidateBaseURLSecure` (an SSRF-safe http/https URL). |
| `calibre_library_path` | string | no |  | `calibre.library_path` — must stat to an existing directory. Validation **stats the local filesystem as a side effect** of applying. |
| `calibre_binary_path` | string | no |  | `calibre.binary_path` — must be an existing, regular, executable file. |
| `calibre_mode` | string | no |  | `calibre.mode` — one of `off`, `calibredb`, `plugin`. |
| `calibre_push_path_remap` | string | no |  | `calibre.push_path_remap` — must match the `from:to[,from:to]` grammar. |
| `calibre_plugin_url` | string | no |  | `calibre.plugin_url` — must be an http/https URL whose host passes the outbound SSRF policy (link-local and cloud-metadata addresses blocked). |
| `calibre_plugin_api_key` | secret string | no |  | `calibre.plugin_api_key` — the other secret key this endpoint allows writing. No format validation beyond being a secret; like `hardcover.api_token`, it **never reads back** (GET 404s identically to unset), so it always reports `Updated`. Credential — redacted in plan output. |
| `cwa_ingest_path` | string | no |  | `cwa.ingest_path` — must be an existing directory. Validation **stats the local filesystem as a side effect** of applying. |
| `import_mode` | string | no |  | `import.mode` — one of `auto`, `move`, `copy`, `hardlink`, `external`. |
| `import_drop_folder` | string | no |  | `import.drop_folder` — must be an existing directory. Validation **stats the local filesystem as a side effect** of applying. |
| `import_drop_layout` | string | no |  | `import.drop_layout` — one of `flat`, `templated`. |
| `import_drop_link_mode` | string | no |  | `import.drop_link_mode` — one of `copy`, `hardlink`. |
| `import_drop_pair_gating` | boolean | no |  | `import.drop_pair_gating` — case-insensitive `true`/`false`. |
| `import_drop_pair_gating_timeout_hours` | integer | no |  | `import.drop_pair_gating_timeout_hours` — a positive integer. |
| `default_media_type` | string | no |  | `default.media_type` — one of `ebook`, `audiobook`, `both`. |
| `default_media_type_strict` | boolean | no |  | `default.media_type_strict` — case-insensitive `true`/`false`. |
| `author_default_monitor_mode` | string | no |  | `author.default_monitor_mode` — one of `all`, `future`, `latest`, `none` (`series` monitoring is per-author only, not settable here). |
| `author_default_monitor_latest_count` | integer | no |  | `author.default_monitor_latest_count` — a positive integer. |
| `library_default_root_folder_id` | integer | no |  | `library.defaultRootFolderId` — must be a positive integer identifying an existing root folder. A ref, not a free-typed value — see the module docs for why this is `i64` rather than `String` like its siblings. References a [`root_folder`](#root-folder) by name (`${ref.root_folder.<key>}`). |
| `metadata_primary_provider` | string | no |  | `metadata.primary_provider` — one of `openlibrary`, `dnb`. |
| `search_interval` | string | no |  | `search.interval` — Go duration string, must be within `[1h, 168h]`. Takes effect only after a restart. |

### Auth Mode

`/api/v1/auth/config` / `/api/v1/auth/mode` — the server's authentication
mode. Singleton: exactly one entry, no natural key.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `mode` | [`auth_mode_value`](#auth-mode-value) | yes |  | The desired authentication mode. |

### Abs Config

`/api/v1/abs/config` — the Audiobookshelf source configuration.
Singleton: exactly one entry, no natural key.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `base_url` | string | no |  | Audiobookshelf base URL; normalized server-side before storage. |
| `label` | string | no |  | Display label for the source; an empty value is stored as "Audiobookshelf". |
| `enabled` | boolean | no |  | Whether this source may be imported from. |
| `library_ids` | array of string | no |  | Target book library ids, de-duplicated and in the given order. The first entry becomes the server's read-only `libraryId`. Not an `Option` — `Vec<Option<..>>`/`Option<Vec<..>>` aren't a modelled field shape; an absent config key is still masked out by `config_present_to_wire`, same as any other unmanaged field. |
| `path_remap` | string | no |  | Path remap rule applied to ABS-reported file paths so they resolve under Bindery-visible storage. |
| `api_key` | secret string | no |  | Audiobookshelf API key. An absent/empty value leaves the stored key untouched. Credential — redacted in plan output. |

### Grimmory Config

`/api/v1/grimmory/config` — the Grimmory integration configuration.
Singleton: exactly one entry, no natural key.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | no |  | Master switch for the integration. |
| `base_url` | string | no |  | Grimmory server URL; validated by a secure-URL check and stored normalized. |
| `api_key` | secret string | no |  | API token. An absent/empty value leaves the stored key untouched. Credential — redacted in plan output. |
| `username` | string | no |  | Login username. Echoed verbatim by the API — trimmed before storage, not treated as a secret there, so it's kept as a plain string here. |
| `password` | secret string | no |  | Login password. An absent/empty value leaves the stored password untouched. Credential — redacted in plan output. |

## Types

### Quality Item

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `quality` | string | yes |  | File-format token. **Trimmed and lower-cased server-side** — a config that writes mixed case (e.g. `EPUB`) churns forever, since every read comes back lower-case. Must be non-empty and unique within the profile. Not restricted to a fixed set, but only the tokens in Bindery's `QualityRank` carry an ordering: `unknown`, `txt`, `rtf`, `pdf`, `mobi`/`azw` (tied), `epub`, `azw3`, `mp3`, `m4a`, `m4b`, `flac`. |
| `allowed` | boolean | no | `false` | Whether releases of this format may be grabbed. At least one item in a profile must set this `true`. |

### Unknown Language Behavior

Allowed values: `pass` / `fail`.

### Custom Condition

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `condition_type` | string | yes |  | What the condition matches against. Convention only (`releaseTitle`, `releaseGroup`, `size`, `indexerFlag`) — the API does not validate this value, so a typo is stored as-is rather than rejected. |
| `pattern` | string | yes |  | The pattern/value matched against the field named by `type`. Stored verbatim and never compiled or range-checked. |
| `negate` | boolean | no | `false` | Invert the match — the condition holds when the pattern does NOT match. |
| `required` | boolean | no | `false` | The condition must match for the format to apply at all, rather than merely contributing to it. |

### Download Client Type

Allowed values: `sabnzbd` / `nzbget` / `qbittorrent` / `transmission` / `deluge` / `rtorrent`.

### User Role

Allowed values: `admin` / `user`.

### Auth Mode Value

Allowed values: `enabled` / `local-only` / `disabled` / `proxy`.

