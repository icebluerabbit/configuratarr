//! The service registry: the `ServiceInstance` dispatch enum, macro-generated
//! from the shared service table in `service_registry`.
//!
//! This module owns core-config's *shape* for the registry — the enum plus the
//! per-method dispatch. The *list* of services lives in
//! [`service_registry::service_registry`]; `config-doc-gen` expands that same
//! list into its own shape. Adding a service is one row there (plus an
//! optional-dep + feature line in this crate's manifest).

use std::time::Duration;

use anyhow::Result;
use core_lib::apply::{ApplyOptions, Report, apply, plan, wait_healthy};
use core_lib::plan::Plan;
use core_lib::{Service, engine};
use serde_json::Value;

/// Generate the `ServiceInstance` registry enum + dispatch from the shared
/// service table (see [`service_registry::service_registry`]). Each row becomes
/// a variant plus the arms of every dispatch method; `#[cfg(feature = <tag>)]`
/// gates the service so a single-service build drops the rest.
macro_rules! gen_instance {
    ($($v:ident => $tag:literal : $ty:path = $title:literal),+ $(,)?) => {
        /// One configured service instance. Variants mirror the `type:` tags.
        /// **Adding a service is one row in [`service_registry::service_registry`]**
        /// (plus an optional-dep + feature line in this crate's manifest).
        ///
        /// Holds the *connection-only* typed service (resources stay empty —
        /// apply is value-driven). The desired resource state lives in
        /// [`crate::Instance::config`].
        pub enum ServiceInstance {
            $(
                #[cfg(feature = $tag)]
                $v(Box<$ty>),
            )+
        }

        impl ServiceInstance {
            /// Dispatch one config entry on its `type:` tag to the matching service.
            pub(crate) fn from_entry(value: &Value) -> Result<Self> {
                let ty = value
                    .get("type")
                    .and_then(Value::as_str)
                    .ok_or_else(|| anyhow::anyhow!("service entry missing `type`"))?;
                Ok(match ty {
                    $(
                        #[cfg(feature = $tag)]
                        $tag => Self::$v(Box::new(
                            engine::decode_service_config::<$ty>(value)?,
                        )),
                    )+
                    other => anyhow::bail!("unknown service type `{other}`"),
                })
            }

            /// The service type tag, for diagnostics.
            pub fn type_name(&self) -> &'static str {
                match self {
                    $(
                        #[cfg(feature = $tag)]
                        Self::$v(_) => $tag,
                    )+
                }
            }

            /// Sync this instance against its API. `config` is the resolved
            /// desired state (the raw entry `Value`).
            pub async fn apply(&self, config: &Value, opts: ApplyOptions) -> Result<Report> {
                match self {
                    $(
                        #[cfg(feature = $tag)]
                        Self::$v(s) => apply(s.as_ref(), config, opts).await,
                    )+
                }
            }

            /// Preview this instance's changes without writing. `config` is the
            /// resolved desired state (the raw entry `Value`).
            pub async fn plan(&self, config: &Value, opts: ApplyOptions) -> Result<Plan> {
                match self {
                    $(
                        #[cfg(feature = $tag)]
                        Self::$v(s) => plan(s.as_ref(), config, opts).await,
                    )+
                }
            }

            /// The service's health-check endpoint, if it declares one.
            pub fn health_check(&self) -> Option<&'static str> {
                match self {
                    $(
                        #[cfg(feature = $tag)]
                        Self::$v(_) => <$ty as Service>::descriptor().health_check,
                    )+
                }
            }

            /// Poll the service's health endpoint until ready (or `timeout`).
            /// No-op if it declares no health endpoint.
            pub async fn wait_healthy(&self, timeout: Duration) -> Result<()> {
                match self {
                    $(
                        #[cfg(feature = $tag)]
                        Self::$v(s) => wait_healthy(s.as_ref(), timeout).await,
                    )+
                }
            }
        }
    };
}

service_registry::service_registry!(gen_instance);
