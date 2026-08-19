//! `/api/v2/users` — Komga user accounts.
//!
//! Modelled from `UserDto` (read), `UserCreationDto` (create) and
//! `UserUpdateDto` (update). It cannot be a plain `sync = crud` collection —
//! three independent shape mismatches between the read and write DTOs:
//!
//! 1. **Shared libraries.** `UserDto` reads back two flat fields,
//!    `sharedAllLibraries` (bool) and `sharedLibrariesIds` (array), but
//!    create/update take one nested object, `sharedLibraries: {all,
//!    libraryIds}` (`SharedLibrariesUpdateDto`, [`crate::resources::shared_libraries`]).
//! 2. **`password`** (`UserCreationDto`) is create-only and never reads
//!    back on any endpoint this crate models (Komga's only read-adjacent
//!    surface for it is `PATCH /api/v2/users/{id}/password`, a separate,
//!    unmodelled action endpoint). A declared password change is therefore
//!    **not detectable as drift** — see the `password` field doc.
//! 3. **`ageRestriction.restriction`** accepts the write-only sentinel
//!    `"NONE"` (clear the restriction) but a cleared restriction reads back
//!    as an *absent* `ageRestriction`, never as `{"restriction": "NONE"}`
//!    (`AgeRestrictionUpdateDto`, [`crate::resources::age_restriction`]).
//!
//! So [`User`] is `sync = custom`, keyed by `email`, reconciled with
//! [`core_lib::reconcile::upsert_prune`]:
//! * `create` → `POST /api/v2/users` with the full encoded wire body
//!   (`UserCreationDto` shape).
//! * `update` → `PATCH /api/v2/users/{live.id}` with the encoded wire body
//!   minus `email`/`password` (`UserUpdateDto` has neither — Komga's PATCH
//!   doesn't support changing either through this endpoint).
//! * `delete` → `DELETE /api/v2/users/{live.id}`, gated on `--prune`.
//!
//! `roles`, `labelsAllow`, `labelsExclude` and the shared-library id list are
//! all `uniqueItems: true` in the spec — Kotlin `Set`s, whose serialized
//! order is not contractual — so [`in_sync`] compares each of them as a set,
//! never element-wise.
//!
//! One more read/write asymmetry the spec does not describe: Komga stores the
//! implicit `USER` role on every account whatever was written, so a set
//! comparison of `roles` has to ignore it or the hook `PATCH`es forever. See
//! [`roles_in_sync`]. Found by the live e2e — a conformance check never reads a
//! server response back, so nothing offline could have caught it.

use std::collections::HashSet;

use core_lib::engine;
use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue};
use core_macros::resource;
use serde_json::Value;

use crate::resources::age_restriction::AgeRestriction;
use crate::resources::shared_libraries::SharedLibraries;

/// A Komga user account.
#[resource(sync = custom, list = get("/api/v2/users"))]
pub struct User {
    /// Server-assigned id (a UUID).
    #[id]
    pub id: Option<String>,
    /// Login email — the user's identity. Sent on create; **not** sent on
    /// update (`UserUpdateDto` has no `email` field — Komga doesn't support
    /// changing an existing user's email through this endpoint).
    #[key]
    pub email: String,
    /// Initial password, set only when the user is created. **Create-only**:
    /// Komga never reads a password back (the only related endpoint,
    /// `PATCH /api/v2/users/{id}/password`, is a separate write-only action
    /// this crate doesn't model), so changing `password` for an
    /// already-created user is **not detected as drift** — it never triggers
    /// an update, and the update body never carries it. To rotate an
    /// existing user's password, use Komga directly.
    pub password: Option<SecretValue>,
    /// Granted roles, e.g. `ADMIN`, `PAGE_STREAMING`, `FILE_DOWNLOAD`,
    /// `KOBO_SYNC`. Compared as a set — Komga's wire order isn't
    /// contractual.
    pub roles: Vec<String>,
    /// Content labels this user is restricted to (an allow-list). Compared
    /// as a set.
    pub labels_allow: Vec<String>,
    /// Content labels this user cannot see (an exclude-list). Compared as a
    /// set.
    pub labels_exclude: Vec<String>,
    /// Age-based content restriction. Omitted from config = not managed by
    /// configuratarr, left as Komga has it. See the module docs for the
    /// `restriction: NONE` read/write asymmetry.
    pub age_restriction: Option<AgeRestriction>,
    /// Library access grant. Omitted from config = not managed by
    /// configuratarr, left as Komga has it. See the module docs for the
    /// nested-vs-flat read/write asymmetry.
    pub shared_libraries: Option<SharedLibraries>,
}

/// A JSON array's string elements, as a set — for comparing `roles` /
/// `labelsAllow` / `labelsExclude` / `libraryIds`, all `uniqueItems: true` in
/// the spec (Kotlin `Set`s with no contractual wire order). A non-array (e.g.
/// absent, `null`) reads as the empty set.
fn as_set(v: &Value) -> HashSet<&str> {
    v.as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

/// The implicit role every Komga account carries. The server adds it to
/// `roles` on write whether or not it was sent, so it can never be part of a
/// meaningful diff — see [`roles_in_sync`].
const IMPLICIT_ROLE: &str = "USER";

/// `roles` comparison, ignoring the implicit [`IMPLICIT_ROLE`].
///
/// Komga stores `USER` on every account regardless of what was written, so a
/// plain set comparison against a config that (reasonably) doesn't spell it out
/// never converges: the hook would `PATCH` the same roles on every apply,
/// forever. Found live — conformance can't see it, since it never reads a
/// server response back.
fn roles_in_sync(desired: &Value, live: &Value) -> bool {
    let strip = |v: &Value| -> HashSet<String> {
        as_set(v)
            .into_iter()
            .filter(|r| *r != IMPLICIT_ROLE)
            .map(str::to_string)
            .collect()
    };
    strip(desired) == strip(live)
}

/// Idempotency predicate for [`User`]'s `sync = custom` reconcile: `desired`
/// is our encoded wire object (`UserCreationDto` shape — nested
/// `sharedLibraries`, `ageRestriction.restriction` possibly `"NONE"`);
/// `live` is one raw `UserDto` element from `GET /api/v2/users` (flat
/// `sharedAllLibraries`/`sharedLibrariesIds`, `ageRestriction` present only
/// when a restriction is actually set). `email`/`password`/`id` are not
/// compared here: `email` is the match key, `id` is server-owned, and
/// `password` never reads back (see the module docs).
fn in_sync(desired: &Value, live: &Value) -> bool {
    if !roles_in_sync(&desired["roles"], &live["roles"]) {
        return false;
    }
    if as_set(&desired["labelsAllow"]) != as_set(&live["labelsAllow"]) {
        return false;
    }
    if as_set(&desired["labelsExclude"]) != as_set(&live["labelsExclude"]) {
        return false;
    }

    // Shared libraries: not declared in config = not managed, always in
    // sync. Declared = compare the nested write shape against the live
    // flattened shape (see module docs, mismatch #1).
    if let Some(shared) = desired.get("sharedLibraries") {
        let want_all = shared.get("all").and_then(Value::as_bool).unwrap_or(false);
        let have_all = live
            .get("sharedAllLibraries")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if want_all != have_all {
            return false;
        }
        let empty = Value::Array(Vec::new());
        let want_ids = as_set(shared.get("libraryIds").unwrap_or(&empty));
        let have_ids = as_set(&live["sharedLibrariesIds"]);
        if want_ids != have_ids {
            return false;
        }
    }

    // Age restriction: not declared in config = not managed, always in
    // sync. Declared `restriction: NONE` matches an *absent* live
    // `ageRestriction` (see module docs, mismatch #3); any other declared
    // restriction must match a present live `ageRestriction` field-by-field.
    if let Some(want_ar) = desired.get("ageRestriction") {
        let want_restriction = want_ar
            .get("restriction")
            .and_then(Value::as_str)
            .unwrap_or("NONE");
        match live.get("ageRestriction").filter(|v| !v.is_null()) {
            None => {
                if want_restriction != "NONE" {
                    return false;
                }
            }
            Some(have_ar) => {
                if want_restriction == "NONE" {
                    return false;
                }
                let want_age = want_ar.get("age").and_then(Value::as_i64);
                let have_age = have_ar.get("age").and_then(Value::as_i64);
                let have_restriction = have_ar.get("restriction").and_then(Value::as_str);
                if want_age != have_age || Some(want_restriction) != have_restriction {
                    return false;
                }
            }
        }
    }

    true
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
            let live: Vec<Value> = client.get("/api/v2/users").await?;
            // Full desired wire, `UserCreationDto` shape (nested
            // `sharedLibraries`, `email`/`password` included). The `update`
            // closure below strips what `UserUpdateDto` doesn't carry.
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert_prune(
                &wire,
                &live,
                "email",
                in_sync,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/v2/users", &w).await?;
                        Ok(())
                    }
                },
                |l, mut w| {
                    let client = client.clone();
                    let id = l
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    // UserUpdateDto has neither `email` nor `password`.
                    if let Some(obj) = w.as_object_mut() {
                        obj.remove("email");
                        obj.remove("password");
                    }
                    async move {
                        let _: Value = client.patch(&format!("/api/v2/users/{id}"), &w).await?;
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
                    async move { client.delete(&format!("/api/v2/users/{id}")).await }
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

    /// Komga adds `USER` to every account on write. A config that doesn't
    /// declare it must still read as in sync, or the hook re-`PATCH`es the same
    /// roles on every apply.
    #[test]
    fn implicit_user_role_does_not_count_as_a_difference() {
        assert!(roles_in_sync(
            &json!(["PAGE_STREAMING", "FILE_DOWNLOAD"]),
            &json!(["FILE_DOWNLOAD", "PAGE_STREAMING", "USER"]),
        ));
        // Declaring it explicitly is equally fine.
        assert!(roles_in_sync(
            &json!(["USER", "PAGE_STREAMING"]),
            &json!(["PAGE_STREAMING", "USER"]),
        ));
        // Order is still irrelevant, and a real difference still shows.
        assert!(!roles_in_sync(
            &json!(["PAGE_STREAMING"]),
            &json!(["PAGE_STREAMING", "FILE_DOWNLOAD", "USER"]),
        ));
        assert!(!roles_in_sync(&json!(["ADMIN"]), &json!(["USER"])));
    }
}
