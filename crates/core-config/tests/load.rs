//! End-to-end: YAML config text → loaded instances.
//!
//! Proves file parse → `${env/file}` resolve → `type:` dispatch → connection
//! decode, with the desired resource state retained as the raw config `Value`
//! (apply is value-driven; the typed service holds connection only).

#[cfg(feature = "radarr-v3")]
use core_config::ServiceInstance;
use core_config::load_str;

const CONFIG: &str = r#"
home-radarr:
  type: radarr-v3
  url: http://localhost:7878
  api_key: secret123
  insecure: true
  tags:
    - label: 4k
    - label: hd
  media_management: {}
"#;

#[cfg(feature = "radarr-v3")]
#[test]
fn loads_a_radarr_instance() {
    let cfg = load_str(CONFIG).unwrap();
    assert_eq!(cfg.len(), 1);

    let inst = cfg.get("home-radarr").expect("instance present");
    assert_eq!(inst.service.type_name(), "radarr-v3");

    let ServiceInstance::RadarrV3(r) = &inst.service else {
        panic!("expected radarr-v3 instance");
    };
    // connection scalars decode into the typed service
    assert_eq!(r.url, "http://localhost:7878");
    assert_eq!(r.api_key.expose(), "secret123");
    assert_eq!(r.insecure, Some(true));
    assert_eq!(r.timeout_secs, None); // omitted → None

    // resources are NOT decoded into the typed service (apply is value-driven)
    assert!(r.tags.is_empty());

    // desired resource state is retained as the raw config Value
    assert_eq!(inst.config["tags"][0]["label"], "4k");
    assert_eq!(inst.config["tags"][1]["label"], "hd");
    assert!(inst.config.get("media_management").is_some());
}

#[cfg(feature = "radarr-v3")]
#[test]
fn resolves_env_in_credentials() {
    // `${env.*}` resolves at load time; `${ref.*}` would be left for apply.
    unsafe { std::env::set_var("CFGTEST_RADARR_KEY", "from-env-123") };
    let yaml = "\
home-radarr:
  type: radarr-v3
  url: http://localhost:7878
  api_key: ${env.CFGTEST_RADARR_KEY}
";
    let cfg = load_str(yaml).unwrap();
    let inst = cfg.get("home-radarr").unwrap();
    let ServiceInstance::RadarrV3(r) = &inst.service else {
        panic!("expected radarr-v3 instance");
    };
    assert_eq!(r.api_key.expose(), "from-env-123");
}

#[test]
fn missing_env_errors() {
    let yaml = "\
x:
  type: radarr-v3
  url: http://h
  api_key: ${env.CFGTEST_DEFINITELY_UNSET}
";
    let err = load_str(yaml).err().unwrap();
    assert!(format!("{err:#}").contains("CFGTEST_DEFINITELY_UNSET"));
}

#[test]
fn empty_config_is_empty_map() {
    let cfg = load_str("{}").unwrap();
    assert!(cfg.is_empty());
}

#[test]
fn unknown_type_errors() {
    let bad = "x:\n  type: bogus-v9\n  url: http://h\n";
    let err = load_str(bad).err().unwrap();
    assert!(err.to_string().contains("instance `x`"));
}

#[test]
fn missing_type_errors() {
    let bad = "x:\n  url: http://h\n";
    let err = load_str(bad).err().unwrap();
    assert!(format!("{err:#}").contains("missing `type`"));
}
