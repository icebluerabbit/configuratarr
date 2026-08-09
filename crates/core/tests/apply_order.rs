//! Apply ordering from static `#[reference]` metadata.

use core_lib::apply::apply_order;
use core_macros::{nested, resource, service};

#[resource(
    sync = crud,
    list = get("/api/v3/tag"),
    create = post("/api/v3/tag"),
    update = put("/api/v3/tag/${self.id}"),
    delete = delete("/api/v3/tag/${self.id}"),
)]
pub struct Tag {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub label: String,
}

/// References `tag` — must be applied *after* tags exist.
#[resource(
    sync = crud,
    list = get("/api/v3/widget"),
    create = post("/api/v3/widget"),
    update = put("/api/v3/widget/${self.id}"),
    delete = delete("/api/v3/widget/${self.id}"),
)]
pub struct Widget {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    #[reference(tag)]
    pub tags: Vec<i32>,
}

// Declared widgets-BEFORE-tags on purpose: a wrong impl that just preserved
// declaration order would fail this.
#[service(name = "lib", auth = none)]
pub struct Lib {
    pub url: String,
    pub widgets: Vec<Widget>,
    pub tags: Vec<Tag>,
}

#[test]
fn referenced_type_comes_first() {
    let order = apply_order::<Lib>().unwrap();
    let tag = order.iter().position(|t| *t == "tag").unwrap();
    let widget = order.iter().position(|t| *t == "widget").unwrap();
    assert!(tag < widget, "tag must precede widget: {order:?}");
}

#[test]
fn order_is_complete() {
    let order = apply_order::<Lib>().unwrap();
    assert_eq!(order.len(), 2);
}

// ── reference nested inside a `Vec<Nested>` ──────────────────────────────────
// The FK lives on a nested element (`Gadget.links[].zone_id`), not a top-level
// field. An `empty()` Gadget has an empty `links` vec, so an instance-based
// reference walk can't see the FK — the bug that let `filter.indexers[].id`
// escape the graph. `zone` sorts *after* `gadget` alphabetically, so only a real
// edge (not fallback ordering) puts it first.

#[resource(
    sync = crud,
    list = get("/api/zone"),
    create = post("/api/zone"),
    update = put("/api/zone/${self.id}"),
    delete = delete("/api/zone/${self.id}"),
)]
pub struct Zone {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

#[nested]
pub struct GadgetLink {
    #[reference(zone)]
    pub zone_id: Option<i32>,
}

#[resource(
    sync = crud,
    list = get("/api/gadget"),
    create = post("/api/gadget"),
    update = put("/api/gadget/${self.id}"),
    delete = delete("/api/gadget/${self.id}"),
)]
pub struct Gadget {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    pub links: Vec<GadgetLink>,
}

#[service(name = "shed", auth = none)]
pub struct Shed {
    pub url: String,
    pub gadgets: Vec<Gadget>,
    pub zones: Vec<Zone>,
}

#[test]
fn reference_inside_vec_nested_creates_an_edge() {
    let order = apply_order::<Shed>().unwrap();
    let zone = order.iter().position(|t| *t == "zone").unwrap();
    let gadget = order.iter().position(|t| *t == "gadget").unwrap();
    assert!(zone < gadget, "zone must precede gadget: {order:?}");
}

// ── self-referential nested type ─────────────────────────────────────────────
// A nested type whose field nests the *same* type (like radarr's quality-profile
// groups). `reference_targets` must terminate (a naive recursion stack-overflows)
// and still collect the FK carried on the recursive node.

#[nested]
pub struct TreeNode {
    #[reference(zone)]
    pub zone_id: Option<i32>,
    pub children: Vec<TreeNode>,
}

#[resource(
    sync = crud,
    list = get("/api/tree"),
    create = post("/api/tree"),
    update = put("/api/tree/${self.id}"),
    delete = delete("/api/tree/${self.id}"),
)]
pub struct Tree {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    pub root: Vec<TreeNode>,
}

#[service(name = "forest", auth = none)]
pub struct Forest {
    pub url: String,
    pub trees: Vec<Tree>,
    pub zones: Vec<Zone>,
}

#[test]
fn recursive_nested_terminates_and_finds_ref() {
    // Would stack-overflow without a cycle guard.
    let order = apply_order::<Forest>().unwrap();
    let zone = order.iter().position(|t| *t == "zone").unwrap();
    let tree = order.iter().position(|t| *t == "tree").unwrap();
    assert!(zone < tree, "zone must precede tree: {order:?}");
}
