//! Provider-resource: metadata. Composes the shared `Provider` envelope with the
//! typed `MetadataProvider` variant, plus the metadata-specific `enable` flag.
//! Both composed pieces use `#[flatten]`, so their keys sit top-level on the wire.

use core_macros::resource;

use crate::resources::metadata_providers::MetadataProvider;
use crate::resources::provider::Provider;

/// Metadata consumer — instructs Lidarr to write sidecar metadata files and
/// artwork alongside downloaded media using a specific plugin.
// Create/update use `?forceSave=true`: the API otherwise runs a live connectivity
// test against the remote service on save and rejects with HTTP 400 when it is
// unreachable from this host or rate-limiting. A declarative sync must converge to
// the desired config regardless; the app still surfaces the failing health check.
#[resource(
    sync = crud,
    list = get("/api/v1/metadata"),
    create = post("/api/v1/metadata?forceSave=true"),
    update = put("/api/v1/metadata/${self.id}?forceSave=true"),
    delete = delete("/api/v1/metadata/${self.id}"),
)]
pub struct Metadata {
    /// Identity (id + name), tag refs, read-only API metadata.
    #[flatten]
    pub common: Provider,
    /// The typed per-implementation settings (fields-blob).
    #[flatten]
    pub config: MetadataProvider,
    /// Whether this metadata consumer is active.
    pub enable: bool,
}
