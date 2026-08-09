use core_lib::SecretValue;
use core_macros::{resource, wire_enum};

use crate::resources::auth_config::AuthConfig;
use crate::resources::logging_config::LoggingConfig;

/// TLS/SSL certificate validation mode for Cleanuparr's outbound HTTP calls
/// (to *arr apps, download clients, and notification targets).
#[wire_enum]
pub enum CertificateValidationType {
    /// Certificates are always validated.
    Enabled,
    /// Certificate validation is skipped for local/private addresses only.
    DisabledForLocalAddresses,
    /// Certificate validation is skipped for all addresses.
    Disabled,
    /// Unknown or future validation mode not yet modelled by this version.
    #[fallback]
    Unknown,
}

/// `/api/configuration/general` — global application behavior, connectivity
/// checks, logging, and authentication settings.
///
/// The GET response additionally carries a server-assigned `id` and other
/// read-only fields not accepted by the PUT contract; those are left
/// unmanaged (`merge(live, desired)` keeps them from the live value).
#[resource(
    sync = singleton,
    read = get("/api/configuration/general"),
    update = put("/api/configuration/general"),
)]
pub struct General {
    /// Shows the "support this project" banner in the Cleanuparr UI.
    #[default(true)]
    pub display_support_banner: bool,
    /// When enabled, Cleanuparr evaluates cleanup rules and logs what it
    /// would do without making any changes to download clients or *arr apps.
    #[default(false)]
    pub dry_run: bool,
    /// Number of times a failed HTTP request to an external service is
    /// retried before giving up.
    #[default(0)]
    pub http_max_retries: i32,
    /// Timeout, in seconds, for outbound HTTP requests to external services.
    #[default(100)]
    pub http_timeout: i32,
    /// TLS/SSL certificate validation mode for outbound HTTP calls.
    pub http_certificate_validation: Option<CertificateValidationType>,
    /// Periodically checks connectivity to configured *arr apps and download
    /// clients and surfaces their status in the UI.
    #[default(true)]
    pub status_check_enabled: bool,
    /// Symmetric key used to encrypt stored credentials at rest.
    // `SecretValue` is a *local* redaction marker, independent of what the API
    // returns: Cleanuparr sends this one in the clear (unlike its
    // `[SensitiveData]` fields), so a diff still sees the real value — we just
    // never print it.
    pub encryption_key: Option<SecretValue>,
    /// Download names or hashes that cleanup rules will never act on.
    pub ignored_downloads: Vec<String>,
    /// Periodically verifies that Cleanuparr can reach the public internet
    /// (used to distinguish a real outage from a misconfiguration).
    #[default(false)]
    pub connectivity_check_enabled: bool,
    /// URLs polled to determine internet connectivity when
    /// `connectivity_check_enabled` is set.
    pub connectivity_check_urls: Vec<String>,
    /// Number of hours of inactivity on a stalled/slow download before a
    /// strike is issued against it.
    #[default(24)]
    pub strike_inactivity_window_hours: i32,
    /// Number of days of cleanup history retained before older entries are
    /// purged.
    #[default(365)]
    pub history_retention_days: i32,
    /// Structured logging and rolling log-file settings.
    pub log: Option<LoggingConfig>,
    /// Authentication bypass and reverse-proxy trust settings.
    pub auth: Option<AuthConfig>,
}
