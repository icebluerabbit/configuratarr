# Configuratarr

[![CI](https://github.com/icebluerabbit/configuratarr/actions/workflows/ci.yml/badge.svg)](https://github.com/icebluerabbit/configuratarr/actions/workflows/ci.yml)
[![Dependency Updates](https://github.com/icebluerabbit/configuratarr/actions/workflows/flake-update.yml/badge.svg)](https://github.com/icebluerabbit/configuratarr/actions/workflows/flake-update.yml)
[![Cachix Cache](https://img.shields.io/badge/Cachix-icebluerabbit--configuratarr-blue.svg)](https://icebluerabbit-configuratarr.cachix.org)
[![Nix Built](https://img.shields.io/badge/Nix-Flake-blue.svg?logo=nixos&logoColor=white)](https://nixos.org)

Declarative, idempotent configuration sync engine for the `*arr` stack (Radarr, Sonarr, Lidarr, Prowlarr) written in Rust.

Reads a desired-state YAML file, diffs it against each app's live REST API, and pushes only the delta. Designed for GitOps workflows and stateless NixOS deployments where application configuration would otherwise live only in a stateful SQLite database.

---

## Supported services

| App | API | Config `type` | Status |
|-----|-----|---------------|--------|
| Radarr | v3 | `radarr-v3` | ✅ Supported |
| Sonarr | v3 | `sonarr-v3` | ✅ Supported |
| Lidarr | v1 | `lidarr-v1` | ✅ Supported |
| Prowlarr | v1 | `prowlarr-v1` | ✅ Supported |
| Jellyfin | v10.11 | `jellyfin-v11` | ✅ Supported |
| Bazarr | v1 | `bazarr-v1` | ✅ Supported |
| Autobrr | v1 | `autobrr-v1` | ✅ Supported |
| LazyLibrarian | v1 | `lazylibrarian-v1` | ✅ Supported |
| Cleanuparr | v1 | `cleanuparr-v1` | ✅ Supported |
| Readarr | — | — | 🚧 Planned |

More of the ecosystem (Jellyseerr, …) is on the radar. The engine is service-agnostic — adding one is filling in a template, not changing the core. See [`docs/contributors.md`](docs/contributors.md).

---

## How it works

Almost all meaningful `*arr` configuration — indexers, download clients, quality profiles, notifications — is stored in a SQLite database, not in the static `config.xml`. There is no built-in way to declare this configuration as code.

Configuratarr acts as a declarative sync layer. You describe the desired state in a YAML file; it compares that against the running app's REST API and makes the necessary calls to converge. It never touches the database directly.

---

## Usage

Preview the planned changes without touching anything:

```bash
configuratarr --config configuratarr.yaml plan
```

Apply — shows the same plan, then asks for confirmation before writing:

```bash
configuratarr --config configuratarr.yaml apply
```

Skip the prompt for CI, scripts, or systemd:

```bash
configuratarr --config configuratarr.yaml apply --auto-approve
```

Also delete server-side resources that aren't in your config:

```bash
configuratarr --config configuratarr.yaml apply --prune
```

Wait for each app's API to be ready before doing anything — handy when configuratarr starts alongside the apps (e.g. on boot), so a still-starting Radarr doesn't fail the run:

```bash
configuratarr --config configuratarr.yaml --wait-for-healthy apply
```

See [`docs/commands.md`](docs/commands.md) for full CLI reference.

---

## Configuration

The config file is a YAML document. Each top-level key is an instance you name, and `type` picks which app it is — so you can manage one app or a whole stack from a single file:

```yaml
my-radarr:                     # any name you like
  type: radarr-v3              # which app this is
  url: "http://radarr.local:7878"
  api_key: "${env.RADARR_API_KEY}"

  tags:
    - label: managed

  quality_profiles:
    - name: HD-1080p
      upgrade_allowed: true
      cutoff: 7

  download_clients:
    - name: qBittorrent
      implementation: QBittorrent   # picks the download-client type
      protocol: torrent
      host: qbittorrent.local       # settings sit right here, no nesting
      tags: ["${ref.tag.managed}"]  # link to another resource by its name

# add another block (e.g. `my-sonarr: { type: sonarr-v3, ... }`) to manage more apps
```

Any field can use a template expression, resolved at apply time:

- `${env.VAR}` — an environment variable (handy for secrets)
- `${file./path/to/secret}` — the contents of a file
- `${ref.tag.managed}` — another resource, by name (configuratarr figures out the id and the right order)

See [`docs/radarr-v3-config.md`](docs/radarr-v3-config.md) and [`docs/sonarr-v3-config.md`](docs/sonarr-v3-config.md) for every field of every resource.

---

## Nix Integration

### NixOS

```nix
{ inputs, ... }: {
  imports = [ inputs.configuratarr.nixosModules.default ];

  services.configuratarr = {
    enable = true;
    prune = true;
    settings = {
      my-radarr = {
        type = "radarr-v3";
        url = "http://localhost:7878";
        api_key = "\${env.RADARR_API_KEY}";
        tags = [{ label = "managed"; }];
      };
    };
  };
}
```

See [`docs/nixos_options.md`](docs/nixos_options.md) for all options.

---

## Development

Requires [Nix](https://nixos.org) with flakes enabled. All tools live inside the devshell.

```bash
nix develop          # default shell: cargo, nextest, OpenAPI tools
cargo build
cargo nextest run
nix flake check      # unit tests + NixOS VM e2e tests
```

### E2E tests (fast local loop)

```bash
nix develop .#e2e-radarr   # starts Radarr, exports RADARR_URL + RADARR_API_KEY
cargo nextest run -p radarr-v3 --run-ignored all
```

### Regenerate docs

```bash
nix run .#generate-docs    # writes docs/commands.md, docs/radarr-v3-config.md,
                           # docs/nixos_options.md
```
