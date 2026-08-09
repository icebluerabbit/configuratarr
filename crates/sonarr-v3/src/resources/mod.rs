//! Sonarr v3 resources.

// --- provider scaffolding (shared machinery) ---
pub mod download_protocol;
pub mod provider;
pub mod provider_message;
pub mod raw_provider;

// --- resources (spliced in as slices land) ---
pub mod auto_tag;
pub mod custom_filter;
pub mod custom_format;
pub mod download_client;
pub mod download_client_config;
pub mod download_clients;
pub mod host_config;
pub mod import_list;
pub mod import_list_config;
pub mod import_list_exclusion;
pub mod import_lists;
pub mod indexer;
pub mod indexer_config;
pub mod indexers;
pub mod language_profile;
pub mod media_management;
pub mod metadata;
pub mod metadata_providers;
pub mod naming;
pub mod notification;
pub mod notifications;
pub mod quality_definition;
pub mod quality_profile;
pub mod release_profile;
pub mod remote_path_mapping;
pub mod root_folder;
pub mod tag;
pub mod ui_config;
