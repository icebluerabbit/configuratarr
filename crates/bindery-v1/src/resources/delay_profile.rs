//! Delay profile — how long to wait before grabbing a release, per protocol,
//! so a better release has time to appear. Migration 003 seeds one profile
//! with no delay and `preferredProtocol` `usenet`. No owner column: delay
//! profiles are shared deployment config.
//!
//! The schema has no name field, so there is no natural key — and `order`
//! can't stand in for one: the spec says several profiles may share an
//! order, and every keyed `reconcile::*` primitive reads its key through
//! `as_str()`, which an integer can't drive. `collection_step` hard-requires
//! a `#[key]`, so this can't be `sync = crud`. Instead it's `sync = custom`
//! using [`core_lib::reconcile::replace`]: a whole-list structural replace —
//! DELETE every live profile, then POST each desired profile in order.
//!
//! Safe because nothing in the spec `#[reference]`s a delay profile: no
//! other schema in `bindery-v1.json` carries a `delayProfileId`-shaped
//! field, so no other resource's identity depends on a profile surviving
//! with a stable id across a recreate.
//!
//! Structural identity for the diff is `(order, usenetDelay, torrentDelay,
//! preferredProtocol, enableUsenet, enableTorrent)` — `id` and `createdAt`
//! are server-assigned/read-only and dropped.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::resource;
use serde_json::Value;

/// A delay profile — minutes to hold a release per protocol before grabbing
/// it, so a better release has time to appear.
#[resource(sync = custom, list = get("/api/v1/delayprofile"))]
pub struct DelayProfile {
    /// Server-assigned row id (SQLite `AUTOINCREMENT`). Read-only.
    #[id]
    pub id: Option<i64>,
    /// Minutes to hold a usenet release before grabbing it. `0` = grab
    /// immediately. Not validated — negative values are accepted and
    /// stored.
    #[default(0)]
    pub usenet_delay: i32,
    /// Minutes to hold a torrent release before grabbing it. `0` = grab
    /// immediately. Not validated — negative values are accepted and
    /// stored.
    #[default(0)]
    pub torrent_delay: i32,
    /// Protocol preferred when both are available (`usenet` or `torrent`,
    /// not enforced server-side). Omitted or empty resolves to `usenet`.
    // The hook fills this key explicitly rather than omitting it, so the
    // result doesn't depend on how the server treats a missing key: every
    // write here is a recreate (POST), and only create applies the fallback.
    pub preferred_protocol: Option<String>,
    /// Whether usenet releases are eligible under this profile.
    #[default(false)]
    pub enable_usenet: bool,
    /// Whether torrent releases are eligible under this profile.
    #[default(false)]
    pub enable_torrent: bool,
    /// Evaluation order; `LIST` returns profiles sorted by this ascending.
    /// Not deduplicated — several profiles may share an order.
    #[default(0)]
    pub order: i32,
    /// Row creation time (SQLite `CURRENT_TIMESTAMP`). Read-only.
    #[wire(read_only)]
    pub created_at: Option<String>,
}

/// Structural identity of one profile for the set diff:
/// `(order, usenetDelay, torrentDelay, preferredProtocol, enableUsenet,
/// enableTorrent)`. `id`/`createdAt` are excluded — they're server-assigned
/// and would make every desired entry look "new".
fn profile_identity(v: &Value) -> (i64, i64, i64, String, bool, bool) {
    (
        v.get("order").and_then(Value::as_i64).unwrap_or(0),
        v.get("usenetDelay").and_then(Value::as_i64).unwrap_or(0),
        v.get("torrentDelay").and_then(Value::as_i64).unwrap_or(0),
        v.get("preferredProtocol")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        v.get("enableUsenet")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        v.get("enableTorrent")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    )
}

impl CustomSync for DelayProfile {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/v1/delayprofile").await?;

            // Encode each desired config to its wire (camelCase) form.
            let mut wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            // `preferredProtocol` is an `Option<String>`, so `encode` omits
            // the key entirely when it's absent. Always send it explicitly
            // (as an empty string when unset) rather than relying on how
            // the server treats a missing key — see the field doc above.
            for profile in &mut wire {
                if let Some(obj) = profile.as_object_mut() {
                    obj.entry("preferredProtocol")
                        .or_insert_with(|| Value::String(String::new()));
                }
            }

            let to_post = wire.clone();
            let live_for_write = live.clone();
            let client = client.clone();
            reconcile::replace(
                &wire,
                &live,
                "delay profiles",
                execute,
                profile_identity,
                move || async move {
                    // Delete every live profile first, then recreate the
                    // whole desired set in order.
                    for item in &live_for_write {
                        if let Some(id) = item.get("id").and_then(Value::as_i64) {
                            client.delete(&format!("/api/v1/delayprofile/{id}")).await?;
                        }
                    }
                    for profile in &to_post {
                        let _: Value = client.post("/api/v1/delayprofile", profile).await?;
                    }
                    Ok(())
                },
            )
            .await
        })
    }
}
