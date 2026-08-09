//! `/Users` — Jellyfin user accounts.
//!
//! A user spans several endpoints with a server-assigned **GUID** id, so it's a
//! `sync = custom` resource:
//!   * create  — `POST /Users/New` (`{ Name, Password }`) → `UserDto` (its `Id`)
//!   * policy  — `POST /Users/{id}/Policy` (a full `UserPolicy`)
//!
//! The reconcile hook creates any missing user, then merges the declared policy
//! bits over the user's live `Policy` and PUSHes it only when it changed. The
//! new GUID is registered into the ref store (exercises the string-id path).
//! Password is create-only (changing it needs the current password). No prune —
//! the bootstrap admin must survive.

use core_lib::merge::merge;
use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefId, RefStore, SecretValue};
use core_macros::resource;
use serde_json::{Map, Value, json};

/// Declared policy bits, as `(config key, wire `UserPolicy` key)`.
const POLICY_FIELDS: &[(&str, &str)] = &[
    ("is_administrator", "IsAdministrator"),
    ("is_disabled", "IsDisabled"),
    ("is_hidden", "IsHidden"),
    ("enable_remote_access", "EnableRemoteAccess"),
    ("enable_content_deletion", "EnableContentDeletion"),
];

/// `/Users` — a Jellyfin user account.
#[resource(sync = custom, list = get("/Users"))]
pub struct User {
    /// Login name — the user's identity.
    #[key]
    pub name: String,
    /// Initial password, set when the user is created (create-only).
    pub password: Option<SecretValue>,
    /// Grant full administrator rights.
    pub is_administrator: Option<bool>,
    /// Disable the account (cannot sign in).
    pub is_disabled: Option<bool>,
    /// Hide the user from login pages.
    pub is_hidden: Option<bool>,
    /// Allow access from outside the local network.
    pub enable_remote_access: Option<bool>,
    /// Allow the user to delete media.
    pub enable_content_deletion: Option<bool>,
}

/// Build the `{ wireKey: value }` policy patch from the declared bits present in
/// `cfg`. Empty if the user declared no policy fields.
fn policy_patch(cfg: &Value) -> Map<String, Value> {
    let mut patch = Map::new();
    for (snake, wire) in POLICY_FIELDS {
        if let Some(v) = cfg.get(snake) {
            patch.insert((*wire).to_string(), v.clone());
        }
    }
    patch
}

impl CustomSync for User {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/Users").await?;
            let mut changes = Vec::with_capacity(desired.len());

            for cfg in desired {
                let name = cfg
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow::anyhow!("user entry is missing `name`"))?;
                let patch = policy_patch(cfg);

                let existing = live
                    .iter()
                    .find(|u| u.get("Name").and_then(Value::as_str) == Some(name));

                // Resolve the user's id + its live policy, creating first if absent.
                let (id, live_policy, created) = match existing {
                    Some(u) => (
                        u.get("Id").cloned().unwrap_or(Value::Null),
                        u.get("Policy").cloned().unwrap_or_else(|| json!({})),
                        false,
                    ),
                    None => {
                        if !execute {
                            // Preview: no create, so no real id — record intent.
                            refs.insert("user", name, RefId::Pending);
                            changes.push(Change::created(name));
                            continue;
                        }
                        let password = cfg.get("password").and_then(Value::as_str).unwrap_or("");
                        let created: Value = client
                            .post("/Users/New", &json!({ "Name": name, "Password": password }))
                            .await?;
                        (
                            created.get("Id").cloned().unwrap_or(Value::Null),
                            created.get("Policy").cloned().unwrap_or_else(|| json!({})),
                            true,
                        )
                    }
                };

                // Register the GUID so downstream `${ref.user.<name>}` resolve.
                if let Some(rid) = RefId::from_value(&id) {
                    refs.insert("user", name, rid);
                }

                // Reconcile the policy: merge the declared bits over live, PUSH on change.
                let id_str = id.as_str().unwrap_or_default().to_string();
                let mut policy_changed = false;
                if !patch.is_empty() {
                    let merged = merge(&live_policy, &Value::Object(patch.clone()));
                    if merged != live_policy {
                        if execute {
                            let _: Value = client
                                .post(&format!("/Users/{id_str}/Policy"), &merged)
                                .await?;
                        }
                        policy_changed = true;
                    }
                }

                changes.push(if created {
                    Change::created(name)
                } else if policy_changed {
                    // Surface the declared policy bits (no secrets) for the plan view.
                    let mut c = Change::updated(name);
                    for (k, v) in &patch {
                        c = c.with(k.clone(), v.to_string());
                    }
                    c
                } else {
                    Change::unchanged(name)
                });
            }

            Ok(changes)
        })
    }
}
