//! The Jellyfin v11 service. One `#[service]` struct carries the connection
//! fields plus every managed resource. Tier-A resources are all `Option<R>`
//! config singletons.
//!
//! Auth: Jellyfin accepts the raw API key in the `X-Emby-Token` header (the
//! `Authorization` header instead wants the `MediaBrowser Token="…"` scheme,
//! which the plain api-key auth can't express). Health is checked against the
//! authenticated `/System/Info` endpoint — it 500s while Jellyfin is still
//! running first-run DB migrations and needs a valid key, so `--wait-for-healthy`
//! blocks until the API is *actually* ready (and the credentials work), unlike
//! the unauthenticated `/System/Info/Public`, which answers too early.

use core_lib::SecretValue;
use core_macros::service;

use crate::resources::auth_key::AuthKey;
use crate::resources::branding_options::BrandingOptions;
use crate::resources::encoding_options::EncodingOptions;
use crate::resources::library::Library;
use crate::resources::metadata_configuration::MetadataConfiguration;
use crate::resources::network_configuration::NetworkConfiguration;
use crate::resources::repository::Repository;
use crate::resources::server_configuration::ServerConfiguration;
use crate::resources::user::User;

/// Jellyfin v11 — desired-state config for one instance.
#[service(
    name = "jellyfin_v11",
    health = "/System/Info",
    auth = api_key(header = "X-Emby-Token"),
)]
pub struct JellyfinV11 {
    // --- connection ---
    pub url: String,
    #[credential(api_key)]
    pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    // --- custom-sync resources ---
    pub repositories: Vec<Repository>,
    pub auth_keys: Vec<AuthKey>,
    pub users: Vec<User>,
    pub libraries: Vec<Library>,

    // --- singletons ---
    pub server_configuration: Option<ServerConfiguration>,
    pub network_configuration: Option<NetworkConfiguration>,
    pub encoding_options: Option<EncodingOptions>,
    pub metadata_configuration: Option<MetadataConfiguration>,
    pub branding_options: Option<BrandingOptions>,
}
