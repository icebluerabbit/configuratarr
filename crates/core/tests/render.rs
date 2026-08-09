//! Plan rendering + secret redaction. Builds `Plan`s directly (no HTTP) and
//! asserts the human output, including the two credential guards:
//! descriptor-marked secret keys → `<redacted>`, and complex values (where the
//! provider `fields` blob lives) → never dumped.

use core_lib::plan::{DisplayValue, FieldChange, Op, Plan, PlanStep};
use core_lib::{Endpoint, HttpMethod};
use serde_json::json;

fn ep() -> Endpoint {
    Endpoint {
        method: HttpMethod::Post,
        path: "/x",
    }
}

#[test]
fn renders_resource_and_field_level() {
    let plan = Plan {
        steps: vec![PlanStep {
            type_name: "tag",
            ops: vec![
                Op::Create {
                    key: "4k".into(),
                    endpoint: ep(),
                    body: json!({ "label": "4k", "priority": 1 }),
                },
                Op::Update {
                    key: "hd".into(),
                    endpoint: ep(),
                    path: "/x/2".into(),
                    body: json!({}),
                    changes: vec![FieldChange {
                        key: "priority".into(),
                        from: Some(json!(1)),
                        to: json!(2),
                    }],
                },
                Op::Delete {
                    key: "old".into(),
                    endpoint: ep(),
                    path: "/x/3".into(),
                },
                Op::Noop { key: "keep".into() },
            ],
            secret_keys: vec![],
        }],
    };
    let out = plan.render();
    assert!(out.contains("+ create 4k"), "{out}");
    assert!(out.contains("label: \"4k\""), "{out}");
    assert!(out.contains("~ update hd"), "{out}");
    assert!(out.contains("priority: 1 -> 2"), "{out}");
    assert!(out.contains("- delete old"), "{out}");
    assert!(out.contains("1 unchanged"), "{out}");

    let s = plan.summary();
    assert_eq!((s.created, s.updated, s.deleted, s.unchanged), (1, 1, 1, 1));
    assert!(!plan.is_empty());
}

#[test]
fn redacts_secret_and_complex_values() {
    let plan = Plan {
        steps: vec![PlanStep {
            type_name: "download_client",
            ops: vec![Op::Create {
                key: "qbit".into(),
                endpoint: ep(),
                body: json!({
                    "name": "qbit",
                    "password": "hunter2",
                    "fields": [ { "name": "apiKey", "value": "s3cr3t" } ]
                }),
            }],
            secret_keys: vec!["password".into()],
        }],
    };
    let out = plan.render();
    assert!(out.contains("password: <redacted>"), "{out}");
    assert!(!out.contains("hunter2"), "secret value leaked: {out}");
    // The provider `fields` blob is a complex value — summarised, never dumped,
    // so credentials inside it (`s3cr3t`) cannot leak.
    assert!(out.contains("fields: (…)"), "{out}");
    assert!(!out.contains("s3cr3t"), "blob credential leaked: {out}");
}

#[test]
fn display_accessors_are_the_redaction_boundary() {
    let secret = vec!["password".to_string()];

    // Create: secret key redacted, complex value summarised, scalar shown — all
    // through the accessor, the only path a view gets values.
    let create = Op::Create {
        key: "qbit".into(),
        endpoint: ep(),
        body: json!({ "name": "qbit", "password": "hunter2", "fields": [1, 2] }),
    };
    let rows = create.created_fields(&secret);
    let get = |k: &str| {
        rows.iter()
            .find(|(key, _)| key == k)
            .map(|(_, v)| v.clone())
    };
    assert_eq!(get("name"), Some(DisplayValue::Scalar("\"qbit\"".into())));
    assert_eq!(get("password"), Some(DisplayValue::Redacted));
    assert_eq!(get("fields"), Some(DisplayValue::Complex));

    // Update: an absent `from` surfaces as Absent; both sides redact secrets.
    let update = Op::Update {
        key: "qbit".into(),
        endpoint: ep(),
        path: "/x/1".into(),
        body: json!({}),
        changes: vec![
            FieldChange {
                key: "port".into(),
                from: None,
                to: json!(8080),
            },
            FieldChange {
                key: "password".into(),
                from: Some(json!("old")),
                to: json!("new"),
            },
        ],
    };
    let diffs = update.changed_fields(&secret);
    assert_eq!(diffs[0].from, DisplayValue::Absent);
    assert_eq!(diffs[0].to, DisplayValue::Scalar("8080".into()));
    assert_eq!(diffs[1].from, DisplayValue::Redacted);
    assert_eq!(diffs[1].to, DisplayValue::Redacted);

    // Other op kinds expose no fields.
    assert!(
        Op::Noop { key: "x".into() }
            .created_fields(&secret)
            .is_empty()
    );
    assert!(
        Op::Delete {
            key: "x".into(),
            endpoint: ep(),
            path: "/x/1".into()
        }
        .changed_fields(&secret)
        .is_empty()
    );
}

#[test]
fn all_noop_plan_is_empty() {
    let plan = Plan {
        steps: vec![PlanStep {
            type_name: "tag",
            ops: vec![Op::Noop { key: "a".into() }],
            secret_keys: vec![],
        }],
    };
    assert!(plan.is_empty());
    assert!(plan.render().contains("1 unchanged"));
}
