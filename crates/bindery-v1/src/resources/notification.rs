//! Notification — an outbound webhook connection.
//!
//! `sync = crud`, keyed by `name`: GET lists, POST creates, PUT updates via
//! `/{id}`. Both create and update use the bare `Notification` schema — the
//! base schema itself already requires `name` + `url`, so there's no
//! separate create-tightened schema the way `DownloadClient`/`Indexer`/
//! `ProwlarrInstance` have one.

use core_macros::resource;

/// A webhook notification connection (`models.Notification`).
///
/// Bindery has exactly one delivery implementation — an outbound JSON HTTP
/// request — so `type` is a stored label rather than a dispatch
/// discriminator; per-target behavior is derived from `url`/`topic`/`method`
/// instead.
#[resource(
    sync = crud,
    list = get("/api/v1/notification"),
    create = post("/api/v1/notification"),
    update = put("/api/v1/notification/${self.id}"),
    delete = delete("/api/v1/notification/${self.id}"),
)]
pub struct Notification {
    /// Server-assigned row id. Ignored on create; on update it is
    /// overwritten with the path `{id}`.
    #[id]
    pub id: Option<i64>,
    /// Display name — natural key. Required on create.
    #[key]
    pub name: String,
    /// Connection kind label. **Not validated and not branched on anywhere
    /// server-side** — dispatch is purely on `url`/`topic`/`method`. The
    /// web UI hard-codes `webhook` for every connection it creates, so
    /// that's the only value seen in practice; defaults to it here too.
    #[wire(name = "type")]
    #[default("webhook")]
    pub notification_type: String,
    /// Webhook endpoint. Required on create. Validated against the outbound
    /// SSRF policy on create, on update when non-empty, and again at send
    /// time including every redirect hop.
    pub url: String,
    /// HTTP method for the outbound request; upper-cased at send time and
    /// defaulted to `POST` when empty. The UI offers POST, PUT and GET.
    #[default("POST")]
    pub method: String,
    /// Extra request headers as a **JSON-encoded string** holding a flat
    /// object of string values, e.g. `{"Authorization": "Bearer ..."}`.
    /// Empty or `{}` sends none; a value that fails to parse is silently
    /// ignored at send time. Typically carries credentials (hence this
    /// whole surface being admin-only) but the spec doesn't mark it
    /// `x-sensitive` the way `apiKey`/`username`/`password` are elsewhere in
    /// this service, so it's modeled as a plain string rather than
    /// `SecretValue`.
    pub headers: Option<String>,
    /// ntfy topic. When set, Bindery POSTs to the URL's server root with a
    /// `topic` field in the JSON body instead of POSTing to the topic URL
    /// directly, so ntfy renders the payload natively.
    pub topic: Option<String>,
    /// Fire on the `grabbed` event.
    pub on_grab: Option<bool>,
    /// Fire on the `bookImported` event.
    pub on_import: Option<bool>,
    /// Fire on the `upgrade` event.
    pub on_upgrade: Option<bool>,
    /// Fire on the `downloadFailed` event.
    pub on_failure: Option<bool>,
    /// Fire on the `health` event.
    pub on_health: Option<bool>,
    /// Disabled connections are skipped by the dispatcher. The test route
    /// ignores this flag.
    pub enabled: Option<bool>,
    /// Row creation timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub created_at: Option<String>,
    /// Row last-modified timestamp. Server-managed, never writable.
    #[wire(read_only)]
    pub updated_at: Option<String>,
}
