//! Prowlarr v1 resources.

// --- provider scaffolding (shared machinery) ---
pub mod download_protocol;
pub mod provider;
pub mod provider_message;
pub mod raw_provider;

// --- enums ---
pub mod application_sync_level;
pub mod authentication_required_type;
pub mod authentication_type;
pub mod certificate_validation_type;
pub mod indexer_privacy;
pub mod proxy_type;
pub mod update_mechanism;

// --- resources ---
pub mod app_profile;
pub mod application;
pub mod applications;
pub mod custom_filter;
pub mod development_config;
pub mod download_client;
pub mod download_client_config;
pub mod download_clients;
pub mod host_config;
pub mod indexer;
pub mod indexer_proxies;
pub mod indexer_proxy;
pub mod notification;
pub mod notifications;
pub mod tag;
pub mod ui_config;
