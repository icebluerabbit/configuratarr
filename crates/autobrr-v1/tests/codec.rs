//! Config → wire encoding for the autobrr resources whose shape the spec check
//! can't pin down: a renamed key, a fallback enum's variant set, and a nested
//! field default that must always be sent.

use autobrr_v1::resources::download_client_auth::DownloadClientAuth;
use autobrr_v1::resources::external_filter::ExternalFilter;
use autobrr_v1::resources::irc_network::IrcNetwork;
use autobrr_v1::resources::notification_event::NotificationEvent;
use core_lib::engine;
use serde_json::json;

/// `auth_type` is renamed to the wire key `type`; the other fields pass
/// through under their own names.
#[test]
fn auth_type_maps_to_wire_type() {
    let cfg = json!({ "enabled": true, "auth_type": "DIGEST_AUTH", "username": "u" });
    let wire = engine::encode(&engine::decode_config::<DownloadClientAuth>(&cfg).unwrap()).unwrap();
    assert_eq!(wire["type"], json!("DIGEST_AUTH"));
    assert_eq!(
        wire.get("auth_type"),
        None,
        "config key must not leak to wire"
    );
    assert_eq!(wire["enabled"], json!(true));
    assert_eq!(wire["username"], json!("u"));
}

/// Every wire value decodes to a real variant and re-encodes to the same
/// string — including `RELEASE_NEW` / `TEST`, which previously fell through
/// to `Unknown` and could not round-trip.
#[test]
fn all_events_round_trip() {
    for s in [
        "PUSH_APPROVED",
        "PUSH_REJECTED",
        "PUSH_ERROR",
        "IRC_DISCONNECTED",
        "IRC_RECONNECTED",
        "APP_UPDATE_AVAILABLE",
        "RELEASE_NEW",
        "TEST",
    ] {
        let ev: NotificationEvent = engine::decode(&json!(s)).unwrap();
        assert!(
            !matches!(ev, NotificationEvent::Unknown),
            "`{s}` must map to a real variant, not the fallback"
        );
        assert_eq!(engine::encode(&ev).unwrap(), json!(s), "round-trip `{s}`");
    }
}

/// A genuinely unknown value still lands on the fallback.
#[test]
fn unknown_event_is_fallback() {
    let ev: NotificationEvent = engine::decode(&json!("SOMETHING_FUTURE")).unwrap();
    assert!(matches!(ev, NotificationEvent::Unknown));
}

/// The newly-modelled webhook/retry/on-error fields encode under their
/// snake_case wire keys with values passed through verbatim.
#[test]
fn external_filter_new_fields_encode_to_wire() {
    let cfg = json!({
        "name": "size-check",
        "external_type": "WEBHOOK",
        "enabled": true,
        "webhook_host": "http://localhost:9000/check",
        "webhook_method": "POST",
        "webhook_headers": "X-Api-Key=abc,Accept=application/json",
        "webhook_expect_status": 200,
        "webhook_retry_status": "500,502,503",
        "webhook_retry_attempts": 3,
        "webhook_retry_delay_seconds": 5,
        "on_error": "REJECT",
    });
    let wire = engine::encode(&engine::decode_config::<ExternalFilter>(&cfg).unwrap()).unwrap();
    assert_eq!(wire["type"], json!("WEBHOOK"));
    assert_eq!(
        wire["webhook_headers"],
        json!("X-Api-Key=abc,Accept=application/json")
    );
    assert_eq!(wire["webhook_retry_status"], json!("500,502,503"));
    assert_eq!(wire["webhook_retry_attempts"], json!(3));
    assert_eq!(wire["webhook_retry_delay_seconds"], json!(5));
    assert_eq!(wire["on_error"], json!("REJECT"));
}

/// Every channel carries `enabled` on the wire, declared or not: autobrr's
/// channel struct has a plain `bool`, so an absent key decodes to `false`
/// and its join workflow then skips the channel. A bare channel must
/// therefore encode `enabled: true` from the field default.
#[test]
fn channels_always_encode_enabled() {
    let cfg = json!({
        "name": "tl",
        "enabled": true,
        "server": "irc.example.org",
        "port": 6697,
        "nick": "mybot",
        "channels": [
            { "name": "#announce" },
            { "name": "#offtopic", "enabled": false },
        ],
    });
    let wire = engine::encode_config::<IrcNetwork>(&cfg).unwrap();
    assert_eq!(wire["channels"][0]["enabled"], json!(true));
    assert_eq!(wire["channels"][1]["enabled"], json!(false));
}
