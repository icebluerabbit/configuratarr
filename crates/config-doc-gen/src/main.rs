//! Generates `docs/<service>-config.md` from the service descriptors. Walks each
//! `Service`'s `ServiceDescriptor` and renders a field table per resource using
//! `engine::field_docs` — no schemars, no derives. Adding a service is one row in
//! [`service_registry::service_registry`].

use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use core_lib::engine::{FieldDoc, NestedRef};
use core_lib::{DefaultLit, Service};

#[derive(Parser)]
#[command(about = "Generate config documentation for all services")]
struct Args {
    /// Directory to write `<service>-config.md` files into.
    #[arg(long, default_value = "docs")]
    output_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    std::fs::create_dir_all(&args.output_dir)?;

    // Registry: rendered from the shared service table. Each row is gated by
    // `#[cfg(feature = <tag>)]`, so a single-service build documents only that
    // service. Adding a service is one row in `service_registry::service_registry`.
    macro_rules! gen_docs {
        ($($v:ident => $tag:literal : $ty:path = $title:literal),+ $(,)?) => {{
            // Pushes are `#[cfg]`-gated per service, so this can't be a `vec![]`.
            let mut docs: Vec<(&'static str, String)> = Vec::new();
            $(
                #[cfg(feature = $tag)]
                docs.push(($tag, render_service::<$ty>($title)));
            )+
            docs
        }};
    }
    #[allow(clippy::vec_init_then_push)]
    let docs = service_registry::service_registry!(gen_docs);

    for (slug, content) in docs {
        let path = args.output_dir.join(format!("{slug}-config.md"));
        std::fs::write(&path, content)?;
        eprintln!("  wrote {}", path.display());
    }
    Ok(())
}

/// Connection fields are consumed by `#[service]` (not part of the resource
/// descriptors), so they're documented from the shared *arr-family shape.
const CONNECTION_SECTION: &str = "\
## Connection

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Base URL of the service API. |
| `api_key` | secret string | yes | API key, sent in the auth header. |
| `insecure` | boolean | no | Skip TLS certificate verification. |
| `timeout_secs` | integer | no | Request timeout in seconds. |

## Resources

";

fn render_service<S: Service>(title: &str) -> String {
    let d = S::descriptor();
    let mut out = format!("# {title} Configuration\n\n");
    if let Some(doc) = d.doc {
        out.push_str(doc);
        out.push_str("\n\n");
    }
    out.push_str(CONNECTION_SECTION);

    // Nested types are rendered once, after the resources. Seed `seen` with the
    // resource type names so a nested ref never duplicates a resource section.
    let mut seen: HashSet<&'static str> = d.fields.iter().map(|f| f.type_name).collect();
    let mut queue: Vec<NestedRef> = Vec::new();

    for f in d.fields {
        let res = title_case(f.type_name);
        out.push_str(&format!("### {res}\n\n"));
        if let Some(doc) = (f.type_doc)() {
            out.push_str(doc);
            out.push_str("\n\n");
        }
        let rd = (f.resource_docs)();
        out.push_str(&render_fields(&rd.fields));

        // Provider enums: a selector linking each discriminator value to its
        // subsection, then one subsection per implementation (the fields-blob).
        for provider in &rd.providers {
            let options: Vec<String> = provider
                .variants
                .iter()
                .map(|v| format!("[`{}`](#{})", v.wire, slug(&format!("{res}: {}", v.name))))
                .collect();
            out.push_str(&format!(
                "Set `{}` to one of: {}.\n\n",
                provider.discriminator,
                options.join(" / "),
            ));
            for v in &provider.variants {
                out.push_str(&format!("#### {res}: {}\n\n", v.name));
                out.push_str(&render_fields(&v.fields));
            }
        }
        enqueue(&rd.nested, &mut seen, &mut queue);
    }

    // Referenced nested types (transitively), each its own linkable section.
    if !queue.is_empty() {
        out.push_str("## Types\n\n");
    }
    let mut i = 0;
    while i < queue.len() {
        let n = queue[i];
        i += 1;
        let rd = (n.docs)();
        out.push_str(&format!("### {}\n\n", title_case(n.type_name)));
        if rd.enum_values.is_empty() {
            out.push_str(&render_fields(&rd.fields));
        } else {
            let vals: Vec<String> = rd.enum_values.iter().map(|v| format!("`{v}`")).collect();
            out.push_str(&format!("Allowed values: {}.\n\n", vals.join(" / ")));
        }
        enqueue(&rd.nested, &mut seen, &mut queue);
    }
    out
}

/// Queue not-yet-seen nested types for rendering.
fn enqueue(nested: &[NestedRef], seen: &mut HashSet<&'static str>, queue: &mut Vec<NestedRef>) {
    for n in nested {
        if seen.insert(n.type_name) {
            queue.push(*n);
        }
    }
}

/// GitHub-flavoured-markdown header anchor: lowercase, drop punctuation, spaces
/// and hyphens become `-`. Matches the slug GitHub assigns to `## Heading`.
fn slug(heading: &str) -> String {
    let mut out = String::new();
    for c in heading.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if c == ' ' || c == '-' {
            out.push('-');
        }
    }
    out
}

fn render_fields(fields: &[FieldDoc]) -> String {
    if fields.is_empty() {
        return "_No user-configurable fields._\n\n".into();
    }
    let mut out = String::from(
        "| Field | Type | Required | Default | Description |\n|---|---|---|---|---|\n",
    );
    for f in fields {
        let required = if f.required { "yes" } else { "no" };
        let default = f.default.map(default_str).unwrap_or_default();

        // Link a nested-type label (`language`, `array of `quality_profile_item``)
        // to that type's section under `## Types`.
        let type_cell = match f.nested_type {
            Some(tn) => f.type_label.replace(
                &format!("`{tn}`"),
                &format!("[`{tn}`](#{})", slug(&title_case(tn))),
            ),
            None => f.type_label.clone(),
        };

        let mut desc = f.doc.unwrap_or_default().replace('\n', " ");
        // Link each ref to its target resource's section (`tag` → `### Tag`).
        match f.reference {
            [] => {}
            [r] => {
                let anchor = slug(&title_case(r));
                desc = format!(
                    "{desc} References a [`{r}`](#{anchor}) by name (`${{ref.{r}.<key>}}`)."
                );
            }
            // A polymorphic FK: the field may address any of these types, so
            // document every option.
            many => {
                let targets: Vec<String> = many
                    .iter()
                    .map(|r| format!("[`{r}`](#{})", slug(&title_case(r))))
                    .collect();
                let forms: Vec<String> = many
                    .iter()
                    .map(|r| format!("`${{ref.{r}.<key>}}`"))
                    .collect();
                desc = format!(
                    "{desc} References either {} by name ({}).",
                    targets.join(" or "),
                    forms.join(" or "),
                );
            }
        }
        if f.secret {
            desc.push_str(" Credential — redacted in plan output.");
        }
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} |\n",
            f.name,
            type_cell,
            required,
            default,
            desc.trim(),
        ));
    }
    out.push('\n');
    out
}

fn default_str(d: DefaultLit) -> String {
    match d {
        DefaultLit::Bool(b) => format!("`{b}`"),
        DefaultLit::Int(i) => format!("`{i}`"),
        DefaultLit::Float(f) => format!("`{f}`"),
        DefaultLit::Str(s) => format!("`{s}`"),
    }
}

/// `download_client` → `Download Client`.
fn title_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(first) => first.to_uppercase().chain(c).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
