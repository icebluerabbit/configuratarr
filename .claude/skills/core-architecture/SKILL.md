---
name: core-architecture
description: >
  Deep reference for configuratarr's engine internals — the descriptor-driven model, the four
  axes, the codecs, value-driven plan/apply, merge, resolution, and the plan/render DisplayValue
  seam. Read this before modifying anything in crates/core, crates/core-macros, the executor, or
  the codecs. Auto-triggers when: editing core/core-macros, changing a codec or the engine,
  touching plan/apply/merge/resolve, or reasoning about how encode/decode/refs work.
---

# Core Architecture

For *using* the system (adding resources/services), see `add-resource` / `add-service`. This is the engine internals — read it before changing `core`.

## The inversion: descriptor-driven, not derive-driven

Proc macros (`core-macros`) emit **only static descriptors** — data describing a resource's fields, roles, endpoints, codec, and sync strategy. They generate no runtime logic beyond field accessor closures. **All behavior is hand-written generic Rust** in `core` (`core-lib`), parameterised on `T: Described` / `S: Service`, dispatching on the descriptor at runtime.

Consequences: no `Interpolated<T>` wrapper, no `CollectRefs`/`Referenceable`/`ServiceResources` derives, no `schemars`. A ref is a plain `i32` with `#[reference]` metadata; a credential is a `SecretValue` (secret-ness inferred from type). Interpolation is resolved on the JSON `Value` tree **before** decode, so typed structs only ever hold resolved values.

## `core/src` map

```
descriptor.rs   ResourceDescriptor, Case (camel|pascal), Endpoint(s), HttpMethod, SyncKind,
                CodecMeta, DefaultLit, FieldDescriptor (name/doc/role/kind/wire_name/read_only/
                default/secret/flatten/fields_map/reference/nested_docs/get/set), VariantDescriptor
described.rs     Described trait (descriptor/empty/encode_variant/decode_variant/...);
                 ResourceErased + ErasedField (the type-erased walk used by nested recursion + doc-gen)
field.rs         FieldKind / FieldRef / FieldValue / FieldRole
codec/           standard, fields_blob, tagged_by_impl, string_enum, config; CodecKind
engine.rs        encode/decode/decode_config dispatch; key_wire_name, reference_targets,
                 secret_wire_keys, field_docs/resource_docs (doc-gen)
resolve.rs       ${env}/${file}/${ref}/${self} on the Value tree (one `substitute` primitive)
resolver.rs      StaticEnv / RefSource traits + SystemEnv; RefId (Int(i64)|Str|Pending) — the
                 bounded ref id; Pending = a not-yet-created id (preview / id-less create response),
                 substitutes as RefId::PENDING (-1)
apply.rs         (…executor…) + CustomSync trait / CustomSyncFn — the `sync = custom` hook seam
reconcile.rs     reconcile primitives for custom hooks: present_keys, create_only (keyed
                 create-or-leave), upsert (keyed create-or-update, no prune, caller-supplied
                 in_sync predicate), replace (whole-list structural replace), echo (copy a
                 server-owned field back into an update body) — each write primitive owns the
                 execute-gate + Change emission so a hook can't write during a preview
merge.rs         sparse-update merge (live ⊕ desired; fields[] by name)
plan.rs          Plan/PlanStep/Op/FieldChange/Report + DisplayValue + render()
apply.rs         plan()/apply()/run() executor; topo order; RefStore; connect/auth
service.rs       Service trait, ServiceDescriptor, ServiceField, Connection, Auth
```

## The four orthogonal axes

| Axis | Declared by | Runtime type | Meaning |
|---|---|---|---|
| **endpoints** | `list/read/create/update/delete = verb("/path")` | `Endpoints` | pure data: verb + path (`${self.*}`-capable) |
| **codec** | which macro you use | `CodecKind` | wire shape |
| **sync** | `sync = crud\|singleton\|custom` | `SyncKind` | write strategy; the executor dispatches on this, **not** struct shape (`Custom` carries its hook) |
| **auth** | `auth = ...` on `#[service]` | `Auth` | None / ApiKey / Bearer / Basic / FormCookie |

Extension model: **select** (name an impl) → **register** (add an enum variant + dispatch arm) → **custom** (`<axis> = custom` → a hand-written hook in the contributor's crate). Dispatch is static enum + `match` (compile-time exhaustive; the central edit is the review gate).

## Codecs

- **Standard** — snake→camelCase JSON object by default; a resource may set `case = pascal`
  (`ResourceDescriptor.case`, applied in `wire_key`) for .NET-style PascalCase APIs (Jellyfin). Casing is
  descriptor *data*, not macro behaviour — one implementation (`to_camel_case`; pascal = +upcase first).
  Two field-level wire options (also descriptor data, standard codec only): `#[wire(null)]` emits a `None`
  optional as an explicit JSON `null` instead of omitting the key (`FieldDescriptor.emit_none`);
  `#[wire(int)]` renders a `bool` as the int `0`/`1` and decodes it back (`FieldDescriptor.as_int`). They
  compose — added so a `sync = custom` reconcile can `engine::encode` instead of hand-rolling `Value`
  translation (bazarr `LanguageProfile`/`Notifier`).
- **FieldsBlob** — `{implementation, configContract, fields:[{name,value}]}`; each typed field → one entry. For an **open** key set (no fixed struct — e.g. Prowlarr Cardigann indexers), a `#[fields_map]` `Json` field on a Standard struct (`RawProvider`) carries a `name: value` map that the standard wire codec splays to / collects from the same `fields:[{name,value}]` array.
- **TaggedByImpl** — reads a discriminator key, delegates to the matching variant's codec.
- **StringEnum** — unit enum ↔ bare wire string.
- **Custom** — `value.custom_encode()` / `T::custom_decode()`.

Two ingest/wire codecs: the **config codec** (`codec/config.rs`, snake `field.name` keys, applies `#[default]`, presence-maskable for singletons) decodes user YAML; the **wire codec** (camelCase, fields-blob) is the API form. `#[flatten]` hoists a nested struct's keys into the parent both ways.

## Value-driven plan/apply

Resources stay as config `Value`s until apply (so unresolved `${ref}` never needs to fit a typed `i32`). Per resource type, in topo order from static `#[reference]` metadata (`engine::reference_targets`, descending `#[flatten]`):

```
GET live → register existing ids → resolve ${ref} → config_to_wire (decode_config→encode)
        → plan_collection / plan_singleton → execute (POST/PUT/DELETE)
A Create registers its new server id into RefStore for downstream refs.
```

**Two-phase, one walk** (`apply::run(.., execute)`): `plan()` previews (no writes; to-be-created ids are `-1` placeholders); `apply()` re-runs and writes. A plan is advisory — apply does not replay a stale Plan, because ids are server-assigned.

## Merge (sparse update)

PUT body = `merge(live, desired)`: live base, desired wins, omitted keys keep live values. The provider `fields:[{name,value}]` array merges **by name** (live-only entries preserved). This is the one *arr-ism; it lives in `merge.rs` (structural, not codec-dispatched) because the blob is flattened into a Standard resource that can't see it.

## Plan model/view seam (+ secret redaction)

`Plan` is a pure model. A view reaches field values **only** through `Op::created_fields` / `Op::changed_fields`, which return a redaction-aware `DisplayValue { Scalar / Redacted / Complex / Absent }`. Redaction lives once in `plan::display_value`: descriptor-marked secret keys → `Redacted`; objects/arrays (the provider `fields` blob, where most credentials live) → `Complex`, never dumped. `Op.body` stays public for the executor to *send*, never for a view to print. `Plan::render()` is the built-in plain view; a future `cli-ui` crate would consume `DisplayValue` for color/TUI — core gets no view deps.

## Doc-gen hooks (descriptor-native)

`engine::resource_docs::<T>()` walks `T::empty().descriptor_erased()`, descends `#[flatten]`, drops read-only/id, and surfaces: flat fields, provider blocks (from flattened tagged enums via `VariantDescriptor.field_docs`), nested types (via `FieldDescriptor.nested_docs` fn-pointers — reachable even when the value is absent), and `#[wire_enum]` allowed values. `config-doc-gen` BFSs that graph into a fully cross-linked markdown doc. No global type registry — the macro emits the fn-pointers.

## Seams

**Built (extend, don't reinvent):**
- **String/GUID ids** — the ref id is [`resolver::RefId`] (`Int(i64)|Str|Pending`); `RefStore`/`RefSource` are keyed by it, resolve substitutes `RefId::to_value()`. *arr stay `Int`. `Pending` is the not-yet-created id (a `plan` preview, or a create whose response carried no id) — it substitutes as `RefId::PENDING` (`-1`); use it, never a bare `-1`. **In conformance tests** the id type must match the FK's Rust type, and one double can't serve both: `core_testkit::check` resolves every `${ref}` through `DummyRefs` (`Int(1)`), so a GUID service uses `check_with_refs(.., &GuidRefs)` instead — an `Int` in a `String` FK fails decode with `expected string, got 1`.
- **Multi-target references** — `FieldDescriptor.reference` is `&'static [&'static str]`, so `#[reference(a, b)]` declares a *polymorphic* FK (cleanuparr's seeker instance id addresses a sonarr **or** radarr instance). This matters because `apply::apply_order` derives the dependency graph **only** from that metadata — `resolve::collect_refs` is not on the apply path, so a `${ref.…}` a field didn't declare creates no edge, and the resource can be applied before its target exists. Ties break alphabetically, so a missing edge breaks in one direction only and is easy to miss.
- **`SyncKind::Custom(CustomSyncFn)`** — the hook is carried by the variant (no separate descriptor field); `apply::custom_step` dispatches to a hand-written async reconcile hook for APIs that don't fit crud/singleton (multi-endpoint, query-keyed identity, server-gen ids). The hook returns `Vec<Change>` (Created/Updated/Unchanged + safe display rows) and the engine builds the report `Op`s — so hooks never touch the plan model or leak a raw body, **but** the hook owns its own HTTP/ordering/idempotency and must honour `execute`. **Don't hand-roll the recurring mechanics** — the `core::reconcile` primitives (`present_keys`, `create_only` = keyed create-or-leave, `upsert` = keyed create-or-update with a caller-supplied `in_sync` predicate + no prune, `replace` = whole-list structural replace) own the execute-gate + `Change` emission so a hook can't accidentally write during a preview; a hook supplies only its service-specific diff/write. `upsert` is the home for a collection that doesn't round-trip its writes (server-rewritten ids, redacted secrets, enriched sub-arrays) — where merge-equality would churn forever, so idempotency is the caller's predicate (autobrr `subset`, indexer `readable_matches`) and an update echoes server-owned fields back with `reconcile::echo`. Encode config→wire once via `engine::encode_config::<Self>`. Only genuinely bespoke flows (multi-endpoint per item like jellyfin `user`, the two-step create + action-reconcile of autobrr `filter`, sparse form singletons like bazarr `settings`) keep an explicit `if execute` — and even those live in the *service* crate, not core.
- **PascalCase codec** — `case = pascal` (see Codecs).
- **Form/cookie auth** — `auth = form_cookie(login_path=…, username=…, password=…)`. `apply::connect` builds a cookie-store client, POSTs `username`/`password` as a form to `login_path`, and the session cookie rides every request. (Bearer/api-key/basic remain header schemes; OAuth2 still open.)
- **Query-param writes** — `HttpClient::post_query(path, &[(k,v)], body)` for APIs that carry a resource's identity/params in the query string, not the JSON body (Jellyfin libraries by `name`/`paths`, api keys by `app`). reqwest percent-encodes each pair (repeated names survive) — a hook never hand-builds an escaped query string. (`post` JSON body, `post_form` urlencoded body are the other write shapes.)

**Still open (build when a service demands):** async `AuthScheme`/`Authenticator` traits (OAuth2 bails today; `Basic` is a header, `FormCookie` is wired); fanned update (`update: Option<Endpoint>` → slice); pagination; normalization-before-diff.

## Non-*arr service shapes (deliberate divergences)

Two patterns that look like slop but are intentional — mirror the reasoning, don't "fix" them:

- **One endpoint, many sections (bazarr).** A service whose entire config is a single endpoint (`/api/system/settings`) is modelled as a `sync = custom` singleton whose fields are `Option<Section>` (present = manage, absent = leave). Sibling concerns that write the *same* endpoint (bazarr `languages`) can be a second custom resource — the shared write is fine, they don't `#[reference]` each other. The "resource = a REST collection with CRUD identity" model is a polite fiction here; that's expected for config-blob APIs.
- **Typed sections vs open blob — the closed/open rule.** Enumerate typed structs (bazarr's 33 provider sections) when the key set is **closed and worth documenting** — you get per-field docs + validation, at the cost of one file per section. Use an open `#[fields_map]` `Json` blob (prowlarr Cardigann `RawProvider`) when the key set is **open/dynamic** and can't be enumerated. Pick by the API's nature, not by file count.
