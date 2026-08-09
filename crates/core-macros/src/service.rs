//! `#[service(...)]` implementation.
//!
//! Emits `impl ::core_lib::Service for S` with:
//! * a static `ServiceDescriptor` listing every resource-holding field
//!   (`Vec<R>` or `Option<R>`)
//! * a `connection()` method that bundles `url`, `auth`, `insecure`, and
//!   `timeout_secs` from the struct's special fields
//!
//! Field-name conventions consumed by the macro:
//! * `url: String` — required
//! * `#[credential(api_key|bearer|user|pass)]` — fields carrying auth credentials
//! * `insecure: Option<bool>` / `timeout_secs: Option<u64>` — optional, by name
//! * `Vec<R>` — a collection resource; `Option<R>` — a singleton

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

use crate::attrs::{AuthSpec, CredentialRole, ServiceArgs, ServiceFieldAttrs, collect_doc};
use crate::field_kind::{Kind, classify};

pub fn expand(args: TokenStream, input: TokenStream) -> darling::Result<TokenStream> {
    let nested = NestedMeta::parse_meta_list(args)?;
    let svc_args = ServiceArgs::from_list(&nested)?;

    let input: DeriveInput = syn::parse2(input).map_err(Error::from)?;
    let ident = &input.ident;

    let syn::Data::Struct(data) = &input.data else {
        return Err(Error::custom("#[service] only supports structs").with_span(ident));
    };
    let syn::Fields::Named(named) = &data.fields else {
        return Err(Error::custom("service struct requires named fields").with_span(ident));
    };

    // Walk fields, partition into:
    //   - url (required)
    //   - credentials (zero or more, with #[credential(role)])
    //   - insecure / timeout_secs (optional by name)
    //   - resource fields (Vec<R> / Option<R>)
    let mut url_field: Option<syn::Ident> = None;
    let mut credential_fields: Vec<(syn::Ident, CredentialRole)> = Vec::new();
    let mut insecure_field: Option<syn::Ident> = None;
    let mut timeout_field: Option<syn::Ident> = None;
    let mut resource_fields: Vec<(syn::Ident, Option<String>, Kind, &Type)> = Vec::new();
    // `from_config` Self-literal initializers, one per field, in declaration order.
    let mut inits: Vec<TokenStream> = Vec::new();

    for f in &named.named {
        let fid = f.ident.as_ref().expect("named field").clone();
        let name = fid.to_string();
        let attrs = ServiceFieldAttrs::parse(&f.attrs)?;
        let doc = collect_doc(&f.attrs);

        if let Some(role) = attrs.credential_role {
            inits.push(scalar_init(&fid, &name, &classify(&f.ty))?);
            credential_fields.push((fid, role));
            continue;
        }
        if name == "url" {
            inits.push(scalar_init(&fid, &name, &classify(&f.ty))?);
            url_field = Some(fid);
            continue;
        }
        if name == "insecure" {
            inits.push(quote!(#fid: value.get(#name).and_then(|v| v.as_bool())));
            insecure_field = Some(fid);
            continue;
        }
        if name == "timeout_secs" {
            inits.push(quote!(#fid: value.get(#name).and_then(|v| v.as_u64())));
            timeout_field = Some(fid);
            continue;
        }

        let kind = classify(&f.ty);
        inits.push(resource_init(&fid, &kind, &f.ty)?);
        resource_fields.push((fid, doc, kind, &f.ty));
    }

    let url_ident = url_field.ok_or_else(|| {
        Error::custom("service struct must contain a `url: Interpolated<String>` field")
            .with_span(ident)
    })?;

    let auth_tok =
        build_auth(&svc_args.auth, &credential_fields).map_err(|e| e.with_span(ident))?;

    let insecure_tok: TokenStream = match insecure_field {
        Some(f) => quote!(self.#f),
        None => quote!(None),
    };
    let timeout_tok: TokenStream = match timeout_field {
        Some(f) => quote!(self.#f),
        None => quote!(None),
    };

    // Build the service descriptor's `fields` slice. Only Vec<R> and
    // Option<R> participate; everything else (url/credential/insecure/timeout)
    // was consumed above.
    let mut field_entries: Vec<TokenStream> = Vec::new();
    for (fid, doc, kind, ty) in &resource_fields {
        let name = fid.to_string();
        let doc_tok = doc_to_tokens(doc);

        // The resource's type name + the iterator shape are the only things that
        // differ between a `Vec<R>` collection and an `Option<R>` singleton. The
        // sync strategy comes from the resource's own descriptor, not the shape.
        let (type_name, iter_tok) = match kind {
            Kind::VecNested { type_name } => (
                type_name.clone(),
                quote!(|s| Box::new(s.#fid.iter().map(|r| r as &dyn ::core_lib::ResourceErased))),
            ),
            Kind::OptNested { type_name } => (
                type_name.clone(),
                quote!(|s| match &s.#fid {
                    Some(r) => Box::new(std::iter::once(r as &dyn ::core_lib::ResourceErased)),
                    None => Box::new(std::iter::empty()),
                }),
            ),
            _ => {
                return Err(Error::custom(format!(
                    "service field `{}` is not a recognised resource shape \
                     (expected `Vec<R>` or `Option<R>` where R is a Described resource)",
                    fid
                ))
                .with_span(fid));
            }
        };
        let inner = inner_type(ty).expect("Vec/Option resource field has an inner type");

        field_entries.push(quote! {
            ::core_lib::ServiceField::<#ident> {
                name: #name,
                doc: #doc_tok,
                type_name: #type_name,
                sync: || <#inner as ::core_lib::Described>::descriptor().sync,
                iter: #iter_tok,
                ref_targets: || ::core_lib::engine::reference_targets::<#inner>(),
                endpoints: || <#inner as ::core_lib::Described>::descriptor().endpoints,
                key_wire: || ::core_lib::engine::key_wire_name::<#inner>(),
                config_to_wire: |v| ::core_lib::engine::encode(
                    &::core_lib::engine::decode_config::<#inner>(v)?
                ),
                config_to_wire_present: |v| ::core_lib::engine::config_present_to_wire::<#inner>(v),
                secret_keys: || ::core_lib::engine::secret_wire_keys::<#inner>(),
                resource_docs: || ::core_lib::engine::resource_docs::<#inner>(),
                type_doc: || <#inner as ::core_lib::Described>::descriptor().doc,
            }
        });
    }

    let svc_name = &svc_args.name;
    let svc_doc = collect_doc(&input.attrs);
    let svc_doc_tok = doc_to_tokens(&svc_doc);
    let health_tok: TokenStream = match &svc_args.health {
        Some(h) => quote!(Some(#h)),
        None => quote!(None),
    };

    let input_stripped = strip_helper_attrs(input.clone());

    Ok(quote! {
        #input_stripped

        impl ::core_lib::Service for #ident {
            fn descriptor() -> &'static ::core_lib::ServiceDescriptor<Self> {
                static D: ::core_lib::ServiceDescriptor<#ident> = ::core_lib::ServiceDescriptor {
                    name: #svc_name,
                    doc: #svc_doc_tok,
                    health_check: #health_tok,
                    fields: &[#(#field_entries),*],
                };
                &D
            }

            fn connection(&self) -> ::core_lib::Connection<'_> {
                ::core_lib::Connection {
                    url: &self.#url_ident,
                    auth: #auth_tok,
                    insecure: #insecure_tok,
                    timeout_secs: #timeout_tok,
                }
            }

            fn from_config(value: &::serde_json::Value) -> ::anyhow::Result<Self> {
                if !value.is_object() {
                    ::anyhow::bail!("service config must be a mapping");
                }
                ::core::result::Result::Ok(Self { #(#inits),* })
            }
        }
    })
}

/// `from_config` initializer for a connection scalar (`url`, credentials).
/// Required string-ish fields error if absent; optional ones default to `None`.
fn scalar_init(fid: &syn::Ident, name: &str, kind: &Kind) -> Result<TokenStream, Error> {
    let req_str = quote! {
        value.get(#name).and_then(|v| v.as_str())
            .ok_or_else(|| ::anyhow::anyhow!(concat!("service config missing `", #name, "`")))?
            .to_string()
    };
    Ok(match kind {
        Kind::String => quote!(#fid: #req_str),
        Kind::Secret => quote!(#fid: ::core_lib::SecretValue::new(#req_str)),
        Kind::OptString => {
            quote!(#fid: value.get(#name).and_then(|v| v.as_str()).map(|s| s.to_string()))
        }
        Kind::OptSecret => quote!(#fid: value.get(#name).and_then(|v| v.as_str())
            .map(|s| ::core_lib::SecretValue::new(s.to_string()))),
        other => {
            return Err(Error::custom(format!(
                "service connection field `{fid}` has unsupported type ({other:?}); \
                 expected String / SecretValue (optional ok)"
            ))
            .with_span(fid));
        }
    })
}

/// `from_config` initializer for a resource field (`Vec<R>` / `Option<R>`).
///
/// Apply is value-driven (desired comes from the raw config `Value`), so the
/// typed service never holds resources — they'd also be undecodable while
/// `${ref}` is still a string in an `i32` field. Resource fields init to empty;
/// their *type* still drives the descriptor (endpoints / config→wire / etc).
fn resource_init(fid: &syn::Ident, kind: &Kind, ty: &Type) -> Result<TokenStream, Error> {
    match kind {
        Kind::VecNested { .. } | Kind::OptNested { .. } => {
            let empty = kind.to_empty_expr(ty);
            Ok(quote!(#fid: #empty))
        }
        _ => Err(Error::custom(format!(
            "service field `{fid}` is not a recognised resource shape \
             (expected `Vec<R>` or `Option<R>`)"
        ))
        .with_span(fid)),
    }
}

/// The single generic argument type of `Vec<T>` / `Option<T>`.
fn inner_type(ty: &Type) -> Option<&Type> {
    let Type::Path(tp) = ty else { return None };
    let seg = tp.path.segments.last()?;
    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    args.args.iter().find_map(|a| match a {
        syn::GenericArgument::Type(t) => Some(t),
        _ => None,
    })
}

fn build_auth(
    spec: &AuthSpec,
    credentials: &[(syn::Ident, CredentialRole)],
) -> Result<TokenStream, Error> {
    let by_role = |role: CredentialRole, ctx: &str| -> Result<&syn::Ident, Error> {
        credentials
            .iter()
            .find(|(_, r)| *r == role)
            .map(|(id, _)| id)
            .ok_or_else(|| Error::custom(ctx.to_string()))
    };

    match spec {
        AuthSpec::None => Ok(quote!(::core_lib::Auth::None)),

        AuthSpec::ApiKey { header } => {
            let kf = by_role(
                CredentialRole::ApiKey,
                "auth = api_key(...) requires a field with `#[credential(api_key)]`",
            )?;
            Ok(quote!(::core_lib::Auth::ApiKey { header: #header, key: &self.#kf }))
        }

        AuthSpec::ApiKeyQuery { param } => {
            let kf = by_role(
                CredentialRole::ApiKey,
                "auth = api_key(query = ...) requires a field with `#[credential(api_key)]`",
            )?;
            Ok(quote!(::core_lib::Auth::ApiKeyQuery { param: #param, key: &self.#kf }))
        }

        AuthSpec::Bearer => {
            let tf = by_role(
                CredentialRole::Bearer,
                "auth = bearer requires a field with `#[credential(bearer)]`",
            )?;
            Ok(quote!(::core_lib::Auth::Bearer { token: &self.#tf }))
        }

        AuthSpec::Basic => {
            let uf = by_role(
                CredentialRole::User,
                "auth = basic requires `#[credential(user)]` and `#[credential(pass)]`",
            )?;
            let pf = by_role(
                CredentialRole::Pass,
                "auth = basic requires `#[credential(user)]` and `#[credential(pass)]`",
            )?;
            Ok(quote!(::core_lib::Auth::Basic { user: &self.#uf, pass: &self.#pf }))
        }

        AuthSpec::FormCookie { login_path } => {
            let uf = by_role(
                CredentialRole::User,
                "auth = form_cookie(...) requires `#[credential(user)]` and `#[credential(pass)]`",
            )?;
            let pf = by_role(
                CredentialRole::Pass,
                "auth = form_cookie(...) requires `#[credential(user)]` and `#[credential(pass)]`",
            )?;
            Ok(
                quote!(::core_lib::Auth::FormCookie { login_path: #login_path, user: &self.#uf, pass: &self.#pf }),
            )
        }
    }
}

fn doc_to_tokens(doc: &Option<String>) -> TokenStream {
    match doc {
        Some(s) => quote!(Some(#s)),
        None => quote!(None),
    }
}

fn strip_helper_attrs(mut input: DeriveInput) -> DeriveInput {
    if let syn::Data::Struct(s) = &mut input.data
        && let syn::Fields::Named(named) = &mut s.fields
    {
        for f in &mut named.named {
            f.attrs.retain(|a| !a.path().is_ident("credential"));
        }
    }
    input
}
