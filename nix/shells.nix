# Dev shells. Returns { default, e2e }.
{
  pkgs,
  rustToolchain,
  tools,
  # Cleanuparr is not in nixpkgs; it comes from the `cleanuparr-flake` input.
  cleanuparr,
  lazylibrarian,
}:
let
  default = pkgs.mkShell {
    buildInputs =
      with pkgs;
      [
        rustToolchain
        python3
        pkg-config
        openssl
        cargo-nextest
        shellcheck
        jq
        curl
      ]
      ++ tools;

    shellHook = ''
      echo "=== Configuratarr Nix DevShell ==="
      rustc --version
      cargo --version

      export RUST_BACKTRACE=1
      export RUST_LOG=info
      export CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true
    '';
  };
in
let
  # Shared bash helpers (port reclaim, non-empty assertions, retry) — see the
  # file header for the failure modes they exist to prevent.
  common = import ./e2e-shells/_common.nix { inherit pkgs; };

  e2e = pkgs.mkShell {
    inputsFrom = [ default ];
    buildInputs = with pkgs; [
      radarr
      sonarr
      prowlarr
      lidarr
      readarr
      jellyfin
    ];
    shellHook = ''
      echo "=== Configuratarr E2E DevShell ==="
    '';
  };
in
{
  inherit default e2e;

  e2e-radarr = import ./e2e-shells/radarr-v3.nix {
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-sonarr = import ./e2e-shells/sonarr-v3.nix {
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-prowlarr = import ./e2e-shells/prowlarr-v1.nix {
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-lidarr = import ./e2e-shells/lidarr-v1.nix {
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-jellyfin = import ./e2e-shells/jellyfin-v11.nix {
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-bazarr = import ./e2e-shells/bazarr-v1.nix {
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-autobrr = import ./e2e-shells/autobrr-v1.nix {
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-lazylibrarian = import ./e2e-shells/lazylibrarian-v1.nix {
    inherit lazylibrarian;
    inherit pkgs;
    inherit common;
    e2eShell = e2e;
  };

  e2e-cleanuparr = import ./e2e-shells/cleanuparr-v1.nix {
    inherit pkgs;
    inherit common;
    inherit cleanuparr;
    e2eShell = e2e;
  };
}
