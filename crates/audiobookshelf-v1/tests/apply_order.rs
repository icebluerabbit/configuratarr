//! Dependency ordering for the Audiobookshelf service.
//!
//! Apply order comes **only** from `#[reference(...)]` metadata
//! (`core_lib::apply::apply_order` → `FieldDescriptor::reference`), never from
//! the `${ref.…}` text in a user's config. A dropped attribute removes an edge
//! silently and the sort falls back to alphabetical order.
//!
//! `ereader_device` → `user` is the case that would actually break: `e` sorts
//! before `u`, so without the edge the devices are applied first and every
//! `${ref.user.…}` in `users[]` resolves against a store that has nothing in it
//! yet.

use audiobookshelf_v1::AudiobookshelfV1;
use core_lib::apply::apply_order;

fn positions() -> impl Fn(&str) -> usize {
    let order = apply_order::<AudiobookshelfV1>().expect("dependency graph is acyclic");
    move |name: &str| {
        order
            .iter()
            .position(|t| *t == name)
            .unwrap_or_else(|| panic!("`{name}` missing from apply order: {order:?}"))
    }
}

/// Both `User.permissions.librariesAccessible` and `Notification.libraryId`
/// address a library by name. `library` sorts first alphabetically either way,
/// so these two assertions guard against a future rename more than against
/// today's order.
#[test]
fn library_is_applied_before_its_dependents() {
    let pos = positions();
    let library = pos("library");
    for dependent in ["user", "notification"] {
        assert!(
            library < pos(dependent),
            "`library` must be applied before `{dependent}`",
        );
    }
}

/// `EreaderDevice.users` lists the user ids allowed to use a device. This is
/// the edge the alphabetical fallback gets wrong.
#[test]
fn ereader_device_is_applied_after_user() {
    let pos = positions();
    assert!(
        pos("user") < pos("ereader_device"),
        "`user` must be applied before `ereader_device`",
    );
}
