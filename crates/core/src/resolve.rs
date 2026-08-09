//! Value-tree interpolation. `${env.*}` / `${file.*}` / `${ref.*}` are resolved
//! on the `serde_json::Value` desired-state tree *before* it is decoded into a
//! typed resource — so typed structs only ever hold resolved values.
//!
//! Two phases:
//! * [`resolve_static`] — env vars, file reads, literals. Runs once, up front.
//!   Leaves `${ref.*}` untouched.
//! * [`collect_refs`] + [`resolve_refs`] — references resolve late, in
//!   topological order, against ids learned during apply.
//!
//! A value that is *exactly* one `${...}` expression is replaced by the
//! resolved value with its JSON type preserved (a ref → a number). An
//! expression embedded in a larger string is substituted textually.

use serde_json::Value;

use crate::refs::RefExpr;
use crate::resolver::{RefSource, StaticEnv};

/// Resolve `${env.*}` and `${file.*}` in place. `${ref.*}` is left for the
/// later ref phase; any other namespace is an error.
pub fn resolve_static(value: &mut Value, env: &dyn StaticEnv) -> anyhow::Result<()> {
    walk(value, &|ns, arg| match ns {
        "env" => env
            .env(arg)
            .map(|s| Some(Value::String(s.to_string())))
            .ok_or_else(|| anyhow::anyhow!("environment variable `{arg}` is not set")),
        "file" => env.file(arg).map(|s| Some(Value::String(s))),
        "ref" => Ok(None), // resolved later, in topo order
        other => Err(anyhow::anyhow!(
            "unknown interpolation namespace `${{{other}.*}}`"
        )),
    })
}

/// Resolve `${ref.<type>.<key>}` in place against the accumulated id map.
pub fn resolve_refs(value: &mut Value, refs: &dyn RefSource) -> anyhow::Result<()> {
    walk(value, &|ns, arg| match ns {
        "ref" => {
            let (ty, key) = arg
                .split_once('.')
                .ok_or_else(|| anyhow::anyhow!("malformed reference `${{ref.{arg}}}`"))?;
            let id = refs
                .lookup(ty, key)
                .ok_or_else(|| anyhow::anyhow!("unresolved reference `${{ref.{arg}}}`"))?;
            // Substitute the id as its native JSON value (Number for *arr, String
            // for GUID APIs) so the FK keeps its wire type.
            Ok(Some(id.to_value()))
        }
        _ => Ok(None),
    })
}

/// Resolve `${self.<field>}` in an endpoint path against a resource's value
/// (the live/merged wire object, where the server id lives). Used by the
/// executor to build update/delete URLs like `/api/v3/tag/${self.id}`.
///
/// Only the `self` namespace is valid here — `${env}`/`${file}`/`${ref}` were
/// resolved earlier. An unknown field or namespace is an error.
pub fn resolve_path(template: &str, obj: &Value) -> anyhow::Result<String> {
    let (out, _) = substitute(template, |ns, field| {
        if ns != "self" {
            anyhow::bail!("only `${{self.*}}` is valid in a path, got `${{{ns}.{field}}}`");
        }
        let v = obj.get(field).ok_or_else(|| {
            anyhow::anyhow!("path `${{self.{field}}}`: no field `{field}` on resource")
        })?;
        Ok(Some(stringify(v)))
    })?;
    Ok(out)
}

/// Collect every `${ref.<type>.<key>}` in the tree as a dependency edge.
pub fn collect_refs(value: &Value) -> Vec<RefExpr> {
    let mut out = Vec::new();
    collect(value, &mut out);
    out
}

fn collect(v: &Value, out: &mut Vec<RefExpr>) {
    match v {
        Value::String(s) => {
            for (ns, arg) in templates(s) {
                if ns == "ref"
                    && let Some((ty, key)) = arg.split_once('.')
                {
                    out.push(RefExpr {
                        target_type: ty.to_string(),
                        key: key.to_string(),
                    });
                }
            }
        }
        Value::Array(a) => a.iter().for_each(|x| collect(x, out)),
        Value::Object(o) => o.values().for_each(|x| collect(x, out)),
        _ => {}
    }
}

/// Recurse the tree, applying `f` to each string leaf. `f(namespace, arg)`
/// returns `Some(value)` to substitute, `None` to leave the expression in place.
fn walk<F>(v: &mut Value, f: &F) -> anyhow::Result<()>
where
    F: Fn(&str, &str) -> anyhow::Result<Option<Value>>,
{
    match v {
        Value::String(s) => {
            if let Some(nv) = resolve_str(s, f)? {
                *v = nv;
            }
        }
        Value::Array(a) => {
            for x in a.iter_mut() {
                walk(x, f)?;
            }
        }
        Value::Object(o) => {
            for x in o.values_mut() {
                walk(x, f)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn resolve_str<F>(s: &str, f: &F) -> anyhow::Result<Option<Value>>
where
    F: Fn(&str, &str) -> anyhow::Result<Option<Value>>,
{
    // Whole-value template: preserve the resolved JSON type.
    if let Some(inner) = whole_template(s) {
        let (ns, arg) = split_ns(inner);
        return f(ns, arg);
    }
    if !s.contains("${") {
        return Ok(None);
    }
    // Embedded: textual substitution; unresolved expressions stay verbatim.
    let (out, changed) = substitute(s, |ns, arg| Ok(f(ns, arg)?.map(|v| stringify(&v))))?;
    Ok(if changed {
        Some(Value::String(out))
    } else {
        None
    })
}

/// Walk every `${...}` in `s`, replacing each via `f`. `f(ns, arg)` returns
/// `Some(text)` to substitute or `None` to keep the `${ns.arg}` literal in
/// place. Returns the rebuilt string plus whether any substitution happened.
/// The single brace-scanning loop behind both embedded resolution
/// ([`resolve_str`]) and path resolution ([`resolve_path`]).
fn substitute<F>(s: &str, f: F) -> anyhow::Result<(String, bool)>
where
    F: Fn(&str, &str) -> anyhow::Result<Option<String>>,
{
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    let mut changed = false;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let end = after
            .find('}')
            .ok_or_else(|| anyhow::anyhow!("unterminated `${{` in `{s}`"))?;
        let inner = &after[..end];
        let (ns, arg) = split_ns(inner);
        match f(ns, arg)? {
            Some(text) => {
                out.push_str(&text);
                changed = true;
            }
            None => {
                out.push_str("${");
                out.push_str(inner);
                out.push('}');
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    Ok((out, changed))
}

/// If `s` is exactly one `${...}`, return its inner text.
fn whole_template(s: &str) -> Option<&str> {
    let inner = s.strip_prefix("${")?.strip_suffix('}')?;
    // Reject `${a}${b}` (the first `}` must be the last char).
    if inner.contains('}') || inner.contains("${") {
        return None;
    }
    Some(inner)
}

/// Split `namespace.rest` on the first dot. `rest` keeps any further dots
/// (file paths, `type.key`).
fn split_ns(inner: &str) -> (&str, &str) {
    inner.split_once('.').unwrap_or((inner, ""))
}

/// Every `${ns.arg}` in a string, as `(ns, arg)` pairs.
fn templates(s: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    let mut rest = s;
    while let Some(start) = rest.find("${") {
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else { break };
        out.push(split_ns(&after[..end]));
        rest = &after[end + 1..];
    }
    out
}

fn stringify(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}
