use core_macros::nested;

/// PostgreSQL backend settings (`settings-postgresql-*`).
///
/// When `enabled`, bazarr stores its database in PostgreSQL instead of the
/// bundled SQLite. Changing the backend takes effect on restart.
#[nested(case = snake)]
pub struct Postgresql {
    /// Use PostgreSQL instead of the bundled SQLite database.
    pub enabled: Option<bool>,
    /// PostgreSQL host.
    pub host: Option<String>,
    /// PostgreSQL port.
    pub port: Option<i32>,
    /// Database name.
    pub database: Option<String>,
    /// Database username.
    pub username: Option<String>,
    /// Database password.
    pub password: Option<String>,
    /// Full connection URL (overrides the discrete host/port/… fields).
    pub url: Option<String>,
}
