//! Enum-flavoured resource macros: `#[tagged]` (discriminator-dispatched
//! provider unions) and `#[wire_enum]` (unit enums rendered as bare strings).
//!
//! Each emits `impl Described` (empty `fields`, populated `variants`) plus the
//! `encode_variant` / `decode_variant` dispatch the codecs call. Always
//! `SyncKind::Embedded`, never a `path` — enums are only ever nested.

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

use crate::attrs::{TaggedArgs, VariantAttrs, WireEnumArgs};
use crate::field_kind::snake_case;
use crate::resource::{doc_to_tokens, emit_described, strip_helper_attrs};

/// `#[tagged(by = "implementation")]` on an enum of typed provider variants.
pub fn expand_tagged(args: TokenStream, input: TokenStream) -> darling::Result<TokenStream> {
    let nested = NestedMeta::parse_meta_list(args)?;
    let targs = TaggedArgs::from_list(&nested)?;
    let by = &targs.by;

    let input: DeriveInput = syn::parse2(input).map_err(Error::from)?;
    let ident = &input.ident;
    let Data::Enum(data) = &input.data else {
        return Err(Error::custom("#[tagged] only supports enums").with_span(ident));
    };

    let mut variants = Vec::new();
    let mut encode_arms = Vec::new();
    let mut decode_arms = Vec::new();
    let mut config_decode_arms = Vec::new();
    let mut fallback_decode: Option<TokenStream> = None;
    let mut fallback_decode_config: Option<TokenStream> = None;
    // empty() candidate: prefer the #[fallback] tuple variant, else the first.
    let mut first_tuple: Option<TokenStream> = None;
    let mut fallback_tuple: Option<TokenStream> = None;

    for v in &data.variants {
        let vattrs = VariantAttrs::parse(&v.attrs)?;
        let vid = &v.ident;
        let name = vid.to_string();
        let inner = tuple_inner(&v.fields);

        // descriptor entry
        let inner_tok = match &inner {
            Some(ty) => {
                let t = snake_case(&type_last_ident(ty));
                quote!(Some(#t))
            }
            None => quote!(None),
        };
        let (wire_tok, fallback) = if vattrs.fallback {
            (quote!(None), true)
        } else {
            let w = vattrs.wire.clone().ok_or_else(|| {
                Error::custom(format!(
                    "tagged variant `{name}` needs `#[variant(\"<implementation>\")]` (or `#[fallback]`)"
                ))
                .with_span(vid)
            })?;
            (quote!(Some(#w)), false)
        };
        // Typed (non-fallback) variants expose their inner fields for doc-gen.
        let field_docs_tok = match (&inner, fallback) {
            (Some(ty), false) => quote!(|| ::core_lib::engine::field_docs::<#ty>()),
            _ => quote!(|| ::std::vec::Vec::new()),
        };
        variants.push(quote! {
            ::core_lib::VariantDescriptor {
                name: #name,
                wire: #wire_tok,
                inner_type: #inner_tok,
                fallback: #fallback,
                field_docs: #field_docs_tok,
            }
        });

        // encode + decode dispatch
        match &inner {
            Some(ty) => {
                let cand = quote!(Self::#vid(<#ty as ::core_lib::Described>::empty()));
                if fallback {
                    fallback_tuple = Some(cand);
                } else if first_tuple.is_none() {
                    first_tuple = Some(cand);
                }
                encode_arms.push(quote!(Self::#vid(__x) => {
                    ::core::result::Result::Ok(Some(::core_lib::engine::encode(__x)?))
                }));
                if fallback {
                    fallback_decode = Some(quote!(::core::result::Result::Ok(Some(
                        Self::#vid(::core_lib::engine::decode::<#ty>(value)?)
                    ))));
                    fallback_decode_config = Some(quote!(::core::result::Result::Ok(Some(
                        Self::#vid(::core_lib::engine::decode_config::<#ty>(value)?)
                    ))));
                } else {
                    let w = vattrs.wire.clone().unwrap();
                    decode_arms.push(quote!(#w => ::core::result::Result::Ok(Some(
                        Self::#vid(::core_lib::engine::decode::<#ty>(value)?)
                    ))));
                    config_decode_arms.push(quote!(#w => ::core::result::Result::Ok(Some(
                        Self::#vid(::core_lib::engine::decode_config::<#ty>(value)?)
                    ))));
                }
            }
            None => {
                // Non-tuple variants (struct/unit) aren't codec-dispatchable yet.
                encode_arms.push(quote!(Self::#vid { .. } => {
                    ::core::result::Result::Err(::anyhow::anyhow!(
                        "encode of non-tuple tagged variant is not supported"
                    ))
                }));
            }
        }
    }

    let fallback_decode = fallback_decode
        .unwrap_or_else(|| quote!(::core::result::Result::Ok(::core::option::Option::None)));
    let fallback_decode_config = fallback_decode_config
        .unwrap_or_else(|| quote!(::core::result::Result::Ok(::core::option::Option::None)));

    let extra = quote! {
        fn encode_variant(&self) -> ::anyhow::Result<::core::option::Option<::serde_json::Value>> {
            match self {
                #(#encode_arms)*
            }
        }
        fn decode_variant(
            discriminator: &str,
            value: &::serde_json::Value,
        ) -> ::anyhow::Result<::core::option::Option<Self>> {
            match discriminator {
                #(#decode_arms,)*
                _ => #fallback_decode,
            }
        }
        fn decode_config_variant(
            discriminator: &str,
            value: &::serde_json::Value,
        ) -> ::anyhow::Result<::core::option::Option<Self>> {
            match discriminator {
                #(#config_decode_arms,)*
                _ => #fallback_decode_config,
            }
        }
    };

    let empty_body = fallback_tuple.or(first_tuple).unwrap_or_else(|| {
        quote!(::core::unimplemented!(
            "tagged enum has no tuple variant to build empty() from"
        ))
    });

    let codec_kind = quote!(::core_lib::CodecKind::TaggedByImpl);
    let codec_meta = quote!(::core_lib::descriptor::CodecMeta::TaggedByImpl { discriminator: #by });
    emit_enum(
        &input,
        ident,
        &codec_kind,
        &codec_meta,
        &variants,
        &empty_body,
        &extra,
    )
}

/// `#[wire_enum(rename_all = "lowercase")]` on a unit enum → a bare-string scalar.
/// A variant may override the derived name with `#[variant("...")]` — for a wire
/// enum whose values don't share one casing rule (e.g. `qBittorrent`/`Deluge`).
pub fn expand_wire_enum(args: TokenStream, input: TokenStream) -> darling::Result<TokenStream> {
    let nested = NestedMeta::parse_meta_list(args)?;
    let wargs = WireEnumArgs::from_list(&nested)?;
    let rule = wargs.rename_all.as_deref();

    let input: DeriveInput = syn::parse2(input).map_err(Error::from)?;
    let ident = &input.ident;
    let Data::Enum(data) = &input.data else {
        return Err(Error::custom("#[wire_enum] only supports enums").with_span(ident));
    };

    let mut variants = Vec::new();
    let mut encode_arms = Vec::new();
    let mut decode_arms = Vec::new();
    let mut fallback_decode: Option<TokenStream> = None;
    let mut first_unit: Option<TokenStream> = None;
    let mut fallback_unit: Option<TokenStream> = None;

    for v in &data.variants {
        let vattrs = VariantAttrs::parse(&v.attrs)?;
        let vid = &v.ident;
        let name = vid.to_string();
        if !matches!(v.fields, Fields::Unit) {
            return Err(Error::custom("#[wire_enum] variants must be unit variants").with_span(vid));
        }
        if vattrs.fallback {
            fallback_unit = Some(quote!(Self::#vid));
        } else if first_unit.is_none() {
            first_unit = Some(quote!(Self::#vid));
        }

        if vattrs.fallback {
            variants.push(quote! {
                ::core_lib::VariantDescriptor {
                    name: #name, wire: None, inner_type: None, fallback: true,
                    field_docs: || ::std::vec::Vec::new(),
                }
            });
            encode_arms.push(quote!(Self::#vid => ::core::result::Result::Err(
                ::anyhow::anyhow!("cannot encode the unknown `{}` variant", #name)
            )));
            fallback_decode = Some(quote!(::core::result::Result::Ok(Some(Self::#vid))));
        } else {
            // `#[variant("...")]` overrides the `rename_all` rule for this one
            // variant — needed when the wire values don't follow a uniform
            // casing rule (Cleanuparr's `DownloadClientTypeName`: `Deluge`,
            // `Transmission` stay PascalCase but `qBittorrent`/`uTorrent`/
            // `rTorrent` are lower-camel). Mirrors `#[tagged]`, where an
            // explicit `#[variant(...)]` always wins over any derived name.
            let w = vattrs
                .wire
                .clone()
                .unwrap_or_else(|| apply_rename(&name, rule));
            variants.push(quote! {
                ::core_lib::VariantDescriptor {
                    name: #name, wire: Some(#w), inner_type: None, fallback: false,
                    field_docs: || ::std::vec::Vec::new(),
                }
            });
            encode_arms
                .push(quote!(Self::#vid => ::core::result::Result::Ok(Some(::serde_json::Value::String(#w.to_string())))));
            decode_arms.push(quote!(#w => ::core::result::Result::Ok(Some(Self::#vid))));
        }
    }

    let fallback_decode = fallback_decode
        .unwrap_or_else(|| quote!(::core::result::Result::Ok(::core::option::Option::None)));

    let extra = quote! {
        fn encode_variant(&self) -> ::anyhow::Result<::core::option::Option<::serde_json::Value>> {
            match self {
                #(#encode_arms,)*
            }
        }
        fn decode_variant(
            discriminator: &str,
            value: &::serde_json::Value,
        ) -> ::anyhow::Result<::core::option::Option<Self>> {
            let _ = value;
            match discriminator {
                #(#decode_arms,)*
                _ => #fallback_decode,
            }
        }
        fn decode_config_variant(
            discriminator: &str,
            value: &::serde_json::Value,
        ) -> ::anyhow::Result<::core::option::Option<Self>> {
            // A unit enum's config form is the same bare string as the wire form.
            Self::decode_variant(discriminator, value)
        }
    };

    let empty_body = fallback_unit.or(first_unit).unwrap_or_else(|| {
        quote!(::core::unimplemented!(
            "wire_enum has no variant to build empty() from"
        ))
    });

    let codec_kind = quote!(::core_lib::CodecKind::StringEnum);
    let codec_meta = quote!(::core_lib::descriptor::CodecMeta::Standard);
    emit_enum(
        &input,
        ident,
        &codec_kind,
        &codec_meta,
        &variants,
        &empty_body,
        &extra,
    )
}

fn emit_enum(
    input: &DeriveInput,
    ident: &syn::Ident,
    codec_kind: &TokenStream,
    codec_meta: &TokenStream,
    variants: &[TokenStream],
    empty_body: &TokenStream,
    extra: &TokenStream,
) -> darling::Result<TokenStream> {
    let doc_tok = doc_to_tokens(&crate::attrs::collect_doc(&input.attrs));
    let type_name = snake_case(&ident.to_string());
    let input_stripped = strip_helper_attrs(input.clone());
    Ok(emit_described(
        &input_stripped,
        ident,
        &quote!(::core_lib::Endpoints::NONE),
        &type_name,
        &doc_tok,
        codec_kind,
        codec_meta,
        &quote!(::core_lib::Case::Camel),
        &quote!(::core_lib::SyncKind::Embedded),
        &[],
        variants,
        empty_body,
        extra,
    ))
}

/// The inner `Type` of a single-field tuple variant, if it is one.
fn tuple_inner(fields: &Fields) -> Option<Type> {
    let Fields::Unnamed(u) = fields else {
        return None;
    };
    if u.unnamed.len() != 1 {
        return None;
    }
    Some(u.unnamed.first().unwrap().ty.clone())
}

fn type_last_ident(ty: &Type) -> String {
    match ty {
        Type::Path(tp) => tp
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default(),
        _ => "unknown".to_string(),
    }
}

/// Apply a serde-style `rename_all` rule to a PascalCase variant ident.
fn apply_rename(name: &str, rule: Option<&str>) -> String {
    match rule {
        None | Some("PascalCase") => name.to_string(),
        Some("lowercase") => name.to_lowercase(),
        Some("UPPERCASE") => name.to_uppercase(),
        Some("snake_case") => snake_case(name),
        Some("SCREAMING_SNAKE_CASE") => snake_case(name).to_uppercase(),
        Some("kebab-case") => snake_case(name).replace('_', "-"),
        Some("camelCase") => {
            let mut c = name.chars();
            match c.next() {
                Some(first) => first.to_lowercase().chain(c).collect(),
                None => String::new(),
            }
        }
        Some(_) => name.to_string(),
    }
}
