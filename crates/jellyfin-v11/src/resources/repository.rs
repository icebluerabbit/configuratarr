//! `/Repositories` — the plugin repository list.
//!
//! Jellyfin has no per-repository id: the whole list is replaced in one POST.
//! That's a `sync = custom` reconcile — GET the live list, and if the desired set
//! differs (order-insensitively) POST the whole thing. The comparison is
//! **structural** (by the repo's own fields), not serialization-based, so it
//! doesn't depend on JSON key ordering.

use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, engine};
use core_macros::resource;
use serde_json::Value;

/// `/Repositories` — a plugin repository (replaced as part of the whole list).
#[resource(sync = custom, case = pascal, list = get("/Repositories"))]
pub struct Repository {
    /// Repository display name.
    pub name: Option<String>,
    /// Manifest URL for the repository.
    pub url: Option<String>,
    /// Whether this repository is enabled.
    pub enabled: bool,
}

/// Order-insensitive identity of one repo: `(Name, Url, Enabled)`. Structural,
/// so the set comparison in [`reconcile::replace`] ignores JSON key ordering.
fn repo_identity(i: &Value) -> (String, String, bool) {
    (
        i.get("Name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        i.get("Url")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        i.get("Enabled").and_then(Value::as_bool).unwrap_or(false),
    )
}

impl CustomSync for Repository {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/Repositories").await?;

            // Encode each desired config to its wire (PascalCase) form.
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            // Whole-list replace: `reconcile::replace` owns the set diff + the
            // preview gate; this hook only supplies the identity and the write.
            let to_post = wire.clone();
            let client = client.clone();
            reconcile::replace(
                &wire,
                &live,
                "repositories",
                execute,
                repo_identity,
                move || async move {
                    let _: Value = client.post("/Repositories", &Value::Array(to_post)).await?;
                    Ok(())
                },
            )
            .await
        })
    }
}
