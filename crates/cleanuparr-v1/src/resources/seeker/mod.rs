//! `/api/configuration/seeker` — Cleanuparr's proactive search engine: the
//! job that periodically asks each configured *arr instance to search for
//! monitored-but-missing or below-cutoff items.
//!
//! Modelled as a `sync = custom` **singleton**, not `sync = singleton`.
//! `merge(live, desired)` (see `core_lib::merge`) treats a non-`fields` array
//! as *desired-authoritative* — so the sparse `instances[]` we send would
//! replace the live, enriched `instances[]` on every run. The live entries
//! carry extra server-owned keys our write contract has no field for
//! (`instanceName`, `instanceType`, `lastProcessedAt`,
//! `arrInstanceEnabled`), so `merged != live` forever and a plain singleton
//! PUT would report a perpetual phantom update. A custom hook sidesteps that:
//! it diffs the *declared* wire against live with [`in_sync`], which ignores
//! those extra live keys instead of demanding a byte-identical object, and
//! compares `instances` by key rather than by position.
//!
//! Apply order is computed from static `#[reference(...)]` metadata, never from
//! the `${ref}` text in a config, so
//! [`instance::SeekerInstance::arr_instance_id`] declares **both** types it can
//! address (`#[reference(sonarr_instance, radarr_instance)]`). That is what puts
//! the seeker after both instance collections in the same run;
//! `tests/apply_order.rs` asserts it.

pub mod instance;

use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::{resource, wire_enum};
use serde_json::Value;

use crate::diff;
use instance::SeekerInstance;

const SEEKER_PATH: &str = "/api/configuration/seeker";

/// Wire key of the seeker's keyed sub-collection, and the key each entry is
/// upserted by server-side.
const INSTANCES_KEY: &str = "instances";
const INSTANCE_KEY_FIELD: &str = "arrInstanceId";

/// Is the declared seeker config already satisfied by `live`?
///
/// Everything except `instances` is a flat scalar and compares with the ordinary
/// [`crate::diff::subset`]. `instances` needs
/// [`crate::diff::subset_keyed`] instead: the API returns one entry per Sonarr /
/// Radarr instance whether or not the config mentions it, and entries the config
/// omits keep their stored settings — so a positional, length-equal comparison
/// would report drift forever as soon as a single unmanaged instance exists.
fn in_sync(wire: &Value, live: &Value) -> bool {
    let scalars_match = wire.as_object().is_none_or(|w| {
        w.iter().all(|(k, v)| {
            k == INSTANCES_KEY || { diff::subset(v, live.get(k).unwrap_or(&Value::Null)) }
        })
    });

    let instances_match = match wire.get(INSTANCES_KEY).and_then(Value::as_array) {
        None => true,
        Some(want) => {
            let have = live
                .get(INSTANCES_KEY)
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            diff::subset_keyed(want, have, INSTANCE_KEY_FIELD)
        }
    };

    scalars_match && instances_match
}

/// Strategy used to pick which eligible item to search next when multiple
/// items are due.
#[wire_enum]
pub enum SelectionStrategy {
    /// Balances age and per-item search frequency.
    BalancedWeighted,
    /// Always searches the item whose last search is oldest.
    OldestSearchFirst,
    /// Weighted towards items whose last search is oldest.
    OldestSearchWeighted,
    /// Always searches the most recently added eligible item.
    NewestFirst,
    /// Weighted towards the most recently added eligible items.
    NewestWeighted,
    /// Picks a random eligible item.
    Random,
}

/// `/api/configuration/seeker` — proactive search behaviour, global and
/// per-*arr-instance.
#[resource(sync = custom, list = get("/api/configuration/seeker"))]
pub struct Seeker {
    /// Enables the seeker job entirely (both scheduled and proactive
    /// search).
    #[default(true)]
    pub search_enabled: bool,
    /// Minutes between seeker runs. Must be one of 2, 3, 4, 5, 6, 10, 12,
    /// 15, 20, 30, 60, 120, 180, 240, 360.
    #[default(3)]
    pub search_interval: i32,
    /// Enables proactively searching for missing/below-cutoff items, rather
    /// than only reacting to *arr events.
    #[default(false)]
    pub proactive_search_enabled: bool,
    /// How the next item to proactively search is chosen among eligible
    /// candidates.
    pub selection_strategy: Option<SelectionStrategy>,
    /// Round-robins proactive search across instances instead of draining
    /// one instance's queue before moving to the next.
    #[default(true)]
    pub use_round_robin: bool,
    /// Hours after an item's release before proactive search will consider
    /// it, giving indexers time to pick it up.
    #[default(6)]
    pub post_release_grace_hours: i32,
    /// Per-*arr-instance proactive-search settings. Upserted by
    /// `arr_instance_id`; instances omitted here keep their stored
    /// settings.
    pub instances: Vec<SeekerInstance>,
}

impl CustomSync for Seeker {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(Vec::new());
            };

            let live: Value = client.get(SEEKER_PATH).await?;
            let wire = engine::encode_config::<Self>(cfg)?;

            if in_sync(&wire, &live) {
                return Ok(vec![Change::unchanged("seeker")]);
            }

            // A preview must perform no writes — only PUT when actually applying.
            if execute {
                let _: Value = client.put(SEEKER_PATH, &wire).await?;
            }
            Ok(vec![Change::updated("seeker")])
        })
    }
}
