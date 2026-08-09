//! Executor integration tests against a real in-process HTTP server (wiremock).
//!
//! A fake service + resources exercise the executor's mechanics — the engine is
//! generic over `Service`, so this is the same code path radarr uses. Desired
//! state is value-driven (the resolved config `Value`); we assert both the
//! responses driving the executor and the requests it emits (method, path, body,
//! auth header), including `${ref}` resolution + topo ordering.

use core_lib::SecretValue;
use core_lib::apply::{ApplyOptions, Report, apply, plan};
use core_macros::{resource, service};
use serde_json::{Value, json};
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[resource(
    sync = crud,
    list = get("/api/v3/server"),
    create = post("/api/v3/server"),
    update = put("/api/v3/server/${self.id}"),
    delete = delete("/api/v3/server/${self.id}"),
)]
pub struct Server {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    pub port: Option<i32>,
}

/// References `server` by name — applied after servers, with `${ref}` resolved.
#[resource(
    sync = crud,
    list = get("/api/v3/pool"),
    create = post("/api/v3/pool"),
    update = put("/api/v3/pool/${self.id}"),
    delete = delete("/api/v3/pool/${self.id}"),
)]
pub struct Pool {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    #[reference(server)]
    pub servers: Vec<i32>,
}

/// A singleton — no key, GET read + PUT update, many non-Option fields.
#[resource(
    sync = singleton,
    read = get("/api/v3/config/ui"),
    update = put("/api/v3/config/ui"),
)]
pub struct UiCfg {
    #[id]
    pub id: Option<i32>,
    pub first_day_of_week: i32,
    pub theme: Option<String>,
}

#[service(name = "lab", auth = api_key(header = "X-Api-Key"))]
pub struct Lab {
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub servers: Vec<Server>,
    pub pools: Vec<Pool>,
    pub ui: Option<UiCfg>,
}

/// The typed service holds only connection + descriptor; resource collections
/// stay empty (desired comes from the instance `Value`).
fn lab(url: String) -> Lab {
    Lab {
        url,
        api_key: SecretValue::new("test-key".into()),
        servers: vec![],
        pools: vec![],
        ui: None,
    }
}

async fn empty_get(mock: &MockServer, p: &str) {
    Mock::given(method("GET"))
        .and(path(p))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(mock)
        .await;
}

#[tokio::test]
async fn creates_missing_resource() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/server"))
        .and(header("X-Api-Key", "test-key")) // auth header flows
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v3/server"))
        .and(body_json(json!({ "name": "main", "port": 8080 }))) // id read-only → absent
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 1, "name": "main" })))
        .expect(1)
        .mount(&mock)
        .await;

    let instance = json!({ "servers": [ { "name": "main", "port": 8080 } ] });
    let report = apply(&lab(mock.uri()), &instance, ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            created: 1,
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn updates_changed_resource_with_merged_body() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/server"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": 3, "name": "main", "port": 8080, "serverOnly": "keep" }
        ])))
        .mount(&mock)
        .await;
    Mock::given(method("PUT"))
        .and(path("/api/v3/server/3"))
        .and(body_json(
            json!({ "id": 3, "name": "main", "port": 9090, "serverOnly": "keep" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&mock)
        .await;

    let instance = json!({ "servers": [ { "name": "main", "port": 9090 } ] });
    let report = apply(&lab(mock.uri()), &instance, ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            updated: 1,
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn unchanged_resource_sends_nothing() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/server"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": 3, "name": "main", "port": 8080 }
        ])))
        .mount(&mock)
        .await;

    let instance = json!({ "servers": [ { "name": "main", "port": 8080 } ] });
    let report = apply(&lab(mock.uri()), &instance, ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn prune_deletes_live_only_resource() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/server"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": 9, "name": "stale", "port": 1 }
        ])))
        .mount(&mock)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/api/v3/server/9"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock)
        .await;

    let opts = ApplyOptions { prune: true };
    let report = apply(&lab(mock.uri()), &json!({ "servers": [] }), opts)
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            deleted: 1,
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn resolves_ref_to_created_resource_id() {
    let mock = MockServer::start().await;
    // server "main" doesn't exist → created, server assigns id 1.
    empty_get(&mock, "/api/v3/server").await;
    Mock::given(method("POST"))
        .and(path("/api/v3/server"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 1, "name": "main" })))
        .expect(1)
        .mount(&mock)
        .await;
    // pool references server "main" → body must carry the resolved id [1].
    empty_get(&mock, "/api/v3/pool").await;
    Mock::given(method("POST"))
        .and(path("/api/v3/pool"))
        .and(body_json(json!({ "name": "p1", "servers": [1] })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 5 })))
        .expect(1)
        .mount(&mock)
        .await;

    let instance = json!({
        "servers": [ { "name": "main", "port": 8080 } ],
        "pools":   [ { "name": "p1", "servers": ["${ref.server.main}"] } ]
    });
    let report = apply(&lab(mock.uri()), &instance, ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            created: 2,
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn singleton_empty_config_is_noop() {
    let mock = MockServer::start().await;
    // live singleton has a non-default theme + day. Empty config must NOT clobber.
    Mock::given(method("GET"))
        .and(path("/api/v3/config/ui"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(
            { "id": 1, "firstDayOfWeek": 3, "theme": "dark" }
        )))
        .mount(&mock)
        .await;
    // No PUT mounted: a clobbering write would 404.

    let instance = json!({ "ui": {} });
    let report = apply(&lab(mock.uri()), &instance, ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            unchanged: 1,
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn singleton_updates_only_written_field() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/config/ui"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(
            { "id": 1, "firstDayOfWeek": 0, "theme": "dark" }
        )))
        .mount(&mock)
        .await;
    // PUT body: only firstDayOfWeek changes; id + theme (unwritten) preserved.
    Mock::given(method("PUT"))
        .and(path("/api/v3/config/ui"))
        .and(body_json(
            json!({ "id": 1, "firstDayOfWeek": 1, "theme": "dark" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&mock)
        .await;

    let instance = json!({ "ui": { "first_day_of_week": 1 } });
    let report = apply(&lab(mock.uri()), &instance, ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            updated: 1,
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn plan_previews_without_sending() {
    let mock = MockServer::start().await;
    empty_get(&mock, "/api/v3/server").await;
    // No POST mounted → a real create would 404. plan() must not send.

    let instance: Value = json!({ "servers": [ { "name": "main", "port": 8080 } ] });
    let p = plan(&lab(mock.uri()), &instance, ApplyOptions::default())
        .await
        .unwrap();
    // The plan previews the create and summarises it, but issues no write.
    assert_eq!(
        p.summary(),
        Report {
            created: 1,
            ..Default::default()
        }
    );
    assert!(!p.is_empty());
}

// ── Basic auth + PATCH (the safe seam additions) ─────────────────────────────

/// A service using HTTP Basic auth.
#[service(name = "basic_lab", auth = basic)]
pub struct BasicLab {
    pub url: String,
    #[credential(user)]
    pub user: String,
    #[credential(pass)]
    pub pass: SecretValue,
    pub servers: Vec<Server>,
}

#[tokio::test]
async fn basic_auth_sends_authorization_header() {
    let mock = MockServer::start().await;
    // base64("admin:secret") = "YWRtaW46c2VjcmV0"; a wrong header → 404 → apply errors.
    Mock::given(method("GET"))
        .and(path("/api/v3/server"))
        .and(header("Authorization", "Basic YWRtaW46c2VjcmV0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock)
        .await;

    let svc = BasicLab {
        url: mock.uri(),
        user: "admin".into(),
        pass: SecretValue::new("secret".into()),
        servers: vec![],
    };
    let report = apply(&svc, &json!({ "servers": [] }), ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(report, Report::default());
}

/// A singleton whose update endpoint is PATCH (not PUT).
#[resource(
    sync = singleton,
    read = get("/api/v3/pc"),
    update = patch("/api/v3/pc/${self.id}"),
)]
pub struct PatchCfg {
    #[id]
    pub id: Option<i32>,
    pub value: Option<String>,
}

#[service(name = "patch_lab", auth = none)]
pub struct PatchLab {
    pub url: String,
    pub pc: Option<PatchCfg>,
}

#[tokio::test]
async fn singleton_update_uses_patch() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/pc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": 7, "value": "old" })))
        .mount(&mock)
        .await;
    // The write must go out as PATCH /api/v3/pc/7 with the merged body.
    Mock::given(method("PATCH"))
        .and(path("/api/v3/pc/7"))
        .and(body_json(json!({ "id": 7, "value": "new" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&mock)
        .await;

    let svc = PatchLab {
        url: mock.uri(),
        pc: None,
    };
    let instance = json!({ "pc": { "value": "new" } });
    let report = apply(&svc, &instance, ApplyOptions::default())
        .await
        .unwrap();
    assert_eq!(
        report,
        Report {
            updated: 1,
            ..Default::default()
        }
    );
}
