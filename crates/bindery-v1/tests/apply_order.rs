//! Dependency ordering for the Bindery service.
//!
//! Apply order is derived **only** from `#[reference(...)]` metadata
//! (`core_lib::apply::apply_order` → `FieldDescriptor::reference`), never from
//! the `${ref.…}` text in a user's config. A dropped attribute therefore
//! doesn't fail loudly — it just removes an edge, and the topological sort
//! falls back to breaking ties alphabetically. That is wrong in one direction
//! only, which is exactly how it escapes review.
//!
//! Bindery is where that matters most: five edges across four resources, and
//! for three of them the alphabetical fallback is already correct, so only the
//! remaining two would ever fail. These assertions pin all five.

use bindery_v1::BinderyV1;
use core_lib::apply::apply_order;

/// Position of `name` in the resolved apply order.
fn positions() -> impl Fn(&str) -> usize {
    let order = apply_order::<BinderyV1>().expect("dependency graph is acyclic");
    move |name: &str| {
        order
            .iter()
            .position(|t| *t == name)
            .unwrap_or_else(|| panic!("`{name}` missing from apply order: {order:?}"))
    }
}

/// `ImportList` carries three FKs — `rootFolderId`, `qualityProfileId` and
/// `ownerUserId`. `import_list` sorts after `quality_profile` and before
/// `root_folder`/`user` alphabetically, so two of the three only hold because
/// the attributes are there.
#[test]
fn import_list_is_applied_after_everything_it_references() {
    let pos = positions();
    let import_list = pos("import_list");
    for target in ["root_folder", "quality_profile", "user"] {
        assert!(
            pos(target) < import_list,
            "`{target}` must be applied before `import_list`",
        );
    }
}

/// `Indexer.prowlarrInstanceId` addresses a linked Prowlarr server whose
/// indexers sync down into this table. Alphabetically `indexer` sorts *before*
/// `prowlarr_instance`, so this only holds via the edge.
#[test]
fn indexer_is_applied_after_prowlarr_instance() {
    let pos = positions();
    assert!(
        pos("prowlarr_instance") < pos("indexer"),
        "`prowlarr_instance` must be applied before `indexer`",
    );
}

/// The settings singleton's `library.defaultRootFolderId` points at a root
/// folder that has to exist first. `root_folder` sorts after `setting`
/// alphabetically, so this too rests entirely on the edge.
#[test]
fn setting_is_applied_after_root_folder() {
    let pos = positions();
    assert!(
        pos("root_folder") < pos("setting"),
        "`root_folder` must be applied before `setting`",
    );
}
