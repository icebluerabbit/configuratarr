//! Shared mechanics for `/api/v1/client-settings/{global,user}` — see
//! [`crate::resources::client_setting_global`] / [`crate::resources::client_setting_user`].
//!
//! Both sections share one contract, which is why neither fits crud/singleton
//! (hence `sync = custom` on both, and this helper underneath):
//!
//! * `GET .../{section}/list` returns a JSON **object** map
//!   (`{"<dotted.key>": {value, allowUnauthorized?}}`), not an array — there is
//!   no per-item identity in the response shape, only a top-level key.
//! * `PATCH /api/v1/client-settings/{section}` takes a whole-map body and
//!   **upsert-merges** server-side (per-key), so a single-entry map PATCH is
//!   exactly a per-item write — no read-modify-write of the full map needed.
//!
//! [`live_list`] turns the GET response into the `Vec<Value>` shape
//! [`core_lib::reconcile::upsert`] expects (one object per entry, keyed by
//! `"key"`); [`patch_one`] performs the single-entry PATCH. What differs between
//! global (`value` + `allowUnauthorized`) and user (`value` only) is the typed
//! shape and the `in_sync` comparison, which is why each resource keeps its own
//! `#[resource]` struct and [`core_lib::CustomSync`] impl rather than sharing one
//! generic resource type.
//!
//! **No prune.** Deletion is `DELETE /api/v1/client-settings/{section}` with a
//! JSON **array body** of keys (`["application.key1", "application.key2"]`) —
//! not a per-item `DELETE .../{key}` path. `core_http::HttpClient` has no
//! body-carrying DELETE (`delete_body` or similar) today, and adding one is a
//! `core` change outside this slice's scope (core is off-limits here). Both
//! [`crate::resources::client_setting_global::ClientSettingGlobal`] and
//! [`crate::resources::client_setting_user::ClientSettingUser`] therefore ignore
//! `--prune` entirely: a key removed from config is left on the server rather
//! than deleted. Revisit once `HttpClient` grows a body-carrying DELETE.

use core_lib::HttpClient;
use serde_json::Value;

/// Turn the `GET .../{section}/list` object-map response into a `Vec<Value>`,
/// one entry per setting, with the dotted map key folded into the entry under
/// `"key"` alongside its `value` (and, for global settings, `allowUnauthorized`)
/// — the shape [`core_lib::reconcile::upsert`] matches live items by
/// `key_field`. Skips any entry whose value isn't a JSON object (defensive; the
/// API always returns objects).
pub(crate) fn live_list(map: &Value) -> Vec<Value> {
    map.as_object()
        .into_iter()
        .flatten()
        .filter_map(|(k, v)| {
            let mut entry = v.as_object()?.clone();
            entry.insert("key".to_string(), Value::String(k.clone()));
            Some(Value::Object(entry))
        })
        .collect()
}

/// PATCH `section` (`"global"` or `"user"`) with a single-entry map
/// `{ "<key>": <value> }`, where `<value>` is `wire` with its `"key"` field
/// stripped back out (the map key carries identity on the wire; the resource
/// struct carries it as a regular field for `#[key]`/diffing purposes only).
/// The endpoint upsert-merges, so this one-entry PATCH is exactly a per-item
/// create-or-update write.
pub(crate) async fn patch_one(
    client: &HttpClient,
    section: &str,
    mut wire: Value,
) -> anyhow::Result<()> {
    let key = wire
        .as_object_mut()
        .and_then(|obj| obj.remove("key"))
        .and_then(|k| k.as_str().map(str::to_string))
        .ok_or_else(|| anyhow::anyhow!("client setting entry is missing `key`"))?;
    let body = serde_json::json!({ key: wire });
    let _: Value = client
        .patch(&format!("/api/v1/client-settings/{section}"), &body)
        .await?;
    Ok(())
}
