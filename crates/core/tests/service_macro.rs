//! Integration tests for the `#[service]` proc-macro.
//!
//! Exercises: name/doc/health capture, Collection/Singleton field kinds and
//! declaration order, the `iter` function on both variants, and all three auth
//! schemes (ApiKey, None, Form) including with/without the optional
//! insecure/timeout_secs fields.

use core_lib::{Auth, SecretValue, Service, SyncKind};
use core_macros::{resource, service};

// ── resources used as service fields ─────────────────────────────────────────

/// A collection-style resource.
#[resource(
    sync = crud,
    list = get("/api/v3/col"),
    create = post("/api/v3/col"),
    update = put("/api/v3/col/${self.id}"),
    delete = delete("/api/v3/col/${self.id}"),
)]
pub struct ColResource {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
}

/// A singleton-style resource.
#[resource(
    sync = singleton,
    read = get("/api/v3/sing"),
    update = put("/api/v3/sing"),
)]
pub struct SingResource {
    pub id: Option<i32>,
    pub value: String,
}

// ── full service (api_key auth, insecure + timeout_secs, doc, health) ─────────

/// Manages collections and singletons against a live API.
#[service(
    name = "full_service",
    health = "/api/v3/system/status",
    auth = api_key(header = "X-Api-Key"),
)]
pub struct FullService {
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,
    /// The list of collection resources.
    pub collections: Vec<ColResource>,
    /// The optional singleton resource.
    pub singleton: Option<SingResource>,
}

#[test]
fn descriptor_name_and_doc() {
    let d = FullService::descriptor();
    assert_eq!(d.name, "full_service");
    assert_eq!(
        d.doc,
        Some("Manages collections and singletons against a live API.")
    );
}

#[test]
fn descriptor_health_check() {
    assert_eq!(
        FullService::descriptor().health_check,
        Some("/api/v3/system/status")
    );
}

#[test]
fn descriptor_fields_count_and_order() {
    let d = FullService::descriptor();
    // url/api_key/insecure/timeout_secs are consumed by the macro — not in fields.
    assert_eq!(d.fields.len(), 2);
    assert_eq!(d.fields[0].name, "collections");
    assert_eq!(d.fields[1].name, "singleton");
}

#[test]
fn descriptor_field_kinds() {
    let d = FullService::descriptor();
    // Vec<ColResource>: type_name is snake_case("ColResource"); the dispatch
    // strategy is the resource's own declared `sync` (crud), not the Vec shape.
    assert_eq!(d.fields[0].type_name, "col_resource");
    assert!(matches!((d.fields[0].sync)(), SyncKind::Crud));
    // Option<SingResource> → snake_case("SingResource"), sync = singleton.
    assert_eq!(d.fields[1].type_name, "sing_resource");
    assert!(matches!((d.fields[1].sync)(), SyncKind::Singleton));
}

#[test]
fn descriptor_field_doc_captured() {
    let d = FullService::descriptor();
    assert_eq!(d.fields[0].doc, Some("The list of collection resources."));
    assert_eq!(d.fields[1].doc, Some("The optional singleton resource."));
}

#[test]
fn service_field_iter_collection() {
    let svc = FullService {
        url: "http://localhost:7878".to_string(),
        api_key: SecretValue::new("test-key".to_string()),
        insecure: None,
        timeout_secs: None,
        collections: vec![
            ColResource {
                id: Some(1),
                name: "alpha".to_string(),
            },
            ColResource {
                id: Some(2),
                name: "beta".to_string(),
            },
        ],
        singleton: None,
    };
    let iter_fn = FullService::descriptor().fields[0].iter;
    let items: Vec<_> = iter_fn(&svc).collect();
    assert_eq!(items.len(), 2);
}

#[test]
fn service_field_iter_singleton_some() {
    let svc = FullService {
        url: "http://localhost:7878".to_string(),
        api_key: SecretValue::new("test-key".to_string()),
        insecure: None,
        timeout_secs: None,
        collections: vec![],
        singleton: Some(SingResource {
            id: Some(1),
            value: "x".to_string(),
        }),
    };
    let iter_fn = FullService::descriptor().fields[1].iter;
    let items: Vec<_> = iter_fn(&svc).collect();
    assert_eq!(items.len(), 1);
}

#[test]
fn service_field_iter_singleton_none() {
    let svc = FullService {
        url: "http://localhost:7878".to_string(),
        api_key: SecretValue::new("test-key".to_string()),
        insecure: None,
        timeout_secs: None,
        collections: vec![],
        singleton: None,
    };
    let iter_fn = FullService::descriptor().fields[1].iter;
    let items: Vec<_> = iter_fn(&svc).collect();
    assert_eq!(items.len(), 0);
}

#[test]
fn connection_url() {
    let svc = FullService {
        url: "http://localhost:7878".to_string(),
        api_key: SecretValue::new("test-key".to_string()),
        insecure: None,
        timeout_secs: None,
        collections: vec![],
        singleton: None,
    };
    let conn = svc.connection();
    assert_eq!(conn.url, "http://localhost:7878");
}

#[test]
fn connection_api_key_auth() {
    let svc = FullService {
        url: "http://localhost:7878".to_string(),
        api_key: SecretValue::new("test-key".to_string()),
        insecure: None,
        timeout_secs: None,
        collections: vec![],
        singleton: None,
    };
    let conn = svc.connection();
    match conn.auth {
        Auth::ApiKey { header, key } => {
            assert_eq!(header, "X-Api-Key");
            assert_eq!(key.expose(), "test-key");
        }
        _ => panic!("expected Auth::ApiKey"),
    }
}

#[test]
fn connection_insecure_and_timeout_forwarded() {
    let svc = FullService {
        url: "http://localhost:7878".to_string(),
        api_key: SecretValue::new("test-key".to_string()),
        insecure: Some(true),
        timeout_secs: Some(30),
        collections: vec![],
        singleton: None,
    };
    let conn = svc.connection();
    assert_eq!(conn.insecure, Some(true));
    assert_eq!(conn.timeout_secs, Some(30));
}

// ── minimal service (no insecure / timeout_secs fields) ───────────────────────

#[service(name = "minimal_service", auth = api_key(header = "X-Api-Key"))]
pub struct MinimalService {
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
}

#[test]
fn connection_insecure_none_when_field_absent() {
    let svc = MinimalService {
        url: "http://localhost:7878".to_string(),
        api_key: SecretValue::new("test-key".to_string()),
    };
    let conn = svc.connection();
    assert_eq!(conn.insecure, None);
    assert_eq!(conn.timeout_secs, None);
}

#[test]
fn minimal_service_has_no_resource_fields() {
    assert_eq!(MinimalService::descriptor().fields.len(), 0);
}

#[test]
fn minimal_service_health_check_none_when_omitted() {
    assert_eq!(MinimalService::descriptor().health_check, None);
}

// ── auth = "none" ─────────────────────────────────────────────────────────────

#[service(name = "anon_service", auth = none)]
pub struct AnonService {
    pub url: String,
}

#[test]
fn connection_auth_none() {
    let svc = AnonService {
        url: "http://localhost:9000".to_string(),
    };
    let conn = svc.connection();
    assert!(matches!(conn.auth, Auth::None));
}

// ── auth = api_key(query = …) ─────────────────────────────────────────────────

#[service(
    name = "query_key_service",
    health = "/api?cmd=getVersion",
    auth = api_key(query = "apikey"),
)]
pub struct QueryKeyService {
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
}

#[test]
fn connection_api_key_query_auth() {
    let svc = QueryKeyService {
        url: "http://localhost:5299".to_string(),
        api_key: SecretValue::new("deadbeef".to_string()),
    };
    let conn = svc.connection();
    match conn.auth {
        Auth::ApiKeyQuery { param, key } => {
            assert_eq!(param, "apikey");
            assert_eq!(key.expose(), "deadbeef");
        }
        _ => panic!("expected Auth::ApiKeyQuery"),
    }
}

// ── auth = form_cookie ────────────────────────────────────────────────────────

#[service(
    name = "form_service",
    health = "/auth/ping",
    auth = form_cookie(login_path = "/auth/login"),
)]
pub struct FormService {
    pub url: String,
    #[credential(user)]
    pub user: String,
    #[credential(pass)]
    pub pass: SecretValue,
}

#[test]
fn connection_auth_form_login_path() {
    let svc = FormService {
        url: "http://localhost:9000".to_string(),
        user: "admin".to_string(),
        pass: SecretValue::new("test-pass".to_string()),
    };
    let conn = svc.connection();
    match conn.auth {
        Auth::FormCookie {
            login_path,
            user,
            pass,
        } => {
            assert_eq!(login_path, "/auth/login");
            assert_eq!(user, "admin");
            assert_eq!(pass.expose(), "test-pass");
        }
        _ => panic!("expected Auth::FormCookie"),
    }
}

#[test]
fn form_service_descriptor_name_and_health() {
    let d = FormService::descriptor();
    assert_eq!(d.name, "form_service");
    assert_eq!(d.health_check, Some("/auth/ping"));
    // No Vec<R>/Option<R> fields — the descriptor's fields slice is empty.
    assert_eq!(d.fields.len(), 0);
}
