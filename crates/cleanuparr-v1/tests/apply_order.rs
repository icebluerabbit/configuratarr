//! Dependency ordering for the Cleanuparr service.
//!
//! The seeker's per-instance settings carry a *polymorphic* FK: one
//! `arr_instance_id` that can address either a Sonarr or a Radarr instance. Apply
//! order is derived **only** from `#[reference(...)]` metadata
//! (`core_lib::apply::apply_order` → `FieldDescriptor::reference`), never from
//! the `${ref.…}` text in a user's config — so a single-target attribute would
//! silently leave one of the two collections unordered, and the seeker could be
//! applied before those instances exist.
//!
//! Ties in the topological sort break alphabetically, and `seeker` sorts between
//! `radarr_instance` and `sonarr_instance`, so without the edges this ordering
//! would be wrong in exactly one direction and easy to miss.

use cleanuparr_v1::CleanuparrV1;
use core_lib::apply::apply_order;

#[test]
fn seeker_is_applied_after_both_instance_collections() {
    let order = apply_order::<CleanuparrV1>().expect("dependency graph is acyclic");

    let pos = |name: &str| {
        order
            .iter()
            .position(|t| *t == name)
            .unwrap_or_else(|| panic!("`{name}` missing from apply order: {order:?}"))
    };

    let seeker = pos("seeker");
    for target in ["sonarr_instance", "radarr_instance"] {
        assert!(
            pos(target) < seeker,
            "`{target}` must be applied before `seeker` (order: {order:?})",
        );
    }
}
