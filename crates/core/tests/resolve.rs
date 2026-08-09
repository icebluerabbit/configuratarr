//! Value-tree interpolation: static (env/file) + ref phases.

use std::collections::HashMap;

use core_lib::resolve;
use core_lib::resolver::{RefSource, StaticEnv};
use serde_json::json;

struct MapEnv {
    env: HashMap<String, String>,
    files: HashMap<String, String>,
}
impl StaticEnv for MapEnv {
    fn env(&self, name: &str) -> Option<&str> {
        self.env.get(name).map(String::as_str)
    }
    fn file(&self, path: &str) -> anyhow::Result<String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("no such file: {path}"))
    }
}

struct MapRefs(HashMap<(String, String), i32>);
impl RefSource for MapRefs {
    fn lookup(&self, ty: &str, key: &str) -> Option<core_lib::RefId> {
        self.0
            .get(&(ty.to_string(), key.to_string()))
            .map(|&id| core_lib::RefId::Int(id.into()))
    }
}

/// A [`RefSource`] with GUID/string ids (Jellyfin-style).
struct StrRefs(HashMap<(String, String), String>);
impl RefSource for StrRefs {
    fn lookup(&self, ty: &str, key: &str) -> Option<core_lib::RefId> {
        self.0
            .get(&(ty.to_string(), key.to_string()))
            .map(|s| core_lib::RefId::Str(s.clone()))
    }
}

fn env_with(pairs: &[(&str, &str)]) -> MapEnv {
    MapEnv {
        env: pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        files: HashMap::new(),
    }
}

#[test]
fn static_whole_value_env() {
    let mut v = json!({ "host": "${env.QBIT_HOST}" });
    resolve::resolve_static(&mut v, &env_with(&[("QBIT_HOST", "qbit.local")])).unwrap();
    assert_eq!(v, json!({ "host": "qbit.local" }));
}

#[test]
fn static_embedded_env() {
    let mut v = json!({ "url": "http://${env.HOST}:${env.PORT}/api" });
    let env = env_with(&[("HOST", "qbit"), ("PORT", "8080")]);
    resolve::resolve_static(&mut v, &env).unwrap();
    assert_eq!(v, json!({ "url": "http://qbit:8080/api" }));
}

#[test]
fn static_leaves_refs_untouched() {
    let mut v = json!({ "tags": ["${ref.tag.4k}"], "host": "${env.H}" });
    resolve::resolve_static(&mut v, &env_with(&[("H", "x")])).unwrap();
    assert_eq!(v, json!({ "tags": ["${ref.tag.4k}"], "host": "x" }));
}

#[test]
fn static_missing_env_errors() {
    let mut v = json!({ "host": "${env.NOPE}" });
    assert!(resolve::resolve_static(&mut v, &env_with(&[])).is_err());
}

#[test]
fn static_file() {
    let mut v = json!({ "api_key": "${file./run/secrets/key}" });
    let env = MapEnv {
        env: HashMap::new(),
        files: [("/run/secrets/key".to_string(), "s3cr3t".to_string())].into(),
    };
    resolve::resolve_static(&mut v, &env).unwrap();
    assert_eq!(v, json!({ "api_key": "s3cr3t" }));
}

#[test]
fn collect_refs_finds_all() {
    let v = json!({
        "tags": ["${ref.tag.4k}", "${ref.tag.hd}"],
        "download_client": "${ref.download_client.qbit}",
        "host": "${env.H}",
    });
    let mut refs = resolve::collect_refs(&v);
    refs.sort_by(|a, b| (a.target_type.as_str(), a.key.as_str()).cmp(&(&b.target_type, &b.key)));
    assert_eq!(refs.len(), 3);
    assert_eq!(refs[0].target_type, "download_client");
    assert_eq!(refs[0].key, "qbit");
    assert_eq!(refs[1].key, "4k");
    assert_eq!(refs[2].key, "hd");
}

#[test]
fn resolve_refs_whole_value_becomes_number() {
    let mut v = json!({ "tags": ["${ref.tag.4k}", "${ref.tag.hd}"] });
    let refs = MapRefs(
        [
            (("tag".to_string(), "4k".to_string()), 3),
            (("tag".to_string(), "hd".to_string()), 5),
        ]
        .into(),
    );
    resolve::resolve_refs(&mut v, &refs).unwrap();
    // whole-value refs resolve to JSON numbers (decode into i32).
    assert_eq!(v, json!({ "tags": [3, 5] }));
}

#[test]
fn resolve_refs_string_id_becomes_string() {
    // GUID/string-id APIs (Jellyfin): a whole-value ref resolves to a JSON
    // string, keeping the FK's native wire type.
    let mut v = json!({ "userId": "${ref.user.alice}" });
    let refs = StrRefs(
        [(
            ("user".to_string(), "alice".to_string()),
            "a1b2c3d4-e5f6".to_string(),
        )]
        .into(),
    );
    resolve::resolve_refs(&mut v, &refs).unwrap();
    assert_eq!(v, json!({ "userId": "a1b2c3d4-e5f6" }));
}

#[test]
fn resolve_refs_missing_errors() {
    let mut v = json!({ "x": "${ref.tag.ghost}" });
    assert!(resolve::resolve_refs(&mut v, &MapRefs(HashMap::new())).is_err());
}
