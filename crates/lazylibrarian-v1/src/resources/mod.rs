//! LazyLibrarian v1 resources.
//!
//! LazyLibrarian is a query-dispatch API (`/api?cmd=…`) with **no** REST CRUD, so
//! every resource here is `sync = custom`:
//!
//! * [`config`] — the full configuration, a bazarr-style custom singleton whose
//!   `Option<Section>` fields are diffed per-variable via `readCFG`/`writeCFG`.
//! * [`providers`] — the six provider families (newznab/torznab/rss/irc/direct/
//!   apprise), each an upsert-by-`DISPNAME` collection driven by
//!   `addProvider`/`changeProvider`/`delProvider` and listed via `listProviders`.
//! * [`magazine`] — magazines keyed by `Title`; create via `addMagazine`, prune
//!   via `removeMagazine`, listed via `getMagazines`.
//! * [`author`] — authors keyed by `AuthorID`; create via `addAuthor`/`addAuthorID`,
//!   prune via `removeAuthor`, listed via `getIndex`.
//!
//! Runtime/content commands (books, issues, comics, history, wanted, searches,
//! scans, lifecycle) are **not** modelled — they are library content, not config.

pub mod author;
pub mod config;
pub mod magazine;
pub mod providers;
