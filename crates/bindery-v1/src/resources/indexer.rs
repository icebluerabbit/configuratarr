//! Indexer — a newznab/torznab indexer, either user-created or synced down
//! from a linked [`crate::resources::prowlarr_instance::ProwlarrInstance`].
//!
//! `sync = crud`, keyed by `name`: GET lists, POST creates (requires `name` +
//! `url`), PUT updates via `/{id}` using the bare schema. `apiKey` is not
//! write-only — it round-trips in full on every read — which is why
//! List/Get on `/api/v1/indexer` are admin-only too.

use core_lib::SecretValue;
use core_macros::resource;

/// A newznab/torznab indexer (`models.Indexer`).
#[resource(
    sync = crud,
    list = get("/api/v1/indexer"),
    create = post("/api/v1/indexer"),
    update = put("/api/v1/indexer/${self.id}"),
    delete = delete("/api/v1/indexer/${self.id}"),
)]
pub struct Indexer {
    /// Server-assigned row id. Ignored on create; on update it is
    /// overwritten with the path `{id}`.
    #[id]
    pub id: Option<i64>,
    /// Display name — natural key. Required (non-empty) on create.
    #[key]
    pub name: String,
    /// Indexer protocol family. Free-form (not an enforced enum on this
    /// API, unlike `DownloadClient.type`). Defaults to `newznab` when
    /// omitted or empty on create.
    #[wire(name = "type")]
    #[default("newznab")]
    pub indexer_type: String,
    /// Base URL of the indexer's newznab/torznab API. Required on create.
    /// Validated by the server's LAN+loopback outbound policy; a rejected
    /// URL yields a 400.
    pub url: String,
    /// Indexer API key. SENSITIVE — Bindery stores it in plaintext and echoes
    /// it back on every read, so the whole `/api/v1/indexer` subtree is
    /// admin-only. It is stripped from search-result URLs, so interactive
    /// search doesn't leak it.
    pub api_key: Option<SecretValue>,
    /// Newznab category ids to query. Omit the key to keep the server's
    /// book/audiobook defaults (`[7000, 7020, 3030]`); sending an empty list
    /// also resets it to those defaults.
    pub categories: Option<Vec<i64>>,
    /// Whether parent newznab categories are implicitly included alongside
    /// the configured `categories`.
    pub include_parent_categories: Option<bool>,
    /// Ranking priority — higher wins when multiple indexers return the
    /// same release.
    pub priority: Option<i32>,
    /// Whether this indexer is used. A disabled indexer stays configured but
    /// is skipped for searches and RSS.
    pub enabled: Option<bool>,
    /// Whether this indexer answers interactive/automatic search queries. An
    /// indexer with this off is used for RSS only.
    pub supports_search: Option<bool>,
    /// Set when this indexer was synced down from a Prowlarr instance; absent
    /// for manually created indexers. Resolved from
    /// `${ref.prowlarr_instance.<name>}` at apply.
    #[reference(prowlarr_instance)]
    pub prowlarr_instance_id: Option<i64>,
    /// This indexer's id on the Prowlarr side (not a local reference — an
    /// opaque id belonging to the remote Prowlarr instance).
    pub prowlarr_indexer_id: Option<i64>,
    /// Per-indexer seed-ratio override for grabbed torrents. Absent/null
    /// means no override; `-1` is the unlimited sentinel.
    pub seed_ratio: Option<f64>,
    /// Restrict automatic grabs to freeleech releases only. Non-freeleech
    /// releases are held for manual approval rather than hidden; interactive
    /// search is unaffected.
    pub freeleech_only: Option<bool>,
    /// Provenance of `seed_ratio`: `""`, `"prowlarr"`, or `"user"`.
    /// Effectively server-controlled — create sets it to `user` whenever
    /// `seed_ratio` is supplied, and **update forces it to `user`
    /// unconditionally** so the Prowlarr syncer won't clobber a manual
    /// choice. Modeled read-only since nothing we send on write is honored.
    #[wire(read_only)]
    pub seed_ratio_source: Option<String>,
    /// Row creation timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub created_at: Option<String>,
    /// Row last-modified timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub updated_at: Option<String>,
}
