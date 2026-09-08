---
name: add-resource
description: >
  Procedure + reference for adding a resource to an existing service crate in configuratarr
  (e.g. a new Radarr resource, or a resource in a sibling *arr crate). Covers the three
  archetypes (collection / singleton / provider), the macro-per-codec mapping, the field
  attribute reference, wiring into mod.rs + the service struct, and the conformance test.
  Auto-triggers when: adding/modeling a resource struct, choosing a codec or archetype,
  deciding which field attribute to use, or wiring a new resource into a service.
---

# Adding a Resource

For a resource **in an *arr crate** (radarr/sonarr/lidarr/prowlarr/whisparr/readarr), mirror `crates/radarr-v3/` exactly — it is the canonical template, every decision there was deliberate, do not invent alternatives. For a resource in a **non-*arr crate** (jellyfin, overseerr, autobrr, bazarr), radarr's *archetypes and codecs* (collection / singleton / provider-fields-blob) are **not guaranteed to apply** — that API may have a different auth/path/codec shape and no provider concept. Don't assume; first know the engine.

> **Required before modeling a resource whose shape you're unsure of:** load the **`core-architecture`** skill and read `crates/core` (`descriptor.rs`, `codec/`, `field.rs`, `service.rs`) + the macro args in `core-macros`. The archetype/attribute tables below are the *common* instantiations of that engine, not its limits. The engine — the four axes (endpoints / codec / sync / auth) and the codec set — is what tells you which shape a given resource needs.

## 0. Inspect the spec first

Use the `openapi-tools` skill before writing any struct: `list_paths` to confirm the endpoint (→ `sync` kind + paths), `get_resource` for the full field list + which fields are `readOnly`/nullable/defaulted. These read the spec only — you write the struct by hand in the macro form (next sections).

## 1. Pick the archetype

| Archetype | When | `sync` |
|---|---|---|
| **Collection** | named, CRUD (tags, profiles) | `crud` |
| **Singleton** | one object, no key (config/*) | `singleton` |
| **Provider** | dynamic `fields[]` blob (download clients, indexers, notifications, import lists) | `crud` + composition |

## 2. The codec is selected by *which macro* you use

| Macro | CodecKind | For |
|---|---|---|
| `#[resource(...)]` | Standard | flat JSON (camelCase wire keys) |
| `#[fields_blob(...)]` | FieldsBlob | provider variant configs (`fields:[{name,value}]`) |
| `#[tagged(by = "...")]` | TaggedByImpl | discriminator-dispatched provider enum |
| `#[wire_enum(rename_all = "...")]` | StringEnum | unit enum → bare string |
| `#[nested]` | Standard, Embedded | shared sub-struct hoisted via `#[flatten]` |

**Macro ordering gotcha:** the resource macro is the **outermost** attribute; field helper attrs (`#[id]`, `#[key]`, `#[wire(...)]`, …) sit **below** it, or they leak through unstripped and rustc rejects them.

## 3. Archetype shapes

### Collection — `tag.rs`
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
    #[key] pub label: String,        // natural key — ${ref.tag.<label>}
}
```

### Singleton — `ui_config.rs`
```rust
#[resource(
    sync = singleton,
    read   = get("/api/v3/config/ui"),
    update = put("/api/v3/config/ui/${self.id}"),   // write path is /{id}
)]
pub struct UiConfig {
    #[id] pub id: Option<i32>,
    pub first_day_of_week: i32,
    pub theme: Option<String>,
}
```

### Provider — three composed pieces

**Envelope** (`provider.rs`, shared) — `#[nested]` struct with identity + tag refs + read-only metadata:
```rust
#[nested]
pub struct Provider {
    #[id]  pub id: Option<i32>,
    #[key] pub name: String,
    #[reference(tag)] pub tags: Vec<i32>,
    #[wire(name = "implementationName", read_only)] pub implementation_name: Option<String>,
    #[wire(read_only)] pub message: Option<ProviderMessage>,
    #[wire(read_only)] pub presets: Vec<Json>,
}
```

**Outer resource** (`download_client.rs`) — flattens envelope + typed config, adds its own fields:
```rust
#[resource(sync = crud, list = get("/api/v3/downloadclient"), create = post(...), update = put(".../${self.id}"), delete = delete(".../${self.id}"))]
pub struct DownloadClient {
    #[flatten] pub common: Provider,
    #[flatten] pub config: DownloadClientProvider,
    pub enable: bool,
    pub protocol: DownloadProtocol,
    #[default(1)] pub priority: i32,
}
```

**Tagged enum** (`download_clients/mod.rs`) — one `#[variant("<Impl>")]` per implementation, `#[fallback]` preserves unmodelled impls via `RawProvider`:
```rust
#[tagged(by = "implementation")]
pub enum DownloadClientProvider {
    #[variant("QBittorrent")] QBittorrent(QBittorrentConfig),
    // ... one per impl, one file each under download_clients/
    #[fallback] Unknown(RawProvider),
}
```

**Variant** (`download_clients/qbittorrent.rs`) — each typed field → one `{name,value}` blob entry:
```rust
#[fields_blob(implementation = "QBittorrent", config_contract = "QBittorrentSettings", protocol = "torrent")]
pub struct QBittorrentConfig {
    pub host: Option<String>,
    pub password: Option<SecretValue>,                    // credential — redacted
    #[wire(name = "useSsl")] pub use_ssl: Option<bool>,   // override irregular API casing
}
```

### Where the provider variant list + `fields[]` come from (the OpenAPI spec does NOT have them)

The static spec types `fields` as a generic `[{name, value}]` array — it tells you **nothing** about which implementations exist or what fields each one has. Two sources fill that gap:

1. **Live `/schema` endpoint** (e2e shell) — ground truth, but needs a running server.
2. **The devopsarr terraform provider for this app** — the best *offline* source. `specs/terraform_providers.txt` lists the repos (read it — it grows). Map `terraform-provider-<app>` ↔ `<app>-vN` (e.g. `sonarr-v3` → `terraform-provider-sonarr`). Clone shallow **into scratch** (never the repo): `git clone --depth 1 <url> <scratch>/...`. Then under `internal/provider/`:
   - **`<resource>_<impl>_resource.go` filenames = the variant list** (e.g. every `download_client_*_resource.go` → one `#[variant(...)]`). This is what you copy into the `#[tagged]` enum + one file each.
   - Inside a file: the `...Implementation` const → `#[fields_blob(implementation = "...")]`; the `...ConfigContract` const → `config_contract = "..."`; the model struct's `` `tfsdk:"..."` `` tags → the snake_case field names; Go types (`types.String/Int64/Bool`) → field types; **`Sensitive: true` on an attribute → `SecretValue`**.

3. **The app's own C# source** — required when the tf provider is stale or targets a different fork (see the whisparr warning below); `src/NzbDrone.Core/{Download/Clients,Indexers,ImportLists,Notifications,Extras/Metadata}` for *arr apps. The settings class's `[FieldDefinition]` properties **are** the `fields[]`; `PrivacyLevel.Password`/`.ApiKey` → `SecretValue`; a non-simple property type nests, and `ClientSchema/SchemaBuilder.cs` flattens it to a **dotted** wire name (`seedCriteria.seedRatio`) by lower-casing only the first character of each segment.

> ⚠ **`terraform-provider-whisparr` does not apply to `whisparr-v3`.** It is pinned at Feb 2024 and targets Whisparr **V2** — a *Sonarr* fork — while `whisparr-v3` models the **eros** line, a *Radarr* fork. It carries `import_list_couch_potato/imdb/plex/trakt_*`, `indexer_rarbg`, and a `metadata_config_resource.go` for an endpoint eros does not have. Following it ships ~10 variants that do not exist. Use source 3 for whisparr, and check any provider's freshness before trusting it.

**Authority + the trap this closes:** the tf provider can lag the live API, so `/schema` wins on conflict — but it is right about the *shape*, and it is **per-app**. Sonarr's Transmission has `tv_category`/`tv_directory`/`recent_tv_priority`; Radarr's has `movieCategory`/`movieDirectory`/`recentMoviePriority`. Cloning radarr's variant and renaming the struct silently ships movie fields to a series API — checking the matching app's provider is how you avoid it. When in doubt about *any* provider variant or field, consult this list before guessing.

## 4. Field attribute reference

| Situation | Attribute |
|---|---|
| Server-assigned id (read-only on write) | `#[id]` |
| Natural key (`${ref.<type>.<key>}` + diff key) | `#[key]` |
| FK to another resource by name | `#[reference(<type>)]` on `i32` / `Option<i32>` / `Vec<i32>` (or `String` for a GUID API) |
| FK that can address several resource types | `#[reference(<type_a>, <type_b>)]` — see below |
| Wire name ≠ snake→camel | `#[wire(name = "...")]` |
| Present on read, never sent | `#[wire(read_only)]` |
| Emit a `None` optional as explicit `null` (not omitted) | `#[wire(null)]` |
| Encode a `bool` as the int `0`/`1` (and decode back) | `#[wire(int)]` |
| Nested struct hoisted to parent keys | `#[flatten]` |
| Non-zero default for an absent config key | `#[default(expr)]` |
| A list the user may omit entirely | `Option<Vec<T>>` (see below) |
| Open `name: value` settings map ↔ *arr `fields:[{name,value}]` blob | `#[fields_map]` on a `Json` field |
| Credential | field type `SecretValue` (no attribute — inferred) |

`#[fields_map]` is for an **open / dynamically-keyed** provider blob whose settings can't be a fixed struct (e.g. Prowlarr indexers — Cardigann definitions vary per tracker). The user authors a plain YAML map (`fields: { baseUrl: ..., apiKey: ... }`); the standard wire codec splays it to `fields:[{name,value}]` on encode and collects it back on decode. `${env}`/`${ref}` resolve inside the values; the whole map redacts as `Complex` in plan output. Used by `raw_provider.rs` (`RawProvider`). For a **closed** variant set, use typed `#[fields_blob]` variants instead — only reach for `#[fields_map]` when the key set is genuinely open.

`Option<Vec<T>>` (scalar `T`: `bool`/`i32`/`i64`/`String`) is the shape for a list whose *absence* means "leave the server's list alone". A plain `Vec<T>` **always encodes**, so an undeclared one sends `[]` and clobbers whatever the server holds — the same trap a non-`Option` scalar has. Reach for it when the API substitutes defaults for an empty array (bindery `Indexer.categories`) or the list is server-owned (komga `Library.scan_directory_exclusions`). `Option<Vec<Json>>` and `Option<Vec<Nested>>` have no carrier and won't compile: use `Option<Json>` for an opaque array.

`#[wire(null)]` and `#[wire(int)]` (standard codec only) exist so a hand-rolled `sync = custom` translator doesn't have to. Reach for them when the API's read shape needs a `None` optional present as an explicit `null` (so a whole-object diff stays structural) or stores a boolean as an int — e.g. bazarr's `LanguageProfile` (`#[wire(null, int)]` on `original_format`) and `Notifier` (`#[wire(null)]` on `url`). They compose (`#[wire(null, int)]`), so a custom reconcile can just `engine::encode(&decode_config::<T>(v)?)` instead of poking `serde_json::Value` field-by-field. Mixed casing within one document is handled by giving a nested child struct its own `case = ...` — not a field attribute.

**A string-id resource must declare `#[id]`.** `engine::id_shape` reads that field to decide whether a `plan`'s not-yet-created-ref placeholder is the integer `-1` or the string `"-1"`, and defaults to integer. Omit `#[id]` on a GUID service and a first `plan` that creates the resource plus something referencing it aborts with `expected string, got -1` (see `core-architecture`). `#[id]` implies read-only, so adding it changes no wire payload.

`#[reference(...)]` takes **one or more** target type names. Its only runtime job is the dependency graph: `apply::apply_order` builds edges purely from this metadata — **not** from the `${ref.…}` text in a user's config (`resolve::collect_refs` exists but nothing in the apply path calls it). So a *polymorphic* FK — one field that can address any of several resource types — must list every target, or the resource can be applied before the ones it points at and the reference fails to resolve. Cleanuparr's seeker instance id is the case that forced this: `#[reference(sonarr_instance, radarr_instance)]` on an `Option<String>`. Ties in the topo sort break alphabetically, which is exactly why a missing edge is easy to miss — it breaks in one direction only.

A `#[wire_enum]` variant may override the derived `rename_all` name with `#[variant("...")]`, for an enum whose wire values don't share one casing rule (Cleanuparr's `DownloadClientTypeName`: `Deluge`/`Transmission` are PascalCase but `qBittorrent`/`uTorrent`/`rTorrent` are lower-camel). Same precedence as `#[tagged]` — an explicit `#[variant]` always wins.

Endpoints are **always explicit** (`verb("/path")`); paths may carry `${self.<field>}` (resolved against the live/merged value at apply, e.g. `/api/v3/tag/${self.id}`).

## 5. Wire it in

1. `pub mod <resource>;` in `src/resources/mod.rs`.
2. Add the field to the `#[service]` struct (`Vec<R>` = collection, `Option<R>` = singleton).
3. Add a `tests/testdata/<resource>/config.yaml` fixture (realistic snake_case user config; may use `${ref}` / `${env}`). **Cover every non-scalar field** — `Option<Nested>`, `Vec<Nested>`, nested objects — not just the scalar happy path. A field the fixture omits is a field no test exercises: that's exactly how `quality_profile.items` (a `Vec<Nested>`) silently encoded as `[]` for ages. (You don't need to include read-only fields — they're never written.)
4. Add one line to `tests/spec_conformance.rs`:
   `conformance!(<name>, resources::<r>::<Type>, "<SchemaName>Resource", "testdata/<r>/config.yaml");`
5. Add a `///` doc comment to the struct + each field (feeds `config-doc-gen`).

## 6. Verify

```bash
nix develop --command cargo nextest run -p <crate> --test spec_conformance   # wire shape vs OpenAPI schema
nix develop --command cargo run -q -p config-doc-gen                  # regenerate docs
```
Conformance validates the encoded payload against the spec schema (`additionalProperties: false` catches mis-casing / hallucinated / mis-typed fields) **and** checks the codec round-trips (decode→encode is byte-stable — catches a field that decodes but doesn't re-encode, which schema validation alone misses since a dropped array still validates as an array). Both only fire on fields the fixture actually includes — hence the "cover every non-scalar field" rule above. For lifecycle/FK behavior, add an e2e case (see the `add-service` skill's e2e section).
