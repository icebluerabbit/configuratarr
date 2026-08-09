//! Config-file registry + loader.
//!
//! A config file is a mapping of **instance label → service entry**, each entry
//! tagged by `type`. All instances are optional, so a file may hold one service
//! (run the binary once) or many (a mono config) — same code path. Each instance
//! is independent: its own connection, auth, and resource set.
//!
//! ```yaml
//! my-radarr:
//!   type: radarr-v3
//!   url: http://localhost:7878
//!   api_key: secret
//!   tags:
//!     - label: 4k
//! ```
//!
//! This crate owns the `type:` registry — the one place that enumerates the
//! concrete service crates. The generic decode lives in core-lib
//! ([`core_lib::engine::decode_service_config`]); this just dispatches.

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use core_lib::apply::{ApplyOptions, Report};
use core_lib::plan::Plan;
use core_lib::{SystemEnv, resolve};
use serde_json::Value;

pub use core_lib::apply::ApplyOptions as Options;

mod registry;
pub use registry::ServiceInstance;

/// A loaded instance: the typed service (connection + descriptor) plus the
/// resolved config `Value` (desired resource state).
pub struct Instance {
    pub service: ServiceInstance,
    pub config: Value,
}

impl Instance {
    /// Sync this instance against its API.
    pub async fn apply(&self, opts: ApplyOptions) -> Result<Report> {
        self.service.apply(&self.config, opts).await
    }

    /// Preview this instance's changes without writing.
    pub async fn plan(&self, opts: ApplyOptions) -> Result<Plan> {
        self.service.plan(&self.config, opts).await
    }

    /// The service's health-check endpoint, if it declares one.
    pub fn health_check(&self) -> Option<&'static str> {
        self.service.health_check()
    }

    /// Poll the service's health endpoint until ready (or `timeout`).
    pub async fn wait_healthy(&self, timeout: Duration) -> Result<()> {
        self.service.wait_healthy(timeout).await
    }
}

/// A loaded config file: instance label → loaded instance.
pub type ConfigFile = HashMap<String, Instance>;

/// Load and parse a config file (YAML or JSON) from disk.
pub fn load(path: impl AsRef<Path>) -> Result<ConfigFile> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading config {}", path.display()))?;
    load_str(&content)
}

/// Parse config text (YAML — a JSON superset) into the instance map.
///
/// `${env.*}` and `${file.*}` are resolved against the process environment +
/// filesystem before each instance decodes; `${ref.*}` is left for the apply
/// phase (resolved in topological order against ids learned from the server).
pub fn load_str(content: &str) -> Result<ConfigFile> {
    let root: Value = serde_saphyr::from_str(content).context("parsing config")?;
    let map = root.as_object().ok_or_else(|| {
        anyhow::anyhow!("config root must be a mapping of instance label → service")
    })?;

    let env = SystemEnv::new();
    let mut out = HashMap::with_capacity(map.len());
    for (label, entry) in map {
        let mut config = entry.clone();
        resolve::resolve_static(&mut config, &env)
            .with_context(|| format!("instance `{label}`"))?;
        let service =
            ServiceInstance::from_entry(&config).with_context(|| format!("instance `{label}`"))?;
        out.insert(label.clone(), Instance { service, config });
    }
    Ok(out)
}
