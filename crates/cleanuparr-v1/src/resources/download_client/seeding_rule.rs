//! `seeding_rules` — per-download-client rules controlling how long/well a
//! torrent must seed before Cleanuparr removes it.
//!
//! Scoped to one download client, so every endpoint is addressed by the
//! owning client's GUID (list/create) or the rule's own GUID (update/delete):
//!   * `GET  /api/seeding-rules/{downloadClientId}`  — list, ordered by priority
//!   * `POST /api/seeding-rules/{downloadClientId}`  — create
//!   * `PUT  /api/seeding-rules/{id}`                — update (id = rule id)
//!   * `DELETE /api/seeding-rules/{id}`               — delete (id = rule id)
//!   * `PUT  /api/seeding-rules/{downloadClientId}/reorder` — reassign priorities
//!
//! [`reconcile_seeding_rules`] is called once per client, after the client's own GUID is
//! resolved (see [`super::DownloadClient`]'s `impl CustomSync`).

use core_lib::{ChangeKind, HttpClient, engine, reconcile};
use core_macros::nested;
use serde_json::{Value, json};

use crate::diff;
use crate::resources::stall_rule::TorrentPrivacyType;

/// One seeding rule scoped to a download client.
///
/// Nested under [`super::DownloadClient::seeding_rules`]. `priority` isn't
/// modelled as a field here — the server auto-assigns it on create and
/// **ignores it on update** (`SeedingRuleRequest`'s own description says so);
/// ordering is instead driven by this list's declared order and reconciled
/// with a dedicated `PUT .../reorder` call (see [`reconcile_seeding_rules`]).
#[nested]
pub struct SeedingRule {
    /// Rule name — its identity among the client's seeding rules.
    #[key]
    pub name: String,
    /// Torrent categories this rule applies to. At least one is required.
    pub categories: Vec<String>,
    /// Tracker domain suffixes this rule applies to, case-insensitive. Empty
    /// matches any tracker.
    pub tracker_patterns: Vec<String>,
    /// Torrent must carry at least one of these tags. Accepted for every
    /// client but silently ignored for Deluge, rTorrent, and µTorrent.
    pub tags_any: Vec<String>,
    /// Torrent must carry all of these tags. Accepted for every client but
    /// silently ignored for Deluge, rTorrent, and µTorrent.
    pub tags_all: Vec<String>,
    /// Restrict this rule to torrents of a given privacy classification.
    /// Omitted, the server treats it as `Public`.
    pub privacy_type: Option<TorrentPrivacyType>,
    /// Seed ratio to reach before removal. `-1` disables. Either `max_ratio`
    /// or `max_seed_time` must be non-negative.
    #[default(-1.0)]
    pub max_ratio: f64,
    /// Hours to seed before removal once the ratio is met.
    #[default(0.0)]
    pub min_seed_time: f64,
    /// Hours to seed before removal regardless of ratio. `-1` disables.
    #[default(-1.0)]
    pub max_seed_time: f64,
    /// Minimum seeders required before this rule evaluates the torrent.
    /// Ignored for rTorrent, which reports no seeder count.
    #[default(0)]
    pub min_seeders: i32,
    /// qBittorrent only: maximum inactive days before removal. `None`
    /// (server) / `-1` disables.
    pub max_inactive_days: Option<f64>,
    /// Delete the torrent's source files from disk (not just remove it from
    /// the client's queue) once this rule removes it.
    #[default(true)]
    pub delete_source_files: bool,
}

/// Reconcile one client's seeding rules: upsert by `name`
/// ([`crate::diff::subset`] for idempotency — live rules carry an `id` the
/// declared config never sets, and unsupported-by-this-client-type fields
/// come back empty/null), prune the rest under `--prune`, then reorder if the
/// declared order (this slice's array order) no longer matches live.
///
/// `desired_cfg` is the client's declared `seeding_rules` — plain user-config
/// `Value`s, snake_case, in the order the user wrote them (that order is the
/// desired priority). Returns a short summary when anything changed, `None`
/// when the rule set already matches.
pub(crate) async fn reconcile_seeding_rules(
    client: &HttpClient,
    client_id: &str,
    desired_cfg: &[Value],
    prune: bool,
    execute: bool,
) -> anyhow::Result<Option<String>> {
    let base = format!("/api/seeding-rules/{client_id}");
    let live: Vec<Value> = client.get(&base).await?;
    let wire: Vec<Value> = desired_cfg
        .iter()
        .map(engine::encode_config::<SeedingRule>)
        .collect::<anyhow::Result<_>>()?;

    let rule_changes = reconcile::upsert(
        &wire,
        &live,
        "name",
        diff::subset,
        execute,
        |w| {
            let client = client.clone();
            let base = base.clone();
            async move {
                let _: Value = client.post(&base, &w).await?;
                Ok(())
            }
        },
        |l, w| {
            let client = client.clone();
            let id = rule_id(l);
            async move {
                let _: Value = client.put(&format!("/api/seeding-rules/{id}"), &w).await?;
                Ok(())
            }
        },
    )
    .await?;
    let created = rule_changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Created)
        .count();
    let updated = rule_changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Updated)
        .count();

    let pruned_changes = reconcile::prune_absent(&wire, &live, "name", prune, execute, |l| {
        let client = client.clone();
        let id = rule_id(l);
        async move {
            client.delete(&format!("/api/seeding-rules/{id}")).await?;
            Ok(())
        }
    })
    .await?;
    let pruned = pruned_changes.len();

    let reordered = reconcile_order(client, &base, &wire, &live, execute).await?;

    if created == 0 && updated == 0 && pruned == 0 && !reordered {
        return Ok(None);
    }
    let mut parts = Vec::new();
    if created > 0 {
        parts.push(format!("{created} created"));
    }
    if updated > 0 {
        parts.push(format!("{updated} updated"));
    }
    if pruned > 0 {
        parts.push(format!("{pruned} pruned"));
    }
    if reordered {
        parts.push("reordered".to_string());
    }
    Ok(Some(parts.join(", ")))
}

/// A live rule's GUID, as a `String` (empty if somehow absent — defensive,
/// never expected from a real server response).
fn rule_id(live: &Value) -> String {
    live.get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

/// Reassign rule priorities via `PUT {base}/reorder` when the declared order
/// (by name) no longer matches live. Needs every rule's real GUID, so it only
/// ever writes when `execute` — after upsert/prune have run, a fresh GET
/// carries the id of anything just created and drops anything just pruned.
///
/// During a preview (`!execute`), order drift is detected best-effort from
/// the pre-write `live`: only rules that already exist on both sides can be
/// compared (a brand-new rule has no id yet to verify a position for), so a
/// plan may under-report a reorder that a create would also trigger.
async fn reconcile_order(
    client: &HttpClient,
    base: &str,
    wire: &[Value],
    live: &[Value],
    execute: bool,
) -> anyhow::Result<bool> {
    if wire.is_empty() {
        return Ok(false);
    }
    if !execute {
        let declared: Vec<&str> = wire
            .iter()
            .filter_map(|w| w.get("name").and_then(Value::as_str))
            .filter(|n| {
                live.iter()
                    .any(|l| l.get("name").and_then(Value::as_str) == Some(*n))
            })
            .collect();
        let current: Vec<&str> = live
            .iter()
            .filter_map(|l| l.get("name").and_then(Value::as_str))
            .filter(|n| declared.contains(n))
            .collect();
        return Ok(declared != current);
    }

    let fresh: Vec<Value> = client.get(base).await?;
    let mut ordered_ids: Vec<String> = Vec::with_capacity(fresh.len());
    for w in wire {
        let name = w.get("name").and_then(Value::as_str).unwrap_or_default();
        if let Some(id) = fresh
            .iter()
            .find(|l| l.get("name").and_then(Value::as_str) == Some(name))
            .and_then(|l| l.get("id").and_then(Value::as_str))
        {
            ordered_ids.push(id.to_string());
        }
    }
    // Rules the config left undeclared (only possible with `--prune` off)
    // must still be listed — the reorder endpoint requires every rule of the
    // client exactly once.
    for l in &fresh {
        if let Some(id) = l.get("id").and_then(Value::as_str)
            && !ordered_ids.iter().any(|x| x == id)
        {
            ordered_ids.push(id.to_string());
        }
    }

    let current_ids: Vec<&str> = fresh
        .iter()
        .filter_map(|l| l.get("id").and_then(Value::as_str))
        .collect();
    if ordered_ids
        .iter()
        .map(String::as_str)
        .eq(current_ids.iter().copied())
    {
        return Ok(false);
    }
    let _: Value = client
        .put(
            &format!("{base}/reorder"),
            &json!({ "orderedIds": ordered_ids }),
        )
        .await?;
    Ok(true)
}
