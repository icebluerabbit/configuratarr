//! The Bindery v1 service. One `#[service]` struct carries the connection
//! fields and every managed resource; `Vec<R>` fields are collections,
//! `Option<R>` fields are singletons. The sync strategy comes from each
//! resource's own descriptor, not from the field shape.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::abs_config::AbsConfig;
use crate::resources::auth_mode::AuthMode;
use crate::resources::custom_format::CustomFormat;
use crate::resources::delay_profile::DelayProfile;
use crate::resources::download_client::DownloadClient;
use crate::resources::grimmory_config::GrimmoryConfig;
use crate::resources::import_list::ImportList;
use crate::resources::import_list_exclusion::ImportListExclusion;
use crate::resources::indexer::Indexer;
use crate::resources::metadata_profile::MetadataProfile;
use crate::resources::notification::Notification;
use crate::resources::oidc_provider::OidcProvider;
use crate::resources::prowlarr_instance::ProwlarrInstance;
use crate::resources::quality_profile::QualityProfile;
use crate::resources::root_folder::RootFolder;
use crate::resources::setting::Setting;
use crate::resources::user::User;

/// Bindery v1 — desired-state config for one instance.
///
/// Auth is the `X-Api-Key` header (note the casing: Komga spells the same idea
/// `X-API-Key`). The key is the instance key shown in Bindery's settings, not a
/// per-user token.
///
/// The health check is `/api/v1/system/status` — authenticated, so a green
/// response proves credentials too. `/api/v1/health` is deliberately not used:
/// it answers a constant `{"status":"ok"}` without checking auth.
#[service(
    name = "bindery_v1",
    health = "/api/v1/system/status",
    auth = api_key(header = "X-Api-Key"),
)]
pub struct BinderyV1 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- collections ---
    /// Filesystem paths Bindery imports into. Create + delete only.
    pub root_folders: Vec<RootFolder>,
    /// Ordered file-format preference lists with an upgrade cutoff.
    pub quality_profiles: Vec<QualityProfile>,
    /// Filters deciding which discovered books get added to the catalogue.
    pub metadata_profiles: Vec<MetadataProfile>,
    /// Named sets of release-matching conditions.
    pub custom_formats: Vec<CustomFormat>,
    /// Per-protocol grab delays. Keyless — the whole list is replaced.
    pub delay_profiles: Vec<DelayProfile>,
    /// Usenet/torrent indexers.
    pub indexers: Vec<Indexer>,
    /// Prowlarr instances whose indexers are synced in.
    pub prowlarr_instances: Vec<ProwlarrInstance>,
    /// Download clients (SABnzbd, NZBGet, qBittorrent, …).
    pub download_clients: Vec<DownloadClient>,
    /// Notification targets.
    pub notifications: Vec<Notification>,
    /// Import lists (e.g. Hardcover shelves) that add works automatically.
    pub import_lists: Vec<ImportList>,
    /// Works blocked from being (re-)added by import-list or catalogue syncs.
    pub import_list_exclusions: Vec<ImportListExclusion>,
    /// Local accounts. Role changes and password resets use their own routes.
    pub users: Vec<User>,
    /// OIDC identity providers, managed as one whole set.
    pub oidc_providers: Vec<OidcProvider>,

    // --- singletons ---
    /// The `/api/v1/setting` key/value table.
    pub setting: Option<Setting>,
    /// Server authentication mode.
    pub auth_mode: Option<AuthMode>,
    /// Audiobookshelf import-source configuration.
    pub abs_config: Option<AbsConfig>,
    /// Grimmory integration configuration.
    pub grimmory_config: Option<GrimmoryConfig>,
}
