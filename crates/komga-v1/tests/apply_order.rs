//! Dependency ordering for the Komga service.
//!
//! Apply order comes **only** from `#[reference(...)]` metadata
//! (`core_lib::apply::apply_order` → `FieldDescriptor::reference`), never from
//! the `${ref.…}` text in a user's config. Dropping the attribute doesn't fail
//! loudly — it removes an edge and the sort falls back to alphabetical order.
//!
//! Here the fallback happens to be wrong: `library` sorts before `user`
//! anyway, so a missing edge would look fine on a fresh server *and* on every
//! re-apply, right up until Komga renames a resource or another one lands
//! between them. Pin it.

use core_lib::apply::apply_order;
use komga_v1::KomgaV1;

/// `User.shared_libraries.library_ids` is reached through a `#[flatten]`ed
/// nested struct, so this also proves `engine::reference_targets` descends into
/// nested types rather than only reading the outer struct's own fields.
#[test]
fn user_is_applied_after_library() {
    let order = apply_order::<KomgaV1>().expect("dependency graph is acyclic");

    let pos = |name: &str| {
        order
            .iter()
            .position(|t| *t == name)
            .unwrap_or_else(|| panic!("`{name}` missing from apply order: {order:?}"))
    };

    assert!(
        pos("library") < pos("user"),
        "`library` must be applied before `user` (order: {order:?})",
    );
}
