# Komga v1 Configuration

Komga v1 — desired-state config for one instance.

Auth is the `X-API-Key` header. Komga also accepts HTTP Basic, but a key is
the credential to configure here: keys are minted per user at
`POST /api/v2/users/me/api-keys` (the only place their secret is shown).

The health check is `/api/v2/users/me` — authenticated, always present, and
cheap, so a green response proves both reachability and credentials.

## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

### Library

A Komga library: a scanned root directory plus its import/scan behavior.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | yes |  | Library name — its identity (`${ref.library.<name>}`). |
| `root` | string | yes |  | Filesystem path Komga scans for this library. Always required — there is no meaningful default for where to look. |
| `oneshots_directory` | string | no |  | Directory name (relative to `root`) treated as containing oneshots. |
| `scan_on_startup` | boolean | no |  | Scan this library on Komga startup. |
| `scan_interval` | [`scan_interval`](#scan-interval) | no |  | Automatic rescan interval. |
| `scan_cbx` | boolean | no |  | Scan `.cbr`/`.cbz` comic archives. |
| `scan_pdf` | boolean | no |  | Scan `.pdf` files. |
| `scan_epub` | boolean | no |  | Scan `.epub` files. |
| `scan_force_modified_time` | boolean | no |  | Force re-checking file modification times during a scan, instead of trusting the last recorded value. |
| `scan_directory_exclusions` | array of string | no |  | Directory names to exclude from scanning. Omit the key to leave the server's current list alone. |
| `empty_trash_after_scan` | boolean | no |  | Empty the trash (remove bookkeeping for files no longer on disk) after each scan. |
| `import_comic_info_book` | boolean | no |  | Read `ComicInfo.xml` metadata for books. |
| `import_comic_info_series` | boolean | no |  | Read `ComicInfo.xml` metadata for series. |
| `import_comic_info_collection` | boolean | no |  | Read `ComicInfo.xml` metadata for collections. |
| `import_comic_info_read_list` | boolean | no |  | Read `ComicInfo.xml` metadata for read lists. |
| `import_comic_info_series_append_volume` | boolean | no |  | Append the volume number from `ComicInfo.xml` to the series title. |
| `import_epub_book` | boolean | no |  | Read epub metadata for books. |
| `import_epub_series` | boolean | no |  | Read epub metadata for series. |
| `import_mylar_series` | boolean | no |  | Import Mylar-style `series.json` metadata. |
| `import_local_artwork` | boolean | no |  | Import local artwork files (e.g. `cover.jpg`) found alongside books. |
| `import_barcode_isbn` | boolean | no |  | Read an ISBN from a barcode found in a book's pages. |
| `repair_extensions` | boolean | no |  | Repair file extensions that don't match their actual content type. |
| `convert_to_cbz` | boolean | no |  | Automatically convert applicable archives to `.cbz`. |
| `series_cover` | [`series_cover`](#series-cover) | no |  | Which book's cover to use as the series cover. |
| `hash_files` | boolean | no |  | Compute a hash of each file for duplicate detection. |
| `hash_pages` | boolean | no |  | Compute a hash of each page within a file. |
| `hash_koreader` | boolean | no |  | Compute a KOReader-compatible hash for each file. |
| `analyze_dimensions` | boolean | no |  | Analyze page dimensions (width/height) during a scan. |

### User

A Komga user account.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `email` | string | yes |  | Login email — the user's identity. Sent on create; **not** sent on update (`UserUpdateDto` has no `email` field — Komga doesn't support changing an existing user's email through this endpoint). |
| `password` | secret string | no |  | Initial password, set only when the user is created. **Create-only**: Komga never reads a password back (the only related endpoint, `PATCH /api/v2/users/{id}/password`, is a separate write-only action this crate doesn't model), so changing `password` for an already-created user is **not detected as drift** — it never triggers an update, and the update body never carries it. To rotate an existing user's password, use Komga directly. Credential — redacted in plan output. |
| `roles` | array of string | no |  | Granted roles, e.g. `ADMIN`, `PAGE_STREAMING`, `FILE_DOWNLOAD`, `KOBO_SYNC`. Compared as a set — Komga's wire order isn't contractual. |
| `labels_allow` | array of string | no |  | Content labels this user is restricted to (an allow-list). Compared as a set. |
| `labels_exclude` | array of string | no |  | Content labels this user cannot see (an exclude-list). Compared as a set. |
| `age_restriction` | [`age_restriction`](#age-restriction) | no |  | Age-based content restriction. Omitted from config = not managed by configuratarr, left as Komga has it. See the module docs for the `restriction: NONE` read/write asymmetry. |
| `shared_libraries` | [`shared_libraries`](#shared-libraries) | no |  | Library access grant. Omitted from config = not managed by configuratarr, left as Komga has it. See the module docs for the nested-vs-flat read/write asymmetry. |

### Api Key

An API key issued to the authenticated user under a free-text comment/label.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `comment` | string | yes |  | Free-text label identifying the key — its identity (Komga calls this field `comment`, not `name`). |

### Client Setting Global

One global (server-wide) client setting.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `key` | string | yes |  | Dotted setting key, e.g. `"application.deletion.protection"` — its identity. |
| `value` | string | yes |  | The setting's value (always a plain string on the wire — the API documents a JSON-object value as a JSON-*encoded string*, not a nested object). |
| `allow_unauthorized` | boolean | no | `false` | Whether an unauthorized (anonymous) client may read this setting. Required by the API on every write; defaults to `false` when the config omits it. |

### Client Setting User

One per-user client setting.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `key` | string | yes |  | Dotted setting key, e.g. `"webui.locale"` — its identity. |
| `value` | string | yes |  | The setting's value (always a plain string on the wire — the API documents a JSON-object value as a JSON-*encoded string*, not a nested object). |

### Settings

Komga's global settings (`/api/v1/settings`). Every field is `Option`:
present = manage it, absent = leave Komga's current value alone.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `delete_empty_collections` | boolean | no |  | Delete collections left empty after a library scan. |
| `delete_empty_read_lists` | boolean | no |  | Delete read lists left empty after a library scan. |
| `kepubify_path` | string | no |  | Path to the `kepubify` binary, used to convert EPUB to KEPUB for Kobo devices. Deprecated on the write side by Komga itself. |
| `kobo_port` | integer | no |  | Port used for the Kobo OPDS/sync proxy. |
| `kobo_proxy` | boolean | no |  | Enable the Kobo sync proxy. |
| `remember_me_duration_days` | integer | no |  | Days a "remember me" session stays valid. |
| `renew_remember_me_key` | boolean | no |  | Write-only action: rotate the "remember me" signing key. Declaring `true` fires the rotation on every apply; there is no read-back state to converge toward, so this is never itself "in sync". |
| `server_context_path` | string | no |  | Path prefix Komga is served under (reverse-proxy sub-path). |
| `server_port` | integer | no |  | HTTP port Komga listens on. |
| `task_pool_size` | integer | no |  | Size of the background task worker pool. |
| `thumbnail_size` | [`thumbnail_size`](#thumbnail-size) | no |  | Default thumbnail size for newly generated thumbnails. |

## Types

### Scan Interval

Allowed values: `DISABLED` / `HOURLY` / `EVERY_6H` / `EVERY_12H` / `DAILY` / `WEEKLY`.

### Series Cover

Allowed values: `FIRST` / `FIRST_UNREAD_OR_FIRST` / `FIRST_UNREAD_OR_LAST` / `LAST`.

### Age Restriction

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `age` | integer | yes |  | The age boundary the restriction is evaluated against. |
| `restriction` | [`age_restriction_kind`](#age-restriction-kind) | yes |  | How `age` is applied. |

### Shared Libraries

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `all` | boolean | yes |  | Grant access to every library, including ones created later. When `true`, `library_ids` is ignored by Komga. |
| `library_ids` | array of string | no |  | Explicit library ids to grant access to (used when `all` is `false`). References a [`library`](#library) by name (`${ref.library.<key>}`). |

### Thumbnail Size

Allowed values: `DEFAULT` / `MEDIUM` / `LARGE` / `XLARGE`.

### Age Restriction Kind

Allowed values: `ALLOW_ONLY` / `EXCLUDE` / `NONE`.

