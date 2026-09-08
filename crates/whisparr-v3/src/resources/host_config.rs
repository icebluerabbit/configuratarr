use core_lib::SecretValue;
use core_macros::resource;

/// `/api/v3/config/host` — Whisparr host, network, authentication, proxy, and
/// backup settings, plus eros-specific caching / validation / matching
/// behaviour (`whisparr*` fields below).
#[resource(
    sync = singleton,
    read = get("/api/v3/config/host"),
    update = put("/api/v3/config/host/${self.id}"),
)]
pub struct HostConfig {
    #[id]
    pub id: Option<i32>,
    /// IP address or hostname Whisparr binds to; `*` binds to all interfaces.
    pub bind_address: Option<String>,
    /// HTTP port Whisparr listens on.
    pub port: i32,
    /// HTTPS port Whisparr listens on when SSL is enabled.
    pub ssl_port: i32,
    /// Enables HTTPS/TLS for the Whisparr web UI.
    pub enable_ssl: bool,
    /// Opens the Whisparr web UI in the default browser on startup.
    pub launch_browser: bool,
    /// Authentication method for the Whisparr web UI: `none`, `basic`, `forms`, or `external`.
    #[default("none")]
    pub authentication_method: String,
    /// Whether authentication is required: `enabled` or `disabledForLocalAddresses`.
    #[default("enabled")]
    pub authentication_required: String,
    /// Sends anonymised usage and error data to the Whisparr team.
    pub analytics_enabled: bool,
    /// Username for basic or forms authentication.
    pub username: Option<String>,
    /// Password for basic or forms authentication.
    pub password: Option<SecretValue>,
    /// Password confirmation field; must match `password` when changing credentials.
    pub password_confirmation: Option<SecretValue>,
    /// Log verbosity level (e.g. `info`, `debug`, `trace`).
    pub log_level: Option<String>,
    /// Maximum size in MB for each log file before it is rotated.
    pub log_size_limit: i32,
    /// Log level for console output; overrides `log_level` for stdout.
    pub console_log_level: Option<String>,
    /// Update channel or branch Whisparr checks for updates (e.g. `main`, `develop`).
    pub branch: Option<String>,
    /// Whisparr API key used to authenticate API requests.
    pub api_key: Option<SecretValue>,
    /// Absolute path to the SSL certificate file (PEM/PFX).
    pub ssl_cert_path: Option<String>,
    /// Password for the SSL certificate if it is password-protected.
    pub ssl_cert_password: Option<SecretValue>,
    /// URL base path for reverse-proxy deployments (e.g. `/whisparr`).
    pub url_base: Option<String>,
    /// Display name for this Whisparr instance shown in the browser title and notifications.
    pub instance_name: Option<String>,
    /// Externally reachable URL for this instance, used in notifications.
    pub application_url: Option<String>,
    /// Allows Whisparr to update itself automatically when a new version is available.
    pub update_automatically: bool,
    /// How Whisparr applies updates: `builtIn`, `script`, `external`, `apt`, or `docker`.
    #[default("docker")]
    pub update_mechanism: String,
    /// Absolute path to the update script; required when `update_mechanism` is `script`.
    pub update_script_path: Option<String>,
    /// Routes Whisparr's outbound HTTP traffic through a proxy server.
    pub proxy_enabled: bool,
    /// Proxy protocol: `http`, `socks4`, or `socks5`.
    #[default("http")]
    pub proxy_type: String,
    /// Hostname or IP address of the proxy server.
    pub proxy_hostname: Option<String>,
    /// Port of the proxy server.
    pub proxy_port: i32,
    /// Username for proxy authentication.
    pub proxy_username: Option<String>,
    /// Password for proxy authentication.
    pub proxy_password: Option<SecretValue>,
    /// Comma-separated list of hosts or IP ranges that bypass the proxy.
    pub proxy_bypass_filter: Option<String>,
    /// Bypasses the proxy for connections to local/private addresses.
    pub proxy_bypass_local_addresses: bool,
    /// TLS certificate validation mode: `enabled`, `disabledForLocalAddresses`, or `disabled`.
    #[default("enabled")]
    pub certificate_validation: String,
    /// Folder path where Whisparr stores automatic database backups.
    pub backup_folder: Option<String>,
    /// Interval in days between automatic backups.
    pub backup_interval: i32,
    /// Number of days to retain automatic backups before they are deleted.
    pub backup_retention: i32,
    /// Trusts Carrier-Grade NAT (CGNAT) IP address ranges for source IP determination.
    pub trust_cgnat_ip_addresses: bool,
    /// Name of the tag added to a scene left unmonitored because of its performer.
    /// When empty, an import-list exclusion is created instead.
    pub whisparr_always_exclude_performers_tag: Option<String>,
    /// Name of the tag added to a scene left unmonitored because of its studio.
    /// When empty, an import-list exclusion is created instead.
    pub whisparr_always_exclude_studios_tag: Option<String>,
    /// Name of the tag added to a scene left unmonitored because of its studio's
    /// after-date rule. When empty, an import-list exclusion is created instead.
    pub whisparr_always_exclude_studios_after_tag: Option<String>,
    /// Name of the tag added to a scene left unmonitored because of its tags.
    /// When empty, an import-list exclusion is created instead.
    pub whisparr_always_exclude_tags_tag: Option<String>,
    /// Accept a scene on studio and date alone. **Can cause false positives.**
    pub whisparr_auto_match_on_date: bool,
    /// Caches responses from the exclusion metadata API to reduce repeat lookups.
    #[wire(name = "whisparrCacheExclusionAPI")]
    pub whisparr_cache_exclusion_api: bool,
    /// Caches responses from the movie metadata API to reduce repeat lookups.
    #[wire(name = "whisparrCacheMovieAPI")]
    pub whisparr_cache_movie_api: bool,
    /// Caches responses from the performer metadata API to reduce repeat lookups.
    #[wire(name = "whisparrCachePerformerAPI")]
    pub whisparr_cache_performer_api: bool,
    /// Caches responses from the studio metadata API to reduce repeat lookups.
    #[wire(name = "whisparrCacheStudioAPI")]
    pub whisparr_cache_studio_api: bool,
    /// Check the file on import; if it is not found to be valid it fails to
    /// import automatically (it is not re-downloaded).
    pub whisparr_corrupt_file_detection: bool,
    /// Preferred source for movie metadata: `none`, `tmdb`, or `tpdb` (ThePornDB).
    pub whisparr_movie_metadata_source: String,
    /// Extract the runtime from the file and validate it against the scene's
    /// expected duration.
    pub whisparr_validate_runtime: bool,
    /// Runtime tolerance in minutes used by `whisparr_validate_runtime` when
    /// comparing the file against the scene's expected duration.
    pub whisparr_validate_runtime_limit: i32,
}
