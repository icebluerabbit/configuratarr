//! The Whisparr v3 (eros) service. One `#[service]` struct carries everything:
//! the connection fields (url + credential + optional tls/timeout overrides) and
//! every managed resource. `Vec<R>` fields are collections; `Option<R>` fields
//! are singletons. The macro derives the service descriptor and the
//! connection/auth bundle from the field shapes + `#[credential]` markers.
//!
//! This models the **eros** line (Whisparr v3), a Radarr fork. Whisparr v2 is a
//! *Sonarr* fork that also serves `/api/v3` with a different resource set — the
//! two are not interchangeable.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::auto_tag::AutoTag;
use crate::resources::custom_filter::CustomFilter;
use crate::resources::custom_format::CustomFormat;
use crate::resources::delay_profile::DelayProfile;
use crate::resources::download_client::DownloadClient;
use crate::resources::download_client_config::DownloadClientConfig;
use crate::resources::host_config::HostConfig;
use crate::resources::import_list::ImportList;
use crate::resources::import_list_config::ImportListConfig;
use crate::resources::import_list_exclusion::ImportListExclusion;
use crate::resources::indexer::Indexer;
use crate::resources::indexer_config::IndexerConfig;
use crate::resources::media_management::MediaManagement;
use crate::resources::metadata::Metadata;
use crate::resources::naming::Naming;
use crate::resources::notification::Notification;
use crate::resources::quality_definition::QualityDefinition;
use crate::resources::quality_profile::QualityProfile;
use crate::resources::release_profile::ReleaseProfile;
use crate::resources::remote_path_mapping::RemotePathMapping;
use crate::resources::root_folder::RootFolder;
use crate::resources::tag::Tag;
use crate::resources::ui_config::UiConfig;

/// Whisparr v3 (eros) — desired-state config for one instance.
#[service(
    name = "whisparr_v3",
    health = "/api/v3/system/status",
    auth = api_key(header = "X-Api-Key"),
)]
pub struct WhisparrV3 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- collections ---
    pub tags: Vec<Tag>,
    pub quality_profiles: Vec<QualityProfile>,
    pub quality_definitions: Vec<QualityDefinition>,
    pub custom_formats: Vec<CustomFormat>,
    pub custom_filters: Vec<CustomFilter>,
    pub delay_profiles: Vec<DelayProfile>,
    pub release_profiles: Vec<ReleaseProfile>,
    pub remote_path_mappings: Vec<RemotePathMapping>,
    pub download_clients: Vec<DownloadClient>,
    pub indexers: Vec<Indexer>,
    pub notifications: Vec<Notification>,
    pub metadata: Vec<Metadata>,
    pub import_lists: Vec<ImportList>,
    pub root_folders: Vec<RootFolder>,
    pub import_list_exclusions: Vec<ImportListExclusion>,
    pub auto_tags: Vec<AutoTag>,

    // --- singletons ---
    pub media_management: Option<MediaManagement>,
    pub naming: Option<Naming>,
    pub ui_config: Option<UiConfig>,
    pub indexer_config: Option<IndexerConfig>,
    pub download_client_config: Option<DownloadClientConfig>,
    pub import_list_config: Option<ImportListConfig>,
    pub host_config: Option<HostConfig>,
}
