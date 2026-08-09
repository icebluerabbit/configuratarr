//! Discriminator-dispatched enum codec.
//!
//! Reads a string field (default `"implementation"`) from the JSON object and
//! delegates to the matching variant's inner codec. Encoding delegates to the
//! active variant, whose inner fields-blob codec stamps the discriminator.
//! Dispatch is provided by the enum's macro-generated `encode_variant` /
//! `decode_variant`.

use serde_json::Value;

use crate::described::Described;
use crate::descriptor::CodecMeta;

pub fn encode<T: Described>(value: &T) -> anyhow::Result<Value> {
    value
        .encode_variant()?
        .ok_or_else(|| anyhow::anyhow!("tagged::encode: type is not an enum with variants"))
}

pub fn decode<T: Described>(value: &Value) -> anyhow::Result<T> {
    let discriminator = match &T::descriptor().codec_meta {
        CodecMeta::TaggedByImpl { discriminator } => *discriminator,
        _ => anyhow::bail!("tagged::decode on a non-TaggedByImpl descriptor"),
    };
    let disc = value
        .get(discriminator)
        .and_then(Value::as_str)
        .ok_or_else(|| {
            anyhow::anyhow!("tagged::decode: missing discriminator `{discriminator}`")
        })?;

    T::decode_variant(disc, value)?.ok_or_else(|| {
        anyhow::anyhow!("tagged::decode: no variant for `{disc}` and no #[fallback]")
    })
}
