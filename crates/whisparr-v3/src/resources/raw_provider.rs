//! Raw fallback for a provider implementation we don't model with a typed
//! variant. Carries the discriminators plus the open `fields` settings as a
//! `name: value` map so an unknown provider still round-trips losslessly.
//! Target of the `#[fallback]` arm on each provider enum.

use core_lib::Json;
use core_macros::nested;

/// Fallback representation for a provider implementation with no typed struct.
///
/// Carries the discriminator fields and the open settings map so an unknown
/// provider can still round-trip losslessly.
#[nested]
pub struct RawProvider {
    /// Provider implementation class name, e.g. `"Transmission"`.
    pub implementation: String,
    /// Configuration contract class name that governs the fields schema.
    #[wire(name = "configContract")]
    pub config_contract: Option<String>,
    /// Open provider settings authored as a `name: value` map. On the wire this
    /// becomes the *arr `fields: [{name, value}]` blob (via `#[fields_map]`);
    /// `${env}`/`${ref}` resolve inside the values, and the whole map is
    /// redacted (rendered `Complex`) in plan output.
    #[fields_map]
    pub fields: Json,
}
