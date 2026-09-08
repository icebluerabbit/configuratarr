//! Provider-variant encoding assertions the spec check cannot make.
//!
//! `spec_conformance.rs` validates one fixture per resource against the OpenAPI
//! schema. That leaves the *inner* provider settings almost entirely uncovered:
//! `#/components/schemas/Field` types `value` as untyped and `fields` as a plain
//! array, so **any** `fields[]` entry name validates. A typo'd wire name —
//! `seedCriteria.seedratio`, `apiKey` where eros wants `aPIKey`, `movieCategory`
//! where the C# says `tvCategory` — sails through conformance and then silently
//! does nothing against a live server: the value is dropped and the resource
//! churns on every apply.
//!
//! Whisparr has 68 variants across 5 families and one fixture per family, so 63
//! of them had no automated coverage at all. This file encodes a config for each
//! *high-risk* variant and asserts the exact `fields[]` names that come out.
//! These are the names verified against the eros C# settings classes and, for
//! the nested ones, against `Whisparr.Http/ClientSchema/SchemaBuilder.cs` (which
//! prefixes nested settings objects with `GetCamelCaseName(prop) + "."`).
//!
//! Scope note: this proves *we emit what we intended*. The names themselves were
//! confirmed against a live eros instance — `GET /api/v3/indexer/schema` returns,
//! for Torznab, exactly:
//!
//! ```text
//! baseUrl, apiPath, apiKey, categories, additionalParameters, multiLanguages,
//! failDownloads, removeYear, minimumSeeders, seedCriteria.seedRatio,
//! seedCriteria.seedTime, rejectBlocklistedTorrentHashesWhileGrabbing,
//! requiredFlags
//! ```
//!
//! An indexer cannot be exercised through `tests/e2e.rs`: eros calls
//! `Test(definition, !forceSave)` unconditionally on create
//! (`ProviderControllerBase.cs:87`), so `?forceSave=true` suppresses only
//! *warnings* — a connection error to an unreachable indexer still fails the
//! create. Hence the schema endpoint, not an apply, is the oracle here.

use core_lib::engine;
use serde_json::Value;
use whisparr_v3::resources;

/// Decode a YAML config into `T`, encode it to the wire, and return the set of
/// `fields[]` entry names.
fn field_names<T: core_lib::Described>(yaml: &str) -> Vec<String> {
    let mut cfg: Value = serde_saphyr::from_str(yaml).expect("yaml parses");
    core_lib::resolve::resolve_static(&mut cfg, &DummyEnv).expect("static resolve");
    let decoded = engine::decode_config::<T>(&cfg).expect("config decodes");
    let wire = engine::encode(&decoded).expect("encodes");
    let mut names: Vec<String> = wire["fields"]
        .as_array()
        .expect("provider payload has a fields[] array")
        .iter()
        .map(|f| f["name"].as_str().expect("field has a name").to_string())
        .collect();
    names.sort();
    names
}

/// `${env}` double. Fixtures in this file use literal values on purpose — the
/// point is the *field names*, so nothing here should depend on interpolation.
struct DummyEnv;
impl core_lib::StaticEnv for DummyEnv {
    fn env(&self, _: &str) -> Option<&str> {
        None
    }
    fn file(&self, path: &str) -> anyhow::Result<String> {
        anyhow::bail!("codec fixtures must not read files: {path}")
    }
}

fn assert_fields<T: core_lib::Described>(yaml: &str, expected: &[&str]) {
    let got = field_names::<T>(yaml);
    let mut want: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
    want.sort();
    assert_eq!(got, want, "encoded fields[] names differ");
}

// ── indexers: the nested seedCriteria.* names ────────────────────────────────

/// The single highest-risk encoding in the crate. eros moved radarr's flat
/// `seedRatio`/`seedTime` into a nested `SeedCriteriaSettings`, which
/// `SchemaBuilder` flattens back out as dotted names. Getting this wrong is
/// invisible to conformance.
#[test]
fn torznab_emits_dotted_seed_criteria() {
    assert_fields::<resources::indexer::Indexer>(
        r#"
name: e2e-torznab
implementation: Torznab
protocol: torrent
enable_rss: true
base_url: http://localhost:9117
api_path: /api
api_key: torznabkey
categories: [2000]
minimum_seeders: 2
seed_ratio: 1.5
seed_time: 60
remove_year: false
reject_blocklisted_torrent_hashes_while_grabbing: false
tags: []
"#,
        &[
            "baseUrl",
            "apiPath",
            "apiKey",
            "categories",
            "multiLanguages",
            "failDownloads",
            "removeYear",
            "minimumSeeders",
            "seedCriteria.seedRatio",
            "seedCriteria.seedTime",
            "rejectBlocklistedTorrentHashesWhileGrabbing",
            "requiredFlags",
        ],
    );
}

/// Newznab is the one indexer with no `seedCriteria` — guards against the dotted
/// fields leaking onto usenet variants.
#[test]
fn newznab_has_no_seed_criteria() {
    assert_fields::<resources::indexer::Indexer>(
        r#"
name: e2e-newznab
implementation: Newznab
protocol: usenet
enable_rss: true
base_url: http://localhost:8080
api_path: /api
api_key: newznabkey
categories: [2000]
remove_year: false
tags: []
"#,
        &[
            "apiKey",
            "apiPath",
            "baseUrl",
            "categories",
            "failDownloads",
            "multiLanguages",
            "removeYear",
        ],
    );
}

// ── import lists: nested filterCriteria.* + the odd tMDbListType ─────────────

#[test]
fn tmdb_popular_emits_dotted_filter_criteria() {
    assert_fields::<resources::import_list::ImportList>(
        r#"
name: e2e-tmdb-popular
implementation: TMDbPopularImport
enabled: true
enable_auto: true
quality_profile_id: 1
root_folder_path: /movies
tmdb_list_type: 1
min_vote_average: "6.0"
min_votes: "100"
tags: []
"#,
        &[
            "filterCriteria.minVoteAverage",
            "filterCriteria.minVotes",
            "tMDbListType",
        ],
    );
}

/// StashDB variants are eros-only — no radarr precedent to fall back on.
#[test]
fn stashdb_performer_field_names() {
    assert_fields::<resources::import_list::ImportList>(
        r#"
name: e2e-stashdb-performer
implementation: StashDBPerformerImport
enabled: true
enable_auto: true
quality_profile_id: 1
root_folder_path: /movies
api_key: stashdbkey
limit: 100
after_date: "2026-01-01"
sort: released
performers: "perf-1"
studios: "studio-1"
studios_filter: includes
stash_tags: "tag-a"
tags_filter: includes
only_favorite_studios: false
tags: []
"#,
        &[
            "apiKey",
            "limit",
            "afterDate",
            "sort",
            "performers",
            "studios",
            "studiosFilter",
            "tags",
            "tagsFilter",
            "onlyFavoriteStudios",
        ],
    );
}

// ── notifications: the casing traps ─────────────────────────────────────────

/// eros's C# property is `APIKey`; `GetCamelCaseName` lowercases only the first
/// character, so the wire name is `aPIKey` — not `apiKey`. radarr hit the mirror
/// image of this bug (see the note in radarr's e2e suite).
#[test]
fn notifiarr_emits_apikey_with_leading_lowercase_only() {
    assert_fields::<resources::notification::Notification>(
        r#"
name: e2e-notifiarr
implementation: Notifiarr
api_key: notifiarrkey
on_grab: true
tags: []
"#,
        &["aPIKey"],
    );
}

/// radarr overrides this to `cC`; the eros C# property is `Cc`, so the wire name
/// is plain `cc`.
#[test]
fn email_emits_lowercase_cc() {
    assert_fields::<resources::notification::Notification>(
        r#"
name: e2e-email
implementation: Email
server: smtp.local
port: 587
from: a@b.c
to: [d@e.f]
cc: [g@h.i]
on_grab: true
tags: []
"#,
        &["bcc", "cc", "from", "port", "server", "to", "useEncryption"],
    );
}

/// Apprise renames radarr's `fieldTags` to `tags`; Ntfy additionally renames
/// `username` to `userName`. Both are silent-drop failures if wrong.
#[test]
fn ntfy_emits_username_camel_and_tags() {
    assert_fields::<resources::notification::Notification>(
        r#"
name: e2e-ntfy
implementation: Ntfy
server_url: http://ntfy.local
user_name: someone
password: secret
topics: [alerts]
message_tags: [warning]
on_grab: true
"#,
        &["password", "serverUrl", "tags", "topics", "userName"],
    );
}

// ── download clients: the Tv*-naming trap ───────────────────────────────────

/// `DownloadStationSettings` keeps Sonarr's `tvCategory`/`tvDirectory` naming in
/// eros. Copying radarr's `movieCategory` here would encode a field the server
/// ignores.
#[test]
fn download_station_keeps_tv_naming() {
    assert_fields::<resources::download_client::DownloadClient>(
        r#"
name: e2e-ds
implementation: TorrentDownloadStation
protocol: torrent
enable: true
host: ds.local
port: 5000
username: user
password: pass
tv_category: whisparr
tags: []
"#,
        &["host", "password", "port", "tvCategory", "username"],
    );
}

/// NzbVortex is new in eros relative to radarr's crate, and also uses `tvCategory`.
#[test]
fn nzbvortex_field_names() {
    assert_fields::<resources::download_client::DownloadClient>(
        r#"
name: e2e-nzbvortex
implementation: NzbVortex
protocol: usenet
enable: true
host: nzbvortex.local
port: 4321
url_base: /api
api_key: vortexkey
tv_category: whisparr
recent_movie_priority: 1
older_movie_priority: 0
tags: []
"#,
        &[
            "apiKey",
            "host",
            "olderMoviePriority",
            "port",
            "recentMoviePriority",
            "tvCategory",
            "urlBase",
        ],
    );
}

// ── metadata: the URL-cased field ───────────────────────────────────────────

/// C# `MovieMetadataURL` → `movieMetadataURL`; snake→camel derivation would emit
/// `movieMetadataUrl` and be dropped.
#[test]
fn xbmc_metadata_emits_uppercase_url() {
    assert_fields::<resources::metadata::Metadata>(
        r#"
name: e2e-xbmc
implementation: XbmcMetadata
enable: true
movie_metadata: true
movie_metadata_url: true
movie_images: true
tags: []
"#,
        &["movieImages", "movieMetadata", "movieMetadataURL"],
    );
}
