//! Attribute-parsing helpers for `#[resource]` and `#[service]`.
//!
//! Centralised so the two macros share consistent grammar for the inert
//! helper attributes (`#[codec]`, `#[key]`, `#[wire(...)]`, `#[credential]`).

use darling::FromMeta;
use syn::{Attribute, Meta};

/// `#[resource(...)]` arguments.
#[derive(Debug, FromMeta)]
pub struct ResourceArgs {
    /// Snake-case type identifier used in `${ref.<type_name>.<key>}` lookups.
    /// Optional — defaults to `snake_case(StructIdent)`. Override only when the
    /// API's spelling diverges from the Rust type name.
    #[darling(default)]
    pub type_name: Option<String>,

    /// Write strategy. Required, always explicit — see [`SyncSpec`].
    pub sync: SyncSpec,

    /// Wire-key casing for fields without an explicit `#[wire(name)]`. Defaults
    /// to `camel` (snake→camelCase, the *arr shape). `pascal` upper-cases the
    /// first character too (PascalCase) — for .NET-style APIs like Jellyfin
    /// whose default JSON serialisation is PascalCase.
    #[darling(default)]
    pub case: CaseSpec,

    /// HTTP operations, each `op = verb("/path")`. Method is always explicit;
    /// path may carry `${self.*}` / `${ref.*}`. The strategy decides which are
    /// required.
    #[darling(default)]
    pub list: Option<EndpointSpec>,
    #[darling(default)]
    pub read: Option<EndpointSpec>,
    #[darling(default)]
    pub create: Option<EndpointSpec>,
    #[darling(default)]
    pub update: Option<EndpointSpec>,
    #[darling(default)]
    pub delete: Option<EndpointSpec>,
}

/// `#[nested(...)]` arguments. A bare `#[nested]` parses to the default.
#[derive(Debug, Default, FromMeta)]
pub struct NestedArgs {
    /// Wire-key casing — see [`ResourceArgs::case`].
    #[darling(default)]
    pub case: CaseSpec,
}

/// Wire-key casing strategy for fields lacking an explicit `#[wire(name)]`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CaseSpec {
    /// snake→camelCase (the *arr default).
    #[default]
    Camel,
    /// snake→PascalCase (camelCase with the first character upper-cased).
    Pascal,
    /// Identity — the wire key is the snake field name (bazarr).
    Snake,
}

impl FromMeta for CaseSpec {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "camel" => Ok(Self::Camel),
            "pascal" => Ok(Self::Pascal),
            "snake" => Ok(Self::Snake),
            other => Err(darling::Error::custom(format!(
                "unknown case `{other}` — expected camel / pascal / snake"
            ))),
        }
    }

    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        match expr {
            syn::Expr::Path(p) => match p.path.get_ident() {
                Some(id) => Self::from_string(&id.to_string()),
                None => Err(darling::Error::custom("expected a bare case ident").with_span(expr)),
            },
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) => Self::from_string(&s.value()),
            _ => Err(
                darling::Error::custom("case must be one of: camel / pascal / snake")
                    .with_span(expr),
            ),
        }
    }
}

/// One `op = verb("/path")` endpoint parsed from a `#[resource]` attribute.
///
/// The value is always a call expression whose callee is one of
/// `get`/`post`/`put`/`patch`/`delete` and whose single argument is the path
/// string. No bare-string form — the method is always explicit.
#[derive(Debug, Clone)]
pub struct EndpointSpec {
    pub method: HttpMethodSpec,
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethodSpec {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethodSpec {
    fn from_ident(id: &syn::Ident) -> Option<Self> {
        match id.to_string().as_str() {
            "get" => Some(Self::Get),
            "post" => Some(Self::Post),
            "put" => Some(Self::Put),
            "patch" => Some(Self::Patch),
            "delete" => Some(Self::Delete),
            _ => None,
        }
    }
}

impl FromMeta for EndpointSpec {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        let syn::Expr::Call(call) = expr else {
            return Err(darling::Error::custom(
                "endpoint must be `verb(\"/path\")`, e.g. `get(\"/api/v3/tag\")`",
            )
            .with_span(expr));
        };
        let syn::Expr::Path(callee) = &*call.func else {
            return Err(
                darling::Error::custom("endpoint verb must be a bare ident").with_span(expr)
            );
        };
        let method = callee
            .path
            .get_ident()
            .and_then(HttpMethodSpec::from_ident)
            .ok_or_else(|| {
                darling::Error::custom("verb must be one of get/post/put/patch/delete")
                    .with_span(&call.func)
            })?;
        if call.args.len() != 1 {
            return Err(
                darling::Error::custom("endpoint takes exactly one path string").with_span(expr),
            );
        }
        let path = match call.args.first().unwrap() {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) => s.value(),
            other => {
                return Err(
                    darling::Error::custom("endpoint path must be a string literal")
                        .with_span(other),
                );
            }
        };
        Ok(EndpointSpec { method, path })
    }
}

/// Write-strategy selector parsed from `sync = crud | singleton | custom`.
///
/// Accepts a bare ident (`sync = crud`) or a string (`sync = "crud"`). Maps
/// 1:1 onto `core_lib::SyncKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncSpec {
    Crud,
    Singleton,
    Custom,
}

impl FromMeta for SyncSpec {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "crud" => Ok(Self::Crud),
            "singleton" => Ok(Self::Singleton),
            "custom" => Ok(Self::Custom),
            other => Err(darling::Error::custom(format!(
                "unknown sync strategy `{other}` — expected crud / singleton / custom"
            ))),
        }
    }

    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        match expr {
            // bare ident: `sync = crud`
            syn::Expr::Path(p) => match p.path.get_ident() {
                Some(id) => Self::from_string(&id.to_string()),
                None => Err(darling::Error::custom("expected a bare sync ident").with_span(expr)),
            },
            // string literal: `sync = "crud"`
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) => Self::from_string(&s.value()),
            _ => Err(
                darling::Error::custom("sync must be one of: crud / singleton / custom")
                    .with_span(expr),
            ),
        }
    }
}

/// `#[service(...)]` arguments.
///
/// Auth uses the call-form selector `auth = scheme(args)` — same grammar as
/// resource endpoints (`get("/path")`). See [`AuthSpec`].
#[derive(Debug, FromMeta)]
pub struct ServiceArgs {
    /// Snake-case service identifier, e.g. `radarr_v3`. Required.
    pub name: String,

    /// Optional health-check path, e.g. `/api/v3/system/status`.
    #[darling(default)]
    pub health: Option<String>,

    /// Auth scheme — `auth = api_key(header = "X-Api-Key")`, `auth = bearer`,
    /// `auth = basic`, `auth = form_cookie(login_path = "...")`, `auth = none`.
    pub auth: AuthSpec,
}

/// Parsed auth scheme. Static params (header name, login path) live here; the
/// per-instance credentials come from `#[credential(role)]`-tagged fields.
#[derive(Debug, Clone)]
pub enum AuthSpec {
    None,
    ApiKey { header: String },
    ApiKeyQuery { param: String },
    Bearer,
    Basic,
    FormCookie { login_path: String },
}

impl FromMeta for AuthSpec {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        match expr {
            // bare scheme, no args: `auth = bearer`
            syn::Expr::Path(p) => {
                let id = p.path.get_ident().ok_or_else(|| {
                    darling::Error::custom("expected an auth scheme ident").with_span(expr)
                })?;
                Self::from_parts(&id.to_string(), &[], expr)
            }
            // scheme with args: `auth = api_key(header = "X-Api-Key")`
            syn::Expr::Call(call) => {
                let syn::Expr::Path(callee) = &*call.func else {
                    return Err(
                        darling::Error::custom("auth scheme must be a bare ident").with_span(expr)
                    );
                };
                let scheme = callee
                    .path
                    .get_ident()
                    .ok_or_else(|| {
                        darling::Error::custom("auth scheme must be a bare ident").with_span(expr)
                    })?
                    .to_string();
                let args = call_named_args(&call.args)?;
                Self::from_parts(&scheme, &args, expr)
            }
            _ => Err(
                darling::Error::custom("auth must be `scheme` or `scheme(arg = \"...\")`")
                    .with_span(expr),
            ),
        }
    }
}

impl AuthSpec {
    fn from_parts(
        scheme: &str,
        args: &[(String, String)],
        span: &syn::Expr,
    ) -> darling::Result<Self> {
        let arg = |k: &str| args.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone());
        match scheme {
            "none" => Ok(Self::None),
            "bearer" => Ok(Self::Bearer),
            "basic" => Ok(Self::Basic),
            "api_key" => {
                // `header = "..."` for the header scheme, or `query = "..."` for
                // the api-key-in-query-string scheme (LazyLibrarian). Exactly one.
                match (arg("header"), arg("query")) {
                    (Some(_), Some(_)) => Err(darling::Error::custom(
                        "auth = api_key(...) takes either `header` or `query`, not both",
                    )
                    .with_span(span)),
                    (Some(header), None) => Ok(Self::ApiKey { header }),
                    (None, Some(param)) => Ok(Self::ApiKeyQuery { param }),
                    (None, None) => Err(darling::Error::custom(
                        "auth = api_key(...) requires `header = \"<name>\"` or `query = \"<name>\"`",
                    )
                    .with_span(span)),
                }
            }
            "form_cookie" => {
                let login_path = arg("login_path").ok_or_else(|| {
                    darling::Error::custom(
                        "auth = form_cookie(...) requires `login_path = \"<path>\"`",
                    )
                    .with_span(span)
                })?;
                Ok(Self::FormCookie { login_path })
            }
            other => Err(darling::Error::custom(format!(
                "unknown auth scheme `{other}` — expected none / api_key / bearer / basic / form_cookie"
            ))
            .with_span(span)),
        }
    }
}

/// Extract `name = "value"` pairs from a call expression's argument list.
fn call_named_args(
    args: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> darling::Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    for a in args {
        let syn::Expr::Assign(assign) = a else {
            return Err(darling::Error::custom("expected `name = \"value\"`").with_span(a));
        };
        let syn::Expr::Path(name) = &*assign.left else {
            return Err(darling::Error::custom("argument name must be a bare ident").with_span(a));
        };
        let name = name
            .path
            .get_ident()
            .ok_or_else(|| {
                darling::Error::custom("argument name must be a bare ident").with_span(a)
            })?
            .to_string();
        let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) = &*assign.right
        else {
            return Err(
                darling::Error::custom("argument value must be a string literal").with_span(a),
            );
        };
        out.push((name, s.value()));
    }
    Ok(out)
}

/// `#[fields_blob(implementation = "...", config_contract = "...", protocol = "...")]`.
#[derive(Debug, FromMeta)]
pub struct FieldsBlobArgs {
    pub implementation: String,
    #[darling(default)]
    pub config_contract: Option<String>,
    #[darling(default)]
    pub protocol: Option<String>,
}

/// `#[tagged(by = "implementation")]` — the discriminator key on the wire.
#[derive(Debug, FromMeta)]
pub struct TaggedArgs {
    pub by: String,
}

/// `#[wire_enum(rename_all = "lowercase")]` — casing applied to variant names
/// when rendering a unit enum to its wire string.
#[derive(Debug, FromMeta)]
pub struct WireEnumArgs {
    #[darling(default)]
    pub rename_all: Option<String>,
}

/// Per-variant attributes on an enum: `#[variant("QBittorrent")]` (the
/// discriminator value this variant matches) and `#[fallback]` (the catch-all).
#[derive(Debug, Default)]
pub struct VariantAttrs {
    pub wire: Option<String>,
    pub fallback: bool,
}

impl VariantAttrs {
    pub fn parse(attrs: &[Attribute]) -> darling::Result<Self> {
        let mut out = VariantAttrs::default();
        for attr in attrs {
            if attr.path().is_ident("fallback") {
                out.fallback = true;
                continue;
            }
            if attr.path().is_ident("variant") {
                let lit: syn::LitStr = attr.parse_args().map_err(darling::Error::from)?;
                out.wire = Some(lit.value());
                continue;
            }
        }
        Ok(out)
    }
}

/// Parsed `#[codec(...)]` sibling attribute.
#[derive(Debug)]
pub enum CodecSpec {
    /// Default. No `#[codec]` attribute present.
    Standard,

    /// `#[codec(FieldsBlob, implementation = "...", config_contract = "...", protocol = "...")]`.
    FieldsBlob {
        implementation: String,
        config_contract: Option<String>,
        protocol: Option<String>,
    },

    /// `#[codec(TaggedByImpl, discriminator = "...")]`.
    TaggedByImpl { discriminator: String },
}

#[derive(Debug, FromMeta)]
struct FieldsBlobMeta {
    implementation: String,
    #[darling(default)]
    config_contract: Option<String>,
    #[darling(default)]
    protocol: Option<String>,
}

#[derive(Debug, FromMeta)]
struct TaggedByImplMeta {
    #[darling(default)]
    discriminator: Option<String>,
}

/// Look for a `#[codec(...)]` attribute in the supplied list and parse it.
/// Returns `CodecSpec::Standard` if no `#[codec]` attribute is present.
pub fn parse_codec_spec(attrs: &[Attribute]) -> darling::Result<CodecSpec> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("codec")) else {
        return Ok(CodecSpec::Standard);
    };

    let Meta::List(list) = &attr.meta else {
        return Err(darling::Error::custom("#[codec(...)] must be a list").with_span(attr));
    };

    // First token is the codec kind (positional). The remainder are
    // key-value pairs interpreted per kind.
    let tokens = list.tokens.clone();
    let parsed: CodecHead =
        syn::parse2(tokens).map_err(|e| darling::Error::custom(e.to_string()).with_span(attr))?;

    match parsed.kind.to_string().as_str() {
        "Standard" => Ok(CodecSpec::Standard),
        "FieldsBlob" => {
            let meta_args = darling::ast::NestedMeta::parse_meta_list(parsed.rest)
                .map_err(darling::Error::from)?;
            let m = FieldsBlobMeta::from_list(&meta_args)?;
            Ok(CodecSpec::FieldsBlob {
                implementation: m.implementation,
                config_contract: m.config_contract,
                protocol: m.protocol,
            })
        }
        "TaggedByImpl" => {
            let meta_args = darling::ast::NestedMeta::parse_meta_list(parsed.rest)
                .map_err(darling::Error::from)?;
            let m = TaggedByImplMeta::from_list(&meta_args)?;
            Ok(CodecSpec::TaggedByImpl {
                discriminator: m
                    .discriminator
                    .unwrap_or_else(|| "implementation".to_string()),
            })
        }
        other => Err(darling::Error::custom(format!(
            "unknown codec kind `{other}` — expected Standard, FieldsBlob, or TaggedByImpl"
        ))
        .with_span(attr)),
    }
}

/// Internal helper: parses the leading ident and the rest of a `#[codec(...)]`
/// argument list. Used because darling can't model "positional ident followed
/// by named args" directly.
struct CodecHead {
    kind: syn::Ident,
    rest: proc_macro2::TokenStream,
}

impl syn::parse::Parse for CodecHead {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kind: syn::Ident = input.parse()?;
        // Optional trailing comma + remaining args.
        let rest = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            input.parse()?
        } else {
            proc_macro2::TokenStream::new()
        };
        Ok(Self { kind, rest })
    }
}

/// Per-field attribute bag parsed from `#[id]` / `#[key]` / `#[wire(...)]` /
/// `#[flatten]` / `#[default(...)]`.
#[derive(Debug, Default)]
pub struct FieldAttrs {
    pub is_id: bool,
    pub is_key: bool,
    pub wire_name: Option<String>,
    pub read_only: bool,
    /// `#[wire(config_only)]` — the user authors this field, but it is not part
    /// of *this* resource's wire body: a `sync = custom` hook writes it to some
    /// other endpoint. Skipped on encode like `read_only`, but **kept in the
    /// generated config docs**, because it is part of the config surface.
    pub config_only: bool,
    /// `#[wire(null)]` — emit a `None` optional as JSON `null` instead of
    /// omitting the key (standard codec).
    pub emit_none: bool,
    /// `#[wire(int)]` — encode a bool as the integer `0`/`1` and decode it back
    /// (standard codec).
    pub as_int: bool,
    pub flatten: bool,
    /// `#[fields_map]` — a `Json` object field encoded as a `[{name, value}]`
    /// array on the wire (the *arr provider fields blob).
    pub fields_map: bool,
    /// Literal from `#[default(<lit>)]` — mapped to `core_lib::DefaultLit` at
    /// emit time.
    pub default: Option<syn::Lit>,
    /// Target type names from `#[reference(<type>, …)]` — marks a plain `i32` /
    /// `Vec<i32>` / `String` field as a ref resolved from `${ref.<type>.<key>}`.
    /// (`ref` is a Rust keyword, so the attribute is spelled `reference`.)
    ///
    /// More than one target is allowed for a polymorphic FK — a field that can
    /// address any of several resource types (Cleanuparr's seeker instance id
    /// points at a Sonarr *or* a Radarr instance). Every listed target becomes a
    /// dependency edge, so the resource is applied after all of them.
    pub reference: Vec<String>,
}

impl FieldAttrs {
    pub fn parse(attrs: &[Attribute]) -> darling::Result<Self> {
        let mut out = FieldAttrs::default();
        for attr in attrs {
            if attr.path().is_ident("id") {
                out.is_id = true;
                continue;
            }
            if attr.path().is_ident("key") {
                out.is_key = true;
                continue;
            }
            if attr.path().is_ident("flatten") {
                out.flatten = true;
                continue;
            }
            if attr.path().is_ident("fields_map") {
                out.fields_map = true;
                continue;
            }
            if attr.path().is_ident("default") {
                let lit: syn::Lit = attr.parse_args().map_err(darling::Error::from)?;
                out.default = Some(lit);
                continue;
            }
            if attr.path().is_ident("reference") {
                // `#[reference(a)]` or `#[reference(a, b)]` — a polymorphic FK
                // lists every type it can address.
                let paths = attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                    )
                    .map_err(darling::Error::from)?;
                if paths.is_empty() {
                    return Err(darling::Error::custom(
                        "#[reference(...)] needs at least one target",
                    )
                    .with_span(attr));
                }
                out.reference = paths
                    .iter()
                    .filter_map(|p| {
                        p.get_ident()
                            .map(|i| i.to_string())
                            .or_else(|| p.segments.last().map(|s| s.ident.to_string()))
                    })
                    .collect();
                continue;
            }
            if attr.path().is_ident("wire") {
                let Meta::List(list) = &attr.meta else {
                    return Err(
                        darling::Error::custom("#[wire(...)] must be a list").with_span(attr)
                    );
                };
                let nested = darling::ast::NestedMeta::parse_meta_list(list.tokens.clone())
                    .map_err(darling::Error::from)?;
                for item in nested {
                    match item {
                        darling::ast::NestedMeta::Meta(Meta::Path(p))
                            if p.is_ident("read_only") =>
                        {
                            out.read_only = true;
                        }
                        darling::ast::NestedMeta::Meta(Meta::Path(p))
                            if p.is_ident("config_only") =>
                        {
                            out.config_only = true;
                        }
                        darling::ast::NestedMeta::Meta(Meta::Path(p)) if p.is_ident("flatten") => {
                            out.flatten = true;
                        }
                        darling::ast::NestedMeta::Meta(Meta::Path(p)) if p.is_ident("null") => {
                            out.emit_none = true;
                        }
                        darling::ast::NestedMeta::Meta(Meta::Path(p)) if p.is_ident("int") => {
                            out.as_int = true;
                        }
                        darling::ast::NestedMeta::Meta(Meta::NameValue(nv))
                            if nv.path.is_ident("name") =>
                        {
                            let syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(s),
                                ..
                            }) = nv.value
                            else {
                                return Err(darling::Error::custom(
                                    "#[wire(name = ...)] expects a string",
                                ));
                            };
                            out.wire_name = Some(s.value());
                        }
                        other => {
                            return Err(darling::Error::custom(format!(
                                "unrecognised #[wire(...)] item: {other:?}"
                            )));
                        }
                    }
                }
            }
        }
        Ok(out)
    }
}

/// Per-field attribute bag parsed from `#[credential(role)]` on a service.
#[derive(Debug, Default)]
pub struct ServiceFieldAttrs {
    pub credential_role: Option<CredentialRole>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialRole {
    ApiKey,
    Bearer,
    User,
    Pass,
}

impl ServiceFieldAttrs {
    pub fn parse(attrs: &[Attribute]) -> darling::Result<Self> {
        let mut out = ServiceFieldAttrs::default();
        for attr in attrs {
            if !attr.path().is_ident("credential") {
                continue;
            }
            let Meta::List(list) = &attr.meta else {
                return Err(
                    darling::Error::custom("#[credential(...)] must be a list").with_span(attr)
                );
            };
            let nested = darling::ast::NestedMeta::parse_meta_list(list.tokens.clone())
                .map_err(darling::Error::from)?;
            for item in nested {
                let darling::ast::NestedMeta::Meta(Meta::Path(p)) = item else {
                    return Err(darling::Error::custom(
                        "#[credential(role)] expects a bare ident",
                    ));
                };
                let role = if p.is_ident("api_key") {
                    CredentialRole::ApiKey
                } else if p.is_ident("bearer") {
                    CredentialRole::Bearer
                } else if p.is_ident("user") {
                    CredentialRole::User
                } else if p.is_ident("pass") {
                    CredentialRole::Pass
                } else {
                    return Err(darling::Error::custom(format!(
                        "unknown credential role `{}` — expected api_key / bearer / user / pass",
                        p.get_ident().map(|i| i.to_string()).unwrap_or_default()
                    )));
                };
                out.credential_role = Some(role);
            }
        }
        Ok(out)
    }
}

/// Collect doc comments (`///` and `#[doc = "..."]`) into one string, joined
/// by newlines. Returns `None` if there are no doc comments.
pub fn collect_doc(attrs: &[Attribute]) -> Option<String> {
    let mut lines: Vec<String> = Vec::new();
    for a in attrs {
        if !a.path().is_ident("doc") {
            continue;
        }
        if let Meta::NameValue(nv) = &a.meta
            && let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &nv.value
        {
            lines.push(s.value().trim().to_string());
        }
    }
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}
