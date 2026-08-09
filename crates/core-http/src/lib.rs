use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use secrecy::{ExposeSecret, SecretString};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct HttpClient {
    base_url: String,
    inner: reqwest::Client,
    /// Query parameters stamped onto **every** request (e.g. an api key carried
    /// in the query string rather than a header — LazyLibrarian's `?apikey=`).
    /// `SecretString` so a `{:?}` of the client stays redacted.
    default_query: Vec<(String, SecretString)>,
}

pub struct HttpClientBuilder {
    base_url: String,
    headers: Vec<(&'static str, SecretString)>,
    default_query: Vec<(String, SecretString)>,
    timeout: Duration,
    insecure: bool,
    cookies: bool,
}

impl HttpClient {
    pub fn builder(base_url: impl Into<String>) -> HttpClientBuilder {
        HttpClientBuilder {
            base_url: base_url.into(),
            headers: Vec::new(),
            default_query: Vec::new(),
            timeout: Duration::from_secs(30),
            insecure: false,
            cookies: false,
        }
    }

    /// Parse `url`, append any `extra` query params, then the default query params
    /// (the api key). The returned [`reqwest::Url`] carries the secret key, so
    /// callers keep the plain `url` string for error context — never the merged
    /// URL. The single place the default query is appended, so every verb (and
    /// `get_query`/`post_query`, which pass their own `extra`) stays consistent —
    /// a security-relevant invariant that must not drift between methods. No-op
    /// when both are empty.
    fn merged_url(&self, url: &str, extra: &[(&str, &str)]) -> Result<reqwest::Url> {
        let mut parsed = reqwest::Url::parse(url).with_context(|| format!("parsing URL {url}"))?;
        if !extra.is_empty() || !self.default_query.is_empty() {
            let mut qp = parsed.query_pairs_mut();
            qp.extend_pairs(extra.iter().copied());
            for (k, v) in &self.default_query {
                qp.append_pair(k, v.expose_secret());
            }
        }
        Ok(parsed)
    }

    /// Establish a form/cookie session: POST `pairs` as an
    /// `application/x-www-form-urlencoded` body to `login_path`, so the server's
    /// `Set-Cookie` session lands in this client's cookie store and rides every
    /// subsequent request. The client must have been built with
    /// [`HttpClientBuilder::cookies`] for the cookie to be retained. Non-2xx →
    /// error (surfaces bad credentials at connect time, like the header auths).
    pub async fn login_form(&self, login_path: &str, pairs: &[(String, String)]) -> Result<()> {
        let url = format!("{}{}", self.base_url, login_path);
        let resp = self
            .inner
            .post(self.merged_url(&url, &[])?)
            .form(pairs)
            .send()
            .await
            .with_context(|| format!("POST(login) {url}"))?;
        self.check_status(resp, &url).await
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .get(self.merged_url(&url, &[])?)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        self.parse_json(resp, &url).await
    }

    /// GET with URL **query parameters**, returning the **raw response body** as a
    /// string. reqwest percent-encodes each `(name, value)` pair; the default query
    /// (api key) is appended too. The transport for **query-dispatch** APIs whose
    /// every command — read *and* write — is `GET /api?cmd=…&<params>`. Non-2xx →
    /// error (body included). The body is returned uninterpreted: whether it is
    /// JSON, a bare `OK`, or a service-specific error string is the **caller's** to
    /// decide (see the LazyLibrarian crate). The `path` (without the secret-bearing
    /// query) is used for error context, so credentials never leak into logs. The
    /// GET analog of [`Self::post_query`].
    pub async fn get_query(&self, path: &str, query: &[(&str, &str)]) -> Result<String> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .get(self.merged_url(&url, query)?)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .with_context(|| format!("reading response body from {url}"))?;
        if !status.is_success() {
            anyhow::bail!("HTTP {status}: {text}");
        }
        Ok(text)
    }

    /// POST a JSON body. Writes return their response as opaque JSON (or
    /// [`Value::Null`] when the server sends no content — see [`Self::parse_write`]).
    pub async fn post<B: Serialize>(&self, path: &str, body: &B) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .post(self.merged_url(&url, &[])?)
            .json(body)
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;
        self.parse_write(resp, &url).await
    }

    /// POST an `application/x-www-form-urlencoded` body. Unlike [`Self::post`]
    /// (JSON), the body is a slice of `(name, value)` pairs so **repeated keys**
    /// survive (form semantics: e.g. bazarr's `languages-enabled` appears once
    /// per language). The seam for services whose write contract is a flat form
    /// blob rather than JSON (bazarr's `system/settings`). Response handled like
    /// any other write (see [`Self::parse_write`]).
    pub async fn post_form(&self, path: &str, pairs: &[(String, String)]) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .post(self.merged_url(&url, &[])?)
            .form(pairs)
            .send()
            .await
            .with_context(|| format!("POST(form) {url}"))?;
        self.parse_write(resp, &url).await
    }

    /// POST a JSON body with URL **query parameters**. reqwest percent-encodes
    /// each `(name, value)` pair (repeated names survive, e.g. Jellyfin's
    /// `paths`) — the seam for APIs that carry a resource's identity/params in the
    /// query string rather than the JSON body (Jellyfin libraries keyed by
    /// `name`/`paths`, api keys by `app`), so a hook never hand-builds an escaped
    /// query string. Response handled like any other write.
    pub async fn post_query<B: Serialize>(
        &self,
        path: &str,
        query: &[(&str, &str)],
        body: &B,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .post(self.merged_url(&url, query)?)
            .json(body)
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;
        self.parse_write(resp, &url).await
    }

    /// PUT a JSON body. See [`Self::post`].
    pub async fn put<B: Serialize>(&self, path: &str, body: &B) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .put(self.merged_url(&url, &[])?)
            .json(body)
            .send()
            .await
            .with_context(|| format!("PUT {url}"))?;
        self.parse_write(resp, &url).await
    }

    /// PATCH a JSON body. See [`Self::post`].
    pub async fn patch<B: Serialize>(&self, path: &str, body: &B) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .patch(self.merged_url(&url, &[])?)
            .json(body)
            .send()
            .await
            .with_context(|| format!("PATCH {url}"))?;
        self.parse_write(resp, &url).await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .delete(self.merged_url(&url, &[])?)
            .send()
            .await
            .with_context(|| format!("DELETE {url}"))?;
        self.check_status(resp, &url).await
    }

    /// GET a resource that may legitimately be **unset**, as opaque JSON.
    ///
    /// Non-2xx → error. An empty 2xx body (HTTP 204, or a 200 whose body is
    /// empty because the handler returned a null object) → [`Value::Null`];
    /// otherwise the parsed JSON. [`Self::get`] is strict about this on purpose —
    /// a read that yields nothing is usually a bug — but some APIs use "200 with
    /// no body" to mean "never configured", and a hook has to tell that apart
    /// from a failure. Cleanuparr's per-download-client sub-configs
    /// (`GET /api/unlinked-config/{id}` before anything is saved) answer exactly
    /// that way.
    pub async fn get_optional(&self, path: &str) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .get(self.merged_url(&url, &[])?)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;
        // Same tolerance as a write response: empty body → Null.
        self.parse_write(resp, &url).await
    }

    /// Parse a **read** (GET) response: non-2xx → error, otherwise the body
    /// deserialized to `T`. Strict — an empty body is an error, since a read is
    /// expected to return content. (Writes use [`Self::parse_write`], which
    /// tolerates an empty body; [`Self::get_optional`] is the read that does.)
    async fn parse_json<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
        url: &str,
    ) -> Result<T> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("HTTP {status}: {body}");
        }
        resp.json::<T>()
            .await
            .with_context(|| format!("deserializing response from {url}"))
    }

    /// Parse a **write** (POST/PUT/PATCH) response as opaque JSON. Non-2xx →
    /// error; an empty 2xx body (HTTP 204 or an empty 200 — common for config
    /// writes, e.g. Jellyfin) → [`Value::Null`]; otherwise the parsed JSON. Value-
    /// typed rather than generic: a write's body is only ever read opaquely (a
    /// created id, or ignored), so `Null` is always a valid stand-in — no risk of
    /// coercing `null` into some concrete `T` that can't represent it.
    async fn parse_write(&self, resp: reqwest::Response, url: &str) -> Result<Value> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("HTTP {status}: {body}");
        }
        let bytes = resp
            .bytes()
            .await
            .with_context(|| format!("reading response body from {url}"))?;
        if bytes.is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_slice(&bytes).with_context(|| format!("deserializing response from {url}"))
    }

    async fn check_status(&self, resp: reqwest::Response, _url: &str) -> Result<()> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("HTTP {status}: {body}");
        }
        Ok(())
    }
}

/// Build the rustls config reqwest uses (`use_preconfigured_tls`). We drive TLS
/// ourselves so the trust anchors are the **bundled** webpki roots rather than
/// the OS trust store — reqwest 0.13's default (`rustls-platform-verifier`)
/// reads the system store, which is absent in the CA-less nix build sandbox and
/// aborts client construction. The ring provider avoids aws-lc/cmake.
///
/// `insecure` swaps root verification for a no-op verifier — the opt-in escape
/// hatch for a self-signed *arr instance (the old `danger_accept_invalid_certs`).
fn tls_config(insecure: bool) -> rustls::ClientConfig {
    let builder = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("ring provider supports the default protocol versions");

    if insecure {
        builder
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(danger::NoVerifier))
            .with_no_client_auth()
    } else {
        let mut roots = rustls::RootCertStore::empty();
        roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        builder.with_root_certificates(roots).with_no_client_auth()
    }
}

/// Certificate verifier that accepts anything — only wired in when the caller
/// asks for `insecure`. Isolated in its own module so the `dangerous` surface is
/// obvious and greppable.
mod danger {
    use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
    use rustls::{DigitallySignedStruct, Error, SignatureScheme};

    #[derive(Debug)]
    pub struct NoVerifier;

    impl ServerCertVerifier for NoVerifier {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp_response: &[u8],
            _now: UnixTime,
        ) -> Result<ServerCertVerified, Error> {
            Ok(ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _message: &[u8],
            _cert: &CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            vec![
                SignatureScheme::RSA_PKCS1_SHA256,
                SignatureScheme::RSA_PKCS1_SHA384,
                SignatureScheme::RSA_PKCS1_SHA512,
                SignatureScheme::ECDSA_NISTP256_SHA256,
                SignatureScheme::ECDSA_NISTP384_SHA384,
                SignatureScheme::RSA_PSS_SHA256,
                SignatureScheme::RSA_PSS_SHA384,
                SignatureScheme::RSA_PSS_SHA512,
                SignatureScheme::ED25519,
            ]
        }
    }
}

impl HttpClientBuilder {
    pub fn header(mut self, name: &'static str, value: SecretString) -> Self {
        self.headers.push((name, value));
        self
    }

    /// Register a query parameter stamped onto every request — the seam for
    /// api-key-in-query auth (`Auth::ApiKeyQuery`). Off by default; only
    /// query-auth services set it.
    pub fn query(mut self, name: impl Into<String>, value: SecretString) -> Self {
        self.default_query.push((name.into(), value));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn insecure(mut self) -> Self {
        self.insecure = true;
        self
    }

    /// Enable an in-memory cookie store, so a session cookie set by
    /// [`HttpClient::login_form`] is retained and sent on later requests. Off by
    /// default — only form/cookie-auth services need it.
    pub fn cookies(mut self) -> Self {
        self.cookies = true;
        self
    }

    pub fn build(self) -> Result<HttpClient> {
        let mut headers = HeaderMap::new();
        for (name, value) in &self.headers {
            let key = HeaderName::from_bytes(name.as_bytes())
                .with_context(|| format!("invalid header name: {name}"))?;
            let mut val = HeaderValue::from_str(value.expose_secret())
                .with_context(|| format!("invalid header value for {name}"))?;
            val.set_sensitive(true);
            headers.insert(key, val);
        }

        let inner = reqwest::Client::builder()
            .timeout(self.timeout)
            .default_headers(headers)
            .cookie_store(self.cookies)
            .use_preconfigured_tls(tls_config(self.insecure))
            .build()
            .context("building HTTP client")?;

        Ok(HttpClient {
            base_url: self.base_url,
            inner,
            default_query: self.default_query,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults() {
        let client = HttpClient::builder("http://localhost:7878")
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://localhost:7878");
    }

    #[test]
    fn builder_with_header() {
        let client = HttpClient::builder("http://localhost:7878")
            .header("X-Api-Key", SecretString::new("test-key".into()))
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://localhost:7878");
    }

    #[test]
    fn builder_insecure() {
        let client = HttpClient::builder("https://localhost:7878")
            .insecure()
            .build()
            .unwrap();
        assert_eq!(client.base_url, "https://localhost:7878");
    }

    #[test]
    fn builder_custom_timeout() {
        let client = HttpClient::builder("http://localhost:7878")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://localhost:7878");
    }

    #[test]
    fn builder_with_default_query() {
        let client = HttpClient::builder("http://localhost:5299")
            .query("apikey", SecretString::new("deadbeef".into()))
            .build()
            .unwrap();
        // The default query param is appended to the merged URL, alongside any
        // query already present on the path (e.g. `?cmd=…`) and the call's `extra`.
        let merged = client
            .merged_url("http://localhost:5299/api?cmd=getVersion", &[("name", "X")])
            .unwrap();
        let pairs: Vec<(String, String)> = merged
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        assert!(pairs.contains(&("cmd".into(), "getVersion".into())));
        assert!(pairs.contains(&("name".into(), "X".into())));
        assert!(pairs.contains(&("apikey".into(), "deadbeef".into())));
    }

    #[test]
    fn merged_url_noop_without_default_query_or_extra() {
        let client = HttpClient::builder("http://localhost:7878")
            .build()
            .unwrap();
        let merged = client
            .merged_url("http://localhost:7878/api/v3/tag", &[])
            .unwrap();
        assert_eq!(merged.as_str(), "http://localhost:7878/api/v3/tag");
    }

    #[test]
    fn builder_invalid_header_name_errors() {
        let result = HttpClient::builder("http://localhost:7878")
            .header("invalid header!", SecretString::new("val".into()))
            .build();
        assert!(result.is_err());
    }
}
