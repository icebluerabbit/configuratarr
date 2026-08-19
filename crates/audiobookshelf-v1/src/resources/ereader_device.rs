//! `/api/emails/ereader-devices` — an e-reader device that can receive EPUBs
//! by email. Devices have no server id; `name` is their whole identity (per
//! the spec's own `EreaderDeviceObject` doc: "there is no separate device
//! id").
//!
//! The only write, `POST /api/emails/ereader-devices`, carries the **entire**
//! array — a textbook [`core_lib::reconcile::replace`]: compute an
//! order-insensitive identity for the desired and live sets and, if they
//! differ, POST the whole desired array. There is no per-item id and no
//! per-item endpoint, so **`--prune` is meaningless here** — the replace
//! write already *is* the full authoritative set; a device dropped from
//! config is already gone the moment the array is next POSTed, with no
//! separate delete step to gate. `prune` is accepted (and ignored) purely
//! because [`CustomSync::reconcile`]'s signature always carries it.
//!
//! Live values come from `GET /api/emails/settings` → `.settings.ereaderDevices`
//! — there's no dedicated list endpoint for devices alone. This mirrors
//! [`crate::resources::email_settings::EmailSettings`], which reads (and,
//! separately, owns writing) the rest of that same object's fields; the two
//! resources never race because each only ever touches its own slice of the
//! object (`EmailSettings`' hook presence-masks away `ereaderDevices`
//! entirely — see its module doc).

use core_lib::{CustomSync, CustomSyncFuture, HttpClient, RefStore, engine, reconcile};
use core_macros::{resource, wire_enum};
use serde_json::{Value, json};

/// Who may send to a device via `POST /api/emails/send-ebook-to-device`.
/// Server-normalized on write: a missing/invalid value, or `specific_users`
/// without a non-empty `users` list, both fall back to `admin_or_up`.
#[wire_enum]
pub enum EreaderAvailability {
    /// Admins and the root user.
    #[variant("adminOrUp")]
    AdminOrUp,
    /// Any logged-in non-guest user, and up.
    #[variant("userOrUp")]
    UserOrUp,
    /// Any logged-in user, including guests.
    #[variant("guestOrUp")]
    GuestOrUp,
    /// Only the users listed in `users`.
    #[variant("specificUsers")]
    SpecificUsers,
}

/// `/api/emails/ereader-devices` — one e-reader device.
#[resource(sync = custom, list = get("/api/emails/settings"))]
pub struct EreaderDevice {
    /// Unique display name — this device's whole identity. Also the name
    /// `POST /api/emails/send-ebook-to-device` targets it by.
    #[key]
    pub name: String,
    /// Destination email address for this device.
    pub email: String,
    /// Who may send to this device. Unset ⇒ server default `adminOrUp`.
    pub availability_option: Option<EreaderAvailability>,
    /// User ids allowed to use the device; meaningful only when
    /// `availability_option` is `specific_users` (the server clears this to
    /// `[]` otherwise). `${ref.user.<username>}` resolves to the referenced
    /// user's server id.
    #[reference(user)]
    pub users: Vec<String>,
}

/// Order-insensitive identity for the whole-array replace: `name`, `email`,
/// the *effective* `availability_option` (server default-normalized, so
/// compare `adminOrUp` when the key is simply absent rather than treating
/// absence itself as a distinguishing value), and the sorted `users` set —
/// matching the task's stated identity, `(name, email, availabilityOption,
/// sorted users)`.
fn identity(v: &Value) -> (String, String, String, Vec<String>) {
    let name = v
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let email = v
        .get("email")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let availability = v
        .get("availabilityOption")
        .and_then(Value::as_str)
        .unwrap_or("adminOrUp")
        .to_string();
    let mut users: Vec<String> = v
        .get("users")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|u| u.as_str().map(str::to_string))
        .collect();
    users.sort();
    (name, email, availability, users)
}

impl CustomSync for EreaderDevice {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        _refs: &'a mut RefStore,
        _prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a> {
        Box::pin(async move {
            let envelope: Value = client.get("/api/emails/settings").await?;
            let live: Vec<Value> = envelope
                .get("settings")
                .and_then(|s| s.get("ereaderDevices"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            // Full typed wire per declared device (no read-only fields to
            // strip — every field here is part of the write).
            let wire: Vec<Value> = desired
                .iter()
                .map(engine::encode_config::<Self>)
                .collect::<anyhow::Result<_>>()?;

            let body = json!({ "ereaderDevices": wire.clone() });
            reconcile::replace(&wire, &live, "ereader_devices", execute, identity, || {
                let client = client.clone();
                async move {
                    let _: Value = client.post("/api/emails/ereader-devices", &body).await?;
                    Ok(())
                }
            })
            .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identity_normalizes_absent_availability_and_sorts_users() {
        let a = identity(&json!({
            "name": "kindle",
            "email": "a@kindle.com",
            "users": ["u2", "u1"],
        }));
        let b = identity(&json!({
            "name": "kindle",
            "email": "a@kindle.com",
            "availabilityOption": "adminOrUp",
            "users": ["u1", "u2"],
        }));
        assert_eq!(a, b);
    }

    #[test]
    fn identity_catches_a_real_diff() {
        let a = identity(&json!({"name": "kindle", "email": "a@kindle.com"}));
        let b = identity(&json!({"name": "kindle", "email": "b@kindle.com"}));
        assert_ne!(a, b);
    }
}
