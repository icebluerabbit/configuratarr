//! Shape conformance: our encoded wire payloads must validate against the
//! Audiobookshelf request schemas.
//!
//! Audiobookshelf publishes only a partial hand-written spec (about 30 of the
//! ~222 routes its Express routers register), so `spec/audiobookshelf-v1.json`
//! is reverse-engineered from the upstream sources (commit `513a067`) by
//! walking the routers and reading each controller.
//!
//! ABS serialises camelCase JSON — the engine's default — but its ids are
//! **UUID strings**, so any fixture carrying a `${ref}` must resolve it through
//! [`core_testkit::GuidRefs`]; the default [`core_testkit::DummyRefs`] yields
//! the integer `1` and fails decode with `expected string, got 1`.
//!
//! `${ref}` / `${env}` in fixtures are dummy-resolved; only the resulting shape
//! matters here, not the values.

use core_testkit::{GuidRefs, check, check_with_refs};
use serde_json::Value;

#[allow(unused_imports)]
use audiobookshelf_v1::resources;

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/audiobookshelf-v1.json")).expect("spec parses")
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

/// Same, for a fixture whose `${ref}` targets one of ABS's string ids.
macro_rules! conformance_guid_refs {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check_with_refs::<$ty>(&spec(), $schema, include_str!($fixture), &GuidRefs);
        }
    };
}

conformance_guid_refs!(
    library,
    resources::library::Library,
    "Library",
    "testdata/library/config.yaml"
);
conformance_guid_refs!(
    notification,
    resources::notification::Notification,
    "Notification",
    "testdata/notification/config.yaml"
);
conformance!(
    notification_settings,
    resources::notification_settings::NotificationSettings,
    "NotificationSettings",
    "testdata/notification_settings/config.yaml"
);
conformance!(
    email_settings,
    resources::email_settings::EmailSettings,
    "EmailSettings",
    "testdata/email_settings/config.yaml"
);
conformance_guid_refs!(
    ereader_device,
    resources::ereader_device::EreaderDevice,
    "EreaderDeviceObject",
    "testdata/ereader_device/config.yaml"
);
conformance_guid_refs!(
    user,
    resources::user::User,
    // `CreateUserRequest`, not the `User` read model: what the hook encodes is
    // a create body, and only the request schema carries `password`. Validating
    // it against `User` passed only because the spec used to be permissive.
    "CreateUserRequest",
    "testdata/user/config.yaml"
);
conformance!(
    server_settings,
    resources::server_settings::ServerSettings,
    "UpdateServerSettingsRequest",
    "testdata/server_settings/config.yaml"
);
