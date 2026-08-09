use core_macros::nested;

use core_lib::SecretValue;

use crate::resources::download_client_auth::DownloadClientAuth;
use crate::resources::download_client_basic::DownloadClientBasic;
use crate::resources::download_client_rules::DownloadClientRules;

/// Client-specific settings nested under a [`DownloadClient`](crate::resources::download_client::DownloadClient).
#[nested(case = snake)]
pub struct DownloadClientSettings {
    /// API key (arr-style clients that authenticate by key rather than user/pass).
    pub apikey: Option<SecretValue>,
    /// HTTP basic-auth credentials (deprecated — prefer `auth`).
    pub basic: Option<DownloadClientBasic>,
    /// Auth for the client's web endpoint (supersedes `basic`).
    pub auth: Option<DownloadClientAuth>,
    /// Throughput/queue rules applied before pushing releases.
    pub rules: Option<DownloadClientRules>,
    /// Id of another download client to delegate to (proxy setups).
    pub external_download_client_id: Option<i32>,
    /// Name of another download client to delegate to (proxy setups).
    pub external_download_client: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_lib::engine;
    use serde_json::json;

    /// The modern `auth` block and the `external_download_client` name field
    /// both round-trip; `auth.auth_type` lands under the nested wire key `type`.
    #[test]
    fn settings_carry_auth_and_external_client() {
        let cfg = json!({
            "auth": { "enabled": true, "auth_type": "BASIC_AUTH", "username": "u" },
            "external_download_client": "other-client",
            "rules": { "enabled": true, "ignore_slow_torrents_condition": "MAX_DOWNLOADS_REACHED" },
        });
        let wire = engine::encode(&engine::decode_config::<DownloadClientSettings>(&cfg).unwrap())
            .unwrap();
        assert_eq!(wire["auth"]["type"], json!("BASIC_AUTH"));
        assert_eq!(wire["auth"]["enabled"], json!(true));
        assert_eq!(wire["external_download_client"], json!("other-client"));
        assert_eq!(
            wire["rules"]["ignore_slow_torrents_condition"],
            json!("MAX_DOWNLOADS_REACHED")
        );
    }
}
