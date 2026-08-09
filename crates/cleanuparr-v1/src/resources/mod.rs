//! Cleanuparr v1 resources.
//!
//! Cleanuparr is **not** an *arr: camelCase JSON (ASP.NET Core web defaults),
//! `X-Api-Key` header auth, GUID ids, and no provider / fields-blob concept. So
//! there is no `Provider` envelope, no `#[fields_blob]`, and no
//! `#[tagged(by = "implementation")]` here — only the engine's four axes applied
//! to what this API actually exposes.
//!
//! The managed surface:
//!
//! * **config singletons** — one endpoint pair each (`GET`/`PUT`), plain
//!   `sync = singleton`. The GET returns more than the PUT contract accepts
//!   (`id`, nested collections); that's harmless, because `merge(live, desired)`
//!   keeps live as the base and the server ignores unknown members.
//! * **`stall_rule` / `slow_rule`** — the only two resources whose endpoints fit
//!   `sync = crud` unchanged (list/create/update/delete over a bare array).
//! * **everything else** — `sync = custom`, because the list endpoint returns a
//!   wrapper object (`{ clients: [...] }`, `{ providers: [...] }`,
//!   `{ instances: [...] }`) rather than a bare array, and/or because writes
//!   don't round-trip (masked secrets, server-rewritten ids). Idempotency for
//!   those is [`crate::diff::subset`].

// config singletons
pub mod blacklist_sync;
pub mod download_cleaner;
pub mod general;
pub mod malware_blocker;
pub mod oidc;
pub mod queue_cleaner;

// crud collections
pub mod slow_rule;
pub mod stall_rule;

// custom-sync families (each owns its own `mod.rs`)
pub mod arr;
pub mod download_client;
pub mod notifications;
pub mod seeker;

// nested types
pub mod auth_config;
pub mod blocklist_settings;
pub mod failed_import_config;
pub mod logging_config;
