//! LazyLibrarian `/api` response handling.
//!
//! LazyLibrarian's single query-dispatch endpoint returns an **uninterpreted**
//! body — JSON (`listProviders`), a JSON scalar/array (`readCFG` → `[77]`), a bare
//! `OK`, or a **200-with-error-text** string like `Missing parameter: id`. Core's
//! [`HttpClient::get_query`] is a neutral transport (raw `String`); this module
//! owns the LazyLibrarian-specific interpretation, so that opinion lives in the
//! service crate, not the engine.

use core_lib::HttpClient;
use serde_json::Value;

/// GET `/api` with the given query params and turn the raw body into a [`Value`].
/// LazyLibrarian's 200-with-error-text convention is surfaced as a real error
/// (otherwise a bad write would silently read as "success").
pub async fn get(client: &HttpClient, query: &[(&str, &str)]) -> anyhow::Result<Value> {
    let text = client.get_query("/api", query).await?;
    if let Some(err) = error_message(&text) {
        anyhow::bail!("lazylibrarian: {err}");
    }
    Ok(parse_body(text))
}

/// Interpret a `/api` body: empty → [`Value::Null`]; JSON → the parsed value;
/// anything else → the raw text as [`Value::String`] (e.g. `OK`).
pub fn parse_body(text: String) -> Value {
    if text.is_empty() {
        return Value::Null;
    }
    serde_json::from_str(&text).unwrap_or(Value::String(text))
}

/// If the body is one of LazyLibrarian's plain-text error responses (returned with
/// HTTP 200), return the message; otherwise `None`. These prefixes are distinctive
/// — a successful response is JSON (`{`/`[`) or `OK`, never one of these.
fn error_message(text: &str) -> Option<&str> {
    let t = text.trim();
    const ERRS: [&str; 4] = [
        "Missing parameter",
        "Invalid parameter",
        "Invalid id",
        "Unknown command",
    ];
    ERRS.iter().any(|e| t.starts_with(e)).then_some(t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_body_json_text_and_empty() {
        assert_eq!(
            parse_body(r#"{"Success":true}"#.into()),
            json!({ "Success": true })
        );
        assert_eq!(parse_body("[77]".into()), json!([77])); // readCFG numeric
        assert_eq!(parse_body("OK".into()), Value::String("OK".into()));
        assert_eq!(parse_body(String::new()), Value::Null);
    }

    #[test]
    fn error_message_detects_ll_text_errors() {
        assert_eq!(
            error_message("Missing parameter: id"),
            Some("Missing parameter: id")
        );
        assert_eq!(
            error_message("Invalid parameter: name"),
            Some("Invalid parameter: name")
        );
        assert_eq!(
            error_message("Unknown command: foo"),
            Some("Unknown command: foo")
        );
        // real successes are not errors
        assert_eq!(error_message("OK"), None);
        assert_eq!(error_message("[77]"), None);
        assert_eq!(error_message(r#"{"newznab":[]}"#), None);
    }
}
