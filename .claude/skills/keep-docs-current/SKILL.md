---
name: keep-docs-current
description: >
  Checklist + map for keeping configuratarr's docs, skills, and README in sync with the code.
  Use before declaring a change "done" or committable — it maps each kind of change (new CLI
  flag, new/changed resource, new service crate, engine/architecture change, removed-or-renamed
  thing) to the doc surfaces that must be updated or regenerated, and how to catch stale
  references. Auto-triggers when: finishing a feature, preparing a commit, changing the CLI /
  a resource / a service / the engine, or whenever the docs might have drifted from the code.
---

# Keep the docs current

Doc drift is the default failure mode here: the code changes, the docs don't, and the next contributor (human or LLM) trusts a stale doc. **A significant change isn't done until the docs that describe it are true again.** Run this reconciliation before calling anything done or committable.

## Two kinds of docs

**Generated — never hand-edit, regenerate:**

| File | Source | Generator |
|---|---|---|
| `docs/commands.md` | the clap CLI (`core-cli`) | `cmd-doc-gen` |
| `docs/<service>-config.md` | resource descriptors + `///` comments | `config-doc-gen` |
| `docs/nixos_options.md`, `docs/home_manager_options.md` | the Nix modules | `docs.nix` |

Regenerate all at once: `nix run .#generate-docs`.

**Hand-written — update by audience:**

- `README.md` — users
- `docs/contributors.md` — human contributors
- `CLAUDE.md` + `.claude/skills/*` — the LLM (process/standards + procedures)

## If you changed… update…

| Change | Surfaces to touch |
|---|---|
| **A CLI flag / command** | regen `commands.md`; the README **Usage** section; `CLAUDE.md` if it's a workflow standard |
| **A resource** (fields, a new one) | regen `<service>-config.md`; write `///` comments on the struct + fields; the `add-resource` skill *if the pattern itself changed* |
| **A new service crate** | the README **Supported services** row (flip to ✅); the `service_registry!` table in `crates/service-registry/` (one row — drives both `core-config` + `config-doc-gen`) plus the optional-dep + `[features]` lines in those two crates' `Cargo.toml`; the `modules/nixos.nix` `after` list; regen `<service>-config.md`; the `add-service` skill *if the checklist changed* |
| **Engine internals / a new concept or seam** | the `core-architecture` skill; `CLAUDE.md` if it's a new standard/guardrail |
| **A contribution procedure** | the `add-resource` / `add-service` skills; `docs/contributors.md` |
| **Removed / renamed a crate, type, or attribute** | grep the whole tree for the old name and fix every doc/comment that still mentions it (see below) |
| **A new "must do / never do" rule** | `CLAUDE.md` standards |

## Catch stale references

After an architectural change, old names linger in prose and comments. Grep for whatever you removed or renamed:

```bash
# example pattern — adapt to your change
grep -rnE 'OldType|old_crate|#\[old_attr\]|old_flag' \
  --include='*.md' --include='*.rs' --include='*.nix' . | grep -v target/
```

Zero hits = clean. (This is how the dead-architecture names — `Interpolated`, `schemars`, `#[apiField]`, `path = …` — were hunted out of the docs.)

## Verify

```bash
nix run .#generate-docs                                  # regenerate, then read the diff
cargo build --workspace && cargo nextest run --workspace # code still green
```

Then skim the diff of every hand-written doc you touched and sanity-check it against reality: does the README config example still load? Does `CLAUDE.md` still describe how things actually work? Did a generated doc change in a way you *didn't* intend (a sign you broke something upstream)?
