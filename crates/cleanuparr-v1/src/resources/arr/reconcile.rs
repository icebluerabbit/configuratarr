//! Shared reconcile for the five *arr instance collections (Sonarr, Radarr,
//! Lidarr, Readarr, Whisparr — see the module doc). Every one of them talks to
//! the *same* endpoint shape:
//!
//!   * `GET    /api/configuration/<app>`                     — an envelope
//!     object (`{ id, type, instances }`), not a bare array; the live
//!     collection is its `.instances`.
//!   * `POST   /api/configuration/<app>/instances`            — create
//!   * `PUT    /api/configuration/<app>/instances/{id}`       — update
//!   * `DELETE /api/configuration/<app>/instances/{id}`       — delete
//!
//! and differs only in the app slug and the [`RefStore`] type name other
//! resources address it by. So the plumbing lives here once, and each app's
//! `CustomSync::reconcile` is a two-line call into [`reconcile_instances`].
//!
//! Idempotency is [`crate::diff::subset`] (`apiKey` reads back masked — see
//! there); the create/update/prune skeleton is
//! [`core_lib::reconcile::upsert_prune`].
//!
//! Ref registration (GUID ids, so [`RefId::Str`] not `RefId::Int`):
//! * every already-live instance's id is registered before reconciling, so an
//!   unrelated re-apply still resolves `${ref.<ref_type>.<name>}`;
//! * a preview (`!execute`) never calls the `create` closure below (see
//!   [`core_lib::reconcile::upsert`]), so any name not yet live is registered
//!   as [`RefId::Pending`] up front instead;
//! * a real create's response carries the freshly minted GUID, which the
//!   `create` closure stashes for registration once the reconcile completes.

use std::collections::HashSet;
use std::sync::Mutex;

use core_lib::{Change, Described, HttpClient, IdShape, RefId, RefStore, engine, reconcile};
use serde_json::Value;

use crate::diff;

/// `GET /api/configuration/<app>` and pluck `.instances` — the live collection
/// (the envelope's `id`/`type` are irrelevant here; the app-config singleton
/// owns those).
async fn fetch_instances(client: &HttpClient, app: &str) -> anyhow::Result<Vec<Value>> {
    let config: Value = client.get(&format!("/api/configuration/{app}")).await?;
    Ok(config
        .get("instances")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default())
}

/// Register each live instance's GUID under `ref_type`, keyed by `name`.
fn register_ids(refs: &mut RefStore, ref_type: &str, instances: &[(String, Value)]) {
    for (name, id) in instances {
        if let Some(rid) = RefId::from_value(id) {
            refs.insert(ref_type, name, rid);
        }
    }
}

/// Reconcile one *arr app's instance collection. `app` is the URL slug
/// (`"sonarr"`, `"radarr"`, …); `ref_type` is the [`RefStore`] type name other
/// resources address via `${ref.<ref_type>.<name>}` (`"sonarr_instance"`, …).
/// `desired` is config-shaped (pre-encode); `T` is the resource type whose
/// codec decodes it.
pub async fn reconcile_instances<T: Described>(
    client: &HttpClient,
    app: &str,
    ref_type: &str,
    desired: &[Value],
    refs: &mut RefStore,
    prune: bool,
    execute: bool,
) -> anyhow::Result<Vec<Change>> {
    let live = fetch_instances(client, app).await?;
    register_ids(
        refs,
        ref_type,
        &live
            .iter()
            .filter_map(|l| {
                let name = l.get("name").and_then(Value::as_str)?;
                Some((
                    name.to_string(),
                    l.get("id").cloned().unwrap_or(Value::Null),
                ))
            })
            .collect::<Vec<_>>(),
    );

    let wire: Vec<Value> = desired
        .iter()
        .map(engine::encode_config::<T>)
        .collect::<anyhow::Result<_>>()?;

    if !execute {
        // No create actually runs during a preview (see
        // `core_lib::reconcile::upsert`), so there is no POST response to read
        // an id from — record intent instead so a downstream `${ref}` still
        // resolves in the plan.
        let live_names: HashSet<&str> = live
            .iter()
            .filter_map(|l| l.get("name").and_then(Value::as_str))
            .collect();
        for w in &wire {
            if let Some(name) = w.get("name").and_then(Value::as_str)
                && !live_names.contains(name)
            {
                // Both instance types this helper serves use GUID ids, and
                // neither declares an `#[id]` field.
                refs.insert(ref_type, name, RefId::Pending(IdShape::Str));
            }
        }
    }

    let created: Mutex<Vec<(String, Value)>> = Mutex::new(Vec::new());
    let instances_path = format!("/api/configuration/{app}/instances");

    let changes = reconcile::upsert_prune(
        &wire,
        &live,
        "name",
        diff::subset,
        prune,
        execute,
        |w| {
            let client = client.clone();
            let path = instances_path.clone();
            let created = &created;
            async move {
                let resp: Value = client.post(&path, &w).await?;
                if let Some(name) = w.get("name").and_then(Value::as_str) {
                    created.lock().expect("created-ids mutex poisoned").push((
                        name.to_string(),
                        resp.get("id").cloned().unwrap_or(Value::Null),
                    ));
                }
                Ok(())
            }
        },
        |l, w| {
            let client = client.clone();
            let id = l.get("id").and_then(Value::as_str).unwrap_or_default();
            let path = format!("{instances_path}/{id}");
            async move {
                let _: Value = client.put(&path, &w).await?;
                Ok(())
            }
        },
        |l| {
            let client = client.clone();
            let id = l.get("id").and_then(Value::as_str).unwrap_or_default();
            let path = format!("{instances_path}/{id}");
            async move {
                client.delete(&path).await?;
                Ok(())
            }
        },
    )
    .await?;

    register_ids(
        refs,
        ref_type,
        &created.into_inner().expect("created-ids mutex poisoned"),
    );

    Ok(changes)
}
