//! `/api/configuration/{sonarr,radarr,lidarr,readarr,whisparr}` — the five
//! *arr applications Cleanuparr talks to.
//!
//! Each app is a pair of resources:
//!
//! * an app-config singleton (`SonarrConfig`, …) — `sync = singleton` over
//!   `GET`/`PUT /api/configuration/<app>`, one managed field
//!   (`failed_import_max_strikes`). The `GET` response is the richer
//!   `ArrConfigDto` (`id`, `type`, `instances`); those extra keys are left
//!   alone by `merge(live, desired)`.
//! * an instance collection (`SonarrInstance`, …) — `sync = custom`, because
//!   the same `GET /api/configuration/<app>` wraps its instance list in that
//!   envelope object rather than exposing a bare array, and writes don't
//!   round-trip (`apiKey` reads back masked). The shared reconcile lives in
//!   [`reconcile`]; each app's `CustomSync::reconcile` is a two-line
//!   delegation into it.
//!
//! Every instance's server-assigned GUID is registered into the `RefStore`
//! under a fixed ref type name per app — `sonarr_instance`, `radarr_instance`,
//! `lidarr_instance`, `readarr_instance`, `whisparr_instance` — so another
//! resource can address it via `${ref.<app>_instance.<name>}`.

pub mod reconcile;

pub mod lidarr;
pub mod radarr;
pub mod readarr;
pub mod sonarr;
pub mod whisparr;
