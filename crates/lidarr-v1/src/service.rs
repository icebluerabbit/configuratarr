//! The Lidarr v1 service. One `#[service]` struct carries everything: the
//! connection fields (url + credential + optional tls/timeout overrides) and
//! every managed resource. `Vec<R>` fields are collections; `Option<R>` fields
//! are singletons. The macro derives the service descriptor and the
//! connection/auth bundle from the field shapes + `#[credential]` markers.

use core_lib::SecretValue;
use core_macros::service;

// --- resource imports ---
use crate::resources::auto_tag::AutoTag;
use crate::resources::custom_filter::CustomFilter;
use crate::resources::custom_format::CustomFormat;
use crate::resources::download_client::DownloadClient;
use crate::resources::download_client_config::DownloadClientConfig;
use crate::resources::host_config::HostConfig;
use crate::resources::import_list::ImportList;
use crate::resources::import_list_exclusion::ImportListExclusion;
use crate::resources::indexer::Indexer;
use crate::resources::indexer_config::IndexerConfig;
use crate::resources::media_management::MediaManagement;
use crate::resources::metadata::Metadata;
use crate::resources::metadata_profile::MetadataProfile;
use crate::resources::metadata_provider_config::MetadataProviderConfig;
use crate::resources::naming::Naming;
use crate::resources::notification::Notification;
use crate::resources::quality_definition::QualityDefinition;
use crate::resources::quality_profile::QualityProfile;
use crate::resources::release_profile::ReleaseProfile;
use crate::resources::remote_path_mapping::RemotePathMapping;
use crate::resources::root_folder::RootFolder;
use crate::resources::tag::Tag;
use crate::resources::ui_config::UiConfig;

/// Lidarr v1 — desired-state config for one instance.
#[service(
    name = "lidarr_v1",
    health = "/api/v1/system/status",
    auth = api_key(header = "X-Api-Key"),
)]
pub struct LidarrV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- collections ---
    /// Tags — labels referenced by other resources.
    pub tags: Vec<Tag>,
    /// Custom formats — release scoring rules.
    pub custom_formats: Vec<CustomFormat>,
    /// Saved UI filters.
    pub custom_filters: Vec<CustomFilter>,
    /// Quality profiles — allowed qualities and upgrade cutoff.
    pub quality_profiles: Vec<QualityProfile>,
    /// Metadata profiles — album-type / release-status filtering.
    pub metadata_profiles: Vec<MetadataProfile>,
    /// Release profiles — preferred/required/ignored release terms.
    pub release_profiles: Vec<ReleaseProfile>,
    /// Auto-tagging rules.
    pub auto_tags: Vec<AutoTag>,
    /// Remote path mappings for download clients.
    pub remote_path_mappings: Vec<RemotePathMapping>,
    /// Root folders where the library lives.
    pub root_folders: Vec<RootFolder>,
    /// Import-list exclusions (artists never to auto-add).
    pub import_list_exclusions: Vec<ImportListExclusion>,
    /// Quality definitions — size limits per quality.
    pub quality_definitions: Vec<QualityDefinition>,
    /// Download clients used to retrieve releases (usenet and torrent).
    pub download_clients: Vec<DownloadClient>,
    /// Indexers (usenet and torrent search sources).
    pub indexers: Vec<Indexer>,
    /// Metadata consumers (Kodi, Roksbox, WDTV).
    pub metadata: Vec<Metadata>,
    /// Notification connections — routes artist/album/track events out.
    pub notifications: Vec<Notification>,
    /// External sources that automatically add artists to the library.
    pub import_lists: Vec<ImportList>,

    // --- singletons ---
    /// File handling and media management settings.
    pub media_management: Option<MediaManagement>,
    /// Track file and artist folder naming configuration.
    pub naming: Option<Naming>,
    /// UI display and localisation settings.
    pub ui_config: Option<UiConfig>,
    /// Global indexer and RSS sync settings.
    pub indexer_config: Option<IndexerConfig>,
    /// Download client handling settings.
    pub download_client_config: Option<DownloadClientConfig>,
    /// Host, network, authentication, proxy, and backup settings.
    pub host_config: Option<HostConfig>,
    /// Music metadata source and audio tag write settings.
    pub metadata_provider_config: Option<MetadataProviderConfig>,
}
