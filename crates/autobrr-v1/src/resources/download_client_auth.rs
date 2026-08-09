use core_macros::nested;

use core_lib::SecretValue;

/// Auth for a download client's web endpoint. Supersedes the deprecated
/// [`DownloadClientBasic`](crate::resources::download_client_basic::DownloadClientBasic);
/// autobrr translates a set `basic` block into this on write.
#[nested(case = snake)]
pub struct DownloadClientAuth {
    /// Whether auth is required.
    pub enabled: Option<bool>,
    /// Auth scheme: `NONE`, `BASIC_AUTH`, or `DIGEST_AUTH`.
    #[wire(name = "type")]
    pub auth_type: Option<String>,
    /// Auth username.
    pub username: Option<String>,
    /// Auth password.
    pub password: Option<SecretValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_lib::engine;
    use serde_json::json;

    /// `auth_type` is renamed to the wire key `type`; the other fields pass
    /// through under their own names.
    #[test]
    fn auth_type_maps_to_wire_type() {
        let cfg = json!({ "enabled": true, "auth_type": "DIGEST_AUTH", "username": "u" });
        let wire =
            engine::encode(&engine::decode_config::<DownloadClientAuth>(&cfg).unwrap()).unwrap();
        assert_eq!(wire["type"], json!("DIGEST_AUTH"));
        assert_eq!(
            wire.get("auth_type"),
            None,
            "config key must not leak to wire"
        );
        assert_eq!(wire["enabled"], json!(true));
        assert_eq!(wire["username"], json!("u"));
    }
}
