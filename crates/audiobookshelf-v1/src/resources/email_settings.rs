//! `/api/emails/settings` — SMTP configuration for outgoing e-book/e-reader
//! email. Managed alongside, but distinct from, the `ereaderDevices` array
//! that lives inside the same server object — that array is owned
//! exclusively by [`crate::resources::ereader_device::EreaderDevice`], which
//! writes it wholesale via `POST /api/emails/ereader-devices`.
//!
//! **Not `sync = singleton`, despite `GET`/`PATCH` sharing the same
//! `EmailSettings` field set.** Per the raw spec, the *response* for both
//! `GET` and `PATCH /api/emails/settings` is wrapped: `{"settings":
//! EmailSettings}`. The `PATCH` *request* body, though, is flat — `host`,
//! `port`, `secure`, ... with no `"settings"` wrapper. A plain `sync =
//! singleton` (`core_lib::apply::singleton_step`) takes the raw `GET`
//! response as `live` verbatim and merges the flat `desired` keys onto it:
//! since `live`'s only top-level key is `"settings"`, none of `desired`'s
//! flat keys ever land on an existing `live` key, so the merged PATCH body
//! can never equal `live` again — every apply would report a change,
//! forever, even with nothing to do. That is the *exact* failure mode
//! already diagnosed and fixed for
//! [`crate::resources::notification_settings::NotificationSettings`] (see
//! its module doc: "A plain `sync = singleton` merges the flat desired keys
//! onto the envelope root ... and would PATCH on every apply — forever") —
//! same envelope-vs-flat shape, same fix: `sync = custom`, unwrap
//! `.settings`, presence-mask the declared keys, diff, conditionally PATCH.
//!
//! `pass` is echoed back **unmasked** on every `GET` (unlike a typical
//! write-only credential), so it is a plain, comparable field for `in_sync`
//! — modeled `SecretValue` purely so plan output redacts it in *our*
//! display, not because the API itself hides it.

use core_lib::{
    Change, CustomSync, CustomSyncFuture, HttpClient, Json, RefStore, SecretValue, engine,
};
use core_macros::resource;
use serde_json::Value;

/// SMTP configuration for outgoing mail. See <https://nodemailer.com/smtp/>.
#[resource(sync = custom, list = get("/api/emails/settings"))]
pub struct EmailSettings {
    /// Always the literal `email-settings` — this singleton's fixed id.
    /// There is no reason to declare it in config.
    // Modeled (rather than left off, or marked `#[id]`) only so a full,
    // unmasked encode carries the key the `EmailSettings` response schema
    // requires. The write path is presence-masked, so it is never sent
    // unless a config explicitly declares it.
    #[default("email-settings")]
    pub id: String,
    /// SMTP host. Unset/`null` disables outgoing email.
    pub host: Option<String>,
    /// SMTP port. Only port 465 is treated as implicit TLS (`secure`); any
    /// other port forces `secure: false` at connection time regardless of
    /// the stored `secure` value. API default `465`.
    #[default(465)]
    pub port: i32,
    /// Whether to use TLS. Coerced to a strict boolean server-side. API
    /// default `true`.
    #[default(true)]
    pub secure: bool,
    /// Whether to reject self-signed/invalid TLS certificates. API default
    /// `true`.
    #[default(true)]
    pub reject_unauthorized: bool,
    /// SMTP auth username.
    pub user: Option<String>,
    /// SMTP auth password. Stored and echoed back **in plaintext** on
    /// subsequent `GET`s (`x-sensitive` in the spec, but not write-only) —
    /// see the module doc for why this is nonetheless modeled `SecretValue`.
    pub pass: Option<SecretValue>,
    /// Recipient used by `POST /api/emails/test`; falls back to
    /// `from_address` if unset.
    pub test_address: Option<String>,
    /// The `From:` address for outgoing mail.
    pub from_address: Option<String>,
    /// Registered e-reader devices, as `GET` reports them — informational
    /// only. Manage them under `ereader_devices` at the top level, not here;
    /// declaring the field here has no effect.
    // Opaque `Json` rather than a typed mirror of `EreaderDevice`: it is
    // never written from this resource, and presence-masking means a
    // declared value is dropped anyway.
    pub ereader_devices: Vec<Json>,
}

/// `want` (a declared wire value) vs `have` (the live value at the same key,
/// if any) — numeric-insensitive so a declared `i32` still compares equal to
/// whatever JSON number shape the live settings carry it as. Mirrors
/// [`crate::resources::notification_settings`]'s `field_in_sync`.
fn field_in_sync(want: &Value, have: Option<&Value>) -> bool {
    match have {
        None => want.is_null(),
        Some(h) => match (want.as_f64(), h.as_f64()) {
            (Some(a), Some(b)) => a == b,
            _ => want == h,
        },
    }
}

/// Every key the presence-masked `wire` declares already matches
/// `.settings` at the same key.
fn in_sync(wire: &Value, live_settings: &Value) -> bool {
    let Some(obj) = wire.as_object() else {
        return true;
    };
    obj.iter()
        .all(|(k, want)| field_in_sync(want, live_settings.get(k)))
}

impl CustomSync for EmailSettings {
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

            // Decode into the typed struct (validate, drop unknown keys),
            // presence-mask to declared keys, emit camelCase wire — the same
            // step a stock `singleton` gets for free, and the reason this
            // hook can PATCH the flat body directly (see the module doc).
            let wire = engine::config_present_to_wire::<Self>(cfg)?;

            let envelope: Value = client.get("/api/emails/settings").await?;
            let live_settings = envelope.get("settings").cloned().unwrap_or(Value::Null);

            if in_sync(&wire, &live_settings) {
                return Ok(vec![Change::unchanged("email_settings")]);
            }

            if execute {
                // Exactly the flat body the route accepts (see the module
                // doc): `wire` never carries a `"settings"` wrapper.
                let _: Value = client.patch("/api/emails/settings", &wire).await?;
            }
            Ok(vec![Change::updated("email_settings")])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn in_sync_numeric_insensitive() {
        let wire = json!({"port": 465});
        let live = json!({"port": 465.0});
        assert!(in_sync(&wire, &live));
    }

    #[test]
    fn absent_live_key_only_in_sync_when_declared_null() {
        assert!(in_sync(&json!({"host": null}), &json!({})));
        assert!(!in_sync(&json!({"port": 465}), &json!({})));
    }

    #[test]
    fn in_sync_catches_a_real_diff() {
        let wire = json!({"host": "smtp.example.com"});
        let live = json!({"host": "smtp.other.com"});
        assert!(!in_sync(&wire, &live));
    }
}
