//! Config → wire encoding for the download client: the irregular enum spellings
//! and the `config_only` nested concerns that must stay out of the client's own
//! wire body.

use cleanuparr_v1::resources::download_client::{DownloadClient, DownloadClientTypeName};
use core_lib::engine;
use serde_json::json;

/// The three irregular `DownloadClientTypeName` variants must render
/// lower-camel on the wire (`#[variant(...)]` override), the two regular
/// ones stay PascalCase (derived, no override needed).
#[test]
fn type_name_wire_values_match_the_spec_enum() {
    let cases = [
        (DownloadClientTypeName::QBittorrent, "qBittorrent"),
        (DownloadClientTypeName::Deluge, "Deluge"),
        (DownloadClientTypeName::Transmission, "Transmission"),
        (DownloadClientTypeName::UTorrent, "uTorrent"),
        (DownloadClientTypeName::RTorrent, "rTorrent"),
    ];
    for (variant, wire) in cases {
        assert_eq!(
            engine::encode(&variant).unwrap(),
            json!(wire),
            "wire = {wire:?}"
        );
    }
}

/// The client's own wire body must carry only the `CreateDownloadClientRequest`
/// keys — the four nested concerns are `#[wire(config_only)]` and must never
/// leak into it, or conformance (`additionalProperties: false`) would fail.
#[test]
fn nested_concerns_are_excluded_from_the_client_wire_body() {
    let cfg = json!({
        "name": "qbit",
        "enabled": true,
        "type_name": "qBittorrent",
        "protocol": "Torrent",
        "host": "http://qbittorrent:8080",
        "seeding_rules": [
            { "name": "r1", "categories": ["sonarr"] }
        ],
        "unlinked_config": { "target_category": "x", "categories": ["sonarr"] },
        "dead_torrent_config": { "target_category": "y", "categories": ["sonarr"] },
        "orphaned_files_config": { "orphaned_directory": "/tmp/orphaned" },
    });
    let wire = engine::encode_config::<DownloadClient>(&cfg).unwrap();
    let obj = wire.as_object().unwrap();
    for leaked in [
        "seedingRules",
        "seeding_rules",
        "unlinkedConfig",
        "unlinked_config",
        "deadTorrentConfig",
        "dead_torrent_config",
        "orphanedFilesConfig",
        "orphaned_files_config",
    ] {
        assert!(
            !obj.contains_key(leaked),
            "nested key `{leaked}` must not appear in the client's own wire body: {wire}"
        );
    }
    assert_eq!(obj.get("name"), Some(&json!("qbit")));
    assert_eq!(obj.get("typeName"), Some(&json!("qBittorrent")));
    assert_eq!(obj.get("type"), Some(&json!("Torrent")));
}
