//! The Autobrr v1 service. One `#[service]` struct carries the connection fields
//! plus every managed resource.
//!
//! Auth: `X-API-Token` header. Health is checked against `/api/config` — an
//! authenticated JSON endpoint whose GET is read-only (so a healthy response also
//! confirms the token; the `/api/healthz/*` probes return plain text, which the
//! JSON client can't parse). autobrr's app config itself isn't managed here — it
//! writes `config.toml`, read-only under the NixOS module.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::api_key::ApiKey;
use crate::resources::download_client::DownloadClient;
use crate::resources::feed::Feed;
use crate::resources::filter::Filter;
use crate::resources::indexer::Indexer;
use crate::resources::irc_network::IrcNetwork;
use crate::resources::list::List;
use crate::resources::notification::Notification;
use crate::resources::proxy::Proxy;
use crate::resources::release_profile_duplicate::ReleaseProfileDuplicate;

/// Autobrr v1 — desired-state config for one instance.
#[service(
    name = "autobrr_v1",
    health = "/api/config",
    auth = api_key(header = "X-API-Token"),
)]
pub struct AutobrrV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- managed resources ---
    /// API keys (create-only).
    pub api_keys: Vec<ApiKey>,
    /// Notification targets (create-only).
    pub notifications: Vec<Notification>,
    /// Proxies (SOCKS5/HTTP) for indexers and IRC.
    pub proxies: Vec<Proxy>,
    /// Download clients.
    pub download_clients: Vec<DownloadClient>,
    /// Indexers (create + update by name; no prune).
    pub indexers: Vec<Indexer>,
    /// IRC networks and their channels (create + update by name; no prune).
    pub irc_networks: Vec<IrcNetwork>,
    /// Duplicate/dedup profiles (create-only; referenced by filters).
    pub release_profile_duplicates: Vec<ReleaseProfileDuplicate>,
    /// Release-matching filters.
    pub filters: Vec<Filter>,
    /// Lists — pull wanted titles from *arr / external sources into filters
    /// (create + update by name; no prune). References download clients + filters.
    pub lists: Vec<List>,
    /// RSS/Torznab/Newznab feeds (create + update by name; no prune). Each
    /// references its feed-type indexer.
    pub feeds: Vec<Feed>,
}
