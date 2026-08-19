//! Bindery v1 resources.
//!
//! Bindery is a Go + chi book manager (the modern Readarr replacement) — **not**
//! an *arr, despite the family resemblance: camelCase JSON, `X-Api-Key` header
//! auth, `int64` ids at the literal wire key `id`, bare-array list endpoints,
//! and no provider/fields-blob concept anywhere.
//!
//! Eight resources are archetype-clean `sync = crud`: [`custom_format`],
//! [`metadata_profile`], [`quality_profile`], [`download_client`], [`indexer`],
//! [`prowlarr_instance`], [`notification`], and [`root_folder`] (which has no
//! update endpoint — legal only because its sole writable field *is* its key).
//!
//! The rest each break an engine assumption and are `sync = custom`:
//!
//! * [`delay_profile`] — the schema has **no name field**, so there is no
//!   possible natural key; the whole list is structurally replaced.
//! * [`import_list`] — `apiKey` is write-only (blanked in every response) and
//!   the update body is a different schema than the create body.
//! * [`import_list_exclusion`] — `DELETE` only, no update route.
//! * [`setting`] — the `/api/v1/setting/{key}` key/value table. Reads omit
//!   secret keys entirely rather than masking them, so a secret key is
//!   indistinguishable from an unset one.
//! * [`auth_mode`] — read and write live at different paths *and* different
//!   shapes; a merged singleton body would ship the instance API key.
//! * [`oidc_provider`] — one bulk array PUT with a server-side secret merge.
//! * [`user`] — no whole-object update; role and password have their own
//!   sub-paths.
//! * [`abs_config`] / [`grimmory_config`] — the read shape reports
//!   `apiKeyConfigured` booleans where the write shape takes the secret itself
//!   (shared helper in [`config_secret`]).

pub mod abs_config;
pub mod auth_mode;
pub mod config_secret;
pub mod custom_condition;
pub mod custom_format;
pub mod delay_profile;
pub mod download_client;
pub mod grimmory_config;
pub mod import_list;
pub mod import_list_exclusion;
pub mod indexer;
pub mod metadata_profile;
pub mod notification;
pub mod oidc_provider;
pub mod prowlarr_instance;
pub mod quality_item;
pub mod quality_profile;
pub mod root_folder;
pub mod setting;
pub mod unknown_language_behavior;
pub mod user;
