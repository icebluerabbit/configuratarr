# Contributing

Configuratarr keeps your `*arr` apps configured from a file you check into git, instead of clicking through web UIs and praying the SQLite database survives the next redeploy. It talks to Radarr, Sonarr, Lidarr, Prowlarr, and friends over their REST APIs and makes the app match your config.

Most contributions are one of these:

- **"My download client / indexer / notification isn't supported."** Add it — usually one small file.
- **"You don't support \<app\> yet."** Add the whole app — a new crate, but it's mostly filling in a template.
- **"This field is wrong / missing."** Fix the struct, the test will tell you if you got it right.

`crates/radarr-v3/` is the worked example for all of it. When in doubt, copy what radarr does — it's the template on purpose.

---

## The one big idea

You don't write sync logic. You write Rust structs that mirror the app's API, sprinkle a few annotations on them, and the engine handles diffing, ordering, secrets, and pushing changes. A struct looks like this:

```rust
#[resource(
    sync = crud,
    list   = get("/api/v3/tag"),
    create = post("/api/v3/tag"),
    update = put("/api/v3/tag/${self.id}"),
    delete = delete("/api/v3/tag/${self.id}"),
)]
pub struct Tag {
    #[id]  pub id: Option<i32>,
    #[key] pub label: String,
}
```

That's a complete, working resource. You told it the endpoints, that it's a normal create/read/update/delete thing (`sync = crud`), which field is the server id, and which field is the name people reference. Done.

`crud` is the common case. A config object with no key is `sync = singleton`; and an API that doesn't fit either (multi-endpoint writes, server-generated GUID ids, whole-list replaces — e.g. Jellyfin users/libraries/repositories) is `sync = custom` with a reconcile hook. For the recurring hook shapes, don't hand-roll the write loop — build on the `core::reconcile` primitives (`create_only`, `upsert`, `replace`), which own the preview gate so a `plan` can't accidentally write. `upsert` covers the common "create-or-update by key, no prune" case for APIs that don't round-trip their writes (you pass an `in_sync` idempotency predicate instead of relying on merge-equality). Non-*arr apps that serialise PascalCase add `case = pascal`. The `core-architecture` skill covers these; reach for them only when the plain `crud`/`singleton` shape doesn't fit.

---

## Adding a download client / indexer / notification / import list

These all share a shape in the *arr apps: a common envelope (name, tags, enabled…) plus a per-implementation settings blob. So they're built from a few pieces, and adding *one more implementation* is the easy case.

Say qBittorrent already works and you want to add Deluge. You:

1. Look at the API's fields for Deluge (the `openapi-tools` scripts pull them straight from the spec).
2. Copy `crates/radarr-v3/src/resources/download_clients/qbittorrent.rs` to `deluge.rs`, change the fields to match.
3. Add one line to `download_clients/mod.rs` registering it: `#[variant("Deluge")] Deluge(DelugeConfig),`.

A variant file is just the settings, named to match the API:

```rust
#[fields_blob(implementation = "Deluge", config_contract = "DelugeSettings", protocol = "torrent")]
pub struct DelugeConfig {
    pub host: Option<String>,
    pub port: Option<i32>,
    pub password: Option<SecretValue>,          // credentials use SecretValue — auto-redacted
    #[wire(name = "useSsl")] pub use_ssl: Option<bool>,   // when the API spells it weird
}
```

Indexers, notifications, and import lists work exactly the same way (`indexers/`, `notifications/`, `import_lists/`). One file, one line in the `mod.rs`, done.

---

## Adding a whole new app (Sonarr, Lidarr, …)

Bigger, but it's a template, not a research project. Copy `crates/radarr-v3/` to `crates/sonarr-v3/`, then:

- Model each resource (most are nearly identical to radarr — tags, quality profiles, download clients, the config screens).
- Tell the app's struct who it is: `#[service(name = "sonarr_v3", auth = api_key(header = "X-Api-Key"), …)]`.
- Register it so users can pick it: add **one row** to the `service_registry!` table in `crates/service-registry/` (this drives both the `core-config` dispatch and the `config-doc-gen` config docs), mark the crate as an optional dep + feature in those two crates' `Cargo.toml`, and flip its row in the README's **Supported services** table to ✅.
- Copy radarr's Nix test files so CI can spin up a real Sonarr in a VM and test against it, and add the app to the NixOS module's startup ordering (one line) so the sync runs after it.

The engine, the diffing, the secret handling, the docs generator — none of that changes. You're describing an app, not extending the core.

---

## The annotations you'll actually use

| You want… | Write |
|---|---|
| the server-assigned id | `#[id]` |
| the name people reference (`${ref.tag.4k}`) | `#[key]` |
| a link to another resource by name | `#[reference(tag)]` on the id field |
| a field the API spells in camelCase | `#[wire(name = "useSsl")]` |
| a field the API returns but won't accept back | `#[wire(read_only)]` |
| a default when the user leaves it out | `#[default(25)]` |
| a password / API key | just make it a `SecretValue` |

Write a `///` comment above each struct and field while you're at it — those become the config reference docs (`docs/<app>-config.md`) automatically.

---

## Testing — the part that keeps you honest

For each resource, drop a realistic `config.yaml` under `tests/testdata/<resource>/` and add one line to `tests/spec_conformance.rs`. That test takes your struct, encodes it the way it'd be sent to the app, and checks it against the app's own OpenAPI schema. If you mistyped a field name, used the wrong case, or invented a field that doesn't exist, **the test fails and tells you exactly where.** It's the safety net that lets you copy-paste confidently.

There's also a live test tier (`tests/e2e.rs`) that talks to a real app started by the dev shell — worth adding for anything with interesting create/update/delete behavior.

---

## Getting set up

Everything lives in the Nix dev shell, so you don't install a Rust toolchain or Python by hand:

```bash
nix develop                 # drops you in with cargo + the spec tools
cargo build && cargo nextest run
nix develop .#e2e-radarr    # starts a real Radarr and points the e2e tests at it
nix run .#generate-docs     # regenerate the docs after you change a resource or the CLI
```

Always peek at the API spec before writing a struct — `get_resource` and friends (in the dev shell) show you exactly what fields an endpoint has. Copy radarr, run the conformance test, and you're contributing.

---

## Not built yet (in case your app needs it)

If your app does something the *arr family doesn't, you might hit a wall here — these are known gaps, and a PR adding them is welcome:

- **Login/cookie or OAuth auth** — right now only API-key and bearer-token apps work out of the box.
- **String ids** — ids are assumed to be numbers; apps like Jellyfin use string GUIDs.
- **Apps that page their lists, or rewrite values when you save** — no pagination or normalization hooks yet.

None of these block the *arr apps; they're for the weirder corners of the ecosystem.
