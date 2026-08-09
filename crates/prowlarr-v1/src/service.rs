//! The Prowlarr v1 service. One `#[service]` struct carries everything: the
//! connection fields (url + credential + optional tls/timeout overrides) and
//! every managed resource. `Vec<R>` fields are collections; `Option<R>` fields
//! are singletons. The macro derives the service descriptor and the
//! connection/auth bundle from the field shapes + `#[credential]` markers.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::app_profile::AppProfile;
use crate::resources::application::Application;
use crate::resources::custom_filter::CustomFilter;
use crate::resources::development_config::DevelopmentConfig;
use crate::resources::download_client::DownloadClient;
use crate::resources::download_client_config::DownloadClientConfig;
use crate::resources::host_config::HostConfig;
use crate::resources::indexer::Indexer;
use crate::resources::indexer_proxy::IndexerProxy;
use crate::resources::notification::Notification;
use crate::resources::tag::Tag;
use crate::resources::ui_config::UiConfig;

/// Prowlarr v1 — desired-state config for one instance.
#[service(
    name = "prowlarr_v1",
    health = "/api/v1/system/status",
    auth = api_key(header = "X-Api-Key"),
)]
pub struct ProwlarrV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- collections ---
    pub tags: Vec<Tag>,
    pub app_profiles: Vec<AppProfile>,
    pub custom_filters: Vec<CustomFilter>,
    pub indexers: Vec<Indexer>,
    pub indexer_proxies: Vec<IndexerProxy>,
    pub download_clients: Vec<DownloadClient>,
    pub applications: Vec<Application>,
    pub notifications: Vec<Notification>,

    // --- singletons ---
    pub host_config: Option<HostConfig>,
    pub ui_config: Option<UiConfig>,
    pub download_client_config: Option<DownloadClientConfig>,
    pub development_config: Option<DevelopmentConfig>,
}
