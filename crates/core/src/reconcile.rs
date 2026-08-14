//! Reconcile primitives for `sync = custom` hooks.
//!
//! The [`Custom`](crate::SyncKind::Custom) seam hands a hook the raw
//! `(client, desired, refs, prune, execute)` and trusts it to own its HTTP,
//! ordering, and idempotency. Two mechanics recur in *every* hook and are easy to get
//! wrong: **honouring `execute`** (a preview must perform no writes) and
//! **turning the outcome into report [`Change`]s**. Forget the first in one write
//! path and a `plan` silently mutates the server.
//!
//! These helpers own exactly those two mechanics for the recurring hook *shapes*,
//! so a hook expresses only its service-specific diff/write and can't leak a
//! write into a preview. Service-specific *translation* (wire shaping, bespoke
//! multi-endpoint flows) still lives in the service crate — this is the shared
//! floor beneath it, not a home for per-service logic.
//!
//! Shapes covered:
//! * [`create_only`] — keyed create-or-leave (no update, no prune).
//! * [`create_only_prune`] — create-or-leave plus a `--prune` delete tail (an API
//!   that creates + deletes but can't update).
//! * [`upsert`] — keyed create-or-update, no prune (the API doesn't round-trip
//!   writes, so idempotency is a service-supplied predicate, not merge-equality).
//! * [`upsert_prune`] — [`upsert`] plus a `--prune` delete tail (the API *does*
//!   round-trip deletes).
//! * [`prune_absent`] — the shared prune tail (delete each live item the config
//!   no longer declares); a bespoke hook can bolt it onto its own reconcile.
//! * [`replace`] — whole-list structural replace (one endpoint owns the set).
//!
//! Genuinely bespoke hooks (multi-endpoint per item, sparse form singletons)
//! keep their own explicit `if execute` branch — that is irreducible, not slop.

use std::future::Future;

use serde_json::Value;

use crate::apply::Change;

/// The identity values already present server-side: the `field` (e.g. `"Name"`,
/// `"AppName"`) of each live item, skipping any that lack it.
pub fn present_keys(live: &[Value], field: &str) -> Vec<String> {
    live.iter()
        .filter_map(|i| i.get(field).and_then(Value::as_str).map(String::from))
        .collect()
}

/// Keyed "create-or-leave" reconcile: for each desired item, [`unchanged`] if a
/// live item with the same `key_field` already exists, else run `create`. No
/// update, no prune — mirrors the *arr "don't delete/modify what you didn't
/// make" caution for server-owned resources (Jellyfin libraries, api keys).
///
/// `create` performs the write and runs **only when `execute`** — the driver
/// owns the preview gate, so the closure can't accidentally write during a plan.
/// Returns one [`Change`] per desired item, in order.
///
/// [`unchanged`]: Change::unchanged
pub async fn create_only<F, Fut>(
    desired: &[Value],
    key_field: &str,
    present: &[String],
    execute: bool,
    create: F,
) -> anyhow::Result<Vec<Change>>
where
    F: Fn(String, Value) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut changes = Vec::with_capacity(desired.len());
    for cfg in desired {
        let key = cfg
            .get(key_field)
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("entry is missing `{key_field}`"))?;

        if present.iter().any(|k| k == key) {
            changes.push(Change::unchanged(key));
            continue;
        }
        if execute {
            create(key.to_string(), cfg.clone()).await?;
        }
        changes.push(Change::created(key));
    }
    Ok(changes)
}

/// Keyed "create-or-update" reconcile, no prune: match each desired item to a
/// live one by `key_field`; if none exists run `create`, else if `in_sync` says
/// it already matches leave it, else run `update`. The recurring shape behind
/// APIs whose collections don't round-trip a write — server-rewritten ids,
/// redacted secrets, enriched sub-arrays — so plain merge-equality would churn
/// forever and the caller supplies its own convergence predicate instead.
///
/// `desired` are already in **wire** form (see [`crate::engine::encode_config`]);
/// `in_sync(desired, live)` decides idempotency; `create(wire)` / `update(live,
/// wire)` perform the writes and run **only when `execute`** — the driver owns
/// the preview gate. `update` receives the live item so it can echo server-owned
/// fields the write must carry back (see [`echo`]) or derive the update path.
/// Returns one [`Change`] per desired item, keyed by `key_field`, in order.
pub async fn upsert<S, C, CFut, U, UFut>(
    desired: &[Value],
    live: &[Value],
    key_field: &str,
    in_sync: S,
    execute: bool,
    create: C,
    update: U,
) -> anyhow::Result<Vec<Change>>
where
    S: Fn(&Value, &Value) -> bool,
    C: Fn(Value) -> CFut,
    CFut: Future<Output = anyhow::Result<()>>,
    U: Fn(&Value, Value) -> UFut,
    UFut: Future<Output = anyhow::Result<()>>,
{
    fn key_of<'v>(v: &'v Value, field: &str) -> Option<&'v str> {
        v.get(field).and_then(Value::as_str)
    }
    let mut changes = Vec::with_capacity(desired.len());
    for wire in desired {
        let key = key_of(wire, key_field)
            .ok_or_else(|| anyhow::anyhow!("entry is missing `{key_field}`"))?;
        let change = match live.iter().find(|l| key_of(l, key_field) == Some(key)) {
            Some(l) if in_sync(wire, l) => Change::unchanged(key),
            Some(l) => {
                if execute {
                    update(l, wire.clone()).await?;
                }
                Change::updated(key)
            }
            None => {
                if execute {
                    create(wire.clone()).await?;
                }
                Change::created(key)
            }
        };
        changes.push(change);
    }
    Ok(changes)
}

/// Delete each live item whose `key_field` isn't in `desired`, gated on `prune`
/// **and** `execute`. Emits one [`Change::removed`] per pruned item (in `live`
/// order). Shared prune tail of [`upsert_prune`] / [`create_only_prune`]; a hook
/// that already reconciles creates/updates itself (autobrr `filter`) can call
/// this directly for its prune half.
///
/// `delete(live)` performs the DELETE for one live item — it receives the live
/// value so it can read the server id / key the delete path needs.
pub async fn prune_absent<D, DFut>(
    desired: &[Value],
    live: &[Value],
    key_field: &str,
    prune: bool,
    execute: bool,
    delete: D,
) -> anyhow::Result<Vec<Change>>
where
    D: Fn(&Value) -> DFut,
    DFut: Future<Output = anyhow::Result<()>>,
{
    if !prune {
        return Ok(Vec::new());
    }
    fn key_of<'v>(v: &'v Value, field: &str) -> Option<&'v str> {
        v.get(field).and_then(Value::as_str)
    }
    let desired_keys: std::collections::HashSet<&str> = desired
        .iter()
        .filter_map(|v| key_of(v, key_field))
        .collect();

    let mut changes = Vec::new();
    for l in live {
        let Some(key) = key_of(l, key_field) else {
            continue;
        };
        if desired_keys.contains(key) {
            continue;
        }
        if execute {
            delete(l).await?;
        }
        changes.push(Change::removed(key));
    }
    Ok(changes)
}

/// Keyed "create-or-update-or-prune": [`upsert`] plus a prune tail. Matches each
/// desired item to a live one by `key_field` (create / update / leave via
/// `in_sync`), then — when `prune` — deletes every live item the config no longer
/// declares. The declarative-collection shape for a custom hook whose API *does*
/// round-trip deletes: mirrors crud's `--prune` gate (`prune` = `opts.prune`),
/// but keeps the caller's own idempotency predicate for the write half (redacted
/// secrets, server-rewritten ids). Emits the [`upsert`] changes in `desired`
/// order followed by one [`Change::removed`] per pruned item.
#[allow(clippy::too_many_arguments)]
pub async fn upsert_prune<S, C, CFut, U, UFut, D, DFut>(
    desired: &[Value],
    live: &[Value],
    key_field: &str,
    in_sync: S,
    prune: bool,
    execute: bool,
    create: C,
    update: U,
    delete: D,
) -> anyhow::Result<Vec<Change>>
where
    S: Fn(&Value, &Value) -> bool,
    C: Fn(Value) -> CFut,
    CFut: Future<Output = anyhow::Result<()>>,
    U: Fn(&Value, Value) -> UFut,
    UFut: Future<Output = anyhow::Result<()>>,
    D: Fn(&Value) -> DFut,
    DFut: Future<Output = anyhow::Result<()>>,
{
    let mut changes = upsert(desired, live, key_field, in_sync, execute, create, update).await?;
    changes.extend(prune_absent(desired, live, key_field, prune, execute, delete).await?);
    Ok(changes)
}

/// Keyed "create-or-leave-or-prune": [`create_only`] plus a prune tail. For an
/// API that creates and deletes but has **no update** (autobrr api keys, dedup
/// profiles) — a present key is left untouched, an absent-from-config live item
/// is deleted when `prune`. Emits one [`Change`] per desired item (created /
/// unchanged) in order, then one [`Change::removed`] per pruned item.
///
/// `create(key, cfg)` runs for each desired key not already live; `delete(live)`
/// runs for each live item the config no longer declares. Both gated on
/// `execute`; `delete` additionally on `prune`.
#[allow(clippy::too_many_arguments)]
pub async fn create_only_prune<C, CFut, D, DFut>(
    desired: &[Value],
    live: &[Value],
    key_field: &str,
    prune: bool,
    execute: bool,
    create: C,
    delete: D,
) -> anyhow::Result<Vec<Change>>
where
    C: Fn(String, Value) -> CFut,
    CFut: Future<Output = anyhow::Result<()>>,
    D: Fn(&Value) -> DFut,
    DFut: Future<Output = anyhow::Result<()>>,
{
    let present = present_keys(live, key_field);
    let mut changes = create_only(desired, key_field, &present, execute, create).await?;
    changes.extend(prune_absent(desired, live, key_field, prune, execute, delete).await?);
    Ok(changes)
}

/// Copy `key` from `from` into `wire` — a server-owned field an update must echo
/// back in its body (an id the API reads from the payload, a server-rewritten
/// identifier). No-op if `from` lacks `key` or `wire` isn't a JSON object.
pub fn echo(wire: &mut Value, key: &str, from: &Value) {
    if let (Some(obj), Some(v)) = (wire.as_object_mut(), from.get(key)) {
        obj.insert(key.to_string(), v.clone());
    }
}

/// Whole-list structural replace: an API with no per-item id where one endpoint
/// owns the entire set (Jellyfin `/Repositories`). Compares the `desired` set to
/// `live` by an order-insensitive `identity`, and if they differ runs `write`
/// (the full-list POST) — **only when `execute`**. Emits a single [`Change`]
/// under `label`: [`unchanged`](Change::unchanged) when the sets match, else
/// [`updated`](Change::updated).
///
/// `identity` maps one item (desired or live, already in the same wire shape) to
/// a comparable key; the comparison sorts both sides so it ignores order and JSON
/// key ordering.
pub async fn replace<K, F, Fut>(
    desired: &[Value],
    live: &[Value],
    label: &'static str,
    execute: bool,
    identity: impl Fn(&Value) -> K,
    write: F,
) -> anyhow::Result<Vec<Change>>
where
    K: Ord,
    F: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut want: Vec<K> = desired.iter().map(&identity).collect();
    let mut have: Vec<K> = live.iter().map(&identity).collect();
    want.sort();
    have.sort();

    if want == have {
        return Ok(vec![Change::unchanged(label)]);
    }
    if execute {
        write().await?;
    }
    Ok(vec![Change::updated(label)])
}
