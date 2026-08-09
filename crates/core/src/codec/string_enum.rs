//! Unit-enum codec: a `#[wire_enum]` type renders to/from a bare JSON string.
//!
//! Dispatch is the enum's macro-generated `encode_variant` (→ a `Value::String`)
//! / `decode_variant` (matches the incoming string).

use serde_json::Value;

use crate::described::Described;

pub fn encode<T: Described>(value: &T) -> anyhow::Result<Value> {
    value
        .encode_variant()?
        .ok_or_else(|| anyhow::anyhow!("string_enum::encode: type is not a wire_enum"))
}

pub fn decode<T: Described>(value: &Value) -> anyhow::Result<T> {
    let s = value
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("string_enum::decode expected a string, got {value}"))?;
    T::decode_variant(s, value)?.ok_or_else(|| {
        anyhow::anyhow!("string_enum::decode: unknown value `{s}` and no #[fallback]")
    })
}
