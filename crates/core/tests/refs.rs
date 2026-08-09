//! `RefExpr` — the dependency-graph node parsed from `${ref.<type>.<key>}`.

use std::collections::HashSet;

use core_lib::RefExpr;

#[test]
fn ref_expr_equality_and_hashing() {
    let a = RefExpr {
        target_type: "tag".into(),
        key: "4k".into(),
    };
    let b = RefExpr {
        target_type: "tag".into(),
        key: "4k".into(),
    };
    let c = RefExpr {
        target_type: "tag".into(),
        key: "hd".into(),
    };

    assert_eq!(a, b);

    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b); // duplicate — collapses
    set.insert(c);
    assert_eq!(set.len(), 2);
}
