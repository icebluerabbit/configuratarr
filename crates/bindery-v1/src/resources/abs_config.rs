//! `/api/v1/abs/config` — the Audiobookshelf import source configuration.
//!
//! The read shape (`ABSConfigResponse`) reports whether a secret is stored
//! via `apiKeyConfigured: bool`; the write shape (`ABSConfigRequest`) takes
//! the API key itself, and an empty/omitted key leaves the stored one alone.
//! A plain singleton merge would compare `apiKeyConfigured: true` against the
//! write side's `apiKey` field forever and PUT on every apply, so this is
//! `sync = custom` — see [`crate::resources::config_secret`] for the shared
//! decision this hook delegates to.
//!
//! `featureEnabled` (a build/deployment flag) and `libraryId` (server-derived
//! — always the first entry of `libraryIds`) are read-only response fields
//! this resource never writes; only `libraryIds` is managed, and its first
//! entry becomes `libraryId` server-side.
//!
//! Caveat: an in-place API-key rotation with an otherwise-identical config is
//! not detected — `apiKeyConfigured` stays `true` across a rotation, so
//! nothing here signals that the stored key is stale.

use core_lib::{Change, CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine};
use core_macros::resource;
use serde_json::Value;

use crate::resources::config_secret::{self, SecretConfigChange};

const PATH: &str = "/api/v1/abs/config";
const SECRET_KEYS: &[&str] = &["apiKey"];
const CONFIGURED_KEYS: &[(&str, &str)] = &[("apiKey", "apiKeyConfigured")];

/// `/api/v1/abs/config` — the Audiobookshelf source configuration.
/// Singleton: exactly one entry, no natural key.
#[resource(sync = custom, list = get("/api/v1/abs/config"))]
pub struct AbsConfig {
    /// Audiobookshelf base URL; normalized server-side before storage.
    pub base_url: Option<String>,
    /// Display label for the source; an empty value is stored as
    /// "Audiobookshelf".
    pub label: Option<String>,
    /// Whether this source may be imported from.
    pub enabled: Option<bool>,
    /// Target book library ids, de-duplicated and in the given order. The
    /// first entry becomes the server's read-only `libraryId`. Not an
    /// `Option` — `Vec<Option<..>>`/`Option<Vec<..>>` aren't a modelled field
    /// shape; an absent config key is still masked out by
    /// `config_present_to_wire`, same as any other unmanaged field.
    pub library_ids: Vec<String>,
    /// Path remap rule applied to ABS-reported file paths so they resolve
    /// under Bindery-visible storage.
    pub path_remap: Option<String>,
    /// Audiobookshelf API key. An absent/empty value leaves the stored key
    /// untouched.
    pub api_key: Option<SecretValue>,
}

impl CustomSync for AbsConfig {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let Some(cfg) = desired.first() else {
                return Ok(Vec::new());
            };
            let wire = engine::config_present_to_wire::<Self>(cfg)?;
            let live: Value = client.get(PATH).await?;

            match config_secret::plan_secret_config(&wire, &live, SECRET_KEYS, CONFIGURED_KEYS) {
                SecretConfigChange::Unchanged => Ok(vec![Change::unchanged("abs_config")]),
                SecretConfigChange::Full(body) => {
                    if execute {
                        let _: Value = client.put(PATH, &body).await?;
                    }
                    Ok(vec![Change::updated("abs_config")])
                }
                SecretConfigChange::SecretOnly(body) => {
                    if execute {
                        let _: Value = client.put(PATH, &body).await?;
                    }
                    Ok(vec![
                        Change::updated("abs_config").with("apiKey", "rotated"),
                    ])
                }
            }
        })
    }
}
