//! `/api/configuration/download_client` — a configured download client, plus
//! four nested per-client concerns that live on their own endpoints:
//! `seeding_rules`, `unlinked_config`, `dead_torrent_config`, and
//! `orphaned_files_config`.
//!
//! Those four are nested here — rather than modelled as four top-level
//! resources — because every one of their endpoints is addressed by the
//! owning client's GUID, and that GUID is only known once the client exists.
//! `sync = custom`, keyed by `name`:
//!   * `GET    /api/configuration/download_client`      → `{ "clients": [...] }`
//!   * `POST   /api/configuration/download_client`       (`CreateDownloadClientRequest`)
//!   * `PUT    /api/configuration/download_client/{id}`  (`UpdateDownloadClientRequest`)
//!   * `DELETE /api/configuration/download_client/{id}`
//!
//! `password` reads back masked and the four nested concerns are enriched
//! server-side data the create/update contract has no field for, so this
//! doesn't round-trip a write — idempotency for the client body is
//! [`crate::diff::subset`], same as every other custom resource in this
//! crate.
//!
//! Reconcile order, once a client's GUID is resolved (existing, or freshly
//! created under `execute`): [`seeding_rule::reconcile_seeding_rules`], then
//! the three simple `GET`(nullable)/`PUT` sub-configs via
//! [`reconcile_simple_config`]. All of it honours `execute` — see each
//! helper's own doc.

use core_lib::{
    Change, ChangeKind, CustomSync, CustomSyncFuture, HttpClient, RefId, RefStore, SecretValue,
    engine, reconcile,
};
use core_macros::{resource, wire_enum};
use serde_json::Value;

use crate::diff;
use crate::resources::download_client::dead_torrent::DeadTorrentConfig;
use crate::resources::download_client::orphaned_files::OrphanedFilesConfig;
use crate::resources::download_client::unlinked::UnlinkedConfig;

pub mod dead_torrent;
pub mod orphaned_files;
pub mod seeding_rule;
pub mod unlinked;

pub use seeding_rule::SeedingRule;

const DOWNLOAD_CLIENT_PATH: &str = "/api/configuration/download_client";

/// Download client implementation. The wire values are mixed-case
/// (`qBittorrent`, `uTorrent`, `rTorrent` are lower-camel; `Deluge`,
/// `Transmission` stay PascalCase), so the three irregular ones override the
/// derived name with `#[variant(...)]`.
#[wire_enum]
pub enum DownloadClientTypeName {
    #[variant("qBittorrent")]
    QBittorrent,
    Deluge,
    Transmission,
    #[variant("uTorrent")]
    UTorrent,
    #[variant("rTorrent")]
    RTorrent,
    /// Unknown or future client implementation not yet modelled by this
    /// version.
    #[fallback]
    Unknown,
}

/// Protocol family a download client handles.
#[wire_enum]
pub enum DownloadClientType {
    Torrent,
    Usenet,
    /// Unknown or future protocol not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// `/api/configuration/download_client` — a configured download client.
///
/// The four nested fields are **not** part of this resource's own wire body
/// (`CreateDownloadClientRequest`/`UpdateDownloadClientRequest` are both
/// `additionalProperties: false`) — each is `#[wire(config_only)]` so the
/// standard codec's encode skips it, while config decode (which doesn't
/// check `read_only`) still reads it from the user's YAML. See the `impl
/// CustomSync` for how they're reconciled against their own endpoints.
#[resource(sync = custom, list = get("/api/configuration/download_client"))]
pub struct DownloadClient {
    /// Server-assigned id (GUID).
    #[id]
    pub id: Option<String>,
    /// Display name — its identity (`${ref.download_client.<name>}`).
    #[key]
    pub name: String,
    /// Whether Cleanuparr manages downloads against this client.
    #[default(false)]
    pub enabled: bool,
    /// Client implementation (`qBittorrent`, `Deluge`, `Transmission`,
    /// `uTorrent`, or `rTorrent`).
    pub type_name: Option<DownloadClientTypeName>,
    /// Protocol family this client handles.
    #[wire(name = "type")]
    pub protocol: Option<DownloadClientType>,
    /// Client host/address.
    pub host: Option<String>,
    /// Client username, where required.
    pub username: Option<String>,
    /// Client password, where required. Masked on read; the masked
    /// placeholder is rejected on create and, sent back on update, keeps the
    /// stored password.
    pub password: Option<SecretValue>,
    /// Path prefix for clients such as Transmission and Deluge.
    pub url_base: Option<String>,
    /// Externally reachable URL for this client (e.g. behind a reverse
    /// proxy), used when Cleanuparr needs to hand the user a clickable link.
    pub external_url: Option<String>,
    /// Path prefix as the client itself reports it. Must be set together
    /// with `download_directory_target`.
    pub download_directory_source: Option<String>,
    /// Local mount path substituted for `download_directory_source` when
    /// Cleanuparr resolves files on disk.
    pub download_directory_target: Option<String>,

    /// Seeding rules scoped to this client. Reconciled against
    /// `/api/seeding-rules` once this client's GUID is known — see
    /// [`seeding_rule`].
    #[wire(config_only)]
    pub seeding_rules: Vec<SeedingRule>,
    /// This client's unlinked-download handling. `None` leaves it unmanaged.
    /// Reconciled against `/api/unlinked-config/{id}` — see [`unlinked`].
    #[wire(config_only)]
    pub unlinked_config: Option<UnlinkedConfig>,
    /// This client's dead-torrent handling. `None` leaves it unmanaged.
    /// Reconciled against `/api/dead-torrent-config/{id}` — see
    /// [`dead_torrent`].
    #[wire(config_only)]
    pub dead_torrent_config: Option<DeadTorrentConfig>,
    /// This client's orphaned-file scanning. `None` leaves it unmanaged.
    /// Reconciled against `/api/orphaned-files-config/{id}` — see
    /// [`orphaned_files`].
    #[wire(config_only)]
    pub orphaned_files_config: Option<OrphanedFilesConfig>,
}

impl CustomSync for DownloadClient {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let env: Value = client.get(DOWNLOAD_CLIENT_PATH).await?;
            let live: Vec<Value> = env
                .get("clients")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            let mut changes = Vec::with_capacity(desired.len());

            for cfg in desired {
                let name = cfg
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow::anyhow!("download client entry is missing `name`"))?;
                let wire = engine::encode_config::<Self>(cfg)?;

                let existing = live
                    .iter()
                    .find(|c| c.get("name").and_then(Value::as_str) == Some(name));

                let (id, kind) = match existing {
                    Some(l) => {
                        let id = l.get("id").cloned().unwrap_or(Value::Null);
                        if diff::subset(&wire, l) {
                            (id, ChangeKind::Unchanged)
                        } else {
                            if execute {
                                let id_str = id.as_str().unwrap_or_default();
                                let _: Value = client
                                    .put(&format!("{DOWNLOAD_CLIENT_PATH}/{id_str}"), &wire)
                                    .await?;
                            }
                            (id, ChangeKind::Updated)
                        }
                    }
                    None => {
                        if !execute {
                            // Preview: no create, so no real GUID yet — the
                            // nested sub-endpoints need one, so skip them and
                            // just record intent (mirrors jellyfin `user`).
                            refs.insert(
                                "download_client",
                                name,
                                RefId::Pending(engine::id_shape::<Self>()),
                            );
                            changes.push(Change::created(name));
                            continue;
                        }
                        let created: Value = client.post(DOWNLOAD_CLIENT_PATH, &wire).await?;
                        (
                            created.get("id").cloned().unwrap_or(Value::Null),
                            ChangeKind::Created,
                        )
                    }
                };

                if let Some(rid) = RefId::from_value(&id) {
                    refs.insert("download_client", name, rid);
                }
                let id_str = id.as_str().unwrap_or_default().to_string();

                // Reconcile the four nested concerns against the resolved GUID.
                let mut detail = Vec::new();
                let mut nested_changed = false;

                // An **absent** `seeding_rules` key means unmanaged — leave the
                // client's rules alone. Only an explicitly declared (possibly
                // empty) list is managed, and only then does `--prune` clear it.
                // Defaulting the absent case to `[]` would make a config that
                // never mentions seeding rules delete every one of them.
                if let Some(rules_cfg) = cfg.get("seeding_rules").and_then(Value::as_array)
                    && let Some(summary) = seeding_rule::reconcile_seeding_rules(
                        client, &id_str, rules_cfg, prune, execute,
                    )
                    .await?
                {
                    nested_changed = true;
                    detail.push(("seeding_rules".to_string(), summary));
                }

                if let Some(summary) = reconcile_simple_config::<UnlinkedConfig>(
                    client,
                    "unlinked-config",
                    &id_str,
                    cfg.get("unlinked_config"),
                    execute,
                )
                .await?
                {
                    nested_changed = true;
                    detail.push(("unlinked_config".to_string(), summary.to_string()));
                }
                if let Some(summary) = reconcile_simple_config::<DeadTorrentConfig>(
                    client,
                    "dead-torrent-config",
                    &id_str,
                    cfg.get("dead_torrent_config"),
                    execute,
                )
                .await?
                {
                    nested_changed = true;
                    detail.push(("dead_torrent_config".to_string(), summary.to_string()));
                }
                if let Some(summary) = reconcile_simple_config::<OrphanedFilesConfig>(
                    client,
                    "orphaned-files-config",
                    &id_str,
                    cfg.get("orphaned_files_config"),
                    execute,
                )
                .await?
                {
                    nested_changed = true;
                    detail.push(("orphaned_files_config".to_string(), summary.to_string()));
                }

                let final_kind = match kind {
                    ChangeKind::Created => ChangeKind::Created,
                    _ if nested_changed => ChangeKind::Updated,
                    other => other,
                };
                let mut change = Change {
                    key: name.to_string(),
                    kind: final_kind,
                    detail: Vec::new(),
                };
                for (label, value) in detail {
                    change = change.with(label, value);
                }
                changes.push(change);
            }

            // Prune clients the config no longer declares — cascades
            // server-side to that client's seeding rules and unlinked /
            // dead-torrent / orphaned-files configs.
            changes.extend(
                reconcile::prune_absent(desired, &live, "name", prune, execute, |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        let id_str = id.as_str().unwrap_or_default();
                        client
                            .delete(&format!("{DOWNLOAD_CLIENT_PATH}/{id_str}"))
                            .await?;
                        Ok(())
                    }
                })
                .await?,
            );

            Ok(changes)
        })
    }
}

/// Reconcile one of the three simple `GET` (nullable) / `PUT` per-client
/// sub-configs (unlinked / dead-torrent / orphaned-files — all three are
/// upserts on the same shape of endpoint). `cfg` is the field's declared
/// value from the client's desired config (`None` = leave this endpoint
/// unmanaged entirely); `path_segment` is the REST path element
/// (`"unlinked-config"`, `"dead-torrent-config"`, `"orphaned-files-config"`).
/// Returns `Some("updated")` when the declared value wasn't already a subset
/// of live, `None` when nothing needed to change.
async fn reconcile_simple_config<T: core_lib::Described>(
    client: &HttpClient,
    path_segment: &str,
    client_id: &str,
    cfg: Option<&Value>,
    execute: bool,
) -> anyhow::Result<Option<&'static str>> {
    let Some(cfg) = cfg else {
        return Ok(None);
    };
    let url = format!("/api/{path_segment}/{client_id}");
    // Until the client has one of these saved, the endpoint answers 200 with an
    // **empty body** (the controller returns a null object), which a strict JSON
    // read rejects with `EOF while parsing a value`. `get_optional` maps that to
    // `Null`, which `subset` then treats as "nothing there yet" → we write.
    let live: Value = client.get_optional(&url).await?;
    let wire = engine::encode_config::<T>(cfg)?;
    if diff::subset(&wire, &live) {
        return Ok(None);
    }
    if execute {
        let _: Value = client.put(&url, &wire).await?;
    }
    Ok(Some("updated"))
}
