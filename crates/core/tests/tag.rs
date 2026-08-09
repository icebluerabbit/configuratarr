//! Ergonomics proving-ground for the descriptor-driven resource model.
//!
//! `Tag` is the simplest collection resource. This file is the target end
//! state for *every* migrated resource: exactly one `#[resource]` attribute,
//! no serde, no schemars, no hand-written ref/key plumbing. The descriptor the
//! macro emits is the single source of truth — wire mapping, identity, write
//! strategy, and (later) doc generation all read from it.

use core_lib::{Described, FieldRole, HttpMethod, SyncKind};
use core_macros::resource;

/// A label applied to movies, indexers, download clients, etc.
#[resource(
    sync = crud,
    list = get("/api/v3/tag"),
    create = post("/api/v3/tag"),
    update = put("/api/v3/tag/${self.id}"),
    delete = delete("/api/v3/tag/${self.id}"),
)]
pub struct Tag {
    /// Server-assigned key. Always explicit via `#[id]` — no magic-name rule.
    #[id]
    pub id: Option<i32>,
    /// The user-facing tag name, e.g. `4k`. Natural key for `${ref.tag.*}`.
    #[key]
    pub label: String,
}

#[test]
fn descriptor_is_the_single_source() {
    let d = Tag::descriptor();

    // type_name auto-defaulted from the struct ident — never hand-written.
    assert_eq!(d.type_name, "tag");
    assert_eq!(d.endpoints.list.unwrap().path, "/api/v3/tag");
    assert_eq!(d.endpoints.create.unwrap().path, "/api/v3/tag");
    assert_eq!(d.endpoints.update.unwrap().path, "/api/v3/tag/${self.id}");
    assert!(matches!(
        d.endpoints.create.unwrap().method,
        HttpMethod::Post
    ));

    // sync strategy is explicit, no inference.
    assert!(matches!(d.sync, SyncKind::Crud));

    // doc strings flow into the descriptor for later doc-gen (replaces schemars).
    assert_eq!(
        d.doc,
        Some("A label applied to movies, indexers, download clients, etc.")
    );
}

#[test]
fn field_roles_inferred_without_serde() {
    let d = Tag::descriptor();
    assert_eq!(d.fields.len(), 2);

    // `id` → Id role by name convention.
    assert_eq!(d.fields[0].name, "id");
    assert!(matches!(d.fields[0].role, FieldRole::Id));
    assert!(d.fields[0].read_only); // id never sent on write

    // `label` → Key role from `#[key]`.
    assert_eq!(d.fields[1].name, "label");
    assert!(matches!(d.fields[1].role, FieldRole::Key));
    assert_eq!(
        d.fields[1].doc,
        Some("The user-facing tag name, e.g. `4k`. Natural key for `${ref.tag.*}`.")
    );
}
