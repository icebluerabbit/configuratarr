---
name: openapi-tools
description: >
  Use this skill whenever working with OpenAPI specs in the configuratarr project — inspecting
  schemas, discovering resources, comparing schemas across APIs, checking what fields a resource
  has, or finding which paths use a schema. Auto-triggers when: investigating what schemas exist,
  comparing radarr vs sonarr schemas, listing a resource's fields/readOnly/defaults before writing
  a struct, or confirming an endpoint's methods.
---

# OpenAPI Tools

Five Python scripts expose OpenAPI **spec exploration** — they read the JSON spec so the
LLM doesn't grep a 300k-line file by hand. They are inspection-only: **no Rust is generated or parsed.**
Use them before writing or editing any resource struct to get the
exact field set, types, nullability, readOnly flags, defaults, and endpoint methods from the spec.

**Where they live + how to run them.** The scripts are at the **repo-root `tools/` directory** (`<repo>/tools/list_resources.py`, …), **NOT** inside this skill's directory — `ls .claude/skills/openapi-tools/tools/` will fail; don't look there. `flake.nix` (`mkTool`) wraps each into a PATH command, so they exist **only inside the nix devshell**. Invoke as bare commands: `nix develop --command list_resources specs/<svc>.json` (or run them once already inside `nix develop`). Paths you pass to them (`specs/<svc>.json`, `crates/<svc>/spec/<svc>.json`) are repo-relative.

## Tools

### Discovery
**`list_resources <spec>`** — all schema names with tags (`[crud]`, `[singleton]`, `[provider]`, `[enum]`).
Flags: `--crud`, `--singleton`, `--provider`, `--enums`, `--no-enums`
```
list_resources crates/radarr-v3/spec/radarr-v3.json --singleton
list_resources crates/radarr-v3/spec/radarr-v3.json --crud --no-enums
```

**`list_paths <spec> [prefix]`** — all API paths with HTTP methods.
```
list_paths crates/radarr-v3/spec/radarr-v3.json /api/v3/config
```

### Schema Inspection
**`get_resource <spec> "<ref>"`** — full schema detail: field types, nullability, readOnly, defaults,
enum values resolved inline, which paths use it, and classification (CRUD/singleton/read-only).
```
get_resource crates/radarr-v3/spec/radarr-v3.json "#/components/schemas/DelayProfileResource"
```

**`get_path <spec> <path>`** — full path detail: methods, parameters, request/response schema refs.
```
get_path crates/radarr-v3/spec/radarr-v3.json /api/v3/config/naming/{id}
```

**`compare_schemas <spec1> "<ref1>" <spec2> "<ref2>" [--all]`** — field diff between two schemas,
optionally across two different specs. Shows only-in-A, only-in-B, type differences. `--all` also shows identical fields.
```
compare_schemas crates/radarr-v3/spec/radarr-v3.json "#/components/schemas/QualityProfileResource" \
                specs/sonarr-v3.json "#/components/schemas/QualityProfileResource"
```

## Workflow: Modeling a New Resource (these tools end at the spec; you write the struct by hand)

1. `list_paths specs/<api>.json /api/v3/<resource>` — confirm endpoint methods (→ `sync` kind + paths)
2. `get_resource specs/<api>.json "#/components/schemas/<Name>"` — the authoritative field list: types,
   nullability, readOnly, defaults, and which paths use it (crud vs singleton vs embedded)
3. Hand-write the struct in the macro form per the `add-resource` skill — `get_resource`'s output is the
   field inventory; map each field to `#[id]`/`#[key]`/`#[reference]`/`#[wire(read_only)]`/`#[default(expr)]`
4. Verify by reading back: re-run `get_resource` and check your struct covers every field, nothing extra.
   The `spec_conformance` suite (`additionalProperties: false`) is the automated backstop.

## Workflow: Multi-API Field Comparison

1. `compare_schemas crates/radarr-v3/spec/radarr-v3.json "#/.../QualityProfileResource" specs/sonarr-v3.json "#/.../QualityProfileResource"`
2. Fields only in one API → API-specific additions to that crate
3. Same fields, different types → may need separate structs or conditional handling

## Spec Files — two locations

`specs/` is a **staging ground for not-yet-built services** (currently: sonarr, lidarr, prowlarr, jellyfin, seerr/overseerr, autobrr, bazarr). Once a service is built, its spec is **moved into the crate** at `crates/<svc>/spec/<svc>.json` and no longer exists in `specs/` — so the radarr reference spec is `crates/radarr-v3/spec/radarr-v3.json`, **not** `specs/radarr-v3.json`. Point a tool at whichever location currently holds the spec. Schema refs always use full format: `"#/components/schemas/FooResource"`.

## Non-*arr specs (inline bodies, composition)

The tools are tuned for the *arr convention — named `*Resource` schemas under `components.schemas` — but handle the common non-*arr shapes:

- **Inline request/response bodies.** Some specs define bodies inline in paths rather than as named schemas (overseerr/seerr does this for ~100 endpoints). `list_resources` lists those under an **"Inline (path-defined) schemas"** section tagged `[inline]`, and `get_path <spec> <path>` **expands** an inline body to its fields. A sparse spec like `bazarr` (only ~8 named schemas) simply has little to list — that's the spec, not a tool gap.
- **Composition (`allOf`/`oneOf`/`anyOf`).** `get_resource`/`compare_schemas` merge `allOf` automatically; for `oneOf`/`anyOf` (jellyfin uses `oneOf`) they merge the first branch and print a **`⚠ oneOf: N branches`** warning — treat those field lists as incomplete and cross-check the raw spec.

## When the spec falls short — provider `fields[]`

For provider resources (download clients, indexers, notifications, import lists, metadata) the spec types
`fields` as a generic `[{name, value}]` array — it does **not** say which implementations exist or what each
one's fields are. These tools can't recover that. Secondary source: the **devopsarr terraform provider** for
the app. `specs/terraform_providers.txt` lists the repos; clone `terraform-provider-<app>` to scratch and read
`internal/provider/<resource>_<impl>_resource.go` (filenames = variant list; structs = fields, with
`Sensitive: true` → `SecretValue`). The `add-resource` skill documents the full mapping. Live `/schema` (e2e)
is the final authority where the two disagree.
