//! Authors — keyed by author name.
//!
//! `getIndex` lists authors (each with `AuthorName` + a server-assigned
//! `AuthorID`); `addAuthor&name=<name>` adds one by name. LazyLibrarian exposes no
//! author update and no `Change`-expressible delete, so this is a `sync = custom`
//! **create-or-leave** resource ([`core_lib::reconcile::create_only`]), matched on
//! `AuthorName` — the "don't remove what you didn't make" caution.
//!
//! Adding by explicit `AuthorID` (`addAuthorID`) and pruning are intentionally not
//! modelled: identity here is the human-facing name, and removal of library
//! content is out of scope (mirrors the other create-only resources).

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// A tracked author.
#[resource(sync = custom, case = snake, list = get("/api?cmd=getIndex"))]
pub struct Author {
    /// Author name — its identity (wire/JSON key `AuthorName`).
    #[key]
    #[wire(name = "AuthorName")]
    pub name: String,
    /// Also fetch the author's books when first added (`&books`). Create-time
    /// hint only — not part of the author's persisted identity.
    pub add_books: Option<bool>,
}

impl CustomSync for Author {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = match crate::http::get(client, &[("cmd", "getIndex")]).await? {
                Value::Array(items) => items,
                _ => Vec::new(),
            };
            let present = reconcile::present_keys(&live, "AuthorName");
            let client = client.clone();
            reconcile::create_only(desired, "name", &present, execute, move |name, cfg| {
                let client = client.clone();
                async move {
                    let mut params = vec![("cmd", "addAuthor"), ("name", name.as_str())];
                    if cfg.get("add_books").and_then(Value::as_bool) == Some(true) {
                        params.push(("books", "1"));
                    }
                    crate::http::get(&client, &params).await?;
                    Ok(())
                }
            })
            .await
        })
    }
}
