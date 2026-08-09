//! `/api/notification` — outbound notification targets (Discord, Telegram, …).
//!
//! `sync = custom`, keyed by `name`: GET lists, `POST /api/notification` creates,
//! `PUT /api/notification/{id}` updates, and `--prune` deletes agents the config
//! no longer declares via `DELETE /api/notification/{id}`
//! ([`core_lib::reconcile::upsert_prune`]). The credential fields (`api_key`,
//! `token`, `password`) are redacted on read, so idempotency is the
//! structural-subset test on the readable fields ([`crate::diff::subset`]) — a
//! declared secret reads back redacted and so re-applies as an update. The write
//! body is the full typed notification, encoded through the descriptor (maps
//! `notification_type` → the wire `type`, and carries the credential fields).

use core_lib::engine;
use core_lib::reconcile;
use core_lib::{CustomSync, CustomSyncFuture, HttpClient, Json, RefStore, SecretValue};
use core_macros::resource;
use serde_json::Value;

use crate::diff;
use crate::resources::notification_event::NotificationEvent;

/// `/api/notification` — a notification target.
#[resource(sync = custom, case = snake, list = get("/api/notification"))]
pub struct Notification {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Display name — its identity.
    #[key]
    pub name: String,
    /// Provider kind. One of: `DISCORD`, `NOTIFIARR`, `IFTTT`, `JOIN`,
    /// `MATTERMOST`, `MATRIX`, `PUSH_BULLET`, `PUSHOVER`, `ROCKETCHAT`, `SLACK`,
    /// `TELEGRAM`, `GOTIFY`, `NTFY`, `LUNASEA`, `SHOUTRRR`, `WEBHOOK`.
    #[wire(name = "type")]
    pub notification_type: String,
    /// Whether the target is active.
    pub enabled: bool,
    /// Events that trigger this notification.
    pub events: Vec<NotificationEvent>,
    /// Webhook URL (Discord and similar).
    pub webhook: Option<String>,
    /// Provider API key / auth token where required.
    pub api_key: Option<SecretValue>,
    /// Bot token (Telegram and similar).
    pub token: Option<SecretValue>,
    /// Target channel / chat id.
    pub channel: Option<String>,
    /// Topic (ntfy and similar).
    pub topic: Option<String>,
    /// Provider host (self-hosted Gotify/ntfy).
    pub host: Option<String>,
    /// Message title (ntfy and similar).
    pub title: Option<String>,
    /// Icon / avatar override.
    pub icon: Option<String>,
    /// Username for auth-protected providers (ntfy, Matrix).
    pub username: Option<String>,
    /// Password for auth-protected providers.
    pub password: Option<SecretValue>,
    /// Rooms to post to (Matrix).
    pub rooms: Option<String>,
    /// Explicit targets (Shoutrrr and similar).
    pub targets: Option<String>,
    /// Target device names (Pushover).
    pub devices: Option<String>,
    /// Message priority (Pushover / Gotify / ntfy).
    pub priority: Option<i32>,
    /// Notification sound (Pushover / ntfy).
    pub sound: Option<String>,
    /// HTTP method (generic webhook providers).
    pub method: Option<String>,
    /// Extra headers (generic webhook providers).
    pub headers: Option<String>,
    /// Per-event sound overrides as an `EVENT: sound` map (Pushover / ntfy),
    /// e.g. `{ PUSH_APPROVED: "magic", PUSH_REJECTED: "falling" }`. Overrides
    /// `sound` for the listed events only.
    pub event_sounds: Option<Json>,
}

impl CustomSync for Notification {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let live: Vec<Value> = client.get("/api/notification").await?;
            // Full desired wire (id is read-only, so absent here). Encode through
            // the descriptor so `type`/secrets land right.
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            reconcile::upsert_prune(
                &wire,
                &live,
                "name",
                diff::subset,
                prune,
                execute,
                |w| {
                    let client = client.clone();
                    async move {
                        let _: Value = client.post("/api/notification", &w).await?;
                        Ok(())
                    }
                },
                |l, mut w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    // autobrr updates by path id; echo it into the body too so a
                    // body-id read still resolves.
                    reconcile::echo(&mut w, "id", l);
                    async move {
                        let _: Value = client.put(&format!("/api/notification/{id}"), &w).await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    async move {
                        client.delete(&format!("/api/notification/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}
