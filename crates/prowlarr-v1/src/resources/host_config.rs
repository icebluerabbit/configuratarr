use core_lib::SecretValue;
use core_macros::resource;

use crate::resources::authentication_required_type::AuthenticationRequiredType;
use crate::resources::authentication_type::AuthenticationType;
use crate::resources::certificate_validation_type::CertificateValidationType;
use crate::resources::proxy_type::ProxyType;
use crate::resources::update_mechanism::UpdateMechanism;

/// `/api/v1/config/host` — Prowlarr host, network, authentication, proxy, and backup settings.
#[resource(
    sync = singleton,
    read = get("/api/v1/config/host"),
    update = put("/api/v1/config/host/${self.id}"),
)]
pub struct HostConfig {
    #[id]
    pub id: Option<i32>,
    /// IP address or hostname Prowlarr binds to; `*` binds to all interfaces.
    pub bind_address: Option<String>,
    /// HTTP port Prowlarr listens on.
    pub port: i32,
    /// HTTPS port Prowlarr listens on when SSL is enabled.
    pub ssl_port: i32,
    /// Enables HTTPS/TLS for the Prowlarr web UI.
    pub enable_ssl: bool,
    /// Opens the Prowlarr web UI in the default browser on startup.
    pub launch_browser: bool,
    /// Authentication method for the Prowlarr web UI.
    pub authentication_method: Option<AuthenticationType>,
    /// Whether authentication is required for local network addresses.
    pub authentication_required: Option<AuthenticationRequiredType>,
    /// Sends anonymised usage and error data to the Prowlarr team.
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
    /// Update channel or branch Prowlarr checks for updates (e.g. `main`, `develop`).
    pub branch: Option<String>,
    /// Prowlarr API key used to authenticate API requests.
    pub api_key: Option<SecretValue>,
    /// Absolute path to the SSL certificate file (PEM/PFX).
    pub ssl_cert_path: Option<String>,
    /// Password for the SSL certificate if it is password-protected.
    pub ssl_cert_password: Option<SecretValue>,
    /// URL base path for reverse-proxy deployments (e.g. `/prowlarr`).
    pub url_base: Option<String>,
    /// Display name for this Prowlarr instance shown in the browser title and notifications.
    pub instance_name: Option<String>,
    /// Externally reachable URL for this instance, used in notifications.
    pub application_url: Option<String>,
    /// Allows Prowlarr to update itself automatically when a new version is available.
    pub update_automatically: bool,
    /// How Prowlarr applies updates.
    pub update_mechanism: Option<UpdateMechanism>,
    /// Absolute path to the update script; required when `update_mechanism` is `Script`.
    pub update_script_path: Option<String>,
    /// Routes Prowlarr's outbound HTTP traffic through a proxy server.
    pub proxy_enabled: bool,
    /// Proxy protocol used for outbound connections.
    pub proxy_type: Option<ProxyType>,
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
    /// TLS certificate validation mode for outbound connections.
    pub certificate_validation: Option<CertificateValidationType>,
    /// Folder path where Prowlarr stores automatic database backups.
    pub backup_folder: Option<String>,
    /// Interval in days between automatic backups.
    pub backup_interval: i32,
    /// Number of days to retain automatic backups before they are deleted.
    pub backup_retention: i32,
    /// Number of days to retain indexer search history entries.
    pub history_cleanup_days: i32,
    /// Trusts Carrier-Grade NAT (CGNAT) IP address ranges for source IP determination.
    pub trust_cgnat_ip_addresses: bool,
}
