//! `/api/v3/delayprofile` — how long to wait before grabbing a release, per
//! protocol, for the media carrying a given tag.
//!
//! `sync = custom` because this resource has **no usable natural key**. eros
//! exposes no name or label, and the only candidate — `order` — is
//! **server-assigned**: verified live, a `POST` with `"order": 100` comes back
//! as `"order": 1`. A `sync = crud` step keyed on `order` would never match its
//! own creation, so every apply would re-create the profile and eros would
//! reject it with `One or more tags is used in another profile`.
//!
//! That error is also the answer: eros enforces **at most one delay profile per
//! tag**, which makes the *tag set* the resource's real identity. The hook
//! stamps a synthetic `_identity` (the sorted tag ids) onto copies of the
//! desired/live values for [`reconcile::upsert_prune`] to match on, then strips
//! it before anything reaches the wire — the same technique
//! `crates/audiobookshelf-v1/src/resources/notification.rs` uses for its
//! composite `(event, library)` identity.
//!
//! **The global profile is managed, but never created or deleted.** eros seeds
//! one catch-all profile at `id: 1` (`001_initial_setup.cs`), live at
//! `order: 2147483647` with an empty tag set; it governs every item carrying no
//! matching tag (`DelayProfileService.cs:100`). It is *updatable* — only
//! `Delete` is guarded (`DelayProfileController.cs:47-55`,
//! `"Cannot delete global delay profile"`); `Update` has no id check. The
//! validator makes "empty tags ⟺ id 1" an invariant:
//!
//! ```csharp
//! SharedValidator.RuleFor(d => d.Tags).NotEmpty().When(d => d.Id != 1);
//! SharedValidator.RuleFor(d => d.Tags).EmptyCollection<…>().When(d => d.Id == 1);
//! ```
//!
//! So declaring `tags: []` addresses the global profile: its identity is the
//! empty string, it matches the seeded row, and the hook issues an UPDATE. It is
//! held out of the **prune** half only — a config that stops declaring it must
//! not attempt a delete the server refuses.

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, engine, reconcile};
use core_macros::resource;
use serde_json::Value;

use crate::resources::download_protocol::DownloadProtocol;

/// Server-owned fields excluded from the idempotency comparison: `id`, and
/// `order` because eros assigns it (see the module doc).
const IGNORED_KEYS: &[&str] = &["id", "order", "_identity"];

#[resource(
    sync = custom,
    list = get("/api/v3/delayprofile"),
    create = post("/api/v3/delayprofile"),
    update = put("/api/v3/delayprofile/${self.id}"),
    delete = delete("/api/v3/delayprofile/${self.id}"),
)]
pub struct DelayProfile {
    #[id]
    pub id: Option<i32>,
    /// Whether usenet releases are subject to this profile.
    pub enable_usenet: bool,
    /// Whether torrent releases are subject to this profile.
    pub enable_torrent: bool,
    /// Which protocol wins when a release is available on both.
    pub preferred_protocol: DownloadProtocol,
    /// Minutes to wait before grabbing a usenet release.
    pub usenet_delay: i32,
    /// Minutes to wait before grabbing a torrent release.
    pub torrent_delay: i32,
    /// Grab immediately when the release is already at the profile's cutoff.
    pub bypass_if_highest_quality: bool,
    /// Grab immediately when the release scores above
    /// `minimum_custom_format_score`.
    pub bypass_if_above_custom_format_score: bool,
    /// The score `bypass_if_above_custom_format_score` compares against.
    pub minimum_custom_format_score: i32,
    /// Evaluation order. **Server-assigned** — eros ignores any value sent and
    /// renumbers profiles itself, so this is read-only here.
    #[wire(read_only)]
    pub order: Option<i32>,
    /// Tag references — this profile's identity. eros permits at most one delay
    /// profile per tag. An **empty** list addresses the seeded global profile
    /// (`id: 1`), which governs everything untagged; any other profile must carry
    /// at least one tag or eros rejects it with `'Tags' must not be empty`.
    #[reference(tag)]
    pub tags: Vec<i32>,
}

/// Sorted tag ids joined — this profile's identity for the reconcile match.
/// Never written to the wire under this name; see [`with_identity`] /
/// [`strip_identity`].
fn identity_of(v: &Value) -> String {
    let mut tags: Vec<i64> = v
        .get("tags")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_i64)
        .collect();
    tags.sort_unstable();
    tags.iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// The seeded global profile's id — the one row eros refuses to delete.
const GLOBAL_PROFILE_ID: i64 = 1;

/// True when `v` is the seeded global profile: `id == 1`, equivalently (by the
/// validator invariant in the module doc) the profile with no tags. Both are
/// checked so a live row missing either field still classifies.
fn is_global(v: &Value) -> bool {
    v.get("id").and_then(Value::as_i64) == Some(GLOBAL_PROFILE_ID)
        || v.get("tags")
            .and_then(Value::as_array)
            .is_none_or(|tags| tags.is_empty())
}

/// Stamp `_identity` onto a copy of `v` for [`reconcile::upsert_prune`].
fn with_identity(mut v: Value) -> Value {
    let identity = identity_of(&v);
    if let Some(obj) = v.as_object_mut() {
        obj.insert("_identity".to_string(), Value::String(identity));
    }
    v
}

/// Remove the synthetic `_identity` key before a value reaches the wire.
fn strip_identity(mut v: Value) -> Value {
    if let Some(obj) = v.as_object_mut() {
        obj.remove("_identity");
    }
    v
}

/// `v` without the server-owned / synthetic keys.
fn stripped_for_compare(v: &Value) -> Value {
    let mut obj = v.as_object().cloned().unwrap_or_default();
    for k in IGNORED_KEYS {
        obj.remove(*k);
    }
    Value::Object(obj)
}

/// Every field we declare must match the live profile. A subset test rather than
/// equality: the live record carries `order` (server-assigned) and `id`, both
/// stripped above.
fn in_sync(desired: &Value, live: &Value) -> bool {
    let d = stripped_for_compare(desired);
    let Some(d) = d.as_object() else { return false };
    d.iter().all(|(k, v)| live.get(k) == Some(v))
}

impl CustomSync for DelayProfile {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/v3/delayprofile").await?;

            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            let desired_keyed: Vec<Value> = wire.into_iter().map(with_identity).collect();
            let live_keyed: Vec<Value> = live.into_iter().map(with_identity).collect();

            // Prune sees every profile *except* the global one — eros refuses to
            // delete it, so a config that simply doesn't mention it must leave it
            // alone rather than fail the apply. Creates/updates still see it, so
            // `tags: []` addresses it. Hence upsert + prune_absent rather than
            // `upsert_prune`, which shares one live set between both halves.
            let prunable: Vec<Value> = live_keyed
                .iter()
                .filter(|v| !is_global(v))
                .cloned()
                .collect();

            let mut changes = reconcile::upsert(
                &desired_keyed,
                &live_keyed,
                "_identity",
                in_sync,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let body = strip_identity(w);
                        let _: Value = client.post("/api/v3/delayprofile", &body).await?;
                        Ok(())
                    }
                },
                |l, w| {
                    let client = client.clone();
                    let id = l.get("id").and_then(Value::as_i64).unwrap_or_default();
                    let mut body = strip_identity(w);
                    // `order` is server-owned: echo the live value back rather
                    // than omitting it, so the PUT cannot renumber the profile.
                    reconcile::echo(&mut body, "id", l);
                    reconcile::echo(&mut body, "order", l);
                    async move {
                        let _: Value = client
                            .put(&format!("/api/v3/delayprofile/{id}"), &body)
                            .await?;
                        Ok(())
                    }
                },
            )
            .await?;

            changes.extend(
                reconcile::prune_absent(
                    &desired_keyed,
                    &prunable,
                    "_identity",
                    prune,
                    execute,
                    |l| {
                        let client = client.clone();
                        let id = l.get("id").and_then(Value::as_i64).unwrap_or_default();
                        async move {
                            client.delete(&format!("/api/v3/delayprofile/{id}")).await?;
                            Ok(())
                        }
                    },
                )
                .await?,
            );

            Ok(changes)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identity_is_order_insensitive_over_tags() {
        assert_eq!(
            identity_of(&json!({ "tags": [3, 1, 2] })),
            identity_of(&json!({ "tags": [1, 2, 3] }))
        );
    }

    #[test]
    fn identity_distinguishes_different_tag_sets() {
        assert_ne!(
            identity_of(&json!({ "tags": [1, 2] })),
            identity_of(&json!({ "tags": [1, 3] }))
        );
    }

    #[test]
    fn global_profile_detected_by_id_or_empty_tags() {
        // The seeded row, as eros actually returns it.
        assert!(is_global(
            &json!({ "id": 1, "order": 2147483647, "tags": [] })
        ));
        // Either signal alone suffices.
        assert!(is_global(&json!({ "id": 1, "tags": [4] })));
        assert!(is_global(&json!({ "id": 9, "tags": [] })));
        // A normal tagged profile is not the global one.
        assert!(!is_global(&json!({ "id": 2, "order": 1, "tags": [4] })));
    }

    #[test]
    fn empty_tags_addresses_the_global_profile() {
        // A config declaring `tags: []` must key to the same identity as the
        // seeded row, so it UPDATEs it rather than attempting a create that
        // eros would reject with `'Tags' must not be empty`.
        assert_eq!(
            identity_of(&json!({ "tags": [] })),
            identity_of(&json!({ "id": 1, "order": 2147483647, "tags": [] }))
        );
    }

    #[test]
    fn in_sync_ignores_server_assigned_order() {
        let desired = json!({ "usenetDelay": 0, "tags": [1] });
        let live = json!({ "id": 2, "order": 1, "usenetDelay": 0, "tags": [1] });
        assert!(in_sync(&desired, &live));
    }

    #[test]
    fn in_sync_catches_a_real_diff() {
        let desired = json!({ "usenetDelay": 30, "tags": [1] });
        let live = json!({ "id": 2, "order": 1, "usenetDelay": 0, "tags": [1] });
        assert!(!in_sync(&desired, &live));
    }
}
