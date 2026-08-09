//! Convergence planner: diff desired wire state against live and emit the
//! ordered [`Op`]s to reconcile them. One diff path, shared by `plan` (preview)
//! and `apply` (execute) in [`crate::apply`]. Each `Op` is fully resolved
//! (method, path, body) so the executor just sends it; an [`Op::Update`] also
//! carries a [`FieldChange`] list for display. Inputs are wire `Value`s with
//! `${ref}` resolved, so planning is pure.
//!
//! ## Model/view seam + secret redaction
//!
//! [`Plan`] is a pure model. A view reaches field values only through
//! [`Op::created_fields`] / [`Op::changed_fields`], which return a
//! redaction-aware [`DisplayValue`] — redaction lives once in [`display_value`],
//! so a new view can't leak a credential. (`Op::*.body` stays public for the
//! executor to send, never for a view to print.) Guards: descriptor-marked
//! secret keys → [`DisplayValue::Redacted`]; complex values (the provider
//! `fields` blob, where most credentials live) → [`DisplayValue::Complex`].

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use serde_json::Value;

use crate::descriptor::{Endpoint, Endpoints};
use crate::merge::merge;
use crate::resolve::resolve_path;

/// One reconciliation operation against the API. `Noop` records an unchanged
/// resource so the report and render account for it without a write.
///
/// The `body` fields are the wire payload the executor *sends*; a view must not
/// print them directly — use [`Op::created_fields`] / [`Op::changed_fields`],
/// which redact secrets (see the module docs).
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    /// In desired but not live — POST `body` to `endpoint`.
    Create {
        key: String,
        endpoint: Endpoint,
        body: Value,
    },
    /// In both, changed — PUT the merged `body` to `path` (endpoint with
    /// `${self.*}` already substituted). `changes` is the field-level diff for
    /// display.
    Update {
        key: String,
        endpoint: Endpoint,
        path: String,
        body: Value,
        changes: Vec<FieldChange>,
    },
    /// Live but not desired — DELETE `path` (prune only).
    Delete {
        key: String,
        endpoint: Endpoint,
        path: String,
    },
    /// In both, no change — no write.
    Noop { key: String },
}

/// One changed field within an [`Op::Update`], for display.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldChange {
    pub key: String,
    /// Live value, or `None` if the key was absent upstream.
    pub from: Option<Value>,
    pub to: Value,
}

/// A redaction-safe rendering of one field value, the only form a view sees.
/// Built by [`display_value`]; views style each variant as they like.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayValue {
    /// A plain scalar, pre-formatted (strings quoted). Safe to print verbatim.
    Scalar(String),
    /// A descriptor-marked secret — its value is withheld.
    Redacted,
    /// An object/array, summarised rather than dumped (may hold secrets).
    Complex,
    /// The field was absent upstream (the `from` side of a newly-set field).
    Absent,
}

impl fmt::Display for DisplayValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisplayValue::Scalar(s) => f.write_str(s),
            DisplayValue::Redacted => f.write_str("<redacted>"),
            DisplayValue::Complex => f.write_str("(…)"),
            DisplayValue::Absent => f.write_str("(absent)"),
        }
    }
}

/// One field's display-side diff for an [`Op::Update`], values already redacted.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDiff {
    pub key: String,
    pub from: DisplayValue,
    pub to: DisplayValue,
}

impl Op {
    /// Display-safe `(field, value)` rows for an [`Op::Create`]'s body (empty for
    /// other ops). Secrets and complex values are redacted/summarised.
    pub fn created_fields(&self, secret: &[String]) -> Vec<(String, DisplayValue)> {
        match self {
            Op::Create { body, .. } => body
                .as_object()
                .map(|o| {
                    o.iter()
                        .map(|(k, v)| (k.clone(), display_value(k, v, secret)))
                        .collect()
                })
                .unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    /// Display-safe field diffs for an [`Op::Update`] (empty for other ops).
    /// Secrets and complex values are redacted/summarised on both sides.
    pub fn changed_fields(&self, secret: &[String]) -> Vec<FieldDiff> {
        match self {
            Op::Update { changes, .. } => changes
                .iter()
                .map(|c| FieldDiff {
                    key: c.key.clone(),
                    from: c
                        .from
                        .as_ref()
                        .map_or(DisplayValue::Absent, |v| display_value(&c.key, v, secret)),
                    to: display_value(&c.key, &c.to, secret),
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

/// The full plan for one service instance: per-resource-type steps in apply
/// order. Returned by `plan` (preview) and derivable from an `apply` run.
#[derive(Debug, Clone, Default)]
pub struct Plan {
    pub steps: Vec<PlanStep>,
}

/// The planned operations for one resource type, plus the descriptor-marked
/// secret wire keys to redact when rendering this type's values.
#[derive(Debug, Clone)]
pub struct PlanStep {
    pub type_name: &'static str,
    pub ops: Vec<Op>,
    pub secret_keys: Vec<String>,
}

/// A tally of a plan or apply run, per operation kind.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Report {
    pub created: u32,
    pub updated: u32,
    pub deleted: u32,
    pub unchanged: u32,
}

impl Plan {
    /// Count of each operation kind across all steps.
    pub fn summary(&self) -> Report {
        let mut r = Report::default();
        for step in &self.steps {
            for op in &step.ops {
                match op {
                    Op::Create { .. } => r.created += 1,
                    Op::Update { .. } => r.updated += 1,
                    Op::Delete { .. } => r.deleted += 1,
                    Op::Noop { .. } => r.unchanged += 1,
                }
            }
        }
        r
    }

    /// True if every operation is a no-op — applying would change nothing.
    pub fn is_empty(&self) -> bool {
        self.steps
            .iter()
            .all(|s| s.ops.iter().all(|o| matches!(o, Op::Noop { .. })))
    }

    /// Human-readable, resource- and field-level plan — the built-in plain view,
    /// and the reference consumer of the [`Op`] display accessors. Each non-empty
    /// resource type gets a section; no-ops are summed into a trailing
    /// `N unchanged` line. Secrets are redacted by the accessors, not here.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for step in &self.steps {
            let secret = &step.secret_keys;
            let mut unchanged = 0u32;
            let mut body = String::new();
            for op in &step.ops {
                match op {
                    Op::Create { key, .. } => {
                        let _ = writeln!(body, "  + create {key}");
                        for (k, v) in op.created_fields(secret) {
                            let _ = writeln!(body, "      {k}: {v}");
                        }
                    }
                    Op::Update { key, .. } => {
                        let _ = writeln!(body, "  ~ update {key}");
                        for ch in op.changed_fields(secret) {
                            let _ = writeln!(body, "      {}: {} -> {}", ch.key, ch.from, ch.to);
                        }
                    }
                    Op::Delete { key, .. } => {
                        let _ = writeln!(body, "  - delete {key}");
                    }
                    Op::Noop { .. } => unchanged += 1,
                }
            }
            if body.is_empty() && unchanged == 0 {
                continue;
            }
            let _ = writeln!(out, "{}:", step.type_name);
            out.push_str(&body);
            if unchanged > 0 {
                let _ = writeln!(out, "  {unchanged} unchanged");
            }
        }
        out
    }
}

/// Map one raw wire value to its redaction-safe [`DisplayValue`] — the single
/// place a value crosses from model to view. Secret keys and complex values
/// never expose their contents (see the module docs).
fn display_value(key: &str, v: &Value, secret: &[String]) -> DisplayValue {
    if secret.iter().any(|s| s == key) {
        return DisplayValue::Redacted;
    }
    match v {
        Value::String(s) => DisplayValue::Scalar(format!("{s:?}")),
        Value::Null => DisplayValue::Scalar("null".to_string()),
        Value::Bool(_) | Value::Number(_) => DisplayValue::Scalar(v.to_string()),
        Value::Array(_) | Value::Object(_) => DisplayValue::Complex,
    }
}

/// Plan a collection: match desired against live by `key`, emitting one
/// create/update/noop per desired element and (with `prune`) a delete per
/// live-only element. Errors if an operation it needs has no declared endpoint.
pub fn plan_collection(
    live: &[Value],
    desired: &[Value],
    key: &str,
    eps: &Endpoints,
    prune: bool,
) -> anyhow::Result<Vec<Op>> {
    let key_of = |v: &Value| v.get(key).map(key_str);
    let live_by_key: HashMap<String, &Value> = live
        .iter()
        .filter_map(|v| key_of(v).map(|k| (k, v)))
        .collect();

    let mut ops = Vec::new();
    for dv in desired {
        let Some(k) = key_of(dv) else { continue };
        match live_by_key.get(&k) {
            None => {
                let endpoint = eps
                    .create
                    .ok_or_else(|| anyhow::anyhow!("no create endpoint"))?;
                ops.push(Op::Create {
                    key: k,
                    endpoint,
                    body: dv.clone(),
                });
            }
            Some(lv) => {
                let body = merge(lv, dv);
                if body == **lv {
                    ops.push(Op::Noop { key: k });
                } else {
                    let endpoint = eps
                        .update
                        .ok_or_else(|| anyhow::anyhow!("no update endpoint"))?;
                    let path = resolve_path(endpoint.path, lv)?;
                    let changes = diff_fields(&body, lv);
                    ops.push(Op::Update {
                        key: k,
                        endpoint,
                        path,
                        body,
                        changes,
                    });
                }
            }
        }
    }

    if prune && let Some(endpoint) = eps.delete {
        let desired_keys: HashSet<String> = desired.iter().filter_map(&key_of).collect();
        for lv in live {
            let Some(k) = key_of(lv) else { continue };
            if !desired_keys.contains(&k) {
                let path = resolve_path(endpoint.path, lv)?;
                ops.push(Op::Delete {
                    key: k,
                    endpoint,
                    path,
                });
            }
        }
    }
    Ok(ops)
}

/// Plan a singleton: merge the presence-masked `desired` over `live`, emitting a
/// single `Update` if anything changed, else `Noop`. No key, no create/delete —
/// a singleton always exists server-side.
pub fn plan_singleton(live: &Value, desired: &Value, eps: &Endpoints) -> anyhow::Result<Vec<Op>> {
    let body = merge(live, desired);
    if body == *live {
        return Ok(vec![Op::Noop { key: String::new() }]);
    }
    let endpoint = eps
        .update
        .ok_or_else(|| anyhow::anyhow!("no update endpoint"))?;
    let path = resolve_path(endpoint.path, live)?;
    let changes = diff_fields(&body, live);
    Ok(vec![Op::Update {
        key: String::new(),
        endpoint,
        path,
        body,
        changes,
    }])
}

/// Field-level diff of the merged `body` against `live`. Only top-level keys are
/// compared; a changed nested object/array surfaces as one entry whose value
/// render summarises (the planner never dives into provider blobs).
fn diff_fields(body: &Value, live: &Value) -> Vec<FieldChange> {
    let (Some(b), l) = (body.as_object(), live.as_object()) else {
        return Vec::new();
    };
    let mut changes = Vec::new();
    for (k, to) in b {
        match l.and_then(|m| m.get(k)) {
            Some(from) if from == to => {}
            Some(from) => changes.push(FieldChange {
                key: k.clone(),
                from: Some(from.clone()),
                to: to.clone(),
            }),
            None => changes.push(FieldChange {
                key: k.clone(),
                from: None,
                to: to.clone(),
            }),
        }
    }
    changes
}

/// Stringify a key field value for desired↔live matching and display.
pub(crate) fn key_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}
