use core_macros::fields_blob;

/// FlareSolverr indexer proxy configuration.
///
/// Routes requests through a FlareSolverr instance to bypass Cloudflare and
/// similar bot-protection layers for indexers that require it.
#[fields_blob(
    implementation = "Flaresolverr",
    config_contract = "FlaresolverrSettings"
)]
pub struct FlaresolverrConfig {
    /// URL of the FlareSolverr instance, e.g. `http://localhost:8191`.
    pub host: Option<String>,
    /// Request timeout in seconds sent to the FlareSolverr session.
    #[wire(name = "requestTimeout")]
    pub request_timeout: Option<i32>,
}
