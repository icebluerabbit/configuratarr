//! Bazarr v1 resources.
//!
//! Bazarr has no per-resource CRUD config surface: its entire configuration is
//! one endpoint (`/api/system/settings`) with a form-encoded write. So there is
//! a single `sync = custom` singleton, [`settings::Settings`], composed of typed
//! per-section sub-structs (one module each) plus language profiles.
//!
//! The section sub-structs document the schema (config docs are generated from
//! their `///` comments and field types) and give users a validated shape. Each
//! is a **curated** view — the connection- and behaviour-relevant keys, not every
//! dynaconf validator (runtime/UI-only keys like `theme`, `page_size`, `debug`
//! are omitted). Field names, types and section names mirror bazarr's
//! `app/config.py` validators exactly (the form key is `settings-<section>-<field>`).
//! Every field is `Option`/`Vec`: presence = "manage this key", absence = "leave
//! bazarr's current value untouched" (sparse update).

// config sections (one struct per module)
pub mod auth;
pub mod backup;
pub mod general;
pub mod jellyfin;
pub mod log;
pub mod plex;
pub mod postgresql;
pub mod proxy;
pub mod radarr;
pub mod sonarr;
pub mod subsync;
pub mod translator;

// subtitle providers (one struct per provider, under providers/)
pub mod providers;

// language profiles (nested list) + the languages custom-sync resource
pub mod language_profile;
pub mod languages;

// notification providers (nested list) + the notifications custom-sync resource
pub mod notifications;
pub mod notifier;

// the settings config resource
pub mod settings;
