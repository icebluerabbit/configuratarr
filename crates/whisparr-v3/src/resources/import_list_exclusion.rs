//! `/api/v3/exclusions` — one scene/movie/studio/performer/tag excluded from
//! every import list.
//!
//! `sync = custom` for one reason: **eros has no plain list endpoint.**
//! `GET /api/v3/exclusions` looks like one but its action is
//! `GetImportListExclusions(string stashId, ImportExclusionType type = Scene)`
//! (`ImportLists/ImportListExclusionController.cs:50`) — it filters by type and
//! defaults to `scene`. Verified against a live instance: with one `scene` and
//! one `performer` exclusion present, the bare GET returned only the scene and
//! `?type=performer` only the performer. A `sync = crud` list step would
//! therefore never see 4 of the 5 types, re-`POST` them on every apply (the API
//! rejects the duplicate with `ImportListExclusionExistsValidator`) and could
//! never prune them.
//!
//! `GET /api/v3/exclusions/paged` *is* the full list, but returns a paged
//! envelope — `{page, pageSize, sortKey, sortDirection, totalRecords,
//! records:[…]}` — so the hook reaches into `.records` itself. One page is
//! fetched with a large `pageSize`; pagination is an open engine seam and no
//! realistic install approaches that many exclusions. `total_records` is checked
//! against what came back so a silently truncated read fails loudly instead of
//! pruning everything past the page boundary.
//!
//! Identity is the `foreignId` string, so [`reconcile::upsert_prune`] keys on it
//! directly — no synthetic key needed, and no cross-type ambiguity:
//! `ImportListExclusionExistsValidator.cs:29` keys on `ForeignId` alone, ignoring
//! `Type`, so eros itself enforces global uniqueness over that one field.
//!
//! The wire schema inherits the provider envelope (`name`, `fields`,
//! `implementation`, `configContract`, `presets`, …) because eros's API layer
//! derives `ImportListExclusionResource` from the same base class as its real
//! providers — which is also why `list_resources` mis-tags this a `[provider]`.
//! The underlying C# model is a plain `ModelBase`, not a `ThingiProvider`, so
//! those fields carry no configuration and are deliberately not modelled;
//! `additionalProperties: false` constrains what we *send*, so a subset payload
//! validates. `crates/radarr-v3/src/resources/import_list_exclusion.rs` makes the
//! same choice.

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, engine, reconcile};
use core_macros::resource;
use serde_json::Value;

/// The list URL the hook actually fetches. One page, sized past any realistic
/// install; the hook hard-fails if `totalRecords` exceeds what came back.
///
/// Must stay identical to the `list = get(...)` literal below — an attribute
/// takes a literal, not a const, so the string is unavoidably written twice. The
/// `list` slot is descriptor/doc-gen metadata only; this const is what runs.
const LIST_PATH: &str = "/api/v3/exclusions/paged?page=1&pageSize=1000";

/// Server-owned fields that must not take part in the idempotency comparison.
const IGNORED_KEYS: &[&str] = &["id"];

#[resource(
    sync = custom,
    list = get("/api/v3/exclusions/paged?page=1&pageSize=1000"),
    create = post("/api/v3/exclusions"),
    update = put("/api/v3/exclusions/${self.id}"),
    delete = delete("/api/v3/exclusions/${self.id}"),
)]
pub struct ImportListExclusion {
    #[id]
    pub id: Option<i32>,
    /// Natural key — the foreign (StashDB/ThePornDB-style) id of the excluded
    /// item.
    #[key]
    pub foreign_id: String,
    /// Kind of item excluded: `scene`, `movie`, `studio`, `performer`, or `tag`.
    #[wire(name = "type")]
    pub exclusion_type: Option<String>,
    /// Title of the excluded item. **Required** — eros validates
    /// `RuleFor(c => c.MovieTitle).NotEmpty()`
    /// (`ImportListExclusionController.cs:43`) on POST *and* PUT
    /// (`RestController.cs:72`), so omitting it 400s on every write.
    pub movie_title: String,
    /// Release year of the excluded item. **Required and must be > 0** — eros
    /// validates `RuleFor(c => c.MovieYear).GreaterThan(0)`
    /// (`ImportListExclusionController.cs:44`) on POST *and* PUT.
    pub movie_year: i32,
}

/// `v` without the server-owned keys, for [`in_sync`]'s comparison.
fn stripped_for_compare(v: &Value) -> Value {
    let mut obj = v.as_object().cloned().unwrap_or_default();
    for k in IGNORED_KEYS {
        obj.remove(*k);
    }
    Value::Object(obj)
}

/// The live record carries the whole provider envelope we deliberately don't
/// model, so plain equality would never hold. Every field *we* declare must
/// match the live value — a subset test, the shape [`reconcile::upsert`] is
/// documented for.
fn in_sync(desired: &Value, live: &Value) -> bool {
    let d = stripped_for_compare(desired);
    let Some(d) = d.as_object() else { return false };
    d.iter().all(|(k, v)| live.get(k) == Some(v))
}

impl CustomSync for ImportListExclusion {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let envelope: Value = client.get(LIST_PATH).await?;
            let live: Vec<Value> = envelope
                .get("records")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            // A truncated page would make every unseen exclusion look absent —
            // re-created on apply, and deleted under `--prune`. Fail instead.
            if let Some(total) = envelope.get("totalRecords").and_then(Value::as_u64)
                && total as usize > live.len()
            {
                anyhow::bail!(
                    "import_list_exclusion: server reports {total} exclusions but one page \
                     returned {}; the list is paginated beyond `{LIST_PATH}` and this hook \
                     does not page",
                    live.len()
                );
            }

            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert_prune(
                &wire,
                &live,
                "foreignId",
                in_sync,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/v3/exclusions", &w).await?;
                        Ok(())
                    }
                },
                |l, w| {
                    let client = client.clone();
                    let id = l.get("id").and_then(Value::as_i64).unwrap_or_default();
                    // The API matches on the body's id as well as the path.
                    let mut body = w;
                    reconcile::echo(&mut body, "id", l);
                    async move {
                        let _: Value = client
                            .put(&format!("/api/v3/exclusions/{id}"), &body)
                            .await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").and_then(Value::as_i64).unwrap_or_default();
                    async move {
                        client.delete(&format!("/api/v3/exclusions/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn in_sync_ignores_the_unmodelled_provider_envelope() {
        let desired = json!({ "foreignId": "sd-1", "type": "performer", "movieYear": 2020 });
        let live = json!({
            "id": 7, "foreignId": "sd-1", "type": "performer", "movieYear": 2020,
            // envelope fields eros returns but we never declare
            "name": null, "fields": [], "implementation": "", "configContract": "",
            "presets": [], "tags": []
        });
        assert!(in_sync(&desired, &live));
    }

    #[test]
    fn in_sync_catches_a_real_diff() {
        let desired = json!({ "foreignId": "sd-1", "type": "performer" });
        let live = json!({ "id": 7, "foreignId": "sd-1", "type": "studio" });
        assert!(!in_sync(&desired, &live));
    }
}
