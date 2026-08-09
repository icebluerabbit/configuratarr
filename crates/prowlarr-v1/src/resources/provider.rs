//! Shared provider envelope. The plumbing fields identical across every
//! provider-style resource (download clients, indexers, applications, indexer
//! proxies, notifications): identity, tag refs, and the read-only metadata the
//! API returns. Deduped here so each concrete provider struct shows only its
//! *distinct* fields. Hoisted into the parent via `#[flatten]`, so on the wire
//! these stay top-level keys.
//!
//! Identity lives here too: the `#[id]`/`#[key]` on the envelope become the
//! parent resource's id/key when flattened.

use core_lib::Json;
use core_macros::nested;

use crate::resources::provider_message::ProviderMessage;

#[nested]
pub struct Provider {
    #[id]
    pub id: Option<i32>,
    /// Provider instance name — the resource's natural key.
    #[key]
    pub name: String,
    /// Tag references — plain ids, resolved from `${ref.tag.<key>}` at apply.
    #[reference(tag)]
    pub tags: Vec<i32>,
    /// Human-readable implementation label, returned by the API (read-only).
    #[wire(name = "implementationName", read_only)]
    pub implementation_name: Option<String>,
    /// URL to documentation for this provider implementation (read-only).
    #[wire(name = "infoLink", read_only)]
    pub info_link: Option<String>,
    /// Health or status message attached to this provider by the API (read-only).
    #[wire(read_only)]
    pub message: Option<ProviderMessage>,
    /// API-supplied preset templates. Opaque, read-only.
    #[wire(read_only)]
    pub presets: Vec<Json>,
}
