//! `reconcile` — the create/update/prune skeletons a `sync = custom` hook builds on.
//!
//! Every helper here is gated on `execute`: with it off the change rows are still
//! reported (that is what `plan` renders) but nothing writes.

use std::cell::Cell;

use core_lib::Change;
use core_lib::reconcile::{
    create_only, create_only_prune, echo, prune_absent, replace, upsert, upsert_prune,
};
use serde_json::{Value, json};

#[tokio::test]
async fn create_only_skips_present_and_honours_execute() {
    let desired = vec![json!({ "app": "a" }), json!({ "app": "b" })];
    let present = vec!["a".to_string()];
    let writes = Cell::new(0);

    let changes = create_only(&desired, "app", &present, true, |_k, _c| {
        writes.set(writes.get() + 1);
        async { Ok(()) }
    })
    .await
    .unwrap();

    assert_eq!(writes.get(), 1); // only "b" created
    assert_eq!(changes[0], Change::unchanged("a"));
    assert_eq!(changes[1], Change::created("b"));
}

#[tokio::test]
async fn create_only_preview_writes_nothing() {
    let desired = vec![json!({ "app": "b" })];
    let writes = Cell::new(0);

    let changes = create_only(&desired, "app", &[], false, |_k, _c| {
        writes.set(writes.get() + 1);
        async { Ok(()) }
    })
    .await
    .unwrap();

    assert_eq!(writes.get(), 0);
    assert_eq!(changes, vec![Change::created("b")]);
}

#[tokio::test]
async fn upsert_creates_updates_and_leaves() {
    // Idempotency: declared subset already present in live (extra live `id`).
    let in_sync = |w: &Value, l: &Value| {
        w.as_object()
            .unwrap()
            .iter()
            .all(|(k, v)| l.get(k) == Some(v))
    };
    let desired = vec![
        json!({ "name": "keep", "port": 1 }),
        json!({ "name": "drift", "port": 2 }),
        json!({ "name": "fresh", "port": 3 }),
    ];
    let live = vec![
        json!({ "name": "keep", "port": 1, "id": 10 }),
        json!({ "name": "drift", "port": 99, "id": 11 }),
    ];
    let creates = Cell::new(0);
    let updates = Cell::new(0);
    let changes = upsert(
        &desired,
        &live,
        "name",
        in_sync,
        true,
        |_w| {
            creates.set(creates.get() + 1);
            async { Ok(()) }
        },
        |_l, _w| {
            updates.set(updates.get() + 1);
            async { Ok(()) }
        },
    )
    .await
    .unwrap();

    assert_eq!(creates.get(), 1);
    assert_eq!(updates.get(), 1);
    assert_eq!(
        changes,
        vec![
            Change::unchanged("keep"),
            Change::updated("drift"),
            Change::created("fresh"),
        ]
    );
}

#[tokio::test]
async fn upsert_preview_writes_nothing() {
    let desired = vec![json!({ "name": "fresh" }), json!({ "name": "drift" })];
    let live = vec![json!({ "name": "drift", "id": 1 })];
    let writes = Cell::new(0);
    let bump = || {
        writes.set(writes.get() + 1);
    };
    let changes = upsert(
        &desired,
        &live,
        "name",
        |_w, _l| false, // everything present is "drifted"
        false,
        |_w| {
            bump();
            async { Ok(()) }
        },
        |_l, _w| {
            bump();
            async { Ok(()) }
        },
    )
    .await
    .unwrap();
    assert_eq!(writes.get(), 0);
    assert_eq!(
        changes,
        vec![Change::created("fresh"), Change::updated("drift")]
    );
}

#[test]
fn echo_copies_live_field_into_wire() {
    let mut wire = json!({ "name": "x" });
    echo(&mut wire, "id", &json!({ "id": 7, "name": "x" }));
    assert_eq!(wire["id"], json!(7));
    // Absent source key → no-op.
    echo(&mut wire, "missing", &json!({ "id": 7 }));
    assert_eq!(wire.get("missing"), None);
}

#[tokio::test]
async fn replace_is_order_insensitive_and_gated() {
    let ident = |v: &Value| v.get("n").and_then(Value::as_i64).unwrap_or(0);
    let live = vec![json!({ "n": 1 }), json!({ "n": 2 })];

    // Same set, different order → unchanged, never writes.
    let same = vec![json!({ "n": 2 }), json!({ "n": 1 })];
    let wrote = Cell::new(false);
    let c = replace(&same, &live, "repos", true, ident, || {
        wrote.set(true);
        async { Ok(()) }
    })
    .await
    .unwrap();
    assert_eq!(c, vec![Change::unchanged("repos")]);
    assert!(!wrote.get());

    // Different set but preview → updated, still no write.
    let diff = vec![json!({ "n": 3 })];
    let wrote = Cell::new(false);
    let c = replace(&diff, &live, "repos", false, ident, || {
        wrote.set(true);
        async { Ok(()) }
    })
    .await
    .unwrap();
    assert_eq!(c, vec![Change::updated("repos")]);
    assert!(!wrote.get());
}

#[tokio::test]
async fn prune_absent_deletes_only_undeclared_and_gates() {
    let desired = vec![json!({ "name": "keep" })];
    let live = vec![
        json!({ "name": "keep", "id": 1 }),
        json!({ "name": "gone", "id": 2 }),
    ];

    // prune off → no delete, no change rows.
    let deletes = Cell::new(0);
    let c = prune_absent(&desired, &live, "name", false, true, |_l| {
        deletes.set(deletes.get() + 1);
        async { Ok(()) }
    })
    .await
    .unwrap();
    assert_eq!(deletes.get(), 0);
    assert!(c.is_empty());

    // prune on → the undeclared live item is removed.
    let deletes = Cell::new(0);
    let c = prune_absent(&desired, &live, "name", true, true, |_l| {
        deletes.set(deletes.get() + 1);
        async { Ok(()) }
    })
    .await
    .unwrap();
    assert_eq!(deletes.get(), 1);
    assert_eq!(c, vec![Change::removed("gone")]);

    // prune on but preview → reports the removal, writes nothing.
    let deletes = Cell::new(0);
    let c = prune_absent(&desired, &live, "name", true, false, |_l| {
        deletes.set(deletes.get() + 1);
        async { Ok(()) }
    })
    .await
    .unwrap();
    assert_eq!(deletes.get(), 0);
    assert_eq!(c, vec![Change::removed("gone")]);
}

#[tokio::test]
async fn upsert_prune_creates_updates_and_prunes() {
    let in_sync = |w: &Value, l: &Value| {
        w.as_object()
            .unwrap()
            .iter()
            .all(|(k, v)| l.get(k) == Some(v))
    };
    let desired = vec![
        json!({ "name": "keep", "port": 1 }),
        json!({ "name": "drift", "port": 5 }),
        json!({ "name": "fresh", "port": 3 }),
    ];
    let live = vec![
        json!({ "name": "keep", "port": 1, "id": 10 }),
        json!({ "name": "drift", "port": 2, "id": 11 }),
        json!({ "name": "orphan", "port": 9, "id": 12 }),
    ];
    let updates = Cell::new(0);
    let deletes = Cell::new(0);
    let changes = upsert_prune(
        &desired,
        &live,
        "name",
        in_sync,
        true,
        true,
        |_w| async { Ok(()) },
        |_l, _w| {
            updates.set(updates.get() + 1);
            async { Ok(()) }
        },
        |_l| {
            deletes.set(deletes.get() + 1);
            async { Ok(()) }
        },
    )
    .await
    .unwrap();
    assert_eq!(updates.get(), 1, "only the drifted 'drift' is updated");
    assert_eq!(deletes.get(), 1, "only undeclared 'orphan' is pruned");
    assert_eq!(
        changes,
        vec![
            Change::unchanged("keep"),
            Change::updated("drift"),
            Change::created("fresh"),
            Change::removed("orphan"),
        ]
    );
}

/// `prune = false` must leave every undeclared live item alone: no delete
/// fires and no `Removed` change is reported (the write half still runs).
#[tokio::test]
async fn upsert_prune_prune_false_leaves_undeclared() {
    let in_sync = |w: &Value, l: &Value| {
        w.as_object()
            .unwrap()
            .iter()
            .all(|(k, v)| l.get(k) == Some(v))
    };
    let desired = vec![json!({ "name": "keep", "port": 1 })];
    let live = vec![
        json!({ "name": "keep", "port": 1, "id": 10 }),
        json!({ "name": "orphan", "port": 9, "id": 11 }),
    ];
    let deletes = Cell::new(0);
    let changes = upsert_prune(
        &desired,
        &live,
        "name",
        in_sync,
        false, // prune off
        true,
        |_w| async { Ok(()) },
        |_l, _w| async { Ok(()) },
        |_l| {
            deletes.set(deletes.get() + 1);
            async { Ok(()) }
        },
    )
    .await
    .unwrap();
    assert_eq!(deletes.get(), 0, "prune off → no delete");
    assert_eq!(
        changes,
        vec![Change::unchanged("keep")],
        "prune off → no Removed row"
    );
}

#[tokio::test]
async fn create_only_prune_leaves_creates_and_prunes() {
    let desired = vec![json!({ "name": "a" }), json!({ "name": "b" })];
    let live = vec![
        json!({ "name": "a", "id": 1 }),
        json!({ "name": "stale", "id": 2 }),
    ];
    let creates = Cell::new(0);
    let deletes = Cell::new(0);
    let changes = create_only_prune(
        &desired,
        &live,
        "name",
        true,
        true,
        |_k, _c| {
            creates.set(creates.get() + 1);
            async { Ok(()) }
        },
        |_l| {
            deletes.set(deletes.get() + 1);
            async { Ok(()) }
        },
    )
    .await
    .unwrap();
    assert_eq!(creates.get(), 1, "only the absent 'b' is created");
    assert_eq!(deletes.get(), 1, "only undeclared 'stale' is pruned");
    assert_eq!(
        changes,
        vec![
            Change::unchanged("a"),
            Change::created("b"),
            Change::removed("stale"),
        ]
    );
}
