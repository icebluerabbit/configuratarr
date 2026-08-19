//! Download client — a client Bindery pushes NZB/torrent grabs to.
//!
//! `sync = crud`, keyed by `name`: GET lists, POST creates (requires `name` +
//! `host`), PUT updates via `/{id}` using the bare schema. Every credential
//! field (`apiKey`, `username`, `password`) is stored and echoed back in
//! plaintext by the server — none is write-only — which is why the whole
//! `/api/v1/downloadclient` subtree is admin-gated. They still round-trip
//! verbatim, so a `SecretValue` diff still converges.

use core_lib::SecretValue;
use core_macros::{nested, resource, wire_enum};

/// Download client implementation (`models.DownloadClient.Type`).
///
/// A closed set per the spec's `enum`. Modeled behind `Option` on
/// [`DownloadClient::client_type`] rather than always-emitted: an omitted
/// value skips the wire key entirely, which is what lets the server apply
/// its own `sabnzbd` default instead of us re-asserting a literal value on
/// every apply.
#[wire_enum(rename_all = "lowercase")]
pub enum DownloadClientType {
    /// SABnzbd (Usenet). The server default when `type` is omitted or
    /// unrecognised.
    Sabnzbd,
    /// NZBGet (Usenet).
    Nzbget,
    /// qBittorrent (torrent). The only client type the server health-probes.
    Qbittorrent,
    /// Transmission (torrent).
    Transmission,
    /// Deluge (torrent).
    Deluge,
    /// rTorrent (torrent).
    Rtorrent,
    /// Any value the API adds later that this version doesn't model yet.
    #[fallback]
    Unknown,
}

/// Live health snapshot the server attaches to a download client
/// (`models.DownloadClientHealth`). Read-only — only enabled qBittorrent
/// clients are probed; every other type never carries one.
#[nested]
pub struct DownloadClientHealth {
    /// `"ok"`, `"checking"`, or `"error"`.
    pub status: Option<String>,
    /// Human-readable health detail, when present.
    pub message: Option<String>,
}

/// A download client (`models.DownloadClient`).
#[resource(
    sync = crud,
    list = get("/api/v1/downloadclient"),
    create = post("/api/v1/downloadclient"),
    update = put("/api/v1/downloadclient/${self.id}"),
    delete = delete("/api/v1/downloadclient/${self.id}"),
)]
pub struct DownloadClient {
    /// Server-assigned row id. Ignored on create; on update it is
    /// overwritten with the path `{id}`.
    #[id]
    pub id: Option<i64>,
    /// Display name — natural key. Required (non-empty) on create.
    #[key]
    pub name: String,
    /// Client implementation. Omitted, the server defaults to `sabnzbd`
    /// (see [`DownloadClientType`]).
    #[wire(name = "type")]
    pub client_type: Option<DownloadClientType>,
    /// Bare hostname or IP. Required (non-empty) on create. A leading
    /// `http://`/`https://` is stripped server-side — the scheme is instead
    /// derived from `use_ssl` — so writing one here churns forever as the
    /// live value reads back without it.
    pub host: String,
    /// Defaults to `8080` when `0`/omitted on create.
    #[default(8080)]
    pub port: i32,
    /// API key for clients that authenticate with one (SABnzbd, NZBGet).
    /// SENSITIVE — Bindery stores this in plaintext and echoes it back
    /// verbatim on every read (List/Get) and in the Create/Update response
    /// body; there is no write-only masking, which is why this whole route
    /// subtree is admin-gated. It still round-trips exactly, so a `crud`
    /// diff converges despite the plaintext echo.
    pub api_key: Option<SecretValue>,
    /// Selects `https` vs `http` for the outbound URL.
    pub use_ssl: Option<bool>,
    /// Path prefix if the client is served under a subpath.
    pub url_base: Option<String>,
    /// Category/label applied to ebook downloads. Defaults to `books` when
    /// empty on create.
    #[default("books")]
    pub category: String,
    /// Category/label for audiobook downloads; falls back to `category`
    /// when empty.
    pub category_audiobook: Option<String>,
    /// Per-client remap applied to the completed-download path before
    /// Bindery stats it; falls back to the global
    /// `BINDERY_DOWNLOAD_PATH_REMAP` when unset.
    pub path_remap: Option<String>,
    /// Client priority relative to other configured download clients.
    pub priority: Option<i32>,
    /// Whether this download client is active.
    pub enabled: Option<bool>,
    /// Row creation timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub created_at: Option<String>,
    /// Row last-modified timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub updated_at: Option<String>,
    /// Live health snapshot attached by the server. Server-managed, never
    /// writable — never read from the request body.
    #[wire(read_only)]
    pub health: Option<DownloadClientHealth>,
    /// Username for credential-authenticating clients (qBittorrent,
    /// Transmission). SENSITIVE — plaintext, echoed back verbatim; see
    /// `api_key` for the full rationale.
    pub username: Option<SecretValue>,
    /// Password for credential-authenticating clients. SENSITIVE —
    /// plaintext, echoed back verbatim; see `api_key` for the full
    /// rationale.
    pub password: Option<SecretValue>,
}
