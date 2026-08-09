//! `/api/irc` — IRC networks autobrr connects to for announce parsing.
//!
//! `sync = custom`. A generic crud diff can't keep IRC idempotent: autobrr
//! enriches each `channels[]` entry on read with server fields (`id`, `detached`,
//! `monitoring`, …), and the whole-array merge would replace the live rich array
//! with the declared minimal one on every apply — a perpetual "update". So, like
//! [`Filter`](crate::resources::filter::Filter), this hook diffs the declared
//! config as a **structural subset** of live (extra server fields ignored) and
//! only writes on real drift.
//!
//! Live state is read per network from `GET /api/irc/network/{id}`, not from the
//! `GET /api/irc` list — the list is a health view whose `channels[]` comes from
//! the running IRC handler (see [`CustomSync::reconcile`]).
//!
//! Write paths are irregular: create is `POST /api/irc`, update is
//! `PUT /api/irc/network/{id}`, delete is `DELETE /api/irc/network/{id}` (a bare
//! `/api/irc/{id}` 404s). Under `--prune`, networks the config no longer declares
//! are deleted ([`core_lib::reconcile::upsert_prune`]). Credentials (`pass`,
//! `auth.password`, channel keys) are write-only — returned `<redacted>`, so a
//! declared value reads as drift and re-applies as an update.

use core_lib::{
    CustomSync, CustomSyncFuture, HttpClient, RefStore, SecretValue, engine, reconcile,
};
use core_macros::resource;
use serde_json::Value;

use crate::diff;
use crate::resources::irc_auth::IrcAuth;
use crate::resources::irc_channel::IrcChannel;

/// `/api/irc` — an IRC network with its channels and auth.
#[resource(sync = custom, case = snake, list = get("/api/irc"))]
pub struct IrcNetwork {
    /// Server-assigned id.
    #[id]
    pub id: Option<i32>,
    /// Network name — its identity (`${ref.irc_network.<name>}`).
    #[key]
    pub name: String,
    /// Whether autobrr connects to this network.
    pub enabled: bool,
    /// IRC server hostname.
    pub server: String,
    /// IRC server port.
    pub port: i32,
    /// Connect over TLS.
    pub tls: Option<bool>,
    /// Skip TLS certificate verification.
    pub tls_skip_verify: Option<bool>,
    /// Bot nick to use on the network.
    pub nick: String,
    /// Server password (PASS), where required (write-only).
    pub pass: Option<SecretValue>,
    /// NickServ / SASL authentication.
    pub auth: Option<IrcAuth>,
    /// Command sent to request an invite (e.g. `/msg gatekeeper !invite`).
    pub invite_command: Option<String>,
    /// Connect through a bouncer instead of directly.
    pub use_bouncer: Option<bool>,
    /// Bouncer address, where `use_bouncer` is set.
    pub bouncer_addr: Option<String>,
    /// Enable IRCv3 bot mode.
    pub bot_mode: Option<bool>,
    /// Route this network through a proxy.
    pub use_proxy: Option<bool>,
    /// Proxy to route through (`${ref.proxy.<name>}`).
    #[reference(proxy)]
    pub proxy_id: Option<i32>,
    /// Channels to join.
    pub channels: Vec<IrcChannel>,
    /// Runtime: current connection health (read-only).
    #[wire(read_only)]
    pub healthy: Option<bool>,
    /// Runtime: whether currently connected (read-only).
    #[wire(read_only)]
    pub connected: Option<bool>,
    /// Runtime: the nick currently in use (read-only).
    #[wire(read_only)]
    pub current_nick: Option<String>,
}

impl CustomSync for IrcNetwork {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let listed: Vec<Value> = client.get("/api/irc").await?;
            // `GET /api/irc` is a *health* view: since autobrr 1.82 it builds an
            // enabled network's `channels[]` from the running IRC handler's
            // in-memory set, not the database — so a network that isn't connected
            // (or whose handler hasn't seeded yet) reports `channels: []` even
            // though the rows exist. Diffing against that would see the declared
            // channels as missing on every apply and PUT forever, and each PUT
            // deletes and re-inserts the channel rows. `GET /api/irc/network/{id}`
            // returns the stored network, so re-read each one for the diff.
            let mut live: Vec<Value> = Vec::with_capacity(listed.len());
            for l in &listed {
                let id = l.get("id").cloned().unwrap_or(Value::Null);
                live.push(client.get(&format!("/api/irc/network/{id}")).await?);
            }
            // Full desired wire (read-only runtime fields omitted by encode).
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
                        let _: Value = client.post("/api/irc", &w).await?;
                        Ok(())
                    }
                },
                |l, mut w| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    // autobrr's update reads the id from the body (WHERE id=?);
                    // `#[id]` omits it on encode, so echo the live id back. The
                    // update path is irregular (`/api/irc/network/{id}`, not
                    // `/api/irc/{id}`, which 404s).
                    reconcile::echo(&mut w, "id", l);
                    async move {
                        let _: Value = client.put(&format!("/api/irc/network/{id}"), &w).await?;
                        Ok(())
                    }
                },
                |l| {
                    let client = client.clone();
                    let id = l.get("id").cloned().unwrap_or(Value::Null);
                    // Delete shares the irregular network path.
                    async move {
                        client.delete(&format!("/api/irc/network/{id}")).await?;
                        Ok(())
                    }
                },
            )
            .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Every channel carries `enabled` on the wire, declared or not: autobrr's
    /// channel struct has a plain `bool`, so an absent key decodes to `false`
    /// and its join workflow then skips the channel. A bare channel must
    /// therefore encode `enabled: true` from the field default.
    #[test]
    fn channels_always_encode_enabled() {
        let cfg = json!({
            "name": "tl",
            "enabled": true,
            "server": "irc.example.org",
            "port": 6697,
            "nick": "mybot",
            "channels": [
                { "name": "#announce" },
                { "name": "#offtopic", "enabled": false },
            ],
        });
        let wire = engine::encode_config::<IrcNetwork>(&cfg).unwrap();
        assert_eq!(wire["channels"][0]["enabled"], json!(true));
        assert_eq!(wire["channels"][1]["enabled"], json!(false));
    }
}
