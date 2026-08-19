//! Shape conformance: our encoded wire payloads must validate against the
//! Bindery request schemas.
//!
//! Bindery publishes no OpenAPI document; `spec/bindery-v1.json` is
//! reverse-engineered from the upstream Go sources (v1.31.0, commit `e11987b`)
//! by walking the chi router and its handlers. Bindery serialises camelCase
//! JSON — the engine's default — and every id is an `int64` at the wire key
//! `id`, so the default [`core_testkit::DummyRefs`] (which yields `1`) is the
//! right double for every `${ref}` here.
//!
//! Note the spec's schemas do **not** set `additionalProperties: false`, so
//! these checks catch a missing or mis-typed required field but not a stray
//! extra key — a property of the upstream API's shape, not of this suite.

use core_testkit::check;
use serde_json::Value;

#[allow(unused_imports)]
use bindery_v1::resources;

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/bindery-v1.json")).expect("spec parses")
}

/// One conformance test per resource: `check` its `config.yaml` fixture against
/// the named OpenAPI schema. The harness lives in [`core_testkit::check`].
macro_rules! conformance {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check::<$ty>(&spec(), $schema, include_str!($fixture));
        }
    };
}

conformance!(
    custom_format,
    resources::custom_format::CustomFormat,
    "CustomFormat",
    "testdata/custom_format/config.yaml"
);
conformance!(
    metadata_profile,
    resources::metadata_profile::MetadataProfile,
    "MetadataProfile",
    "testdata/metadata_profile/config.yaml"
);
conformance!(
    quality_profile,
    resources::quality_profile::QualityProfile,
    "QualityProfile",
    "testdata/quality_profile/config.yaml"
);
conformance!(
    download_client,
    resources::download_client::DownloadClient,
    "DownloadClient",
    "testdata/download_client/config.yaml"
);
conformance!(
    indexer,
    resources::indexer::Indexer,
    "Indexer",
    "testdata/indexer/config.yaml"
);
conformance!(
    prowlarr_instance,
    resources::prowlarr_instance::ProwlarrInstance,
    "ProwlarrInstance",
    "testdata/prowlarr_instance/config.yaml"
);
conformance!(
    notification,
    resources::notification::Notification,
    "Notification",
    "testdata/notification/config.yaml"
);
conformance!(
    root_folder,
    resources::root_folder::RootFolder,
    "CreateRootFolderRequest",
    "testdata/root_folder/config.yaml"
);
conformance!(
    delay_profile,
    resources::delay_profile::DelayProfile,
    "DelayProfile",
    "testdata/delay_profile/config.yaml"
);
conformance!(
    import_list,
    resources::import_list::ImportList,
    "ImportList",
    "testdata/import_list/config.yaml"
);
conformance!(
    import_list_exclusion,
    resources::import_list_exclusion::ImportListExclusion,
    "CreateImportListExclusionRequest",
    "testdata/import_list_exclusion/config.yaml"
);
conformance!(
    user,
    resources::user::User,
    "CreateUserRequest",
    "testdata/user/config.yaml"
);
conformance!(
    oidc_provider,
    resources::oidc_provider::OidcProvider,
    "OIDCProviderConfig",
    "testdata/oidc_provider/config.yaml"
);
conformance!(
    auth_mode,
    resources::auth_mode::AuthMode,
    "AuthModeRequest",
    "testdata/auth_mode/config.yaml"
);
conformance!(
    abs_config,
    resources::abs_config::AbsConfig,
    "ABSConfigRequest",
    "testdata/abs_config/config.yaml"
);
// ── settings singleton (per-key PUT /api/v1/setting/{key}) ───────────────────
// `ValidatedSettings` is a synthetic schema: the real API takes one key per
// request, so there is no published shape for the whole document. Same
// situation, same treatment as lazylibrarian's `Config`. It types each key as
// the value is *validated*, before `scalar` stringifies it for the wire — which
// is exactly what `encode` produces.
conformance!(
    setting,
    resources::setting::Setting,
    "ValidatedSettings",
    "testdata/setting/config.yaml"
);

conformance!(
    grimmory_config,
    resources::grimmory_config::GrimmoryConfig,
    "GrimmoryConfigRequest",
    "testdata/grimmory_config/config.yaml"
);
