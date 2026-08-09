//! The LazyLibrarian v1 service. One `#[service]` struct carries the connection
//! fields plus every managed resource.
//!
//! Auth: the api key rides the query string (`?apikey=`), so this uses the
//! `auth = api_key(query = "apikey")` scheme (engine `Auth::ApiKeyQuery`). Health
//! is checked against the authenticated `/api?cmd=getVersion` — a cheap read that
//! also confirms the key.
//!
//! Every field below is a `sync = custom` resource (LazyLibrarian exposes no REST
//! CRUD): `config` is a bazarr-style custom singleton (per-var `readCFG`/`writeCFG`),
//! the six provider `Vec`s are upsert-by-`DISPNAME` collections
//! (`addProvider`/`changeProvider`/`delProvider`), and `magazines`/`authors` are
//! create-plus-prune collections.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::author::Author;
use crate::resources::config::Config;
use crate::resources::magazine::Magazine;
use crate::resources::providers::direct::GenProvider;
use crate::resources::providers::irc::IrcProvider;
use crate::resources::providers::newznab::NewznabProvider;
use crate::resources::providers::rss::RssProvider;
use crate::resources::providers::torznab::TorznabProvider;

/// LazyLibrarian v1 — desired-state config for one instance.
#[service(
    name = "lazylibrarian_v1",
    health = "/api?cmd=getVersion",
    auth = api_key(query = "apikey"),
)]
pub struct LazyLibrarianV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- config singleton (per-var readCFG/writeCFG) ---
    pub config: Option<Config>,

    // --- provider families (upsert by DISPNAME; no round-trip) ---
    /// Newznab (usenet) providers.
    pub newznab: Vec<NewznabProvider>,
    /// Torznab (torrent-over-newznab) providers.
    pub torznab: Vec<TorznabProvider>,
    /// RSS / wishlist feeds.
    pub rss: Vec<RssProvider>,
    /// IRC providers.
    pub irc: Vec<IrcProvider>,
    /// Direct / generic (GEN) providers.
    pub direct: Vec<GenProvider>,

    // --- collections (create-only) ---
    /// Magazines tracked by title.
    pub magazines: Vec<Magazine>,
    /// Authors tracked in the library.
    pub authors: Vec<Author>,
}
