//! Autobrr v1 resources.
//!
//! autobrr is **not** an *arr: snake_case JSON, `X-API-Token` header auth, no
//! provider/fields-blob concept. The API is asymmetric — resources differ in
//! which verbs they expose — so the managed surface is:
//!
//! * `download_client` / `proxy` — `sync = crud` (proxy also exposes delete, so
//!   it prunes; download clients don't);
//! * `api_key` + `notification` — create-only (via
//!   [`core_lib::reconcile::create_only`]);
//! * `filter` (two-step create), `indexer` (server-rewritten identifier +
//!   write-only settings), `irc_network` (server-enriched `channels[]` need a
//!   structural-subset diff), and `list` / `feed` (redacted secrets on read → a
//!   structural-subset diff; FKs to download clients / filters / indexers) —
//!   bespoke `sync = custom`, matched on `name`, create + update, no prune.
//!
//! autobrr's app config (`/api/config`) is **not** modelled: it writes
//! `config.toml`, read-only under the NixOS `services.autobrr` module (own it
//! there via `services.autobrr.settings`, not here).

// create-only collections (reconcile::create_only)
pub mod api_key;
pub mod notification;
pub mod notification_event;

// crud collections
pub mod download_client;
pub mod download_client_auth;
pub mod download_client_basic;
pub mod download_client_rules;
pub mod download_client_settings;
pub mod proxy;

// custom-sync resources + their nested types
pub mod action;
pub mod external_filter;
pub mod feed;
pub mod feed_settings;
pub mod filter;
pub mod filter_indexer;
pub mod indexer;
pub mod irc_auth;
pub mod irc_channel;
pub mod irc_network;
pub mod list;
pub mod list_filter;
pub mod release_profile_duplicate;
