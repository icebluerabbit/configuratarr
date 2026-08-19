//! Audiobookshelf resources.
//!
//! ABS is a Node/Express + Sequelize server: **bearer** auth (a JWT access
//! token, not an API key), UUID string ids, camelCase JSON, and no version
//! segment in its paths (`/api/...`).
//!
//! Almost nothing here fits the engine's crud/singleton archetypes, and the
//! reason is nearly always the same: ABS wraps its payloads in an envelope.
//! `GET /api/libraries` returns `{libraries: […]}`, `GET /api/users` returns
//! `{users: […]}`, and the settings endpoints return `{settings: {…}}` while
//! accepting a *flat* PATCH body — so a plain singleton could never converge.
//! Each of those is `sync = custom`, unwraps its envelope, and (for the keyed
//! collections) registers its own ids into the `RefStore`, since the engine's
//! generic post-hook registration only understands a bare-array list.
//!
//! Two further traps worth knowing before editing anything here:
//!
//! * [`library`]'s folder PATCH is a **destructive replace-by-id** — omitting
//!   an existing folder's id deletes it *and* every library item sourced from
//!   it.
//! * [`server_settings`] has no `GET` anywhere in the API (real ABS pushes it
//!   over Socket.IO), so it has no live baseline to diff against.

pub mod email_settings;
pub mod ereader_device;
pub mod library;
pub mod library_folder;
pub mod library_settings;
pub mod media_type;
pub mod notification;
pub mod notification_settings;
pub mod server_settings;
pub mod user;
