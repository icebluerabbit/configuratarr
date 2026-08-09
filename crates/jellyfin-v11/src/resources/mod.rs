//! Jellyfin v11 resources.
//!
//! Config **singletons** (GET a config object, POST the merged result) plus
//! `sync = custom` resources for the parts of the API that don't fit crud/
//! singleton (users, libraries, api keys, the repository list). Jellyfin
//! serialises PascalCase JSON, so resources set `case = pascal`.

// nested sub-structs
pub mod cast_receiver_application;
pub mod metadata_options;
pub mod name_value_pair;
pub mod path_substitution;
pub mod repository_info;
pub mod trickplay_options;

// custom-sync resources (reconcile shapes come from `core_lib::reconcile`)
pub mod auth_key;
pub mod library;
pub mod repository;
pub mod user;

// singletons
pub mod branding_options;
pub mod encoding_options;
pub mod metadata_configuration;
pub mod network_configuration;
pub mod server_configuration;
