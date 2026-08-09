//! LazyLibrarian v1 service crate.
//!
//! LazyLibrarian is **not** an *arr and **not** REST. Its entire API is a single
//! query-dispatch endpoint — `GET /api?apikey=<key>&cmd=<command>&<params>`. Two
//! consequences shape this crate:
//!
//! * **Auth is the api key in the query string** (`?apikey=`), so the service
//!   declares `auth = api_key(query = "apikey")` — the engine's `Auth::ApiKeyQuery`
//!   seam stamps it onto every request, including the health probe.
//! * **No resource has REST CRUD** (a path + verb + JSON body). Every managed
//!   resource is therefore `sync = custom`, the same seam bazarr (settings
//!   singleton) and autobrr (upsert/create-only collections) use. There is no
//!   stock `crud`/`singleton` resource in this crate.
//!
//! Managed surface (config only — books/issues/comics/history are runtime content,
//! excluded like radarr excludes `MovieResource`): the full `Config` singleton,
//! the six provider families (newznab/torznab/rss/irc/direct/apprise), magazines,
//! and authors.

pub mod http;
pub mod resources;
pub mod service;

pub use service::LazyLibrarianV1;
