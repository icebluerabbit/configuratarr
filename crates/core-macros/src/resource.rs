//! `#[resource(...)]` implementation.
//!
//! Emits `impl ::core_lib::Described for T` with a static `ResourceDescriptor`
//! describing every field, plus codec selection driven by an optional
//! sibling `#[codec(...)]` attribute.

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::attrs::{
    CaseSpec, CodecSpec, EndpointSpec, FieldAttrs, HttpMethodSpec, NestedArgs, ResourceArgs,
    SyncSpec, collect_doc, parse_codec_spec,
};
use crate::field_kind::{Kind, classify, inner_generic, snake_case};

pub fn expand(args: TokenStream, input: TokenStream) -> darling::Result<TokenStream> {
    let nested = NestedMeta::parse_meta_list(args)?;
    let resource_args = ResourceArgs::from_list(&nested)?;

    let input: DeriveInput = syn::parse2(input).map_err(Error::from)?;
    let ident = &input.ident;
    let codec_spec = parse_codec_spec(&input.attrs)?;
    let struct_doc = collect_doc(&input.attrs);
    let struct_doc_tok = doc_to_tokens(&struct_doc);

    let endpoints_tok = endpoints_tokens(&resource_args);

    // Default the type identifier to snake_case(StructIdent); the explicit
    // `type_name = "..."` arg only exists for API-spelling overrides. Keeping
    // these in sync by construction means `Ref<Tag>` (whose target_type is
    // also snake_case(ident)) can never silently miss its resource.
    let type_name: String = resource_args
        .type_name
        .clone()
        .unwrap_or_else(|| snake_case(&ident.to_string()));
    let (codec_kind, codec_meta) = codec_spec_tokens(&codec_spec);
    // `sync = custom` carries its reconcile hook inline (the struct must impl
    // `CustomSync`); every other strategy is a bare variant.
    let sync_tok = match resource_args.sync {
        SyncSpec::Custom => {
            quote!(::core_lib::SyncKind::Custom(<#ident as ::core_lib::CustomSync>::reconcile))
        }
        other => sync_spec_tokens(other),
    };

    let case_tok = case_tokens(resource_args.case);
    let (fields_tok, empty_body) = match &input.data {
        syn::Data::Struct(s) => {
            let (fields, empties) = emit_struct_fields(ident, &s.fields)?;
            (fields, quote!(Self { #(#empties),* }))
        }
        syn::Data::Enum(_) => (
            emit_enum_fields()?,
            quote!(::core::unimplemented!(
                "enum resources are decoded by their codec, not empty()"
            )),
        ),
        syn::Data::Union(_) => {
            return Err(Error::custom("unions are not supported").with_span(ident));
        }
    };

    // Strip our consumed attributes from the re-emitted input so rustc doesn't
    // complain about unknown attribute names like `#[codec]`, `#[key]`,
    // `#[wire]`.
    let input_stripped = strip_helper_attrs(input.clone());

    Ok(emit_described(
        &input_stripped,
        ident,
        &endpoints_tok,
        &type_name,
        &struct_doc_tok,
        &codec_kind,
        &codec_meta,
        &case_tok,
        &sync_tok,
        &fields_tok,
        &[],
        &empty_body,
        &quote!(),
    ))
}

/// Build the `Endpoints { .. }` literal for a `#[resource]` from its parsed
/// per-operation endpoint specs.
fn endpoints_tokens(args: &ResourceArgs) -> TokenStream {
    let slot = |e: &Option<EndpointSpec>| -> TokenStream {
        match e {
            Some(EndpointSpec { method, path }) => {
                let m = http_method_tokens(*method);
                quote!(Some(::core_lib::Endpoint { method: #m, path: #path }))
            }
            None => quote!(None),
        }
    };
    let list = slot(&args.list);
    let read = slot(&args.read);
    let create = slot(&args.create);
    let update = slot(&args.update);
    let delete = slot(&args.delete);
    quote! {
        ::core_lib::Endpoints {
            list: #list,
            read: #read,
            create: #create,
            update: #update,
            delete: #delete,
        }
    }
}

fn http_method_tokens(m: HttpMethodSpec) -> TokenStream {
    match m {
        HttpMethodSpec::Get => quote!(::core_lib::HttpMethod::Get),
        HttpMethodSpec::Post => quote!(::core_lib::HttpMethod::Post),
        HttpMethodSpec::Put => quote!(::core_lib::HttpMethod::Put),
        HttpMethodSpec::Patch => quote!(::core_lib::HttpMethod::Patch),
        HttpMethodSpec::Delete => quote!(::core_lib::HttpMethod::Delete),
    }
}

/// Emit `impl Described` from pre-computed descriptor pieces. Shared by every
/// resource-flavoured macro (`#[resource]`, `#[nested]`, `#[fields_blob]`,
/// `#[tagged]`, `#[wire_enum]`) — they differ only in how they fill these in.
#[allow(clippy::too_many_arguments)]
pub(crate) fn emit_described(
    input_stripped: &DeriveInput,
    ident: &syn::Ident,
    endpoints_tok: &TokenStream,
    type_name: &str,
    doc_tok: &TokenStream,
    codec_kind: &TokenStream,
    codec_meta: &TokenStream,
    case_tok: &TokenStream,
    sync_tok: &TokenStream,
    fields_tok: &[TokenStream],
    variants_tok: &[TokenStream],
    empty_body: &TokenStream,
    extra_methods: &TokenStream,
) -> TokenStream {
    quote! {
        #input_stripped

        impl ::core_lib::Described for #ident {
            fn descriptor() -> &'static ::core_lib::ResourceDescriptor<Self> {
                static D: ::core_lib::ResourceDescriptor<#ident> = ::core_lib::ResourceDescriptor {
                    endpoints: #endpoints_tok,
                    type_name: #type_name,
                    doc: #doc_tok,
                    codec: #codec_kind,
                    case: #case_tok,
                    sync: #sync_tok,
                    codec_meta: #codec_meta,
                    fields: &[#(#fields_tok),*],
                    variants: &[#(#variants_tok),*],
                };
                &D
            }

            fn empty() -> Self {
                #empty_body
            }

            #extra_methods
        }
    }
}

/// `#[nested]` — an embedded sub-resource: no path, `SyncKind::Embedded`,
/// type_name defaulted from the ident, standard codec (or a `#[codec]` sibling).
pub fn expand_nested(args: TokenStream, input: TokenStream) -> darling::Result<TokenStream> {
    let nested_args = NestedArgs::from_list(&NestedMeta::parse_meta_list(args)?)?;
    let input: DeriveInput = syn::parse2(input).map_err(Error::from)?;
    let ident = &input.ident;
    let codec_spec = parse_codec_spec(&input.attrs)?;
    let (codec_kind, codec_meta) = codec_spec_tokens(&codec_spec);
    let doc_tok = doc_to_tokens(&collect_doc(&input.attrs));
    let type_name = snake_case(&ident.to_string());

    let case_tok = case_tokens(nested_args.case);
    let (fields_tok, empties) = match &input.data {
        syn::Data::Struct(s) => emit_struct_fields(ident, &s.fields)?,
        _ => return Err(Error::custom("#[nested] only supports structs").with_span(ident)),
    };

    let input_stripped = strip_helper_attrs(input.clone());
    Ok(emit_described(
        &input_stripped,
        ident,
        &quote!(::core_lib::Endpoints::NONE),
        &type_name,
        &doc_tok,
        &codec_kind,
        &codec_meta,
        &case_tok,
        &quote!(::core_lib::SyncKind::Embedded),
        &fields_tok,
        &[],
        &quote!(Self { #(#empties),* }),
        &quote!(),
    ))
}

/// `#[fields_blob(implementation = "...", config_contract = "...", protocol = "...")]`
/// — an embedded provider-config variant whose fields render to the *arr
/// `{name, value}` array. Sugar for `#[nested]` + the FieldsBlob codec.
pub fn expand_fields_blob(args: TokenStream, input: TokenStream) -> darling::Result<TokenStream> {
    let nested = NestedMeta::parse_meta_list(args)?;
    let fb = crate::attrs::FieldsBlobArgs::from_list(&nested)?;
    let spec = CodecSpec::FieldsBlob {
        implementation: fb.implementation,
        config_contract: fb.config_contract,
        protocol: fb.protocol,
    };
    let (codec_kind, codec_meta) = codec_spec_tokens(&spec);

    let input: DeriveInput = syn::parse2(input).map_err(Error::from)?;
    let ident = &input.ident;
    let doc_tok = doc_to_tokens(&collect_doc(&input.attrs));
    let type_name = snake_case(&ident.to_string());
    // Provider fields-blob variants are always *arr camelCase.
    let case_tok = quote!(::core_lib::Case::Camel);

    let (fields_tok, empties) = match &input.data {
        syn::Data::Struct(s) => emit_struct_fields(ident, &s.fields)?,
        _ => return Err(Error::custom("#[fields_blob] only supports structs").with_span(ident)),
    };

    let input_stripped = strip_helper_attrs(input.clone());
    Ok(emit_described(
        &input_stripped,
        ident,
        &quote!(::core_lib::Endpoints::NONE),
        &type_name,
        &doc_tok,
        &codec_kind,
        &codec_meta,
        &case_tok,
        &quote!(::core_lib::SyncKind::Embedded),
        &fields_tok,
        &[],
        &quote!(Self { #(#empties),* }),
        &quote!(),
    ))
}

/// Map a `#[default(<lit>)]` literal onto a `core_lib::DefaultLit` constructor.
fn default_lit_tokens(lit: &syn::Lit) -> darling::Result<TokenStream> {
    match lit {
        syn::Lit::Bool(b) => Ok(quote!(::core_lib::DefaultLit::Bool(#b))),
        syn::Lit::Int(i) => Ok(quote!(::core_lib::DefaultLit::Int(#i))),
        syn::Lit::Float(f) => Ok(quote!(::core_lib::DefaultLit::Float(#f))),
        syn::Lit::Str(s) => Ok(quote!(::core_lib::DefaultLit::Str(#s))),
        other => Err(Error::custom(
            "#[default(...)] supports only bool / int / float / string literals",
        )
        .with_span(other)),
    }
}

fn sync_spec_tokens(spec: SyncSpec) -> TokenStream {
    match spec {
        SyncSpec::Crud => quote!(::core_lib::SyncKind::Crud),
        SyncSpec::Singleton => quote!(::core_lib::SyncKind::Singleton),
        SyncSpec::Custom => quote!(::core_lib::SyncKind::Custom),
    }
}

fn case_tokens(spec: CaseSpec) -> TokenStream {
    match spec {
        CaseSpec::Camel => quote!(::core_lib::Case::Camel),
        CaseSpec::Pascal => quote!(::core_lib::Case::Pascal),
        CaseSpec::Snake => quote!(::core_lib::Case::Snake),
    }
}

fn codec_spec_tokens(spec: &CodecSpec) -> (TokenStream, TokenStream) {
    match spec {
        CodecSpec::Standard => (
            quote!(::core_lib::CodecKind::Standard),
            quote!(::core_lib::descriptor::CodecMeta::Standard),
        ),
        CodecSpec::FieldsBlob {
            implementation,
            config_contract,
            protocol,
        } => {
            let cc = match config_contract {
                Some(c) => quote!(Some(#c)),
                None => quote!(None),
            };
            let proto = match protocol {
                Some(p) => quote!(Some(#p)),
                None => quote!(None),
            };
            (
                quote!(::core_lib::CodecKind::FieldsBlob),
                quote!(::core_lib::descriptor::CodecMeta::FieldsBlob {
                    implementation: #implementation,
                    config_contract: #cc,
                    protocol: #proto,
                }),
            )
        }
        CodecSpec::TaggedByImpl { discriminator } => (
            quote!(::core_lib::CodecKind::TaggedByImpl),
            quote!(::core_lib::descriptor::CodecMeta::TaggedByImpl {
                discriminator: #discriminator,
            }),
        ),
    }
}

/// Emit per-field descriptors plus the matching `empty()` initializers
/// (`field: <empty-expr>`), so the caller can build both the descriptor slice
/// and the `Described::empty()` constructor body.
fn emit_struct_fields(
    struct_ident: &syn::Ident,
    fields: &syn::Fields,
) -> darling::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let syn::Fields::Named(named) = fields else {
        return Err(Error::custom("only named-field structs are supported").with_span(struct_ident));
    };

    let mut out = Vec::with_capacity(named.named.len());
    let mut empties = Vec::with_capacity(named.named.len());
    for f in &named.named {
        let field_ident = f.ident.as_ref().expect("named field has ident");
        let name_str = field_ident.to_string();
        let attrs = FieldAttrs::parse(&f.attrs)?;
        let doc = collect_doc(&f.attrs);
        let doc_tok = doc_to_tokens(&doc);
        let kind = classify(&f.ty);
        empties.push({
            let e = kind.to_empty_expr(&f.ty);
            quote!(#field_ident: #e)
        });

        if attrs.is_id && attrs.is_key {
            return Err(Error::custom(
                "a field cannot be both #[id] (server key) and #[key] (natural key)",
            )
            .with_span(field_ident));
        }

        let role_tok: TokenStream = if attrs.is_id {
            quote!(::core_lib::FieldRole::Id)
        } else if attrs.is_key {
            quote!(::core_lib::FieldRole::Key)
        } else {
            quote!(::core_lib::FieldRole::Normal)
        };

        let kind_tok = kind.to_kind_expr();
        let get_tok = kind.to_get_expr(field_ident);
        let set_tok = kind.to_set_expr(field_ident, &f.ty);

        let wire_name_tok: TokenStream = match &attrs.wire_name {
            Some(n) => quote!(Some(#n)),
            None => quote!(None),
        };
        // Id fields are server-assigned and never sent on write.
        // `config_only` is skipped on encode just like `read_only`, so fold it
        // in here; the descriptor keeps them distinct for doc-gen.
        let read_only = attrs.read_only || attrs.is_id || attrs.config_only;
        let config_only_tok = if attrs.config_only {
            quote!(true)
        } else {
            quote!(false)
        };
        let read_only_tok = if read_only {
            quote!(true)
        } else {
            quote!(false)
        };
        let emit_none_tok = if attrs.emit_none {
            quote!(true)
        } else {
            quote!(false)
        };
        let as_int_tok = if attrs.as_int {
            quote!(true)
        } else {
            quote!(false)
        };

        let default_tok: TokenStream = match &attrs.default {
            Some(lit) => {
                let dl = default_lit_tokens(lit)?;
                quote!(Some(#dl))
            }
            None => quote!(None),
        };

        let secret_tok = if kind.is_secret() {
            quote!(true)
        } else {
            quote!(false)
        };
        let flatten_tok = if attrs.flatten {
            quote!(true)
        } else {
            quote!(false)
        };
        let fields_map_tok = if attrs.fields_map {
            quote!(true)
        } else {
            quote!(false)
        };
        let reference_tok: TokenStream = {
            let targets = &attrs.reference;
            quote!(&[#(#targets),*])
        };

        // For nested-type fields, emit a provider for the inner type's docs so
        // doc-gen can render its section even when the value is absent.
        let nested_docs_tok: TokenStream = match &kind {
            Kind::Nested { .. } => {
                let inner = &f.ty;
                quote!(Some(|| ::core_lib::engine::resource_docs::<#inner>()))
            }
            Kind::OptNested { .. } | Kind::VecNested { .. } => match inner_generic(&f.ty) {
                Some(inner) => quote!(Some(|| ::core_lib::engine::resource_docs::<#inner>())),
                None => quote!(None),
            },
            _ => quote!(None),
        };

        // Mirror of `nested_docs`, but for `#[reference]` targets: lets
        // `reference_targets` descend into a nested type's FKs without an instance
        // (a `Vec<Nested>`/`Option<Nested>` is empty in `empty()`).
        let nested_ref_targets_tok: TokenStream = match &kind {
            Kind::Nested { .. } => {
                let inner = &f.ty;
                quote!(Some(|out, seen| {
                    ::core_lib::engine::collect_reference_targets::<#inner>(out, seen)
                }))
            }
            Kind::OptNested { .. } | Kind::VecNested { .. } => match inner_generic(&f.ty) {
                Some(inner) => quote!(Some(|out, seen| {
                    ::core_lib::engine::collect_reference_targets::<#inner>(out, seen)
                })),
                None => quote!(None),
            },
            _ => quote!(None),
        };

        // For a nested single object (`Nested` / `Option<Nested>`), emit the
        // inner type's presence-masked config→wire so `present_to_wire` recurses.
        // `Vec<Nested>` is excluded — a list has no per-element presence mask.
        let nested_present_tok: TokenStream = match &kind {
            Kind::Nested { .. } => {
                let inner = &f.ty;
                quote!(Some(|v| ::core_lib::engine::config_present_to_wire::<#inner>(v)))
            }
            Kind::OptNested { .. } => match inner_generic(&f.ty) {
                Some(inner) => {
                    quote!(Some(|v| ::core_lib::engine::config_present_to_wire::<#inner>(v)))
                }
                None => quote!(None),
            },
            _ => quote!(None),
        };

        out.push(quote! {
            ::core_lib::FieldDescriptor::<#struct_ident> {
                name: #name_str,
                doc: #doc_tok,
                role: #role_tok,
                kind: #kind_tok,
                wire_name: #wire_name_tok,
                read_only: #read_only_tok,
                config_only: #config_only_tok,
                emit_none: #emit_none_tok,
                as_int: #as_int_tok,
                default: #default_tok,
                secret: #secret_tok,
                flatten: #flatten_tok,
                fields_map: #fields_map_tok,
                reference: #reference_tok,
                nested_docs: #nested_docs_tok,
                nested_reference_targets: #nested_ref_targets_tok,
                nested_present: #nested_present_tok,
                get: |t| { #get_tok },
                set: |t, v| { #set_tok },
            }
        });
    }
    Ok((out, empties))
}

fn emit_enum_fields() -> darling::Result<Vec<TokenStream>> {
    // A bare `#[resource]` enum keeps an empty `fields` slice; the tagged codec
    // dispatches via `codec_meta.discriminator`. The real enum path is the
    // `#[tagged]` / `#[wire_enum]` macros (see `enums.rs`).
    Ok(Vec::new())
}

pub(crate) fn doc_to_tokens(doc: &Option<String>) -> TokenStream {
    match doc {
        Some(s) => quote!(Some(#s)),
        None => quote!(None),
    }
}

/// Remove the inert helper attributes (`#[codec]`, `#[key]`, `#[wire]`) from
/// the re-emitted struct/enum so the standard Rust attribute lint doesn't
/// reject them — they have no real attribute macro to bind to.
pub(crate) fn strip_helper_attrs(mut input: DeriveInput) -> DeriveInput {
    input.attrs.retain(|a| {
        let p = a.path();
        !(p.is_ident("codec"))
    });
    match &mut input.data {
        syn::Data::Struct(s) => {
            if let syn::Fields::Named(named) = &mut s.fields {
                for f in &mut named.named {
                    f.attrs.retain(|a| {
                        let p = a.path();
                        !(p.is_ident("id")
                            || p.is_ident("key")
                            || p.is_ident("wire")
                            || p.is_ident("flatten")
                            || p.is_ident("fields_map")
                            || p.is_ident("default")
                            || p.is_ident("reference"))
                    });
                }
            }
        }
        syn::Data::Enum(e) => {
            for v in &mut e.variants {
                v.attrs.retain(|a| {
                    let p = a.path();
                    !(p.is_ident("key")
                        || p.is_ident("wire")
                        || p.is_ident("variant")
                        || p.is_ident("fallback"))
                });
            }
        }
        syn::Data::Union(_) => {}
    }
    input
}
