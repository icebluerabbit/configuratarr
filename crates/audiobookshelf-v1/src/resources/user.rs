//! `/api/users` — local (and OpenID-linked) accounts.
//!
//! `GET /api/users` returns an envelope (`{ users: [...] }`), not a bare
//! array — mirrors [`crate::resources::library::Library`]'s own `GET
//! /api/libraries` envelope, and for the same reason `sync = crud` doesn't
//! fit: the engine's automatic `${ref}` id-registration only understands a
//! bare-array list response, so it would see zero live users on every apply
//! and `${ref.user.<username>}` would never resolve downstream (e.g.
//! [`crate::resources::ereader_device::EreaderDevice::users`]). This hook
//! plucks `.users` out of the envelope itself and registers ids into the
//! [`RefStore`] manually, before and after reconciling — same pattern as
//! `Library`.
//!
//! Create is `POST /api/users` (`CreateUserRequest`; `username`+`password`
//! required — a user declared with no `password` cannot be created, a 400
//! from the API, not a bug in this resource). Update is `PATCH
//! /api/users/{id}` (`UpdateUserRequest`, all fields optional; `id`, `pash`,
//! `token`, `extraData`, `bookmarks` are rejected with 400 if present — none
//! of which this struct models, so the body is always clean). Delete (under
//! `--prune`) is `DELETE /api/users/{id}`.
//!
//! `password` never reads back — `User`'s response schema has no `password`
//! property at all (only the unrelated legacy `token`) — so there is nothing
//! in the live `GET` to diff a stored password against. Because ABS also
//! *requires* a password to create an account, the two concerns are split the
//! same way `bindery-v1`'s `User` splits them:
//!
//! * `password` is **create-only**. Sent by [`create_body`], stripped by
//!   [`update_body`], never diffed. Leaving it in config is free.
//! * `reset_password` is the **one-shot action**. [`update_body`] promotes it
//!   into the `password` key the PATCH reads, and [`in_sync`] treats its mere
//!   presence as drift — so it fires on every apply until removed.
//!
//! Conflating them (diffing `password` itself) looks tidier and is wrong: a
//! user created by configuratarr would then report drift forever, because the
//! config that created it still declares the password it was created with.
//! Found by the live e2e.
//!
//! `permissions` is a genuinely nested object on both read and write —
//! unlike `EmailSettings`' envelope, `User`/`CreateUserRequest`/
//! `UpdateUserRequest` all nest it under `"permissions"` consistently — and
//! `UpdateUserRequest` merges it onto the user's *existing* permissions key
//! by key, so this hook presence-masks into it the same way `Library`
//! presence-masks into `settings`. `permissions.librariesAccessible`/
//! `itemTagsSelected` are the *canonical* location for that data;
//! `User`'s own top-level mirrors of the same two fields (kept only for
//! legacy pre-permissions-object clients, and themselves read-only echoes on
//! `User`) are intentionally not modeled here — same data, redundant path.
//!
//! `type: "root"` is unassignable via this API (enforced server-side, per
//! the spec: "create/update only accept admin/user/guest") so [`UserType`]
//! only carries the three creatable/updatable roles; a live root user is
//! matched/read as raw JSON in this hook, never decoded through that type.

use core_lib::{
    CustomSync, CustomSyncFuture, HttpClient, RefId, RefStore, SecretValue, engine, reconcile,
};
use core_macros::{nested, resource, wire_enum};
use serde_json::Value;

/// A local account's role. `root` (the initial admin, created via `/init`)
/// cannot be assigned through this API and so has no variant here.
#[wire_enum(rename_all = "lowercase")]
pub enum UserType {
    /// Full administrative access.
    Admin,
    /// Standard, non-administrative access.
    User,
    /// Access further scoped by `permissions`; read/download only by
    /// server-side default.
    Guest,
}

/// Boolean capability flags plus library/tag scoping (`UserPermissions`).
/// Any key this struct's config omits is left at the server's own default —
/// see each field's doc for its per-account-type default from the spec —
/// which is exactly why no field here carries a literal `#[default(...)]`:
/// this resource's hook always presence-masks (see the module doc and
/// [`crate::resources::library_settings::LibrarySettings`]'s identical
/// reasoning), so a `#[default]` would only apply to an unmasked encode path
/// this resource never uses.
#[nested]
pub struct UserPermissions {
    /// Defaults true for all account types.
    pub download: Option<bool>,
    /// Defaults true for root/admin, false for user/guest.
    pub update: Option<bool>,
    /// Defaults true for root only.
    pub delete: Option<bool>,
    /// Defaults true for root/admin.
    pub upload: Option<bool>,
    /// Defaults true for root/admin.
    pub create_ereader: Option<bool>,
    /// Defaults true. When true, `libraries_accessible` is ignored/cleared
    /// server-side.
    pub access_all_libraries: Option<bool>,
    /// Defaults true. When true, `item_tags_selected` is unused.
    pub access_all_tags: Option<bool>,
    /// Defaults true for root/admin.
    pub access_explicit_content: Option<bool>,
    /// Inverts `item_tags_selected` into a denylist instead of an allowlist
    /// when true. Defaults false.
    pub selected_tags_not_accessible: Option<bool>,
    /// Library ids the user may access when `access_all_libraries` is false.
    #[reference(library)]
    pub libraries_accessible: Vec<String>,
    /// Tag names selected as allow- (or deny-, if
    /// `selected_tags_not_accessible`) list when `access_all_tags` is false.
    pub item_tags_selected: Vec<String>,
}

/// `/api/users` — a local or OpenID-linked account.
#[resource(sync = custom, list = get("/api/users"))]
pub struct User {
    /// Server-assigned id (a UUID). Read-only; declared so
    /// `engine::id_shape` knows a `${ref.user.*}` placeholder has to be a
    /// string, not the integer `-1`.
    #[id]
    pub id: Option<String>,
    /// Login name — this account's identity. Changing it server-side
    /// regenerates the legacy API token and invalidates JWT sessions; a
    /// `username` change via config is not specially handled and, like any
    /// resource keyed by a mutable field, just looks like a new user to this
    /// hook's keyed match (the old name is orphaned, not renamed).
    #[key]
    pub username: String,
    /// Account email, if any.
    pub email: Option<String>,
    /// The account's role. Required on create; the API itself defaults to
    /// `user` if omitted there.
    #[wire(name = "type")]
    pub user_type: Option<UserType>,
    /// Initial password, plaintext — bcrypt-hashed server-side, never echoed
    /// back on any read. **Create-only and required to create**: ABS rejects
    /// a `POST /api/users` without one. It is never sent on update and never
    /// diffed, so leaving it in config costs nothing. To change an existing
    /// account's password, use `reset_password`.
    pub password: Option<SecretValue>,
    /// Opt-in password reset for an existing account. When set, **every apply**
    /// PATCHes this value — remove the field from config once the reset has
    /// taken effect, or it keeps firing. Deliberately separate from the
    /// create-only `password`; see the module doc.
    pub reset_password: Option<SecretValue>,
    /// Whether the account can log in. API create default: `false` — an
    /// omitted value here creates an inactive account.
    pub is_active: Option<bool>,
    /// Whether the account is locked out (failed-login lockout). Reported on
    /// read, but writable through neither `CreateUserRequest` nor
    /// `UpdateUserRequest` — ABS only clears it internally — so it is
    /// read-only here and declaring it in config has no effect.
    #[wire(read_only)]
    pub is_locked: Option<bool>,
    /// Epoch millis of last activity. Rarely client-set; primarily
    /// server-maintained. Accepted on update but **not** on create
    /// (`CreateUserRequest` has no such property), so [`create_body`] strips
    /// it.
    pub last_seen: Option<i64>,
    /// Capability flags + library/tag scoping. See [`UserPermissions`] for
    /// how an omitted key behaves on create vs. update.
    pub permissions: Option<UserPermissions>,
}

/// Register `(username -> id)` into the [`RefStore`] under `"user"` for
/// every entry that has both — the manual counterpart of the engine's own
/// `register_refs`, which can't see inside `GET /api/users`'s `{users:
/// [...]}` envelope (see the module doc; mirrors `Library`'s
/// `register_library_ids`).
fn register_user_ids<'a>(refs: &mut RefStore, entries: impl IntoIterator<Item = &'a Value>) {
    for e in entries {
        if let (Some(name), Some(id)) = (
            e.get("username").and_then(Value::as_str),
            e.get("id").and_then(RefId::from_value),
        ) {
            refs.insert("user", name, id);
        }
    }
}

/// True when every key `wire` declares (other than `password`, handled
/// separately below) matches `live`'s corresponding value; nested objects
/// (`permissions`) recurse the same way, so an unset permission sub-flag
/// never forces an update. Extra `live` keys — `id`, `token`, timestamps,
/// the legacy top-level `librariesAccessible`/`itemTagsSelected` mirrors —
/// are ignored, since `wire` never declares them.
fn fields_match(want: &Value, have: &Value) -> bool {
    let (Some(w), Some(h)) = (want.as_object(), have.as_object()) else {
        return want == have;
    };
    w.iter().all(|(k, wv)| {
        if k == "password" {
            return true;
        }
        match h.get(k) {
            Some(hv) if wv.is_object() && hv.is_object() => fields_match(wv, hv),
            Some(hv) => wv == hv,
            None => false,
        }
    })
}

/// [`reconcile::upsert`]'s idempotency predicate.
///
/// `resetPassword`, when present, always counts as "changed": the server never
/// echoes a password back, so there is no live value to compare against. That
/// is what makes it a one-shot action field that has to be removed from config
/// once applied.
///
/// `password` is deliberately *not* treated that way. ABS requires it to create
/// an account, so if it forced an update too, every user this crate creates
/// would report drift on every subsequent apply for as long as the config that
/// created it stayed in place — permanent, unavoidable churn. It is create-only
/// instead, and [`create_body`]/[`update_body`] enforce that split on the wire.
fn in_sync(wire: &Value, live: &Value) -> bool {
    if wire.get("resetPassword").is_some_and(|p| !p.is_null()) {
        return false;
    }
    fields_match(wire, live)
}

/// `POST /api/users` body: the encoded wire minus the keys
/// `CreateUserRequest` doesn't accept — `resetPassword` (this crate's own
/// one-shot field) and `lastSeen` (update-only).
fn create_body(mut w: Value) -> Value {
    if let Value::Object(o) = &mut w {
        o.remove("resetPassword");
        o.remove("lastSeen");
    }
    w
}

/// `PATCH /api/users/{id}` body: drop the create-only `password`, and promote a
/// declared `resetPassword` into the `password` key the API actually reads.
fn update_body(mut w: Value) -> Value {
    if let Value::Object(o) = &mut w {
        o.remove("password");
        if let Some(new) = o.remove("resetPassword").filter(|v| !v.is_null()) {
            o.insert("password".to_string(), new);
        }
    }
    w
}

impl CustomSync for User {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let envelope: Value = client.get("/api/users").await?;
            let live: Vec<Value> = envelope
                .get("users")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            register_user_ids(refs, &live);

            // Present-masked wire: only the keys the user's config actually
            // wrote, at the top level and (recursively) within
            // `permissions` — exactly what `in_sync` and the create/update
            // bodies need. See the module doc for why `permissions` must
            // stay partial.
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::config_present_to_wire::<Self>)
                .collect::<anyhow::Result<_>>()?;

            let created = std::sync::Mutex::new(Vec::<Value>::new());

            let changes = reconcile::upsert_prune(
                &wire,
                &live,
                "username",
                in_sync,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    let created = &created;
                    async move {
                        let resp: Value = client.post("/api/users", &create_body(w)).await?;
                        if let Some(user) = resp.get("user") {
                            created
                                .lock()
                                .expect("created-users mutex poisoned")
                                .push(user.clone());
                        }
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
                    async move {
                        let _: Value = client
                            .patch(&format!("/api/users/{id}"), &update_body(w))
                            .await?;
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
                        client.delete(&format!("/api/users/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await?;

            let created = created.into_inner().expect("created-users mutex poisoned");
            register_user_ids(refs, &created);

            Ok(changes)
        })
    }
}

// A small helper used only by [`in_sync`]/[`fields_match`], not part of the
// public surface — keep the test private too, matching
// `crate::resources::notification`'s convention for this crate.
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reset_password_present_always_forces_update() {
        let wire = json!({"username": "alice", "resetPassword": "new-secret"});
        let live = json!({"username": "alice", "id": "u-1"});
        assert!(!in_sync(&wire, &live));
    }

    /// The create-only counterpart: a still-declared `password` must not read
    /// as drift, or every account this crate creates churns forever.
    #[test]
    fn create_only_password_is_not_drift() {
        let wire = json!({"username": "alice", "password": "created-with"});
        let live = json!({"username": "alice", "id": "u-1"});
        assert!(in_sync(&wire, &live));
    }

    #[test]
    fn create_body_drops_reset_password_and_update_body_promotes_it() {
        let wire = json!({
            "username": "alice",
            "password": "created-with",
            "resetPassword": "rotated-to",
        });
        assert_eq!(
            create_body(wire.clone()),
            json!({"username": "alice", "password": "created-with"}),
        );
        // `lastSeen` is accepted on update but not on create.
        assert_eq!(
            create_body(json!({"username": "alice", "lastSeen": 1})),
            json!({"username": "alice"}),
        );
        assert_eq!(
            update_body(wire),
            json!({"username": "alice", "password": "rotated-to"}),
        );
    }

    /// With no reset declared, an update carries no password at all.
    #[test]
    fn update_body_without_a_reset_sends_no_password() {
        assert_eq!(
            update_body(json!({"username": "alice", "password": "created-with"})),
            json!({"username": "alice"}),
        );
    }

    #[test]
    fn declared_permissions_subset_matches_a_richer_live_object() {
        let wire = json!({
            "username": "alice",
            "permissions": {"download": true},
        });
        let live = json!({
            "username": "alice",
            "id": "u-1",
            "permissions": {"download": true, "update": false, "delete": false},
        });
        assert!(in_sync(&wire, &live));
    }

    #[test]
    fn in_sync_catches_a_real_diff() {
        let wire = json!({"username": "alice", "isActive": true});
        let live = json!({"username": "alice", "isActive": false});
        assert!(!in_sync(&wire, &live));
    }
}
