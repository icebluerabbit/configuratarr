//! Shape conformance: our encoded wire payloads must validate against the
//! Jellyfin OpenAPI schemas.
//!
//! Jellyfin serialises .NET **PascalCase** JSON by default (verified live on
//! 10.11.11). The resources use `#[resource(case = pascal)]`, so the encoded
//! wire keys are PascalCase and validate directly against the (PascalCase)
//! OpenAPI schemas.

use core_testkit::check;
use jellyfin_v11::resources;
use serde_json::{Value, json};

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/jellyfin-v11.json")).expect("spec parses")
}

macro_rules! conformance {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check::<$ty>(&spec(), $schema, include_str!($fixture));
        }
    };
}

// ── bulk-replace collections ──────────────────────────────────────────────────
conformance!(
    repository,
    resources::repository::Repository,
    "RepositoryInfo",
    "testdata/repository/config.yaml"
);

// ── singletons ────────────────────────────────────────────────────────────────
conformance!(
    server_configuration,
    resources::server_configuration::ServerConfiguration,
    "ServerConfiguration",
    "testdata/server_configuration/config.yaml"
);
conformance!(
    network_configuration,
    resources::network_configuration::NetworkConfiguration,
    "NetworkConfiguration",
    "testdata/network_configuration/config.yaml"
);
conformance!(
    encoding_options,
    resources::encoding_options::EncodingOptions,
    "EncodingOptions",
    "testdata/encoding_options/config.yaml"
);
conformance!(
    metadata_configuration,
    resources::metadata_configuration::MetadataConfiguration,
    "MetadataConfiguration",
    "testdata/metadata_configuration/config.yaml"
);
conformance!(
    branding_options,
    resources::branding_options::BrandingOptions,
    "BrandingOptionsDto",
    "testdata/branding_options/config.yaml"
);

/// Sanity: the camelCase validator actually bites — a hallucinated field is
/// rejected (`additionalProperties: false`).
#[test]
fn validator_rejects_unknown_field() {
    let s = spec();
    check::<resources::branding_options::BrandingOptions>(
        &s,
        "BrandingOptionsDto",
        "custom_css: \"body {}\"\nsplashscreen_enabled: true\n",
    );
    // The schema is closed (`additionalProperties: false`), so a mis-cased or
    // hallucinated key fails — the conformance test has teeth.
    let schema = &s["components"]["schemas"]["BrandingOptionsDto"];
    assert_eq!(
        schema["additionalProperties"],
        json!(false),
        "schema must be closed for the conformance test to have teeth"
    );
}
