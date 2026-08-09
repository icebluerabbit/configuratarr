//! Apply orchestration: connect → GET live → plan → resolve `${ref}` → merge →
//! (optionally) write, in dependency order.
//!
//! Two entry points share one graph walk ([`run`]): [`plan`] previews (no
//! writes; to-be-created ids are `-1` placeholders) and [`apply`] executes.
//! Because `${ref}` ids are server-assigned during apply, a plan is advisory —
//! `apply` re-runs the walk rather than replaying a stale [`Plan`].
//!
//! Apply order is computed from **static** `#[reference(t)]` metadata
//! ([`crate::ServiceField::ref_targets`]) — not from values. A resource type is
//! applied only after every type it references, so the server [`RefId`]s those refs
//! resolve to already exist.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::{Duration, Instant};

use std::future::Future;
use std::pin::Pin;

use anyhow::Context;
use core_http::HttpClient;
use secrecy::SecretString;
use serde_json::Value;

use crate::plan::{self, Op, Plan, PlanStep};
use crate::resolver::RefId;
use crate::service::{Auth, Connection, Service, ServiceField};
use crate::{Endpoint, HttpMethod, SyncKind};

pub use crate::plan::Report;

/// What a [`CustomSync`] hook did to one desired item — the engine turns this
/// into the [`Op`] for the report, so hooks never touch the plan model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeKind {
    Created,
    Updated,
    Unchanged,
    Removed,
}

/// One reconcile outcome reported by a [`CustomSync`] hook. `detail` is a small
/// set of already-safe display rows for the plan view (label → value); the hook
/// chooses what to surface and **must not put secrets here** — but unlike a raw
/// wire body there's nothing to accidentally dump, only what the hook lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub key: String,
    pub kind: ChangeKind,
    pub detail: Vec<(String, String)>,
}

impl Change {
    /// A created item with no extra detail rows.
    pub fn created(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            kind: ChangeKind::Created,
            detail: Vec::new(),
        }
    }
    /// An updated item with no extra detail rows.
    pub fn updated(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            kind: ChangeKind::Updated,
            detail: Vec::new(),
        }
    }
    /// An unchanged item (no write).
    pub fn unchanged(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            kind: ChangeKind::Unchanged,
            detail: Vec::new(),
        }
    }
    /// A pruned item — live but not declared, deleted by the hook.
    pub fn removed(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            kind: ChangeKind::Removed,
            detail: Vec::new(),
        }
    }
    /// Attach a display row (label → value). Never pass a secret.
    pub fn with(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.detail.push((label.into(), value.into()));
        self
    }
}

/// Future returned by a [`CustomSync::reconcile`] hook.
pub type CustomSyncFuture<'a> =
    Pin<Box<dyn Future<Output = anyhow::Result<Vec<Change>>> + Send + 'a>>;

/// Type-erased pointer to a [`CustomSync::reconcile`], carried by
/// [`SyncKind::Custom`] and dispatched by the executor. The two trailing flags
/// are `(prune, execute)` — `prune` mirrors crud's `--prune` gate (delete live
/// items absent from desired), `execute` gates writes off during a preview.
pub type CustomSyncFn =
    for<'a> fn(&'a HttpClient, &'a [Value], &'a mut RefStore, bool, bool) -> CustomSyncFuture<'a>;

/// A hand-written reconcile hook for a `sync = custom` resource — the escape
/// hatch for APIs that don't fit crud/singleton (multi-endpoint writes,
/// query-keyed identity, server-generated ids: Jellyfin users, libraries, api
/// keys). It is the one place the engine's guarantees don't hold: **the hook
/// owns its own HTTP, ordering, and idempotency**, and must honour `execute`
/// itself (`false` = plan/preview → perform no writes).
///
/// The engine hands it: the live `client`; the resolved `desired` configs
/// (`${ref}` substituted, still snake_case config form — encode via `Self` with
/// [`crate::engine`]); the `refs` store (register created ids so downstream
/// `${ref}` resolve); `prune` (the run's `--prune` gate — when set, delete live
/// items the config no longer declares, mirroring crud); and `execute`. A hook
/// that has no prunable delete simply ignores `prune`. It returns one [`Change`]
/// per item (plus one [`Change::removed`] per pruned item); the engine builds
/// the report [`Op`]s from those, so hooks stay clear of the plan model and
/// can't leak a raw body.
pub trait CustomSync {
    fn reconcile<'a>(
        client: &'a HttpClient,
        desired: &'a [Value],
        refs: &'a mut RefStore,
        prune: bool,
        execute: bool,
    ) -> CustomSyncFuture<'a>;
}

/// Resource type names of `S` in apply order: every referenced type precedes the
/// types that reference it. Errors on a reference cycle. Static — needs no
/// instance.
pub fn apply_order<S: Service>() -> anyhow::Result<Vec<&'static str>> {
    let fields = S::descriptor().fields;
    let managed: BTreeSet<&'static str> = fields.iter().map(|f| f.type_name).collect();

    // node -> the managed types it must be applied *after* (its dependencies).
    let mut deps: BTreeMap<&'static str, BTreeSet<&'static str>> = BTreeMap::new();
    for f in fields {
        let tn = f.type_name;
        let entry = deps.entry(tn).or_default();
        for target in (f.ref_targets)() {
            if managed.contains(target) && target != tn {
                entry.insert(target);
            }
        }
    }

    topo_sort(&deps)
}

/// Kahn's algorithm over `deps` (node → dependencies-it-follows). Deterministic:
/// ties broken alphabetically (BTree ordering).
fn topo_sort(
    deps: &BTreeMap<&'static str, BTreeSet<&'static str>>,
) -> anyhow::Result<Vec<&'static str>> {
    let mut indeg: BTreeMap<&'static str, usize> =
        deps.iter().map(|(n, d)| (*n, d.len())).collect();

    // reverse edges: dependency -> nodes that follow it
    let mut dependents: BTreeMap<&'static str, Vec<&'static str>> = BTreeMap::new();
    for (n, ds) in deps {
        for d in ds {
            dependents.entry(*d).or_default().push(*n);
        }
    }

    let mut ready: BTreeSet<&'static str> = indeg
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(n, _)| *n)
        .collect();
    let mut out = Vec::with_capacity(deps.len());

    while let Some(&n) = ready.iter().next() {
        ready.remove(n);
        out.push(n);
        if let Some(following) = dependents.get(n) {
            for &m in following {
                let e = indeg.get_mut(m).expect("node in indeg");
                *e -= 1;
                if *e == 0 {
                    ready.insert(m);
                }
            }
        }
    }

    if out.len() != deps.len() {
        anyhow::bail!("reference cycle among resources — cannot order apply");
    }
    Ok(out)
}

// ── HTTP execution ───────────────────────────────────────────────────────────

/// Knobs for one plan/apply run.
#[derive(Debug, Default, Clone, Copy)]
pub struct ApplyOptions {
    /// Delete live resources absent from desired state.
    pub prune: bool,
}

/// Accumulates `(resource type, key) -> server id` as resources are applied, so
/// `${ref.type.key}` in later (topologically-downstream) resources resolves to a
/// real id. A [`crate::RefSource`]. Populated from both live (existing) ids and
/// create responses.
#[derive(Debug, Default)]
pub struct RefStore {
    /// (type, key) → the server-assigned [`RefId`] (int for *arr, string for GUID
    /// APIs).
    ids: HashMap<(String, String), RefId>,
}

impl RefStore {
    pub fn insert(&mut self, type_name: &str, key: &str, id: RefId) {
        self.ids
            .insert((type_name.to_string(), key.to_string()), id);
    }
}

impl crate::resolver::RefSource for RefStore {
    fn lookup(&self, type_name: &str, key: &str) -> Option<RefId> {
        self.ids
            .get(&(type_name.to_string(), key.to_string()))
            .cloned()
    }
}

/// Build an HTTP client from a service's connection bundle. Async because the
/// form/cookie scheme performs a login round-trip before the client is usable;
/// the header schemes just stamp a default header and return immediately.
pub async fn connect(conn: &Connection<'_>) -> anyhow::Result<HttpClient> {
    let mut b = HttpClient::builder(conn.url.clone());
    if conn.insecure == Some(true) {
        b = b.insecure();
    }
    if let Some(t) = conn.timeout_secs {
        b = b.timeout(Duration::from_secs(t));
    }
    b = match &conn.auth {
        Auth::None => b,
        Auth::ApiKey { header, key } => b.header(header, SecretString::new(key.expose().into())),
        Auth::ApiKeyQuery { param, key } => b.query(*param, SecretString::new(key.expose().into())),
        Auth::Bearer { token } => b.header(
            "Authorization",
            SecretString::new(format!("Bearer {}", token.expose()).into()),
        ),
        Auth::Basic { user, pass } => {
            use base64::Engine;
            let creds = base64::engine::general_purpose::STANDARD.encode(format!(
                "{}:{}",
                user,
                pass.expose()
            ));
            b.header(
                "Authorization",
                SecretString::new(format!("Basic {creds}").into()),
            )
        }
        // Form/cookie auth keeps a session cookie rather than a header; enable
        // the cookie store now and log in below once the client exists.
        Auth::FormCookie { .. } => b.cookies(),
    };
    let client = b.build()?;

    // Establish the session before returning, so every later request carries the
    // cookie. The credentials POST as the conventional `username`/`password` form
    // fields; a non-2xx login surfaces here (like a bad api key would).
    if let Auth::FormCookie {
        login_path,
        user,
        pass,
    } = &conn.auth
    {
        let pairs = vec![
            ("username".to_string(), user.to_string()),
            ("password".to_string(), pass.expose().to_string()),
        ];
        client.login_form(login_path, &pairs).await?;
    }

    Ok(client)
}

/// Poll the service's declared `health` endpoint until it responds OK, or
/// `timeout` elapses. No-op if the service declares no health endpoint. The poll
/// is authenticated (uses the connection's auth), so a ready response also
/// confirms credentials. On timeout it surfaces the last error (connection
/// refused / 401 / 503 / …), so misconfiguration is diagnosable, not just "slow".
pub async fn wait_healthy<S: Service>(svc: &S, timeout: Duration) -> anyhow::Result<()> {
    let Some(path) = S::descriptor().health_check else {
        return Ok(());
    };
    let client = connect(&svc.connection()).await?;
    let interval = Duration::from_secs(2);
    let deadline = Instant::now() + timeout;
    loop {
        let err = match client.get::<Value>(path).await {
            Ok(_) => return Ok(()),
            Err(e) => e,
        };
        if Instant::now() >= deadline {
            anyhow::bail!(
                "`{}` not healthy after {}s: {err:#}",
                S::descriptor().name,
                timeout.as_secs()
            );
        }
        tokio::time::sleep(interval).await;
    }
}

/// Preview the changes for a service instance without writing anything. Walks
/// the dependency graph, GETs live state, and diffs — but to-be-created ids are
/// `-1` placeholders (the server hasn't assigned them), so refs downstream of a
/// pending create resolve to `-1` in the preview. For display / confirmation.
pub async fn plan<S: Service>(
    svc: &S,
    instance: &Value,
    opts: ApplyOptions,
) -> anyhow::Result<Plan> {
    run(svc, instance, opts, false).await
}

/// Sync a service instance against its API and return a [`Report`] tally. Same
/// walk as [`plan`], but writes each operation and registers real server ids as
/// it goes, so `${ref}` resolves to live ids in dependency order.
pub async fn apply<S: Service>(
    svc: &S,
    instance: &Value,
    opts: ApplyOptions,
) -> anyhow::Result<Report> {
    Ok(run(svc, instance, opts, true).await?.summary())
}

/// The shared graph walk behind both [`plan`] and [`apply`]. With `execute`,
/// operations are sent and create responses supply real ids; without it nothing
/// is written and creates register a `-1` placeholder. Either way it returns the
/// full [`Plan`] (the executed or previewed operations).
async fn run<S: Service>(
    svc: &S,
    instance: &Value,
    opts: ApplyOptions,
    execute: bool,
) -> anyhow::Result<Plan> {
    let client = connect(&svc.connection()).await?;
    let fields = S::descriptor().fields;
    let mut refs = RefStore::default();
    let mut steps = Vec::new();

    for tn in apply_order::<S>()? {
        let field = fields
            .iter()
            .find(|f| f.type_name == tn)
            .expect("ordered type has a field");
        // Dispatch on the declared sync strategy, not the Vec/Option shape.
        let step = match (field.sync)() {
            SyncKind::Crud => {
                collection_step(&client, instance, field, &mut refs, opts, execute).await?
            }
            SyncKind::Singleton => {
                singleton_step(&client, instance, field, &mut refs, execute).await?
            }
            SyncKind::Custom(hook) => {
                custom_step(&client, instance, field, &mut refs, hook, opts, execute).await?
            }
            SyncKind::Embedded => {
                anyhow::bail!("embedded resource `{tn}` cannot be a top-level service field")
            }
        };
        if let Some(step) = step {
            steps.push(step);
        }
    }
    Ok(Plan { steps })
}

/// Plan one collection (performs writes when `execute`). `None` if unmanaged.
async fn collection_step<S: Service>(
    client: &HttpClient,
    instance: &Value,
    field: &ServiceField<S>,
    refs: &mut RefStore,
    opts: ApplyOptions,
    execute: bool,
) -> anyhow::Result<Option<PlanStep>> {
    let tn = field.type_name;
    let eps = (field.endpoints)();
    let key = (field.key_wire)()
        .ok_or_else(|| anyhow::anyhow!("collection `{tn}` has no natural key"))?;

    // Absent config key = unmanaged: leave the server untouched. An empty
    // array IS managed (prune clears it).
    let Some(cfgs) = instance.get(field.name).and_then(Value::as_array).cloned() else {
        return Ok(None);
    };

    // Register live ids first so refs to existing resources resolve.
    let list_ep = eps
        .list
        .ok_or_else(|| anyhow::anyhow!("collection `{tn}` has no list endpoint"))?;
    let live: Vec<Value> = client.get(list_ep.path).await?;
    register_live_ids(refs, tn, &key, &live);

    let desired = resolve_all(&cfgs, refs, |r| (field.config_to_wire)(&r))?;

    let ops = plan::plan_collection(&live, &desired, &key, &eps, opts.prune)
        .with_context(|| format!("planning collection `{tn}`"))?;
    run_ops(client, tn, &ops, refs, execute).await?;
    Ok(Some(PlanStep {
        type_name: tn,
        ops,
        secret_keys: (field.secret_keys)(),
    }))
}

/// Resolve `${ref}` in each desired config entry, applying `xform` to the result
/// (wire-encode for crud, identity for custom which encodes via its own type).
/// Shared by the `Vec<R>` step functions; the caller does the absent-key
/// (unmanaged) check first, since that must precede any API call.
fn resolve_all(
    cfgs: &[Value],
    refs: &RefStore,
    xform: impl Fn(Value) -> anyhow::Result<Value>,
) -> anyhow::Result<Vec<Value>> {
    cfgs.iter()
        .map(|cfg| {
            let mut resolved = cfg.clone();
            crate::resolve::resolve_refs(&mut resolved, refs)?;
            xform(resolved)
        })
        .collect()
}

/// Dispatch a `sync = custom` resource to its [`CustomSync`] `hook`. Resolves
/// `${ref}` in each desired config (still snake_case form — the hook encodes via
/// its own type), runs the hook, and turns the returned [`Change`]s into the
/// report's [`Op`]s. `None` if the config key is absent (unmanaged).
async fn custom_step<S: Service>(
    client: &HttpClient,
    instance: &Value,
    field: &ServiceField<S>,
    refs: &mut RefStore,
    hook: CustomSyncFn,
    opts: ApplyOptions,
    execute: bool,
) -> anyhow::Result<Option<PlanStep>> {
    let tn = field.type_name;
    // A custom resource is `Vec<R>` (array config) or `Option<R>` (a *custom
    // singleton* — one object config). Normalise the singleton object to a
    // one-element list so the hook always sees `&[Value]`; absent/null = unmanaged.
    let cfgs: Vec<Value> = match instance.get(field.name) {
        None | Some(Value::Null) => return Ok(None),
        Some(Value::Array(a)) => a.clone(),
        Some(obj) => vec![obj.clone()],
    };

    let desired = resolve_all(&cfgs, refs, Ok)?;
    let changes = hook(client, &desired, refs, opts.prune, execute)
        .await
        .with_context(|| format!("custom sync `{tn}`"))?;

    // Export this resource's server ids into the RefStore so later resources can
    // `${ref.<tn>.<key>}` it — the same registration `collection_step` does, which
    // is why crud resources are referenceable and custom ones weren't. Keyed
    // custom collections (indexer, filter, irc) qualify; custom singletons (no
    // `key_wire`) don't. Runs post-hook so freshly-created ids are included, and
    // in both plan and apply (a read-only list GET), matching topological order.
    register_refs(client, tn, field, refs).await;

    let ops = changes.into_iter().map(change_to_op).collect();
    Ok(Some(PlanStep {
        type_name: tn,
        ops,
        secret_keys: Vec::new(),
    }))
}

/// Register `(<key> -> server id)` into `refs` for each live item that has both,
/// so `${ref.<tn>.<key>}` resolves. The shared core of both ref-registration
/// paths ([`collection_step`] inline, [`register_refs`] after a custom hook).
fn register_live_ids(refs: &mut RefStore, tn: &'static str, key: &str, live: &[Value]) {
    for lv in live {
        if let (Some(k), Some(id)) = (
            lv.get(key).map(plan::key_str),
            lv.get("id").and_then(RefId::from_value),
        ) {
            refs.insert(tn, &k, id);
        }
    }
}

/// Register refs from a raw list-endpoint response value: only a plain JSON array
/// carries `[{id, <key>}]` items. A non-array (e.g. Jellyfin's `{Items:[…]}`
/// wrapper) yields nothing — see [`register_refs`].
fn register_list_refs(refs: &mut RefStore, tn: &'static str, key: &str, live: &Value) {
    if let Some(arr) = live.as_array() {
        register_live_ids(refs, tn, key, arr);
    }
}

/// Register a keyed resource's server ids so `${ref.<type>.<key>}` resolves in
/// later-applied resources. No-op when the resource has no natural key or list
/// endpoint (custom singletons).
///
/// **Best-effort**, unlike the sync itself. It costs a second list GET (the
/// custom hook already fetched its own copy but owns that HTTP, so the engine
/// can't reuse it), and any failure is swallowed: a list that isn't a plain
/// `[{id, <key>}]` array (Jellyfin's `{Items:[…]}`) or a list GET the API rejects
/// simply yields no refs rather than failing the apply. A resource that genuinely
/// *is* referenced but can't be listed flatly surfaces later as an unresolved
/// ref — the clearer error.
async fn register_refs<S: Service>(
    client: &HttpClient,
    tn: &'static str,
    field: &ServiceField<S>,
    refs: &mut RefStore,
) {
    let (Some(key), Some(list_ep)) = ((field.key_wire)(), (field.endpoints)().list) else {
        return;
    };
    if let Ok(live) = client.get::<Value>(list_ep.path).await {
        register_list_refs(refs, tn, &key, &live);
    }
}

/// Build the display-only report [`Op`] for one custom [`Change`]. Custom ops are
/// never executed (the hook already did its HTTP), so the endpoint/path are inert
/// placeholders; the `detail` rows become the op body for the plan view.
fn change_to_op(c: Change) -> Op {
    // Inert — a custom op is only ever read by the report/renderer, never sent.
    const INERT: Endpoint = Endpoint {
        method: HttpMethod::Post,
        path: "",
    };
    let body = Value::Object(
        c.detail
            .into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect(),
    );
    match c.kind {
        ChangeKind::Created => Op::Create {
            key: c.key,
            endpoint: INERT,
            body,
        },
        ChangeKind::Updated => Op::Update {
            key: c.key,
            endpoint: INERT,
            path: String::new(),
            body,
            changes: Vec::new(),
        },
        ChangeKind::Unchanged => Op::Noop { key: c.key },
        ChangeKind::Removed => Op::Delete {
            key: c.key,
            endpoint: INERT,
            path: String::new(),
        },
    }
}

/// Plan one singleton: GET, merge presence-masked desired over it, PUT if
/// changed (performs the write when `execute`). `None` if unmanaged.
async fn singleton_step<S: Service>(
    client: &HttpClient,
    instance: &Value,
    field: &ServiceField<S>,
    refs: &mut RefStore,
    execute: bool,
) -> anyhow::Result<Option<PlanStep>> {
    let tn = field.type_name;
    let Some(cfg) = instance.get(field.name) else {
        return Ok(None);
    };
    if cfg.is_null() {
        return Ok(None);
    }

    let eps = (field.endpoints)();
    let read_ep = eps
        .read
        .ok_or_else(|| anyhow::anyhow!("singleton `{tn}` has no read endpoint"))?;
    let live: Value = client.get(read_ep.path).await?;

    let mut resolved = cfg.clone();
    crate::resolve::resolve_refs(&mut resolved, &*refs)?;
    let desired = (field.config_to_wire_present)(&resolved)?;

    let ops = plan::plan_singleton(&live, &desired, &eps)
        .with_context(|| format!("planning singleton `{tn}`"))?;
    run_ops(client, tn, &ops, refs, execute).await?;
    Ok(Some(PlanStep {
        type_name: tn,
        ops,
        secret_keys: (field.secret_keys)(),
    }))
}

/// Perform the side effects of planned ops, registering ids into `refs` so
/// downstream resources resolve their `${ref}`s. A `Create` learns its id from
/// the create response under `execute`, or a `-1` placeholder in preview mode;
/// `Update`/`Delete` only send under `execute`. Shared by collections and
/// singletons (a singleton only ever yields `Update`/`Noop`).
async fn run_ops(
    client: &HttpClient,
    tn: &'static str,
    ops: &[Op],
    refs: &mut RefStore,
    execute: bool,
) -> anyhow::Result<()> {
    for op in ops {
        match op {
            Op::Create {
                key,
                endpoint,
                body,
            } => {
                let id = if execute {
                    let resp = send(client, endpoint.method, endpoint.path, Some(body)).await?;
                    resp.get("id")
                        .and_then(RefId::from_value)
                        .unwrap_or(RefId::Pending)
                } else {
                    // Preview: the id is server-assigned, so it stays pending.
                    RefId::Pending
                };
                refs.insert(tn, key, id);
            }
            Op::Update {
                endpoint,
                path,
                body,
                ..
            } => {
                if execute {
                    send(client, endpoint.method, path, Some(body)).await?;
                }
            }
            Op::Delete { endpoint, path, .. } => {
                if execute {
                    send(client, endpoint.method, path, None).await?;
                }
            }
            Op::Noop { .. } => {}
        }
    }
    Ok(())
}

/// Dispatch one HTTP call by method. Returns the response body as JSON
/// (`Null` for DELETE).
async fn send(
    client: &HttpClient,
    method: HttpMethod,
    path: &str,
    body: Option<&Value>,
) -> anyhow::Result<Value> {
    let null = Value::Null;
    match method {
        HttpMethod::Get => client.get::<Value>(path).await,
        HttpMethod::Post => client.post(path, body.unwrap_or(&null)).await,
        HttpMethod::Put => client.put(path, body.unwrap_or(&null)).await,
        HttpMethod::Patch => client.patch(path, body.unwrap_or(&null)).await,
        HttpMethod::Delete => {
            client.delete(path).await?;
            Ok(Value::Null)
        }
    }
}

#[cfg(test)]
mod register_ref_tests {
    use super::*;
    use crate::resolver::{RefId, RefSource};
    use serde_json::json;

    #[test]
    fn registers_id_and_key_from_array() {
        let mut refs = RefStore::default();
        let live = vec![
            json!({ "name": "TL", "id": 7 }),
            json!({ "name": "PTP", "id": 9 }),
        ];
        register_live_ids(&mut refs, "indexer", "name", &live);
        assert_eq!(refs.lookup("indexer", "TL"), Some(RefId::Int(7)));
        assert_eq!(refs.lookup("indexer", "PTP"), Some(RefId::Int(9)));
    }

    #[test]
    fn skips_items_missing_id_or_key() {
        let mut refs = RefStore::default();
        let live = vec![
            json!({ "name": "no-id" }),
            json!({ "id": 3 }),
            json!({ "name": "ok", "id": 5 }),
        ];
        register_live_ids(&mut refs, "t", "name", &live);
        assert_eq!(refs.lookup("t", "no-id"), None);
        assert_eq!(refs.lookup("t", "ok"), Some(RefId::Int(5)));
    }

    #[test]
    fn list_refs_only_reads_plain_arrays() {
        // A bare array registers…
        let mut refs = RefStore::default();
        register_list_refs(&mut refs, "t", "name", &json!([{ "name": "a", "id": 1 }]));
        assert_eq!(refs.lookup("t", "a"), Some(RefId::Int(1)));

        // …but a wrapper object (Jellyfin's `{Items:[…]}`) or scalar registers nothing.
        let mut refs = RefStore::default();
        register_list_refs(
            &mut refs,
            "t",
            "name",
            &json!({ "Items": [{ "name": "a", "id": 1 }] }),
        );
        assert_eq!(refs.lookup("t", "a"), None);
    }
}
