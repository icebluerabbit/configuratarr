//! `/api/download_clients` — download clients (qBittorrent, Deluge, arr, …).
//!
//! `sync = crud`, keyed by `name`: GET lists, POST creates, PUT updates. The
//! update PUT targets the **collection** path (autobrr reads the id from the
//! body), not a `/{id}` path — so the update endpoint carries no `${self.id}`.
//! Delete is `DELETE /api/download_clients/${self.id}`; declaring it makes the
//! resource prune-capable, so `--prune` deletes clients the config no longer
//! declares (like [`Proxy`](crate::resources::proxy::Proxy)).

use core_lib::SecretValue;
use core_macros::resource;

use crate::resources::download_client_settings::DownloadClientSettings;

/// `/api/download_clients` — a download client autobrr pushes releases to.
#[resource(
    sync = crud,
    case = snake,
    list = get("/api/download_clients"),
    create = post("/api/download_clients"),
    update = put("/api/download_clients"),
    delete = delete("/api/download_clients/${self.id}"),
)]
pub struct DownloadClient {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity (`${ref.download_client.<name>}`).
    #[key]
    pub name: String,
    /// Client kind: `QBITTORRENT`, `DELUGE_V2`, `RADARR`, …
    #[wire(name = "type")]
    pub client_type: String,
    /// Whether the client is active.
    pub enabled: bool,
    /// Client host (URL or hostname).
    pub host: String,
    /// Client port.
    pub port: Option<i32>,
    /// Connect over TLS.
    pub tls: Option<bool>,
    /// Skip TLS certificate verification.
    pub tls_skip_verify: Option<bool>,
    /// Username, where the client authenticates by user/pass.
    pub username: Option<String>,
    /// Password, where the client authenticates by user/pass.
    pub password: Option<SecretValue>,
    /// Client-specific settings (auth, rules, delegation).
    pub settings: Option<DownloadClientSettings>,
}
