//! Shape conformance: our encoded wire payloads must validate against the
//! LazyLibrarian OpenAPI schemas, and the codec must round-trip (decode→encode
//! stable). Harness is [`core_testkit::check`].
//!
//! Phase 2 adds one `conformance!` line per resource (with a `testdata/<r>/config.yaml`
//! fixture and the matching schema name). Until then this just asserts the spec
//! parses, so the crate has a green test target from the start.

use core_testkit::check;
use serde_json::Value;

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/lazylibrarian-v1.json")).expect("spec parses")
}

macro_rules! conformance {
    ($name:ident, $ty:path, $schema:literal, $fixture:literal) => {
        #[test]
        fn $name() {
            check::<$ty>(&spec(), $schema, include_str!($fixture));
        }
    };
}

#[test]
fn spec_loads() {
    let s = spec();
    assert!(s.get("paths").is_some(), "spec has paths");
    assert!(
        s.pointer("/components/schemas/Config").is_some(),
        "spec has the Config schema"
    );
}

// ── collections (create-only) ─────────────────────────────────────────────────
conformance!(
    magazine,
    lazylibrarian_v1::resources::magazine::Magazine,
    "Magazine",
    "testdata/magazine/config.yaml"
);
conformance!(
    author,
    lazylibrarian_v1::resources::author::Author,
    "Author",
    "testdata/author/config.yaml"
);

// ── providers (upsert by DISPNAME) ────────────────────────────────────────────
conformance!(
    newznab,
    lazylibrarian_v1::resources::providers::newznab::NewznabProvider,
    "ConfigNewznab",
    "testdata/newznab/config.yaml"
);
conformance!(
    torznab,
    lazylibrarian_v1::resources::providers::torznab::TorznabProvider,
    "ConfigTorznab",
    "testdata/torznab/config.yaml"
);
conformance!(
    rss,
    lazylibrarian_v1::resources::providers::rss::RssProvider,
    "ConfigRss",
    "testdata/rss/config.yaml"
);
conformance!(
    irc,
    lazylibrarian_v1::resources::providers::irc::IrcProvider,
    "ConfigIrc",
    "testdata/irc/config.yaml"
);
conformance!(
    direct,
    lazylibrarian_v1::resources::providers::direct::GenProvider,
    "ConfigGen",
    "testdata/direct/config.yaml"
);

// ── config singleton (per-var readCFG/writeCFG) ───────────────────────────────
conformance!(
    config,
    lazylibrarian_v1::resources::config::Config,
    "Config",
    "testdata/config/config.yaml"
);
