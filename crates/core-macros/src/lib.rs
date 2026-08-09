//! Proc macros that emit [`core_lib::Described`] / [`core_lib::Service`] impls.
//! They produce only static descriptors — no runtime logic; all behaviour lives
//! in `core`, generic over `T: Described` / `S: Service`.
//!
//! The codec is chosen by *which* macro you use: `#[resource]` (Standard),
//! `#[fields_blob]`, `#[tagged(by = "...")]`, `#[wire_enum]`, `#[nested]`
//! (embedded). Endpoints are declared inline as `list/read/create/update/delete
//! = verb("/path")`.
//!
//! **Ordering:** the resource macro must be the **outermost** attribute. Field
//! helper attrs — `#[id]`, `#[key]`, `#[reference(t)]`, `#[wire(name = "..",
//! read_only)]`, `#[flatten]`, `#[default(expr)]` — must sit below it in source
//! order, or they leak through unstripped and rustc rejects them. `#[service]`
//! consumes `#[credential(api_key|user|pass|bearer)]`.

mod attrs;
mod enums;
mod field_kind;
mod resource;
mod service;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn resource(args: TokenStream, input: TokenStream) -> TokenStream {
    resource::expand(args.into(), input.into())
        .unwrap_or_else(|e| e.write_errors())
        .into()
}

/// Embedded sub-resource — no path, `SyncKind::Embedded`. See module docs.
#[proc_macro_attribute]
pub fn nested(args: TokenStream, input: TokenStream) -> TokenStream {
    resource::expand_nested(args.into(), input.into())
        .unwrap_or_else(|e| e.write_errors())
        .into()
}

/// Provider-config variant whose fields render to the *arr `{name, value}` blob.
#[proc_macro_attribute]
pub fn fields_blob(args: TokenStream, input: TokenStream) -> TokenStream {
    resource::expand_fields_blob(args.into(), input.into())
        .unwrap_or_else(|e| e.write_errors())
        .into()
}

/// Discriminator-dispatched provider enum (`#[variant(\"..\")]` / `#[fallback]`).
#[proc_macro_attribute]
pub fn tagged(args: TokenStream, input: TokenStream) -> TokenStream {
    enums::expand_tagged(args.into(), input.into())
        .unwrap_or_else(|e| e.write_errors())
        .into()
}

/// Unit enum rendered as a bare wire string.
#[proc_macro_attribute]
pub fn wire_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    enums::expand_wire_enum(args.into(), input.into())
        .unwrap_or_else(|e| e.write_errors())
        .into()
}

#[proc_macro_attribute]
pub fn service(args: TokenStream, input: TokenStream) -> TokenStream {
    service::expand(args.into(), input.into())
        .unwrap_or_else(|e| e.write_errors())
        .into()
}
