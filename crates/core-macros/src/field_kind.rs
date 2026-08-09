//! Maps Rust field types to `core::FieldKind` / `core::FieldRef` / `core::FieldValue`
//! token streams.
//!
//! Structural and conservative: scalars, `Option`, `Vec`, `SecretValue`, opaque
//! `Json`, and nested resources; anything else is treated as a nested resource.
//! There are no resolution-wrapper types — refs are plain `i32` carrying
//! `#[reference(..)]` metadata, resolved on the Value tree before decode, so the
//! classifier never sees a wrapper.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, PathArguments, Type, TypePath};

/// Classified shape of a single field, sufficient to emit the descriptor's
/// `kind`, `get`, and `set` entries.
#[derive(Debug)]
pub enum Kind {
    Bool,
    Int32,
    Int64,
    Float64,
    String,

    OptBool,
    OptInt32,
    OptInt64,
    OptFloat64,
    OptString,

    VecBool,
    VecInt32,
    VecInt64,
    VecString,

    /// `SecretValue` — structurally a string, redacted in memory.
    Secret,
    /// `Option<SecretValue>`.
    OptSecret,

    /// `Option<T>` where T is a nested resource.
    OptNested {
        type_name: String,
    },

    /// Any other path type — assumed to be a nested resource (`T: Described`)
    /// whose type_name is the snake-cased struct ident.
    Nested {
        type_name: String,
    },

    /// `Vec<T>` where T is a nested resource.
    VecNested {
        type_name: String,
    },

    /// `Json` (alias of `serde_json::Value`) — opaque passthrough.
    Json,
    /// `Option<Json>`.
    OptJson,
    /// `Vec<Json>`.
    VecJson,
}

impl Kind {
    /// Whether this field is a credential (`SecretValue`), driving the
    /// descriptor's `secret` flag.
    pub fn is_secret(&self) -> bool {
        matches!(self, Self::Secret | Self::OptSecret)
    }

    /// Emit the `&'static FieldKind` reference for the descriptor.
    pub fn to_kind_expr(&self) -> TokenStream {
        match self {
            Self::Bool => quote!(&::core_lib::FieldKind::Bool),
            Self::Int32 => quote!(&::core_lib::FieldKind::Int32),
            Self::Int64 => quote!(&::core_lib::FieldKind::Int64),
            Self::Float64 => quote!(&::core_lib::FieldKind::Float64),
            Self::String => quote!(&::core_lib::FieldKind::String),

            Self::OptBool => quote!(&::core_lib::FieldKind::Optional(
                &::core_lib::FieldKind::Bool
            )),
            Self::OptInt32 => quote!(&::core_lib::FieldKind::Optional(
                &::core_lib::FieldKind::Int32
            )),
            Self::OptInt64 => quote!(&::core_lib::FieldKind::Optional(
                &::core_lib::FieldKind::Int64
            )),
            Self::OptFloat64 => quote!(&::core_lib::FieldKind::Optional(
                &::core_lib::FieldKind::Float64
            )),
            Self::OptString => quote!(&::core_lib::FieldKind::Optional(
                &::core_lib::FieldKind::String
            )),

            Self::VecBool => quote!(&::core_lib::FieldKind::Vec(&::core_lib::FieldKind::Bool)),
            Self::VecInt32 => quote!(&::core_lib::FieldKind::Vec(&::core_lib::FieldKind::Int32)),
            Self::VecInt64 => quote!(&::core_lib::FieldKind::Vec(&::core_lib::FieldKind::Int64)),
            Self::VecString => quote!(&::core_lib::FieldKind::Vec(&::core_lib::FieldKind::String)),

            // Secrets are structurally strings; the `secret` flag carries the rest.
            Self::Secret => quote!(&::core_lib::FieldKind::String),
            Self::OptSecret => quote!(&::core_lib::FieldKind::Optional(
                &::core_lib::FieldKind::String
            )),

            Self::Nested { type_name } => {
                quote!(&::core_lib::FieldKind::Nested { type_name: #type_name })
            }
            Self::OptNested { type_name } => quote!(
                &::core_lib::FieldKind::Optional(&::core_lib::FieldKind::Nested { type_name: #type_name })
            ),
            Self::VecNested { type_name } => quote!(
                &::core_lib::FieldKind::Vec(&::core_lib::FieldKind::Nested { type_name: #type_name })
            ),

            Self::Json => quote!(&::core_lib::FieldKind::Json),
            Self::OptJson => quote!(&::core_lib::FieldKind::Optional(
                &::core_lib::FieldKind::Json
            )),
            Self::VecJson => quote!(&::core_lib::FieldKind::Vec(&::core_lib::FieldKind::Json)),
        }
    }

    /// Emit the `get: fn(&T) -> FieldRef<'_>` closure body for the descriptor.
    pub fn to_get_expr(&self, field: &syn::Ident) -> TokenStream {
        match self {
            Self::Bool => quote!(::core_lib::FieldRef::Bool(&t.#field)),
            Self::Int32 => quote!(::core_lib::FieldRef::Int32(&t.#field)),
            Self::Int64 => quote!(::core_lib::FieldRef::Int64(&t.#field)),
            Self::Float64 => quote!(::core_lib::FieldRef::Float64(&t.#field)),
            Self::String => quote!(::core_lib::FieldRef::String(&t.#field)),

            Self::OptBool => quote!(::core_lib::FieldRef::OptBool(&t.#field)),
            Self::OptInt32 => quote!(::core_lib::FieldRef::OptInt32(&t.#field)),
            Self::OptInt64 => quote!(::core_lib::FieldRef::OptInt64(&t.#field)),
            Self::OptFloat64 => quote!(::core_lib::FieldRef::OptFloat64(&t.#field)),
            Self::OptString => quote!(::core_lib::FieldRef::OptString(&t.#field)),

            Self::VecBool => quote!(::core_lib::FieldRef::VecBool(&t.#field)),
            Self::VecInt32 => quote!(::core_lib::FieldRef::VecInt32(&t.#field)),
            Self::VecInt64 => quote!(::core_lib::FieldRef::VecInt64(&t.#field)),
            Self::VecString => quote!(::core_lib::FieldRef::VecString(&t.#field)),

            Self::Secret => quote!(::core_lib::FieldRef::Secret(&t.#field)),
            Self::OptSecret => quote!(::core_lib::FieldRef::OptSecret(&t.#field)),

            Self::Nested { .. } => quote!(::core_lib::FieldRef::Nested(&t.#field)),
            Self::OptNested { .. } => quote!({
                // Erased view for an Option<Nested>. Standard codec skips None
                // for optionals; the placeholder None covers the absent case.
                match &t.#field {
                    Some(__inner) => ::core_lib::FieldRef::Nested(__inner),
                    None => ::core_lib::FieldRef::OptString(&::std::option::Option::None),
                }
            }),
            Self::VecNested { .. } => quote!(::core_lib::FieldRef::VecNested(
                ::std::boxed::Box::new(
                    t.#field.iter().map(|r| r as &dyn ::core_lib::ResourceErased)
                )
            )),

            Self::Json => quote!(::core_lib::FieldRef::Json(&t.#field)),
            Self::OptJson => quote!(::core_lib::FieldRef::OptJson(&t.#field)),
            Self::VecJson => quote!(::core_lib::FieldRef::VecJson(&t.#field)),
        }
    }

    /// Emit an empty/default initializer for this field, used by the generated
    /// `Described::empty()` constructor. `ty` is the field's Rust type (needed
    /// to call `Described::empty()` on nested resources).
    pub fn to_empty_expr(&self, ty: &syn::Type) -> TokenStream {
        match self {
            Self::Bool => quote!(false),
            Self::Int32 => quote!(0i32),
            Self::Int64 => quote!(0i64),
            Self::Float64 => quote!(0.0f64),
            Self::String => quote!(::std::string::String::new()),

            Self::OptBool
            | Self::OptInt32
            | Self::OptInt64
            | Self::OptFloat64
            | Self::OptString
            | Self::OptSecret
            | Self::OptNested { .. }
            | Self::OptJson => {
                quote!(::std::option::Option::None)
            }

            Self::VecBool
            | Self::VecInt32
            | Self::VecInt64
            | Self::VecString
            | Self::VecNested { .. }
            | Self::VecJson => quote!(::std::vec::Vec::new()),

            Self::Secret => quote!(::core_lib::SecretValue::new(::std::string::String::new())),

            Self::Nested { .. } => quote!(<#ty as ::core_lib::Described>::empty()),

            Self::Json => quote!(::serde_json::Value::Null),
        }
    }

    /// Emit the `set: fn(&mut T, FieldValue) -> Result<()>` closure body.
    /// `ty` is the field's Rust type, used to call the concrete decode for
    /// nested resources.
    pub fn to_set_expr(&self, field: &syn::Ident, ty: &syn::Type) -> TokenStream {
        match self {
            Self::Bool => quote!(match v {
                ::core_lib::FieldValue::Bool(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected Bool, got {other:?}")),
            }),
            Self::Int32 => quote!(match v {
                ::core_lib::FieldValue::Int32(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected Int32, got {other:?}")),
            }),
            Self::Int64 => quote!(match v {
                ::core_lib::FieldValue::Int64(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected Int64, got {other:?}")),
            }),
            Self::Float64 => quote!(match v {
                ::core_lib::FieldValue::Float64(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected Float64, got {other:?}")),
            }),
            Self::String => quote!(match v {
                ::core_lib::FieldValue::String(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected String, got {other:?}")),
            }),

            Self::OptBool => quote!(match v {
                ::core_lib::FieldValue::OptBool(x) => { t.#field = x; Ok(()) }
                ::core_lib::FieldValue::Null => { t.#field = None; Ok(()) }
                other => Err(::anyhow::anyhow!("expected OptBool, got {other:?}")),
            }),
            Self::OptInt32 => quote!(match v {
                ::core_lib::FieldValue::OptInt32(x) => { t.#field = x; Ok(()) }
                ::core_lib::FieldValue::Null => { t.#field = None; Ok(()) }
                other => Err(::anyhow::anyhow!("expected OptInt32, got {other:?}")),
            }),
            Self::OptInt64 => quote!(match v {
                ::core_lib::FieldValue::OptInt64(x) => { t.#field = x; Ok(()) }
                ::core_lib::FieldValue::Null => { t.#field = None; Ok(()) }
                other => Err(::anyhow::anyhow!("expected OptInt64, got {other:?}")),
            }),
            Self::OptFloat64 => quote!(match v {
                ::core_lib::FieldValue::OptFloat64(x) => { t.#field = x; Ok(()) }
                ::core_lib::FieldValue::Null => { t.#field = None; Ok(()) }
                other => Err(::anyhow::anyhow!("expected OptFloat64, got {other:?}")),
            }),
            Self::OptString => quote!(match v {
                ::core_lib::FieldValue::OptString(x) => { t.#field = x; Ok(()) }
                ::core_lib::FieldValue::Null => { t.#field = None; Ok(()) }
                other => Err(::anyhow::anyhow!("expected OptString, got {other:?}")),
            }),

            Self::VecBool => quote!(match v {
                ::core_lib::FieldValue::VecBool(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected VecBool, got {other:?}")),
            }),
            Self::VecInt32 => quote!(match v {
                ::core_lib::FieldValue::VecInt32(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected VecInt32, got {other:?}")),
            }),
            Self::VecInt64 => quote!(match v {
                ::core_lib::FieldValue::VecInt64(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected VecInt64, got {other:?}")),
            }),
            Self::VecString => quote!(match v {
                ::core_lib::FieldValue::VecString(x) => { t.#field = x; Ok(()) }
                other => Err(::anyhow::anyhow!("expected VecString, got {other:?}")),
            }),

            // A nested resource decodes from the JSON object the codec hands us
            // (the parent object itself, for `#[flatten]`). The macro knows the
            // concrete inner type, so it can call its decode directly.
            Self::Nested { .. } => quote!(match v {
                ::core_lib::FieldValue::Nested(jv) => {
                    t.#field = ::core_lib::engine::decode::<#ty>(&jv)?;
                    Ok(())
                }
                ::core_lib::FieldValue::NestedConfig(jv) => {
                    t.#field = ::core_lib::engine::decode_config::<#ty>(&jv)?;
                    Ok(())
                }
                other => Err(::anyhow::anyhow!("expected Nested, got {other:?}")),
            }),

            Self::Secret => quote!(match v {
                ::core_lib::FieldValue::Secret(s) => {
                    t.#field = ::core_lib::SecretValue::new(s);
                    Ok(())
                }
                other => Err(::anyhow::anyhow!("expected Secret, got {other:?}")),
            }),
            Self::OptSecret => quote!(match v {
                ::core_lib::FieldValue::Secret(s) => {
                    t.#field = ::std::option::Option::Some(::core_lib::SecretValue::new(s));
                    Ok(())
                }
                ::core_lib::FieldValue::Null => { t.#field = None; Ok(()) }
                other => Err(::anyhow::anyhow!("expected Secret, got {other:?}")),
            }),

            Self::Json => quote!(match v {
                ::core_lib::FieldValue::Json(j) => { t.#field = j; Ok(()) }
                ::core_lib::FieldValue::Null => { t.#field = ::serde_json::Value::Null; Ok(()) }
                other => Err(::anyhow::anyhow!("expected Json, got {other:?}")),
            }),
            Self::OptJson => quote!(match v {
                ::core_lib::FieldValue::Json(j) => { t.#field = ::core::option::Option::Some(j); Ok(()) }
                ::core_lib::FieldValue::Null => { t.#field = ::core::option::Option::None; Ok(()) }
                other => Err(::anyhow::anyhow!("expected Json, got {other:?}")),
            }),
            Self::VecJson => quote!(match v {
                ::core_lib::FieldValue::VecJson(s) => { t.#field = s; Ok(()) }
                other => Err(::anyhow::anyhow!("expected VecJson, got {other:?}")),
            }),

            Self::OptNested { .. } => {
                let inner = inner_generic(ty).expect("Option<T> has an inner type");
                quote!(match v {
                    ::core_lib::FieldValue::Nested(jv) => {
                        t.#field =
                            ::core::option::Option::Some(::core_lib::engine::decode::<#inner>(&jv)?);
                        Ok(())
                    }
                    ::core_lib::FieldValue::NestedConfig(jv) => {
                        t.#field = ::core::option::Option::Some(
                            ::core_lib::engine::decode_config::<#inner>(&jv)?,
                        );
                        Ok(())
                    }
                    ::core_lib::FieldValue::Null => {
                        t.#field = ::core::option::Option::None;
                        Ok(())
                    }
                    other => Err(::anyhow::anyhow!("expected nested, got {other:?}")),
                })
            }
            Self::VecNested { .. } => {
                let inner = inner_generic(ty).expect("Vec<T> has an inner type");
                quote!(match v {
                    ::core_lib::FieldValue::VecNested(arr) => {
                        let mut out = ::std::vec::Vec::with_capacity(arr.len());
                        for jv in arr {
                            out.push(::core_lib::engine::decode::<#inner>(&jv)?);
                        }
                        t.#field = out;
                        Ok(())
                    }
                    ::core_lib::FieldValue::VecNestedConfig(arr) => {
                        let mut out = ::std::vec::Vec::with_capacity(arr.len());
                        for jv in arr {
                            out.push(::core_lib::engine::decode_config::<#inner>(&jv)?);
                        }
                        t.#field = out;
                        Ok(())
                    }
                    other => Err(::anyhow::anyhow!("expected vec-nested, got {other:?}")),
                })
            }
        }
    }
}

/// Inspect a `syn::Type` and produce a `Kind`. Everything unrecognised is a
/// nested resource using the snake-cased type ident as `type_name`.
pub fn classify(ty: &Type) -> Kind {
    let Some(path) = type_path(ty) else {
        return Kind::Nested {
            type_name: "unknown".to_string(),
        };
    };

    let last = path.path.segments.last().unwrap();
    let ident = last.ident.to_string();

    match ident.as_str() {
        "bool" => return Kind::Bool,
        "i32" => return Kind::Int32,
        "i64" => return Kind::Int64,
        "f64" => return Kind::Float64,
        "String" => return Kind::String,
        "SecretValue" => return Kind::Secret,
        // `Json` alias or a bare `serde_json::Value`.
        "Json" | "Value" => return Kind::Json,
        _ => {}
    }

    if let Some(inner) = generic_arg(last) {
        match ident.as_str() {
            "Option" => return classify_option_inner(inner),
            "Vec" => return classify_vec_inner(inner),
            _ => {}
        }
    }

    // Path with no generics, not a scalar — assume nested resource.
    Kind::Nested {
        type_name: snake_case(&ident),
    }
}

fn classify_option_inner(inner: &Type) -> Kind {
    let Some(path) = type_path(inner) else {
        return Kind::OptNested {
            type_name: "unknown".to_string(),
        };
    };
    let ident = path.path.segments.last().unwrap().ident.to_string();

    match ident.as_str() {
        "bool" => Kind::OptBool,
        "i32" => Kind::OptInt32,
        "i64" => Kind::OptInt64,
        "f64" => Kind::OptFloat64,
        "String" => Kind::OptString,
        "SecretValue" => Kind::OptSecret,
        "Json" | "Value" => Kind::OptJson,
        _ => Kind::OptNested {
            type_name: snake_case(&ident),
        },
    }
}

fn classify_vec_inner(inner: &Type) -> Kind {
    let Some(path) = type_path(inner) else {
        return Kind::VecNested {
            type_name: "unknown".to_string(),
        };
    };
    let ident = path.path.segments.last().unwrap().ident.to_string();
    match ident.as_str() {
        "bool" => Kind::VecBool,
        "i32" => Kind::VecInt32,
        "i64" => Kind::VecInt64,
        "String" => Kind::VecString,
        "Json" | "Value" => Kind::VecJson,
        _ => Kind::VecNested {
            type_name: snake_case(&ident),
        },
    }
}

fn type_path(ty: &Type) -> Option<&TypePath> {
    match ty {
        Type::Path(tp) => Some(tp),
        _ => None,
    }
}

fn generic_arg(seg: &syn::PathSegment) -> Option<&Type> {
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    args.args.iter().find_map(|a| match a {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    })
}

/// The single generic argument of `Option<T>` / `Vec<T>` — the `T`.
pub(crate) fn inner_generic(ty: &Type) -> Option<&Type> {
    generic_arg(type_path(ty)?.path.segments.last()?)
}

/// Convert `DownloadClient` → `download_client`.
pub(crate) fn snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}
