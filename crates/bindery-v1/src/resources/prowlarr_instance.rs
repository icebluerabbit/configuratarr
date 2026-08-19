//! Prowlarr instance — connection config for a linked Prowlarr server that
//! syncs indexers down into Bindery's own
//! [`crate::resources::indexer::Indexer`] table.
//!
//! `sync = crud`, keyed by `name`: GET lists, POST creates (requires only
//! `url`), PUT updates via `/{id}` using the bare schema. `apiKey` is not
//! write-only — it round-trips in read responses — which is why the entire
//! `/api/v1/prowlarr` subtree is admin-only.
//!
//! **Deleting an instance cascades.** Every indexer carrying this instance's
//! `prowlarrInstanceId` is deleted with it (verified live, not inferred from
//! the schema). Under `--prune` that means dropping an instance from config
//! silently takes its synced indexers too — and because `prowlarr_instance`
//! is applied before `indexer`, the indexer step then finds nothing left to
//! delete and reports no change for them.

use core_lib::SecretValue;
use core_macros::resource;

/// Connection config for a Prowlarr server (`models.ProwlarrInstance`).
#[resource(
    sync = crud,
    list = get("/api/v1/prowlarr"),
    create = post("/api/v1/prowlarr"),
    update = put("/api/v1/prowlarr/${self.id}"),
    delete = delete("/api/v1/prowlarr/${self.id}"),
)]
pub struct ProwlarrInstance {
    /// Server-assigned row id. Ignored on create; on update it is
    /// overwritten with the path `{id}`.
    #[id]
    pub id: Option<i64>,
    /// Display name — natural key. Unlike the other three resources in this
    /// family, create does *not* require this: the server defaults it to
    /// `Prowlarr` when empty. Modeled the same as the others' natural key
    /// regardless, since a config always names the instance it's declaring.
    #[key]
    pub name: String,
    /// Base URL of the Prowlarr server. Required on create; validated by the
    /// server's LAN+loopback outbound policy.
    pub url: String,
    /// Prowlarr API key. SENSITIVE — Bindery stores it in plaintext and echoes
    /// it back on every read, so the whole `/api/v1/prowlarr` subtree is
    /// admin-only. Changing it also rewrites the stored key of every indexer
    /// synced from this instance.
    pub api_key: Option<SecretValue>,
    /// Trigger an indexer sync from this Prowlarr instance whenever Bindery
    /// starts up.
    pub sync_on_startup: Option<bool>,
    /// Whether this instance is used. A disabled instance stays configured
    /// but is skipped when indexers are synced.
    pub enabled: Option<bool>,
    /// Timestamp of the last successful sync. Attached by the server as sync
    /// state, not accepted on input, so it's modeled read-only even though
    /// the spec doesn't set `readOnly` on it explicitly.
    #[wire(read_only)]
    pub last_sync_at: Option<String>,
    /// Row creation timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub created_at: Option<String>,
    /// Row last-modified timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub updated_at: Option<String>,
}
