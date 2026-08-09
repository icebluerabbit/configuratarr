# Configuratarr — Agent Guide

Declarative, idempotent config-sync engine for the *arr stack (Radarr, Sonarr, …): reads a desired-state YAML/JSON file, diffs it against each app's REST API, pushes the delta. Outputs a Rust CLI + Nix modules. This file is your **process + standards** guide; the technical reference lives in skills.

## Where knowledge lives

| Need | Go to |
|---|---|
| Add a resource to a service | **`add-resource`** skill |
| Add a new service crate | **`add-service`** skill |
| Modify the engine (`core`/`core-macros`/codecs/plan/apply) | **`core-architecture`** skill |
| Keep docs/README/skills in sync after a change | **`keep-docs-current`** skill |
| Inspect OpenAPI specs | **`openapi-tools`** skill |
| Run build/test, the devshell | **`nix-devshell`** skill |
| Human contributor how-to (the shared, tracked doc) | `docs/contributors.md` |

Reach for the skill rather than re-deriving — they hold the exact procedures and the canonical shapes.

## Standards (non-negotiable)

- **`crates/radarr-v3/` is the canonical template — for the *arr family.** Every decision in it was deliberate. When adding a service or resource to an *arr crate (radarr, sonarr, lidarr, prowlarr, whisparr, readarr), mirror it exactly — same macros, same provider layout, same test shape; do not invent alternatives. **Non-*arr services (jellyfin, overseerr, autobrr, bazarr) are different**: they diverge on auth, paths, and resource shape and may have no provider/fields-blob concept — radarr's *mechanics* (descriptor macros, harness, registration, nix) still apply, but its *shapes* may not. Before modeling a non-*arr service, or whenever unsure radarr's shape fits, **load the `core-architecture` skill and study `crates/core` first** — the engine (the four axes: endpoints/codec/sync/auth) is the source of truth for what's possible, not radarr.
- **No shared structs across API crates.** APIs diverge subtly; duplication is intentional. The *mechanics* are shared (`core`, `core-testkit`), the resource structs are not.
- **The service registry is one table.** Known services live in `crates/service-registry/` (`service_registry!`); `core-config`'s `ServiceInstance` enum and `config-doc-gen`'s renderer are macro-generated from it. **Never hand-write a per-service `match` arm or doc-gen entry** — add a row. Each row's `type:` tag doubles as the Cargo feature gating that service (deps are `optional`; `default` enables all). See the `add-service` skill.
- **All deps via `[workspace.dependencies]`** in the root `Cargo.toml`, referenced with `.workspace = true`. Never pin a version in a crate manifest.
- **Async is intentional** (`reqwest` + `tokio` are first-class). Do not replace with sync.
- **The old architecture is gone.** If you see `Interpolated<T>`, `CollectRefs`/`Referenceable`/`ServiceResources`, `schemars`, or `#[apiField]`/`path = ...`, it is **stale/corrupt** — fix it, never copy it.

## Process / how we work

- **Everything runs in the Nix devshell** — `cargo`, `python3`, the OpenAPI scripts. Never assume host tools. See the `nix-devshell` skill.
- **Verify, don't assume:**
  - `nix develop --command cargo build --workspace` — must be clean.
  - `nix develop --command cargo nextest run --workspace` — unit + spec-conformance.
  - `nix develop .#e2e-<svc> --command cargo nextest run -p <svc> --test e2e --run-ignored all -j1` — live e2e (`-j1`: tests share one live instance, parallel runs race).
  - Before calling anything done: build + conformance green; for engine/HTTP changes, run the live e2e too.
- **Keep it lint/format-clean:** `cargo clippy --workspace --all-targets` (0 warnings) and `cargo fmt --check` (0 diffs) before declaring a committable state.
- **Keep the docs current.** A change isn't done until the README / docs / skills that describe it are true again — use the **`keep-docs-current`** skill (it maps each kind of change to the surfaces to update). Generated docs (`commands.md`, `<service>-config.md`, `*_options.md`) are never hand-edited — `nix run .#generate-docs`. Resource `///` comments feed the config docs; write them.
- **Security:** plan/render output must never leak secrets. Redaction lives in `plan::display_value` (see `core-architecture`); don't print `Op.body` directly.

## Git

- **Never commit or push.** Git is read-only here — `status`/`diff`/`log`/`show` only. Leave work uncommitted; the user commits.
- `.claude/` internals (settings, projects) are gitignored; `.claude/skills/`, `docs/`, and this file are tracked and shared.
