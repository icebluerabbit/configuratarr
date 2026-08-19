//! `/api/libraries` — an Audiobookshelf library: a named collection of
//! filesystem folders scanned into library items, plus per-library
//! scan/matching/display settings ([`LibrarySettings`]).
//!
//! `sync = custom`, keyed by `name`. This can't be `sync = crud` for two
//! independent reasons, both verified against the ABS source
//! (`LibraryController`):
//!
//! 1. **`GET /api/libraries` returns an envelope**, not a bare array —
//!    `{"libraries": [...]}` — while the engine's crud path (and its
//!    automatic `${ref}` id-registration, [`core_lib::apply`]'s
//!    `register_refs`) both expect a plain `[...]` list response. A crud
//!    resource here would see zero live libraries on every apply, and
//!    `${ref.library.<name>}` would never resolve for a downstream
//!    resource. So this hook plucks `.libraries` out of the envelope itself,
//!    and — since the engine's own best-effort re-list can't see inside a
//!    non-array body either — registers every live library's id into the
//!    [`RefStore`] itself (`refs.insert("library", name, RefId::Str(id))`).
//!    (The engine's post-hook `register_pending_ids` still covers the
//!    preview/not-yet-created case automatically, because this resource
//!    declares a `#[key]` — see `core_lib::apply::custom_step`.)
//! 2. **`folders` has three incompatible shapes.** `GET`'s `Library.folders`
//!    is server-enriched (`{id, fullPath, libraryId, addedAt}`); `POST`'s
//!    `CreateLibraryRequest.folders` takes bare paths (`{fullPath}`, no
//!    `id`); `PATCH`'s `UpdateLibraryRequest.folders` is `{id}`-or-`{fullPath}`
//!    and is a **destructive replace-by-id**, not a merge (see the danger
//!    note on [`LibraryFolder`] and below). A single wire codec can't
//!    round-trip three different shapes for the same field, which is exactly
//!    what crud's live/desired merge would need.
//!
//! Create → `POST /api/libraries`, update → `PATCH /api/libraries/{live.id}`,
//! delete (under `--prune`) → `DELETE /api/libraries/{live.id}`
//! ([`core_lib::reconcile::upsert_prune`]).
//!
//! **Danger — the folders `PATCH` is a destructive replace-by-id.** Per the
//! ABS source (`LibraryController.update`): any folder entry in the request
//! *without* an `id` is created as new; any folder the library *currently*
//! has whose `id` does not appear in the request is deleted, and deleting a
//! folder cascades to every library item sourced from it (including
//! now-orphaned authors/series). This hook makes that safe for a
//! declarative apply by treating the request as authoritative: on every
//! update it matches each currently-live folder to a declared
//! [`LibraryFolder`] by `full_path` and echoes the live folder's `id` back
//! into the request for every match — so a folder stays only if its path is
//! still declared. **Consequence for the user:** removing a `library_folder`
//! entry from config is not a safe "stop tracking it" — the next apply
//! deletes that folder's entire scanned library content on the server.
//!
//! `in_sync` compares `displayOrder`, `icon`, `provider`, the `settings`
//! keys present in the user's config (via
//! [`core_lib::engine::config_present_to_wire`], which recurses into
//! `settings` too — see [`LibrarySettings`]'s module doc for why no field
//! there carries a literal default), and the folder set by `full_path`. It
//! ignores `id`/`createdAt`/`lastUpdate`/`lastScan`/`lastScanVersion` (never
//! modeled — server-only, never part of config) and `mediaType`
//! (technically PATCH-able, but changing it on an existing library doesn't
//! migrate `settings`/items, so it's effectively create-only — see
//! [`MediaType`]) and every server-side-only folder field (`id`, `libraryId`,
//! `addedAt`).

use std::collections::BTreeSet;
use std::sync::Mutex;

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefId, RefStore, engine, reconcile};
use core_macros::resource;
use serde_json::Value;

use crate::resources::library_folder::LibraryFolder;
use crate::resources::library_settings::LibrarySettings;
use crate::resources::media_type::MediaType;

/// `/api/libraries` — a library of either books or podcasts (never mixed).
#[resource(sync = custom, list = get("/api/libraries"))]
pub struct Library {
    /// Server-assigned id (a UUID). Read-only; declared so
    /// `engine::id_shape` knows a `${ref.library.*}` placeholder has to be a
    /// string, not the integer `-1`.
    #[id]
    pub id: Option<String>,
    /// Display name — its identity (`${ref.library.<name>}`). Required.
    #[key]
    pub name: String,
    /// Filesystem folders scanned into this library. Required, non-empty on
    /// create. See the module doc + [`LibraryFolder`] for the destructive
    /// replace-by-id danger on update.
    pub folders: Vec<LibraryFolder>,
    /// 1-based position among all libraries. Server-computed on create
    /// (always appended as `max+1`; the create request never accepts it, so
    /// this hook strips it from the `POST` body even if declared).
    /// Settable via this resource's `PATCH`, but note the live API doesn't
    /// renormalize other libraries' `display_order` when this one changes.
    pub display_order: Option<i32>,
    /// Icon key shown in the UI (e.g. `database`, `audiobookshelf`,
    /// `podcast`); not validated against a fixed enum server-side. Server
    /// default on create: `"database"`.
    pub icon: Option<String>,
    /// Selects which `LibrarySettings` default template seeds this library
    /// and which content model its items use. Server default on create:
    /// `book`. Effectively create-only — see the module doc.
    pub media_type: Option<MediaType>,
    /// Preferred metadata provider slug for matching (e.g. `google`,
    /// `audible`, `itunes`, or `custom-<slug>` for a configured custom
    /// provider, which must already exist). Server default on create:
    /// `"google"`.
    pub provider: Option<String>,
    /// Per-library scan/matching/display settings.
    pub settings: Option<LibrarySettings>,
}

/// The declared `full_path` set of a wire-shaped `folders` array — either the
/// bare desired shape (`{fullPath}`) or the server-enriched live shape
/// (`{id, fullPath, libraryId, addedAt}`); only `fullPath` is compared.
fn folder_paths(folders: Option<&Value>) -> BTreeSet<&str> {
    folders
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|f| f.get("fullPath").and_then(Value::as_str))
        .collect()
}

/// Idempotency predicate for [`reconcile::upsert_prune`]: `desired` is the
/// present-masked wire object (only keys the user's config actually wrote —
/// see [`core_lib::engine::config_present_to_wire`]), `live` is the raw `GET`
/// entry for the matched library. `mediaType` is deliberately not compared
/// (see the module doc); `folders` compares the `full_path` set, not the
/// enriched live objects or declaration order.
fn in_sync(desired: &Value, live: &Value) -> bool {
    let (Some(want), Some(have)) = (desired.as_object(), live.as_object()) else {
        return desired == live;
    };
    for (k, wv) in want {
        match k.as_str() {
            "mediaType" => continue,
            "folders" => {
                if folder_paths(want.get("folders")) != folder_paths(have.get("folders")) {
                    return false;
                }
            }
            "settings" => {
                let Some(settings_want) = wv.as_object() else {
                    continue;
                };
                let settings_have = have.get("settings").and_then(Value::as_object);
                for (sk, swv) in settings_want {
                    let matches = match settings_have.and_then(|h| h.get(sk)) {
                        Some(shv) => shv == swv,
                        None => swv.is_null(),
                    };
                    if !matches {
                        return false;
                    }
                }
            }
            _ => {
                let matches = match have.get(k) {
                    Some(hv) => hv == wv,
                    None => wv.is_null(),
                };
                if !matches {
                    return false;
                }
            }
        }
    }
    true
}

/// Build the `PATCH` folders array: for each declared folder, echo the
/// matching live folder's `id` if one exists at the same `full_path` (keep),
/// else send the bare `{fullPath}` (create-new). A live folder whose path
/// isn't declared here contributes no entry at all — which is precisely what
/// deletes it server-side; see the module doc's danger note.
fn folders_for_update(desired_folders: &Value, live_folders: &Value) -> Value {
    let live: &[Value] = live_folders.as_array().map(Vec::as_slice).unwrap_or(&[]);
    let out: Vec<Value> = desired_folders
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[])
        .iter()
        .map(|f| {
            let path = f.get("fullPath").and_then(Value::as_str);
            let live_id = path.and_then(|p| {
                live.iter()
                    .find(|l| l.get("fullPath").and_then(Value::as_str) == Some(p))
                    .and_then(|l| l.get("id"))
            });
            match live_id {
                Some(id) => serde_json::json!({ "id": id, "fullPath": path }),
                None => f.clone(),
            }
        })
        .collect();
    Value::Array(out)
}

/// Register `(name -> id)` into the [`RefStore`] under `"library"` for every
/// entry that has both — the manual counterpart of the engine's own
/// `register_refs`, which can't see inside `GET /api/libraries`'s `{libraries:
/// [...]}` envelope (see the module doc).
fn register_library_ids<'a>(refs: &mut RefStore, entries: impl IntoIterator<Item = &'a Value>) {
    for e in entries {
        if let (Some(name), Some(id)) = (
            e.get("name").and_then(Value::as_str),
            e.get("id").and_then(RefId::from_value),
        ) {
            refs.insert("library", name, id);
        }
    }
}

impl CustomSync for Library {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let envelope: Value = client.get("/api/libraries").await?;
            let live: Vec<Value> = envelope
                .get("libraries")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            register_library_ids(refs, &live);

            // Present-masked wire: only the keys the user's config actually
            // wrote, at the top level and (recursively) within `settings` —
            // exactly what `in_sync` and the create/update bodies need. See
            // the module doc for why `settings` must stay partial.
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::config_present_to_wire::<Self>)
                .collect::<anyhow::Result<_>>()?;

            let created: Mutex<Vec<Value>> = Mutex::new(Vec::new());

            let changes = reconcile::upsert_prune(
                &wire,
                &live,
                "name",
                in_sync,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    let created = &created;
                    async move {
                        // `displayOrder` is server-computed on create and not
                        // part of `CreateLibraryRequest` — strip it even if
                        // declared; a later apply's `update` still applies it.
                        let mut body = w;
                        if let Value::Object(obj) = &mut body {
                            obj.remove("displayOrder");
                        }
                        let resp: Value = client.post("/api/libraries", &body).await?;
                        created
                            .lock()
                            .expect("created-libraries mutex poisoned")
                            .push(resp);
                        Ok(())
                    }
                },
                |l, w| {
                    let client = client.clone();
                    let id = l
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    // Build the id-echoed folders array synchronously, from
                    // owned clones — `l` only borrows for this closure body,
                    // not the future it returns (see `folders_for_update`).
                    let mut body = w;
                    if let Value::Object(obj) = &mut body {
                        let desired_folders = obj.get("folders").cloned().unwrap_or(Value::Null);
                        let live_folders = l.get("folders").cloned().unwrap_or(Value::Null);
                        obj.insert(
                            "folders".to_string(),
                            folders_for_update(&desired_folders, &live_folders),
                        );
                    }
                    async move {
                        let _: Value = client.patch(&format!("/api/libraries/{id}"), &body).await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    async move {
                        client.delete(&format!("/api/libraries/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await?;

            let created = created
                .into_inner()
                .expect("created-libraries mutex poisoned");
            register_library_ids(refs, &created);

            Ok(changes)
        })
    }
}
