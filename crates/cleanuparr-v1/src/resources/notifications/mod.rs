//! `/api/configuration/notification_providers` — outbound notification
//! providers: Notifiarr, Apprise, Ntfy, Pushover, Telegram, Discord, Gotify.
//! Each provider type is its own `sync = custom` collection, keyed by `name`,
//! one file per type.
//!
//! **Provider names are unique across all seven types** — Cleanuparr enforces
//! this server-side (`name` is the row's identity regardless of `type`), even
//! though each of these seven resources reconciles independently against its
//! own slug. Reusing a `name` already claimed by a different provider type
//! fails the write.
//!
//! Endpoints, shared by every provider type (only the URL slug / `type`
//! discriminator differ):
//! ```text
//! GET    /api/configuration/notification_providers        → { "providers": [...] }
//! POST   /api/configuration/notification_providers/<slug>          (flat create body)
//! PUT    /api/configuration/notification_providers/<slug>/{id}     (flat update body)
//! DELETE /api/configuration/notification_providers/{id}            (NOT slug-scoped)
//! ```
//!
//! The read shape is structurally different from the write shape: a live
//! provider nests its per-event flags under `events` and its provider-specific
//! settings under `configuration`:
//! ```json
//! { "id": "…", "name": "…", "type": "Ntfy", "isEnabled": true,
//!   "events": { "onFailedImportStrike": true, … },
//!   "configuration": { "serverUrl": "…", "topics": [...], "password": "••••••••", … } }
//! ```
//! while the create/update body is flat — `name`, `isEnabled`, the eight
//! `onX` flags, and the provider's own fields, all side by side. [`normalise`]
//! hoists `events.*`/`configuration.*` up to the top level before the
//! [`crate::diff::subset`] idempotency check runs, so the comparison is
//! apples-to-apples. Secrets (`apiKey`, `password`, `webhookUrl`, …) read back
//! as `••••••••`, which `subset` already treats as in sync.
//!
//! An update **deletes and recreates the row** server-side, so a provider's
//! `id` changes on every update — never key on it; `name` is the only stable
//! identity ([`core_lib::reconcile::upsert_prune`]).
//!
//! [`reconcile_provider`] is the one piece of logic shared by all seven
//! `CustomSync` impls (GET + filter-by-type + normalise + `upsert_prune`);
//! each provider file supplies only its `slug` and discriminator `type`
//! string in a couple of lines, mirroring `autobrr-v1`'s
//! [`crate::diff::subset`]-based custom resources and lazylibrarian's shared
//! `reconcile_family` (`crates/lazylibrarian-v1/src/resources/providers/mod.rs`).

use core_lib::{Change, Described, HttpClient, engine, reconcile};
use serde_json::Value;

pub mod apprise;
pub mod discord;
pub mod gotify;
pub mod notifiarr;
pub mod ntfy;
pub mod pushover;
pub mod telegram;

/// `GET` wrapper endpoint — lists every provider, of every type, together.
const LIST_PATH: &str = "/api/configuration/notification_providers";

/// Hoist a live provider's nested `events.*` and `configuration.*` keys up to
/// the top level and drop the `events`/`configuration` wrappers themselves —
/// the flat shape the create/update body uses. `name`, `type`, `isEnabled` are
/// already top-level on read and pass through unchanged.
///
/// **The provider's own `id` is restored after the hoist.** The nested
/// `configuration` object is a separate persisted row with an `id` of its own
/// (`NtfyConfig.Id`, …), so hoisting it blindly overwrites the provider id that
/// the update path builds its URL from — and `PUT …/ntfy/{configurationId}`
/// answers `404 … provider with ID … not found`.
fn normalise(mut live: Value) -> Value {
    let Some(obj) = live.as_object() else {
        return live;
    };
    let events = obj.get("events").and_then(Value::as_object).cloned();
    let configuration = obj.get("configuration").and_then(Value::as_object).cloned();
    let provider_id = obj.get("id").cloned();

    let obj = live.as_object_mut().expect("checked above");
    obj.remove("events");
    obj.remove("configuration");
    if let Some(events) = events {
        obj.extend(events);
    }
    if let Some(configuration) = configuration {
        obj.extend(configuration);
    }
    match provider_id {
        Some(id) => {
            obj.insert("id".to_string(), id);
        }
        None => {
            obj.remove("id");
        }
    }
    live
}

/// GET the shared wrapper, keep only entries whose `type` is `type_tag`, and
/// [`normalise`] each into the flat write shape.
async fn live_of_type(client: &HttpClient, type_tag: &str) -> anyhow::Result<Vec<Value>> {
    let resp: Value = client.get(LIST_PATH).await?;
    let providers = resp
        .get("providers")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok(providers
        .into_iter()
        .filter(|p| p.get("type").and_then(Value::as_str) == Some(type_tag))
        .map(normalise)
        .collect())
}

/// Shared reconcile for every notification provider type: GET + filter +
/// normalise the live set of `type_tag`, encode `desired` to wire form, then
/// [`core_lib::reconcile::upsert_prune`] keyed by `name` with
/// [`crate::diff::subset`] deciding idempotency (secrets read back masked;
/// extra server-added keys are ignored).
///
/// `slug` is the URL path segment (`"ntfy"`, `"discord"`, …); `type_tag` is
/// the live `type` discriminator this provider's rows carry (`"Ntfy"`,
/// `"Discord"`, …). Create/update POST/PUT `.../<slug>[/…]`; delete is
/// **not** slug-scoped (`DELETE /api/configuration/notification_providers/{id}`).
pub(crate) async fn reconcile_provider<T: Described>(
    client: &HttpClient,
    desired: &[Value],
    prune: bool,
    execute: bool,
    slug: &str,
    type_tag: &str,
) -> anyhow::Result<Vec<Change>> {
    let live = live_of_type(client, type_tag).await?;
    let wire: Vec<Value> = desired
        .iter()
        .map(engine::encode_config::<T>)
        .collect::<anyhow::Result<_>>()?;
    let create_path = format!("{LIST_PATH}/{slug}");

    reconcile::upsert_prune(
        &wire,
        &live,
        "name",
        crate::diff::subset,
        prune,
        execute,
        |w| {
            let client = client.clone();
            let path = create_path.clone();
            async move {
                let _: Value = client.post(&path, &w).await?;
                Ok(())
            }
        },
        |l, w| {
            let client = client.clone();
            let id = l.get("id").and_then(Value::as_str).unwrap_or_default();
            let path = format!("{LIST_PATH}/{slug}/{id}");
            async move {
                let _: Value = client.put(&path, &w).await?;
                Ok(())
            }
        },
        |l| {
            let client = client.clone();
            let id = l.get("id").and_then(Value::as_str).unwrap_or_default();
            let path = format!("{LIST_PATH}/{id}");
            async move {
                client.delete(&path).await?;
                Ok(())
            }
        },
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn normalise_hoists_events_and_configuration() {
        let live = json!({
            "id": "abc-123",
            "name": "my-ntfy",
            "type": "Ntfy",
            "isEnabled": true,
            "events": { "onFailedImportStrike": true, "onStalledStrike": false },
            "configuration": { "serverUrl": "https://ntfy.sh", "topics": ["a", "b"] },
        });
        let flat = normalise(live);
        assert_eq!(flat["onFailedImportStrike"], json!(true));
        assert_eq!(flat["onStalledStrike"], json!(false));
        assert_eq!(flat["serverUrl"], json!("https://ntfy.sh"));
        assert_eq!(flat["topics"], json!(["a", "b"]));
        assert_eq!(flat["id"], json!("abc-123"));
        assert!(flat.get("events").is_none());
        assert!(flat.get("configuration").is_none());
    }

    /// The nested `configuration` row carries its own `id`. Hoisting it must not
    /// clobber the provider's id — the update path builds `…/{slug}/{id}` from
    /// it, and the config row's GUID 404s.
    #[test]
    fn configuration_id_does_not_clobber_the_provider_id() {
        let live = json!({
            "id": "provider-guid",
            "name": "my-ntfy",
            "type": "Ntfy",
            "configuration": { "id": "configuration-guid", "serverUrl": "https://ntfy.sh" },
        });
        let flat = normalise(live);
        assert_eq!(flat["id"], json!("provider-guid"));
        assert_eq!(flat["serverUrl"], json!("https://ntfy.sh"));
    }
}
