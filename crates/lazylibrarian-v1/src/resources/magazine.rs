//! Magazines — keyed by `Title`.
//!
//! `getMagazines` lists them; `addMagazine&name=<title>` creates one (setting the
//! title plus server defaults). LazyLibrarian exposes **no** magazine update or a
//! `Change`-expressible delete, so this is a `sync = custom` **create-or-leave**
//! resource ([`core_lib::reconcile::create_only`], keyed by `Title`) — the same
//! "don't remove what you didn't make" caution the *arr create-only resources use.
//!
//! Every command is `GET /api?cmd=…`; writes carry their params in the query
//! string ([`HttpClient::get_query`]), and the api key rides every request via the
//! `Auth::ApiKeyQuery` seam.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore};
use core_macros::resource;
use serde_json::Value;

/// A tracked magazine.
#[resource(sync = custom, case = snake, list = get("/api?cmd=getMagazines"))]
pub struct Magazine {
    /// Magazine title — its identity (wire/JSON key `Title`).
    #[key]
    #[wire(name = "Title")]
    pub title: String,
}

impl CustomSync for Magazine {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> =
                match crate::http::get(client, &[("cmd", "getMagazines")]).await? {
                    Value::Array(items) => items,
                    _ => Vec::new(),
                };
            let present = reconcile::present_keys(&live, "Title");
            let client = client.clone();
            // Desired items are config form (snake) — key on `title`.
            reconcile::create_only(desired, "title", &present, execute, move |title, _cfg| {
                let client = client.clone();
                async move {
                    crate::http::get(&client, &[("cmd", "addMagazine"), ("name", &title)]).await?;
                    Ok(())
                }
            })
            .await
        })
    }
}
