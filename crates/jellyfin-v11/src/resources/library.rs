//! `/Library/VirtualFolders` — media libraries.
//!
//! A library is keyed by name and created through **query parameters** (plus an
//! `AddVirtualFolderDto` body), with no per-library id — so it's a `sync =
//! custom` resource. The reconcile hook creates any declared library that isn't
//! present yet (create-or-leave; no prune, no path reconcile in this MVP).

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::{Value, json};

/// `/Library/VirtualFolders` — a media library (name + collection type + paths).
#[resource(sync = custom, list = get("/Library/VirtualFolders"))]
pub struct Library {
    /// Library display name — its identity.
    #[key]
    pub name: String,
    /// Collection type: `movies`, `tvshows`, `music`, `books`, … Omit for mixed.
    pub collection_type: Option<String>,
    /// Filesystem paths that make up the library.
    pub paths: Vec<String>,
}

impl CustomSync for Library {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/Library/VirtualFolders").await?;
            let present = reconcile::present_keys(&live, "Name");
            let client = client.clone();
            reconcile::create_only(desired, "name", &present, execute, move |name, cfg| {
                let client = client.clone();
                async move {
                    // Identity + paths ride the query string (core-http encodes
                    // them); `refreshLibrary=false` avoids a scan during apply.
                    let mut query: Vec<(&str, &str)> =
                        vec![("refreshLibrary", "false"), ("name", name.as_str())];
                    if let Some(ct) = cfg.get("collection_type").and_then(Value::as_str) {
                        query.push(("collectionType", ct));
                    }
                    for p in cfg
                        .get("paths")
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                    {
                        query.push(("paths", p));
                    }
                    let _: Value = client
                        .post_query(
                            "/Library/VirtualFolders",
                            &query,
                            &json!({ "LibraryOptions": {} }),
                        )
                        .await?;
                    Ok(())
                }
            })
            .await
        })
    }
}
