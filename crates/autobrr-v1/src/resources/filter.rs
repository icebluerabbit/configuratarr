//! `/api/filters` — the flagship resource: release-matching filters.
//!
//! autobrr creates a filter in **two steps**: `POST /api/filters` accepts only
//! the small `FilterCreate` subset (name + a few list fields), returns the new
//! id, and the full ~40-field config is then applied via
//! `PATCH /api/filters/{id}`. A single `crud` POST would ship a half-configured
//! filter, so this is a `sync = custom` hook that owns the two-step create, the
//! update (PATCH), and the change detection. Under `--prune`, filters the config
//! no longer declares are deleted via `DELETE /api/filters/{id}`
//! ([`core_lib::reconcile::prune_absent`] bolted onto the bespoke loop).
//!
//! **Idempotency.** Live filters carry server-added fields — an `id`, and ids on
//! nested `actions` — that the desired config never sets. So "changed?" is a
//! **structural subset** test ([`crate::diff::subset`]): the filter is in sync
//! when every declared key/value already matches live (extra server keys are
//! ignored), rather than a merge-equality that server ids would always trip.

use std::collections::HashSet;

use core_lib::engine;
use core_lib::reconcile;
use core_lib::{Change, ChangeKind, CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::{Map, Value, json};

use crate::diff;
use crate::resources::action::Action;
use crate::resources::external_filter::ExternalFilter;
use crate::resources::filter_indexer::FilterIndexer;

/// Array keys the `POST /api/filters` (`FilterCreate`) body **must** carry —
/// autobrr stores them `NOT NULL`, so they're forced to `[]` when the config
/// omits them. (`name`/`enabled` are copied straight from the wire.)
const CREATE_ARRAY_KEYS: &[&str] = &["resolutions", "codecs", "sources", "containers", "origins"];

/// `/api/filters` — a release-matching filter.
#[resource(sync = custom, case = snake, list = get("/api/filters"))]
pub struct Filter {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Filter name — its identity.
    #[key]
    pub name: String,
    /// Whether the filter is active.
    pub enabled: Option<bool>,
    /// Evaluation priority (higher wins).
    pub priority: Option<i32>,
    /// Treat match/except patterns as regular expressions.
    pub use_regex: Option<bool>,
    /// Match release years (e.g. `2020-2024`).
    pub years: Option<String>,
    /// Match seasons (e.g. `1,3-5`).
    pub seasons: Option<String>,
    /// Match episodes.
    pub episodes: Option<String>,
    /// Releases must match these terms.
    pub match_releases: Option<String>,
    /// Releases must not match these terms.
    pub except_releases: Option<String>,
    /// Match these release groups.
    pub match_release_groups: Option<String>,
    /// Exclude these release groups.
    pub except_release_groups: Option<String>,
    /// Match these categories.
    pub match_categories: Option<String>,
    /// Exclude these categories.
    pub except_categories: Option<String>,
    /// Match these tags.
    pub tags: Option<String>,
    /// Exclude these tags.
    pub except_tags: Option<String>,
    /// Logic for `tags` (`ANY` / `ALL`).
    pub tags_match_logic: Option<String>,
    /// Logic for `except_tags` (`ANY` / `ALL`).
    pub except_tags_match_logic: Option<String>,
    /// Smarter episode/season handling.
    pub smart_episode: Option<bool>,
    /// Match these resolutions.
    pub resolutions: Vec<String>,
    /// Match these video codecs.
    pub codecs: Vec<String>,
    /// Match these sources.
    pub sources: Vec<String>,
    /// Match these containers.
    pub containers: Vec<String>,
    /// Match these HDR formats.
    pub match_hdr: Vec<String>,
    /// Exclude these HDR formats.
    pub except_hdr: Vec<String>,
    /// Match these "other" tags (REPACK, PROPER, …).
    pub match_other: Vec<String>,
    /// Exclude these "other" tags.
    pub except_other: Vec<String>,
    /// Match these languages.
    pub match_language: Vec<String>,
    /// Exclude these languages.
    pub except_language: Vec<String>,
    /// Match these audio formats.
    pub formats: Vec<String>,
    /// Match these audio qualities.
    pub quality: Vec<String>,
    /// Match these media types.
    pub media: Vec<String>,
    /// Match these release types.
    pub match_release_types: Vec<String>,
    /// Match these origins (INTERNAL, SCENE, …).
    pub origins: Vec<String>,
    /// Exclude these origins.
    pub except_origins: Vec<String>,
    /// Minimum release size (e.g. `1GB`).
    pub min_size: Option<String>,
    /// Maximum release size.
    pub max_size: Option<String>,
    /// Delay before pushing, seconds.
    pub delay: Option<i32>,
    /// Cap the number of grabs.
    pub max_downloads: Option<i32>,
    /// Window the `max_downloads` cap applies over: `HOUR`, `DAY`, `WEEK`,
    /// `MONTH`, or `EVER`.
    pub max_downloads_unit: Option<String>,
    /// Match these announce types (`NEW`, `PROMO`, `PROMO_GP`, `RESURRECTED`).
    pub announce_types: Vec<String>,
    /// Match only scene releases.
    pub scene: Option<bool>,
    /// Match these bonus/reward tags (tracker-specific).
    pub bonus: Vec<String>,
    /// Match only freeleech releases.
    pub freeleech: Option<bool>,
    /// Match these freeleech percentages (e.g. `50,100`).
    pub freeleech_percent: Option<String>,
    /// Match these show/title terms.
    pub shows: Option<String>,
    /// Match months (e.g. `1,6-8`).
    pub months: Option<String>,
    /// Match days.
    pub days: Option<String>,
    /// Match these artists (music).
    pub artists: Option<String>,
    /// Match these albums (music).
    pub albums: Option<String>,
    /// Exclude these release types.
    pub except_release_types: Option<String>,
    /// Match only perfect FLAC (music).
    pub perfect_flac: Option<bool>,
    /// Require a CUE file (music).
    pub cue: Option<bool>,
    /// Require a log (music).
    pub log: Option<bool>,
    /// Minimum log score (music).
    pub log_score: Option<i32>,
    /// Match these uploaders.
    pub match_uploaders: Option<String>,
    /// Exclude these uploaders.
    pub except_uploaders: Option<String>,
    /// Match these record labels (music).
    pub match_record_labels: Option<String>,
    /// Exclude these record labels (music).
    pub except_record_labels: Option<String>,
    /// Match if the release carries any of these tags.
    pub tags_any: Option<String>,
    /// Exclude if the release carries any of these tags.
    pub except_tags_any: Option<String>,
    /// Match these release tags.
    pub match_release_tags: Option<String>,
    /// Exclude these release tags.
    pub except_release_tags: Option<String>,
    /// Treat release-tag patterns as regular expressions.
    pub use_regex_release_tags: Option<bool>,
    /// Match these terms in the release description.
    pub match_description: Option<String>,
    /// Exclude these terms in the release description.
    pub except_description: Option<String>,
    /// Treat description patterns as regular expressions.
    pub use_regex_description: Option<bool>,
    /// Minimum seeders.
    pub min_seeders: Option<i32>,
    /// Maximum seeders.
    pub max_seeders: Option<i32>,
    /// Minimum leechers.
    pub min_leechers: Option<i32>,
    /// Maximum leechers.
    pub max_leechers: Option<i32>,
    /// Dedup profile to apply (`${ref.release_profile_duplicate.<name>}`).
    #[reference(release_profile_duplicate)]
    pub release_profile_duplicate_id: Option<i32>,
    /// Indexers this filter is attached to.
    pub indexers: Vec<FilterIndexer>,
    /// Actions run on a matched release.
    pub actions: Vec<Action>,
    /// External (webhook/exec) checks.
    pub external: Vec<ExternalFilter>,
}

/// The `FilterCreate` body: `name` + `enabled` from the wire, plus the five
/// required array fields forced to `[]` when absent (autobrr rejects a NULL
/// there). The rest of the config is applied by the follow-up PATCH.
fn create_body(wire: &Value) -> Value {
    let obj = wire.as_object();
    let mut out = Map::new();
    for k in ["name", "enabled"] {
        if let Some(v) = obj.and_then(|o| o.get(k)) {
            out.insert(k.to_string(), v.clone());
        }
    }
    for k in CREATE_ARRAY_KEYS {
        let v = obj
            .and_then(|o| o.get(*k))
            .cloned()
            .unwrap_or_else(|| json!([]));
        out.insert((*k).to_string(), v);
    }
    Value::Object(out)
}

/// Actions are keyed by `name` for identity. autobrr's action store is
/// *upsert-by-id with no delete* (unlike the filter's indexers/external, which
/// it replaces wholesale), so a managed filter's actions must carry a stable,
/// unique key — otherwise every apply re-inserts them. Reject blank/duplicate
/// action names up front. See [[keyless-collection-constraint]].
fn validate_action_names(filter: &str, wire: &Value) -> anyhow::Result<()> {
    let mut seen = HashSet::new();
    if let Some(actions) = wire.get("actions").and_then(Value::as_array) {
        for a in actions {
            let n = a.get("name").and_then(Value::as_str).unwrap_or("");
            if n.is_empty() {
                anyhow::bail!(
                    "filter '{filter}': every action needs a non-empty `name` \
                     (it is the action's identity for sync)"
                );
            }
            if !seen.insert(n) {
                anyhow::bail!(
                    "filter '{filter}': duplicate action name '{n}' \
                     (action names must be unique within a filter)"
                );
            }
        }
    }
    Ok(())
}

/// Reconcile the filter's `actions` against what autobrr has stored. Matches
/// each desired action to a live one by `name`, claiming one live id per
/// desired action, and **injects that id into `wire`** so autobrr updates in
/// place instead of inserting a duplicate. Returns `(actions_in_sync, prune)`:
/// `prune` is the ids of every live action left unclaimed — undeclared actions
/// *and* any accumulated same-name duplicates — which the caller deletes via
/// `DELETE /api/actions/{id}`.
fn plan_actions(wire: &mut Value, full: &Value) -> (bool, Vec<i64>) {
    let live: Vec<Value> = full
        .get("actions")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut claimed: HashSet<i64> = HashSet::new();
    let mut synced = true;

    if let Some(desired) = wire.get_mut("actions").and_then(Value::as_array_mut) {
        for d in desired.iter_mut() {
            let name = d.get("name").and_then(Value::as_str).unwrap_or_default();
            // First live action of this name whose id isn't already claimed.
            let matched = live.iter().find(|l| {
                l.get("name").and_then(Value::as_str) == Some(name)
                    && l.get("id")
                        .and_then(Value::as_i64)
                        .is_some_and(|id| !claimed.contains(&id))
            });
            match matched {
                Some(l) => {
                    let id = l.get("id").and_then(Value::as_i64).unwrap_or_default();
                    claimed.insert(id);
                    // Compare before injecting the id (the live side carries it).
                    if !diff::subset(d, l) {
                        synced = false;
                    }
                    if let Some(obj) = d.as_object_mut() {
                        obj.insert("id".to_string(), json!(id));
                    }
                }
                // No live match → a new action; autobrr will insert it.
                None => synced = false,
            }
        }
    }

    let prune: Vec<i64> = live
        .iter()
        .filter_map(|l| l.get("id").and_then(Value::as_i64))
        .filter(|id| !claimed.contains(id))
        .collect();
    if !prune.is_empty() {
        synced = false;
    }

    (synced, prune)
}

impl CustomSync for Filter {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/filters").await?;
            let mut changes = Vec::with_capacity(desired.len());

            for cfg in desired {
                let name = cfg
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow::anyhow!("filter entry is missing `name`"))?;
                // Full desired wire (id is read-only, so absent here).
                let mut wire = engine::encode_config::<Self>(cfg)?;
                validate_action_names(name, &wire)?;

                // The list endpoint returns a *trimmed* filter (no match/except
                // fields), so match by name here but fetch the full object by id
                // for the diff.
                let existing_id = live
                    .iter()
                    .find(|f| f.get("name").and_then(Value::as_str) == Some(name))
                    .and_then(|f| f.get("id").cloned());

                let kind = match existing_id {
                    Some(id) => {
                        let full: Value = client.get(&format!("/api/filters/{id}")).await?;

                        // Reconcile actions by identity: inject matched live ids
                        // (so autobrr updates in place, never duplicates) and
                        // collect undeclared/duplicate actions to prune.
                        let (actions_synced, prune) = plan_actions(&mut wire, &full);

                        // Diff the rest of the filter separately — `subset`
                        // compares arrays positionally, which is wrong for
                        // actions (matched by name above).
                        let mut scalars = wire.clone();
                        if let Some(obj) = scalars.as_object_mut() {
                            obj.remove("actions");
                        }

                        if diff::subset(&scalars, &full) && actions_synced {
                            // Already in sync — nothing to do.
                            ChangeKind::Unchanged
                        } else {
                            // Drifted — PATCH the full config (actions now carry
                            // ids), then prune the orphaned/duplicate actions.
                            if execute {
                                let _: Value =
                                    client.patch(&format!("/api/filters/{id}"), &wire).await?;
                                for action_id in &prune {
                                    client.delete(&format!("/api/actions/{action_id}")).await?;
                                }
                            }
                            ChangeKind::Updated
                        }
                    }
                    // Absent — two-step create: POST the subset, then PATCH the rest.
                    None => {
                        if execute {
                            let created: Value =
                                client.post("/api/filters", &create_body(&wire)).await?;
                            let id = created.get("id").cloned().unwrap_or(Value::Null);
                            let _: Value =
                                client.patch(&format!("/api/filters/{id}"), &wire).await?;
                        }
                        ChangeKind::Created
                    }
                };

                changes.push(match kind {
                    ChangeKind::Created => Change::created(name),
                    ChangeKind::Updated => Change::updated(name),
                    ChangeKind::Unchanged => Change::unchanged(name),
                    // The loop only ever creates/updates/leaves a declared filter.
                    ChangeKind::Removed => unreachable!("filter loop never prunes"),
                });
            }

            // Prune: delete live filters the config no longer declares. Deleting a
            // filter cascades to its actions server-side.
            changes.extend(
                reconcile::prune_absent(desired, &live, "name", prune, execute, |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        client.delete(&format!("/api/filters/{id}")).await?;
                        Ok(())
                    }
                })
                .await?,
            );

            Ok(changes)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(name: &str) -> Value {
        json!({ "name": name, "type": "WEBHOOK", "enabled": true })
    }
    fn live_action(id: i64, name: &str) -> Value {
        json!({ "id": id, "name": name, "type": "WEBHOOK", "enabled": true })
    }

    #[test]
    fn validate_rejects_blank_and_duplicate_names() {
        assert!(validate_action_names("f", &json!({ "actions": [action("a")] })).is_ok());
        assert!(
            validate_action_names("f", &json!({ "actions": [{ "type": "WEBHOOK" }] })).is_err(),
            "blank/absent name must error"
        );
        assert!(
            validate_action_names("f", &json!({ "actions": [action("a"), action("a")] })).is_err(),
            "duplicate name must error"
        );
    }

    #[test]
    fn plan_injects_matched_id_and_reports_in_sync() {
        let mut wire = json!({ "actions": [action("keep")] });
        let full = json!({ "actions": [live_action(7, "keep")] });
        let (synced, prune) = plan_actions(&mut wire, &full);
        assert!(synced, "single matched action, no orphans");
        assert!(prune.is_empty());
        assert_eq!(
            wire["actions"][0]["id"],
            json!(7),
            "live id injected for update-in-place"
        );
    }

    #[test]
    fn plan_prunes_undeclared_action() {
        let mut wire = json!({ "actions": [action("keep")] });
        let full = json!({ "actions": [live_action(7, "keep"), live_action(9, "orphan")] });
        let (synced, prune) = plan_actions(&mut wire, &full);
        assert!(!synced, "an undeclared live action means out of sync");
        assert_eq!(prune, vec![9], "the orphan is pruned, the match kept");
        assert_eq!(wire["actions"][0]["id"], json!(7));
    }

    #[test]
    fn plan_dedups_same_name_duplicates() {
        // The real-world failure: N copies of the same action accumulated.
        let mut wire = json!({ "actions": [action("sonarr")] });
        let full = json!({ "actions": [
            live_action(1, "sonarr"),
            live_action(3, "sonarr"),
            live_action(5, "sonarr"),
        ] });
        let (synced, mut prune) = plan_actions(&mut wire, &full);
        prune.sort_unstable();
        assert!(!synced);
        assert_eq!(prune, vec![3, 5], "keep one, prune the duplicate copies");
        assert_eq!(wire["actions"][0]["id"], json!(1), "claims the first copy");
    }

    #[test]
    fn plan_new_action_is_not_synced_and_inserts() {
        let mut wire = json!({ "actions": [action("fresh")] });
        let full = json!({ "actions": [] });
        let (synced, prune) = plan_actions(&mut wire, &full);
        assert!(!synced, "a brand-new action needs an insert");
        assert!(prune.is_empty());
        assert!(
            wire["actions"][0].get("id").is_none(),
            "no id → autobrr inserts"
        );
    }

    #[test]
    fn plan_empty_desired_prunes_all_live() {
        let mut wire = json!({ "actions": [] });
        let full = json!({ "actions": [live_action(1, "a"), live_action(2, "b")] });
        let (synced, mut prune) = plan_actions(&mut wire, &full);
        prune.sort_unstable();
        assert!(!synced);
        assert_eq!(prune, vec![1, 2], "declaring no actions prunes them all");
    }
}
