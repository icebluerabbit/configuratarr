use core_macros::nested;

/// An external filter check — autobrr calls a webhook or runs a command and
/// gates the release on the result.
#[nested(case = snake)]
pub struct ExternalFilter {
    /// Server-assigned id.
    pub id: Option<i32>,
    /// Display name.
    pub name: Option<String>,
    /// Evaluation order among external checks.
    pub index: Option<i32>,
    /// Check kind: `EXEC` or `WEBHOOK`.
    #[wire(name = "type")]
    pub external_type: Option<String>,
    /// Whether the check is active.
    pub enabled: Option<bool>,
    /// Webhook URL (`WEBHOOK` type).
    pub webhook_host: Option<String>,
    /// HTTP method for the webhook.
    pub webhook_method: Option<String>,
    /// Request body sent to the webhook.
    pub webhook_data: Option<String>,
    /// Extra webhook headers (`Key=value,Key2=value2`).
    pub webhook_headers: Option<String>,
    /// HTTP status the webhook must return to pass.
    pub webhook_expect_status: Option<i32>,
    /// HTTP status(es) that trigger a retry.
    pub webhook_retry_status: Option<String>,
    /// Number of times to retry the webhook.
    pub webhook_retry_attempts: Option<i32>,
    /// Delay between webhook retries, seconds.
    pub webhook_retry_delay_seconds: Option<i32>,
    /// Command to run (`EXEC` type).
    pub exec_cmd: Option<String>,
    /// Arguments passed to the command.
    pub exec_args: Option<String>,
    /// Exit status the command must return to pass.
    pub exec_expect_status: Option<i32>,
    /// Behaviour when the check errors: `CONTINUE` or `REJECT`.
    pub on_error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_lib::engine;
    use serde_json::json;

    /// The newly-modelled webhook/retry/on-error fields encode under their
    /// snake_case wire keys with values passed through verbatim.
    #[test]
    fn new_fields_encode_to_wire() {
        let cfg = json!({
            "name": "size-check",
            "external_type": "WEBHOOK",
            "enabled": true,
            "webhook_host": "http://localhost:9000/check",
            "webhook_method": "POST",
            "webhook_headers": "X-Api-Key=abc,Accept=application/json",
            "webhook_expect_status": 200,
            "webhook_retry_status": "500,502,503",
            "webhook_retry_attempts": 3,
            "webhook_retry_delay_seconds": 5,
            "on_error": "REJECT",
        });
        let wire = engine::encode(&engine::decode_config::<ExternalFilter>(&cfg).unwrap()).unwrap();
        assert_eq!(wire["type"], json!("WEBHOOK"));
        assert_eq!(
            wire["webhook_headers"],
            json!("X-Api-Key=abc,Accept=application/json")
        );
        assert_eq!(wire["webhook_retry_status"], json!("500,502,503"));
        assert_eq!(wire["webhook_retry_attempts"], json!(3));
        assert_eq!(wire["webhook_retry_delay_seconds"], json!(5));
        assert_eq!(wire["on_error"], json!("REJECT"));
    }
}
