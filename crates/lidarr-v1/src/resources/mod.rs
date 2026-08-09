//! Lidarr v1 resources.

// --- provider scaffolding (shared machinery) ---
pub mod download_protocol;
pub mod provider;
pub mod provider_message;
pub mod raw_provider;

// --- collections ---
pub mod auto_tag;
pub mod custom_filter;
pub mod custom_format;
pub mod import_list_exclusion;
pub mod metadata_profile;
pub mod quality_definition;
pub mod quality_profile;
pub mod release_profile;
pub mod remote_path_mapping;
pub mod root_folder;
pub mod tag;

// --- provider resources ---
pub mod download_client;
pub mod download_clients;
pub mod import_list;
pub mod import_lists;
pub mod indexer;
pub mod indexers;
pub mod metadata;
pub mod metadata_providers;
pub mod notification;
pub mod notifications;

// --- singletons ---
pub mod download_client_config;
pub mod host_config;
pub mod indexer_config;
pub mod media_management;
pub mod metadata_provider_config;
pub mod naming;
pub mod ui_config;
