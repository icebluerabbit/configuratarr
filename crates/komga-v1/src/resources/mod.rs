//! Komga v1 resources.
//!
//! Komga is **not** an *arr: a Kotlin/Spring server with opaque **string** ids,
//! camelCase JSON, `X-API-Key` header auth (HTTP Basic is the bootstrap path
//! only), and no provider/fields-blob concept. Its API also straddles two
//! versions — libraries and settings live under `/api/v1`, users and API keys
//! under `/api/v2`.
//!
//! The managed surface:
//!
//! * [`library`] — `sync = crud`. The one archetype-clean resource here: a
//!   bare-array list, a string `id`, `name` as the natural key, and `PATCH` for
//!   the update.
//! * [`user`] — `sync = custom`. Read exposes flat `sharedAllLibraries` +
//!   `sharedLibrariesIds`; create/update take a nested `sharedLibraries`
//!   object. `password` never reads back, and a cleared age restriction reads
//!   as an absent field rather than the `NONE` sentinel it is written as.
//! * [`settings`] — `sync = custom` singleton. Three fields read as
//!   `{configurationSource, databaseSource, effectiveValue}` triples but write
//!   as plain scalars, so a plain singleton would diff forever.
//! * [`api_key`] — `sync = custom`, create + prune only. Komga exposes no
//!   update verb, and the secret is returned exactly once, on create.
//! * [`client_setting_global`] / [`client_setting_user`] — `sync = custom`.
//!   Dotted-key maps, not arrays: the list endpoint returns a JSON object and
//!   writes are a whole-map `PATCH` that upsert-merges.

pub mod age_restriction;
pub mod api_key;
pub mod client_setting_global;
pub mod client_setting_user;
pub mod client_settings;
pub mod library;
pub mod scan_interval;
pub mod series_cover;
pub mod settings;
pub mod shared_libraries;
pub mod user;
