//! `/api/v1/auth/users` — local accounts.
//!
//! A bare array of `User` (`{id, username, role, email, displayName,
//! createdAt}`, `id` an int64), but there's no whole-object update: `role`
//! goes to `PUT /api/v1/auth/users/{id}/role`, and a password reset goes to
//! `PUT /api/v1/auth/users/{id}/reset-password` — `password` never reads
//! back, so there's nothing in the live GET to diff a stored password
//! against. `sync = custom` runs a small bespoke state machine per declared
//! user:
//!
//! * absent → `POST /api/v1/auth/users` (`{username, password, role}`) → created
//!   (an absent/empty `password` is an error, not an empty credential)
//! * present, same role → unchanged
//! * present, different role → `PUT .../role` → updated
//!
//! then [`reconcile::prune_absent`] deletes any live user the config no
//! longer declares.
//!
//! **The last admin is undeletable and undemotable.** Bindery 400s on both
//! (`cannot delete the last admin user` / `cannot demote the last admin
//! user`), so once an instance has an admin, a config that stops declaring it
//! makes every `--prune` apply fail — there is no way for this hook to satisfy
//! it. Keep at least one admin declared.
//!
//! Password reset is opt-in only, via `reset_password` — deliberately a
//! *different* field from the create-only `password`, and never inferred
//! from a diff (there's nothing to diff against: the API never returns a
//! password, hashed or otherwise, so this hook can't tell whether the stored
//! password already matches). Declaring `reset_password` resets the password
//! on **every apply it stays set** — there is no live signal that says the
//! reset already happened. Treat it as a one-shot action field: set it,
//! apply, then remove it from the config.

use core_lib::reconcile;
use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine};
use core_macros::{resource, wire_enum};
use serde_json::{Value, json};

/// A local account's role.
#[wire_enum(rename_all = "lowercase")]
pub enum UserRole {
    /// Full administrative access.
    Admin,
    /// Standard, non-administrative access.
    User,
    /// Unrecognised role.
    #[fallback]
    Unknown,
}

/// `/api/v1/auth/users` — a local account.
#[resource(sync = custom, list = get("/api/v1/auth/users"))]
pub struct User {
    /// Login name — the account's identity.
    #[key]
    pub username: String,
    /// The account's role.
    pub role: UserRole,
    /// Initial password, used only when creating this user
    /// (`POST /api/v1/auth/users`). Required to create one — declaring a user
    /// that doesn't exist yet without a password is an error. Ignored for an
    /// already-existing user; see `reset_password` to change one.
    pub password: Option<SecretValue>,
    /// Opt-in password reset for an existing user. When set, **every apply**
    /// PUTs this value to `.../reset-password` — remove the field from the
    /// config once the reset has taken effect, or it keeps firing.
    pub reset_password: Option<SecretValue>,
}

/// The integer `id` a live user JSON object carries. Required to hit the
/// `{id}`-keyed role/reset-password/delete sub-paths.
fn user_id(u: &Value) -> anyhow::Result<i64> {
    u.get("id")
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow::anyhow!("user entry is missing an integer `id`"))
}

impl CustomSync for User {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/v1/auth/users").await?;
            let mut changes = Vec::with_capacity(desired.len());

            for cfg in desired {
                let wire = engine::encode_config::<Self>(cfg)?;
                let username = wire
                    .get("username")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow::anyhow!("user entry is missing `username`"))?
                    .to_string();
                let role = wire
                    .get("role")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();

                let existing = live
                    .iter()
                    .find(|u| u.get("username").and_then(Value::as_str) == Some(username.as_str()));

                let Some(u) = existing else {
                    // Creating without a password would POST `""`, which the
                    // server accepts as a real (empty) credential — fail loudly
                    // instead, in plan as well as apply.
                    let password = wire
                        .get("password")
                        .and_then(Value::as_str)
                        .filter(|s| !s.is_empty())
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "user `{username}` does not exist and declares no `password`; \
                                 a password is required to create it"
                            )
                        })?
                        .to_string();
                    if execute {
                        let _: Value = client
                            .post(
                                "/api/v1/auth/users",
                                &json!({ "username": username, "password": password, "role": role }),
                            )
                            .await?;
                    }
                    changes.push(Change::created(username.as_str()));
                    continue;
                };

                let mut updated = false;
                let live_role = u.get("role").and_then(Value::as_str).unwrap_or_default();
                if live_role != role {
                    if execute {
                        let id = user_id(u)?;
                        let _: Value = client
                            .put(
                                &format!("/api/v1/auth/users/{id}/role"),
                                &json!({ "role": role }),
                            )
                            .await?;
                    }
                    updated = true;
                }

                if let Some(new_password) = wire
                    .get("resetPassword")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                {
                    if execute {
                        let id = user_id(u)?;
                        let _: Value = client
                            .put(
                                &format!("/api/v1/auth/users/{id}/reset-password"),
                                &json!({ "password": new_password }),
                            )
                            .await?;
                    }
                    updated = true;
                }

                changes.push(if updated {
                    Change::updated(username.as_str()).with("role", role)
                } else {
                    Change::unchanged(username.as_str())
                });
            }

            changes.extend(
                reconcile::prune_absent(desired, &live, "username", prune, execute, |u| {
                    let client = client.clone();
                    let id = u.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        let id = id.as_i64().ok_or_else(|| {
                            anyhow::anyhow!("user entry is missing an integer `id`")
                        })?;
                        client.delete(&format!("/api/v1/auth/users/{id}")).await
                    }
                })
                .await?,
            );

            Ok(changes)
        })
    }
}
