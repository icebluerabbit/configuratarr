//! Shape conformance: our encoded wire payloads must validate against the
//! Prowlarr OpenAPI request schemas.
//!
//! Round-trip unit tests only check codec self-consistency (we author both
//! sides). Here each resource's `testdata/<r>/config.yaml` fixture is run
//! through the *real* encode path (`decode_config` → `encode`) and the payload
//! is validated against the spec's resource schema with a JSON Schema validator.
//! The Prowlarr schemas are `additionalProperties: false`, so a hallucinated,
//! mis-cased, or mis-typed field fails — with a field-level error.
//!
//! `${ref}` / `${env}` in fixtures are dummy-resolved (real resolution needs a
//! live server); we only care about the resulting shape, not the values.
//!
//! Covers Standard resources + provider envelopes. A provider's per-impl inner
//! fields aren't in the static spec (`fields` is a generic array) — those stay
//! an e2e / live-`/schema` concern.

use core_testkit::check;
use serde_json::{Value, json};

#[allow(unused_imports)]
use prowlarr_v1::resources;

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/prowlarr-v1.json")).expect("spec parses")
}

/// One conformance test per resource: `check` its `config.yaml` fixture against
/// the named OpenAPI schema. The harness lives in [`core_testkit::check`].
#[allow(unused_macros)]
macro_rules! conformance {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check::<$ty>(&spec(), $schema, include_str!($fixture));
        }
    };
}

// ── collections ──────────────────────────────────────────────────────────────
conformance!(
    tag,
    resources::tag::Tag,
    "TagResource",
    "testdata/tag/config.yaml"
);
conformance!(
    app_profile,
    resources::app_profile::AppProfile,
    "AppProfileResource",
    "testdata/app_profile/config.yaml"
);
conformance!(
    custom_filter,
    resources::custom_filter::CustomFilter,
    "CustomFilterResource",
    "testdata/custom_filter/config.yaml"
);
conformance!(
    indexer,
    resources::indexer::Indexer,
    "IndexerResource",
    "testdata/indexer/config.yaml"
);
conformance!(
    indexer_proxy,
    resources::indexer_proxy::IndexerProxy,
    "IndexerProxyResource",
    "testdata/indexer_proxy/config.yaml"
);
conformance!(
    download_client,
    resources::download_client::DownloadClient,
    "DownloadClientResource",
    "testdata/download_client/config.yaml"
);
conformance!(
    application,
    resources::application::Application,
    "ApplicationResource",
    "testdata/application/config.yaml"
);
conformance!(
    notification,
    resources::notification::Notification,
    "NotificationResource",
    "testdata/notification/config.yaml"
);

// ── singletons ───────────────────────────────────────────────────────────────
conformance!(
    host_config,
    resources::host_config::HostConfig,
    "HostConfigResource",
    "testdata/host_config/config.yaml"
);
conformance!(
    ui_config,
    resources::ui_config::UiConfig,
    "UiConfigResource",
    "testdata/ui_config/config.yaml"
);
conformance!(
    download_client_config,
    resources::download_client_config::DownloadClientConfig,
    "DownloadClientConfigResource",
    "testdata/download_client_config/config.yaml"
);
conformance!(
    development_config,
    resources::development_config::DevelopmentConfig,
    "DevelopmentConfigResource",
    "testdata/development_config/config.yaml"
);

/// Sanity: the validator actually bites — a hallucinated / mis-typed field is
/// rejected (`additionalProperties: false`). Without this a green run could be a
/// silent no-op.
#[test]
fn validator_rejects_unknown_field() {
    let s = spec();
    let mut schema = s["components"]["schemas"]["TagResource"].clone();
    schema["components"] = s["components"].clone();
    let v = jsonschema::validator_for(&schema).unwrap();
    assert!(
        v.is_valid(&json!({ "label": "ok" })),
        "valid payload must pass"
    );
    assert!(
        !v.is_valid(&json!({ "label": "x", "bogusField": 1 })),
        "extra field must fail"
    );
    assert!(
        !v.is_valid(&json!({ "label": 123 })),
        "wrong type must fail"
    );
}
