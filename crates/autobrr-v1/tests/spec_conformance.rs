//! Shape conformance: our encoded wire payloads must validate against the
//! autobrr OpenAPI schemas, and the codec must round-trip (decode→encode stable).
//!
//! autobrr serialises snake_case JSON, so the resources use `case = snake` and
//! the encoded keys match the (snake_case) schemas directly. The autobrr schemas
//! are **not** `additionalProperties: false`, so a hallucinated field wouldn't be
//! rejected by schema validation alone — but the decode→encode round-trip in
//! [`core_testkit::check`] still catches a field that decodes but doesn't
//! re-encode, and property types/casing are still validated.

use autobrr_v1::resources;
use core_testkit::check;
use serde_json::Value;

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/autobrr-v1.json")).expect("spec parses")
}

macro_rules! conformance {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check::<$ty>(&spec(), $schema, include_str!($fixture));
        }
    };
}

// create-only collections
conformance!(
    api_key,
    resources::api_key::ApiKey,
    "ApiKeyCreate",
    "testdata/api_key/config.yaml"
);
conformance!(
    notification,
    resources::notification::Notification,
    "Notification",
    "testdata/notification/config.yaml"
);
conformance!(
    notification_webhook,
    resources::notification::Notification,
    "Notification",
    "testdata/notification_webhook/config.yaml"
);
conformance!(
    notification_matrix,
    resources::notification::Notification,
    "Notification",
    "testdata/notification_matrix/config.yaml"
);

// crud collections
conformance!(
    download_client,
    resources::download_client::DownloadClient,
    "DownloadClient",
    "testdata/download_client/config.yaml"
);
conformance!(
    proxy,
    resources::proxy::Proxy,
    "Proxy",
    "testdata/proxy/config.yaml"
);
conformance!(
    irc_network,
    resources::irc_network::IrcNetwork,
    "IrcNetwork",
    "testdata/irc_network/config.yaml"
);

// custom-sync resources (two-step create / server-rewritten identifier)
conformance!(
    filter,
    resources::filter::Filter,
    "Filter",
    "testdata/filter/config.yaml"
);
conformance!(
    indexer,
    resources::indexer::Indexer,
    "Indexer",
    "testdata/indexer/config.yaml"
);
conformance!(
    release_profile_duplicate,
    resources::release_profile_duplicate::ReleaseProfileDuplicate,
    "ReleaseProfileDuplicate",
    "testdata/release_profile_duplicate/config.yaml"
);
conformance!(
    list,
    resources::list::List,
    "List",
    "testdata/list/config.yaml"
);
conformance!(
    feed,
    resources::feed::Feed,
    "Feed",
    "testdata/feed/config.yaml"
);
