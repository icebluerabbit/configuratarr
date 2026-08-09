//! Descriptor-driven runtime for configuratarr resources.
//!
//! Proc macros emit only static descriptors; the engine functions here read them
//! at runtime to encode, decode, resolve, and diff. All logic is hand-written
//! generic Rust in [`engine`] / [`codec`] — macros contribute no executable code
//! beyond field accessors.

pub mod apply;
pub mod codec;
pub mod described;
pub mod descriptor;
pub mod engine;
pub mod field;
pub mod form;
pub mod merge;
pub mod plan;
pub mod reconcile;
pub mod refs;
pub mod resolve;
pub mod resolver;
pub mod secret;
pub mod service;

pub use apply::{Change, ChangeKind, CustomSync, CustomSyncFn, CustomSyncFuture, RefStore};
pub use codec::CodecKind;
pub use core_http::HttpClient;
pub use described::{Described, ResourceErased};
pub use descriptor::{
    Case, DefaultLit, Endpoint, Endpoints, FieldDescriptor, HttpMethod, ResourceDescriptor,
    SyncKind, VariantDescriptor,
};
pub use plan::{Op, PlanStep};

/// Opaque JSON, for API blobs we don't model with typed fields. Backed by
/// `serde_json::Value`; classified as [`FieldKind::Json`].
pub type Json = serde_json::Value;
pub use field::{FieldKind, FieldRef, FieldRole, FieldValue};
pub use refs::RefExpr;
pub use resolver::{RefId, RefSource, StaticEnv, SystemEnv};
pub use secret::SecretValue;
pub use service::{Auth, Connection, Service, ServiceDescriptor, ServiceField};
