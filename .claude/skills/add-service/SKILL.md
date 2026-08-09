---
name: add-service
description: >
  Full checklist for adding a new API service crate to configuratarr (sonarr-v3, lidarr-v1,
  prowlarr-v1, jellyfin, etc.). Covers scaffolding, Cargo wiring, the #[service] struct,
  registering in the central service-registry table (one row, feeds core-config + config-doc-gen),
  the test layout via core-testkit, and the Nix e2e wiring. Auto-triggers when: starting a new
  service crate, scaffolding <api>-<version>, wiring a service into the registry, or setting up a
  service's tests/Nix.
---

# Adding a New Service Crate

## Invariants (each stated once; the procedure references these by number)

1. **Engine first, radarr second.** Load the `core-architecture` skill and read `crates/core` (`descriptor.rs`, `service.rs`, `codec/`, the `core-macros` args) before judging any shape. `radarr-v3` is *one* instantiation of the engine (axes: endpoints / codec / sync / auth), not the engine.
2. **radarr is a *structural* template only** — macros, file layout, provider/test shape. For the *arr family (radarr, sonarr, lidarr, prowlarr, whisparr, readarr — shared `X-Api-Key`, `/api/vN`, provider/fields-blob, `Provider` envelope, singleton config) its *shapes* transfer too. Non-*arr (jellyfin, seerr, autobrr, bazarr) diverge on auth/paths/resource shape and may have no provider concept — take the mechanics, derive shapes from Invariant 1 + the spec. **Never use radarr's resource *list* to decide what to model** (Invariant 3).
3. **The spec is the source of truth** for which resources exist and every field (type / nullability / readOnly / default). Conformance (`additionalProperties: false`) rejects invented or mis-cased fields. **Never `cp -r crates/radarr-v3`** — it forks radarr's resource set and ships movie-shaped structs for a series API. Model each struct from the spec and verify it covers every field.
4. **Provider variants are not in the spec and are not radarr's.** Source them from the offline devopsarr terraform-provider — the only plan-time source; the live `/schema` endpoint is e2e-only (details: §0.3).
5. **Plan mode before any tool call**, including read-only spec exploration — §0 *is* the planning (gate: §0).

**Tools + paths.** Per-resource modeling: `add-resource` skill. Spec exploration: `openapi-tools` — five inspection-only scripts (`list_resources`, `list_paths`, `get_resource`, `get_path`, `compare_schemas`) living at the **repo-root `tools/`** dir, wrapped onto PATH by `flake.nix`; invoke them **bare inside the devshell** (`nix develop --command list_resources specs/<svc>.json`), not from the skill dir. Build/test: `nix-devshell` skill.

**Naming + spec location.** `<svc>` = `<api>-<version>` (e.g. `sonarr-v3`). `specs/` stages **not-yet-built** services only; once built, a service's spec lives **in-crate** at `crates/<svc>/spec/<svc>.json` (keeping the full name; that's why `specs/` has no `radarr-v3.json`). Lifecycle: plan against `specs/<svc>.json` → §1 `git mv`s it in-crate → every later reference (conformance `include_str!`, any `tools/` run) uses the in-crate path.

## 0. Plan from the spec — first action is `EnterPlanMode` (Invariant 5)

Read-only exploration is **not** exempt — it is the planning. "It's only read-only" is the rationalization that skips this. Enter plan mode, then produce a written plan before scaffolding:

1. `list_resources specs/<svc>.json` — **scope = every `[crud]/[singleton]/[provider]` resource the spec exposes; model all of them.** Exclude only pure content/data (`SeriesResource`, `EpisodeResource`, `MovieResource` — runtime library content). Build the full config surface from *this* spec, not radarr's list (Invariant 2).
2. Field source, per resource: analog exists in radarr → `compare_schemas crates/radarr-v3/spec/radarr-v3.json "#/.../<R>Resource" specs/<svc>.json "#/.../<R>Resource"` for the delta; no analog → `get_resource specs/<svc>.json "#/.../<R>Resource"` fresh. Spec authoritative either way (Invariant 3).
3. Provider variants (Invariant 4) — clone **now, during planning**, not at execution:
   - `/projects/configuratarr/specs/terraform_providers.txt` lists the devopsarr repo per app.
   - `git clone --depth 1 <repo-url> <scratch>/terraform-provider-<app>`.
   - `internal/provider/<resource>_<impl>_resource.go`: **filenames = the variant list, structs = the inner `fields[]`** (full map: `add-resource` → "Where the provider variant list + `fields[]` come from").
   - **Write each provider's variant list into the plan file.** A provider family planned without the cloned repo is planned blind.
4. Singletons + config endpoints: confirm each path still exists (`list_paths specs/<svc>.json /api/v3/config`).
5. Carve the resources into execution slices (next section): each provider family, the static-spec batch, and `service.rs` + tests + registration become slices — note which files each owns and which `pub mod` / `#[service]` lines it returns. Surface the resource list **and** the slice map to the user; they approve the fan-out, not just the resource set.

**── GATE.** Be in plan mode before §0's first tool call; create/edit nothing until the plan artifact exists and is surfaced: the in-scope resource list (collections + singletons) + per-resource field deltas + per-provider variant lists. A wrong resource set is expensive to unwind after 90 files exist.

## Execute with subagents

Invoking this skill **is** the authorization to spawn per-slice agents — dispatch without re-asking and without falling back to solo inline work. (Single-resource changes use `add-resource`, no agents.) **Dispatch every agent on sonnet** (`model: "sonnet"`, standing preference) — the slice work is mechanical and well-scoped.

A service is ~15–20 resources plus ~4 provider families of 10–28 variants each; slices are independent (each family owns its `mod.rs` + variant files; each static resource owns one file). One agent per slice. **Agents start cold — they inherit no skills or `CLAUDE.md`.** Arm each prompt:
- read `add-resource` + `core-architecture` first;
- restate Invariant 3 (never `cp -r`, radarr is shape-only, model + verify every field from the spec);
- hand it the slice's spec refs, the matching radarr files (shape reference only), and the §0 field deltas;
- a **provider-family** agent additionally gets the cloned tf-provider repo — its only variant + `fields[]` source (`get_resource` can't see inside the blob; without it the agent hallucinates or copies radarr's variants).

**Write-conflict rule.** The main thread owns every **shared** file: `service.rs`, `src/resources/mod.rs`, `lib.rs`, and all registration/nix/docs wiring. Each agent owns only **its own** files (its resource `.rs` files, and for a family its `<family>/` dir incl. that family's `mod.rs`) and **returns** its `pub mod` line(s) + `#[service]` field(s) for the main thread to splice in. Two agents never edit the same file. Verify each slice with the `spec_conformance` suite as it lands.

## 1. Scaffold

Create the tree by hand (or file-by-file) — do **not** clone radarr's `src/resources/`. **Move** the staged spec in: `git mv specs/<svc>.json crates/<svc>/spec/<svc>.json` — it must no longer exist in `specs/`. `lib.rs`/`service.rs`/`mod.rs` are written to *this* service's resource set from §0.

```
crates/<svc>/                  # <svc> = <api>-<version>, e.g. crates/sonarr-v3/
    Cargo.toml
    spec/<svc>.json            # moved from specs/<svc>.json (keep the full name)
    src/
        lib.rs                 # pub mod resources; pub mod service; pub use service::<Api>;
        service.rs             # the #[service] struct
        resources/
            mod.rs
            <resource>.rs ...  # + <provider>s/ subdirs for providers
    tests/
        spec_conformance.rs    # thin — uses core_testkit::check
        e2e.rs                 # thin — uses core_testkit::{env_pair, instance}
        testdata/<r>/config.yaml
```

## 2. Cargo.toml

```toml
[package]
name = "<api>-<version>"
version = "0.1.0"
edition = "2024"

[dependencies]
core-lib    = { path = "../core" }
core-macros = { path = "../core-macros" }
anyhow.workspace     = true
serde_json.workspace = true

[dev-dependencies]
core-testkit = { path = "../core-testkit" }
serde-saphyr.workspace = true
tokio.workspace        = true
jsonschema.workspace   = true
```
Local crates by path; everything else `.workspace = true` (never pin a version in a crate manifest). Add `"crates/<api>-<version>"` to `[workspace.members]` in the root `Cargo.toml`.

## 3. Resources

Per the `add-resource` skill (archetype shapes: collection / singleton / provider). Field list via `get_resource crates/<svc>/spec/<svc>.json "#/.../<R>Resource"` (in-crate post-move). Macro form: FK `i32` → `#[reference(t)]`; credential → `SecretValue`; read-only → `#[wire(read_only)]`; non-zero API default → `#[default(expr)]`. Provider variants per §0.3 (Invariant 4).

## 4. service.rs — `#[service]`

One struct carries connection fields **and** every managed resource. `Vec<R>` = collection, `Option<R>` = singleton (the macro reads each resource's `SyncKind` from its descriptor). **The `health`/`auth` values below are *arr defaults** — derive yours from the spec + `core-architecture` (auth scheme, health path, id type all vary off the *arr stack).

```rust
#[service(name = "<api>_<version>", health = "/api/v3/system/status", auth = api_key(header = "X-Api-Key"))]  // *arr values — replace per service
pub struct <Api> {
    pub url: String,
    #[credential(api_key)] pub api_key: SecretValue,
    pub insecure: Option<bool>,
    pub timeout_secs: Option<u64>,

    pub tags: Vec<Tag>,                       // collections
    pub media_management: Option<MediaManagement>,  // singletons
}
```
Auth schemes: `none`, `api_key(header = "...")`, `bearer`, `basic`, `form_cookie(login_path = "...")` with `#[credential(api_key|bearer|user|pass)]`. The executor implements None/ApiKey/Bearer today; Basic/FormCookie `bail!` (async auth is a pending seam).

Declaring `health = "<path>"` wires the service into `--wait-for-healthy`: the CLI polls that endpoint (authenticated) until it responds before any plan/apply. Omit it and the service isn't waited on.

## 5. Register + announce

The registry is centralized in `crates/service-registry/src/lib.rs` — **one table, two consumers**. A row is `Variant => "<api>-<version>" : <api_version>::<Api> = "<Human Name>"`, and the `"<api>-<version>"` tag **doubles as the Cargo feature** that gates the service. Do **not** hand-edit a `ServiceInstance` match arm or the doc-gen array — both are macro-generated from this table.

- `crates/service-registry/src/lib.rs` — add one row to the `service_registry!` table. This feeds **both** the `core-config` dispatch enum (`ServiceInstance`) and the `config-doc-gen` renderer.
- `crates/core-config/Cargo.toml` **and** `crates/config-doc-gen/Cargo.toml` — add the crate as an `optional = true` dep, add a `<api>-<version> = ["dep:<api>-<version>"]` line under `[features]`, and add the feature to each `default = [...]` list. (The macro gates every variant/arm/doc-entry with `#[cfg(feature = "<api>-<version>")]`, so a single-service build is `--no-default-features --features <api>-<version>`.)
- `README.md` — flip the service's row in the **Supported services** table to `✅ Supported` (add a row if it's a new app, with its API version + config `type`).

(Doc-gen is generic — providers/nested types document themselves.)

## 6. Tests (thin — harness is shared)

**`spec_conformance.rs`** — `spec()` loader + a `conformance!` line per resource + the teeth test. Harness is `core_testkit::check`.

**`e2e.rs`** — env-guarded, `#[ignore]`:
```rust
use core_testkit::{env_pair, instance};
fn env() -> Option<(String, String)> { env_pair("<SERVICE>_URL", "<SERVICE>_API_KEY") }
async fn run(url, key, resources, opts) -> Report {
    let (svc, value) = instance::<<Api>>(url, key, resources);
    apply(&svc, &value, opts).await.unwrap()
}
```
Keep both thin — declarations only; the service-agnostic mechanics live in `core-testkit`.

## 7. Nix wiring (five touches — copy the radarr-v3 equivalents)

- `nix/e2e/<api>-<version>.nix` — the NixOS VM test.
- `nix/e2e-shells/<api>-<version>.nix` — local fast-iteration shell (starts the service, exports `<SERVICE>_URL` + `<SERVICE>_API_KEY`).
- `nix/shells.nix` — `e2e-<api> = import ./e2e-shells/<api>-<version>.nix { inherit pkgs; e2eShell = e2e; };`
- `flake.nix` — merge `mkServiceChecks "<api>-<version>" (import ./nix/e2e/<api>-<version>.nix { inherit pkgs; })` with `//`.
- `modules/nixos.nix` — add `"<app>.service"` to the systemd `after` list (ordering only; ignored when the app is remote or absent).

## 8. Verify — definition of done

Per resource: re-`get_resource` the spec and confirm the struct covers every field, right type/nullability, nothing missing or invented (Invariant 3). Then the full gate (criteria otherwise scattered across `CLAUDE.md` + `keep-docs-current`):

```bash
nix develop --command cargo build --workspace                              # clean
nix develop --command cargo nextest run -p <svc>                           # unit + conformance green
nix develop --command cargo clippy --workspace --all-targets               # 0 warnings
nix develop --command cargo fmt --check                                    # 0 diffs
nix run .#generate-docs                                                     # regen docs/<svc>-config.md + options
nix develop .#e2e-<api> --command cargo nextest run -p <svc> --test e2e --run-ignored all -j1   # live e2e (HTTP/auth path; -j1: shared live instance, parallel races)
```
Plus non-command checks: README **Supported services** row → ✅ (§5); the `service_registry!` table has the new row and both consumers carry its feature/dep (verify the gated slice builds: `cargo build -p core-config -p config-doc-gen --no-default-features --features <api>-<version>`); `specs/<svc>.json` no longer exists (moved in-crate). Full doc-sync map: `keep-docs-current` skill.
