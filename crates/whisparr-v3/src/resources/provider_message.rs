//! Embedded sub-resource template (`#[nested]`): never synced on its own, has
//! no path and no sync strategy. Only ever appears flattened/nested inside a
//! parent resource's descriptor.

use core_macros::nested;

/// Read-only health/status message the API attaches to a provider.
#[nested]
pub struct ProviderMessage {
    /// The health or status message text.
    pub message: Option<String>,
    /// Severity or category of the message, e.g. `"warning"`, `"error"`.
    #[wire(name = "type")]
    pub message_type: Option<String>,
}
