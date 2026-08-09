{
  description = "Configuratarr - A declarative configuration stack-sync engine for Sonarr, Radarr, Prowlarr, Lidarr, and Readarr.";

  nixConfig = {
    extra-substituters = [
      "https://icebluerabbit-configuratarr.cachix.org"
      # Cleanuparr builds .NET + Angular from source; without its cache the
      # cleanuparr-v1 e2e check rebuilds all of it.
      "https://icebluerabbit-cleanuparr-flake.cachix.org"
    ];
    extra-trusted-public-keys = [
      "icebluerabbit-configuratarr.cachix.org-1:dEEK2uZ8exjLoOh01aGi9GfRqnmd/DjrUUmIZmcCiu8="
      "icebluerabbit-cleanuparr-flake.cachix.org-1:K0JIcbUOshVOfIpRhQSCwIl5UH34qxlnB13RDwl/p7s="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    # Cleanuparr is not in nixpkgs — and cannot be built from a plain
    # `buildDotnetModule`, since two of its NuGet dependencies live only on a
    # PAT-gated GitHub Packages feed. This flake packages those dependencies too,
    # and ships the `services.cleanuparr` NixOS module the e2e VM test drives.
    cleanuparr-flake = {
      url = "github:icebluerabbit/cleanuparr-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      fenix,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { moduleWithSystem, ... }:
      {
        systems = [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ];

        perSystem =
          { pkgs, inputs', ... }:
          let
            mkTool =
              name:
              pkgs.writeShellScriptBin name ''
                exec ${pkgs.python3}/bin/python3 ${toString ./.}/tools/${name}.py "$@"
              '';
            tools = map mkTool [
              "list_resources"
              "get_resource"
              "list_paths"
              "get_path"
              "compare_schemas"
            ];

            inherit
              (import ./nix/crane.nix {
                inherit pkgs fenix crane;
                root = ./.;
              })
              rustToolchain
              craneLib
              commonArgs
              cargoArtifacts
              ;

            mkServiceChecks = import ./nix/mk-service-checks.nix {
              inherit
                pkgs
                craneLib
                commonArgs
                cargoArtifacts
                ;
            };

            configDocGen = craneLib.buildPackage (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "config-doc-gen";
                cargoExtraArgs = "-p config-doc-gen --bin config-doc-gen";
              }
            );

            cmdDocGen = craneLib.buildPackage (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "cmd-doc-gen";
                cargoExtraArgs = "-p cmd-doc-gen --bin cmd-doc-gen";
              }
            );

            configuratarr = pkgs.callPackage ./nix/package.nix {
              docs = pkgs.callPackage ./modules/docs.nix { };
            };
          in
          {
            packages = {
              inherit configuratarr;
              default = configuratarr;
            };

            checks = {
              workspace-tests = craneLib.cargoNextest (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoNextestExtraArgs = "--workspace";
                }
              );

              clippy = craneLib.cargoClippy (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoClippyExtraArgs = "--workspace --all-targets -- -D warnings";
                }
              );

              rustfmt = craneLib.cargoFmt { inherit (commonArgs) src; };
            }
            // mkServiceChecks "radarr-v3" (import ./nix/e2e/radarr-v3.nix { inherit pkgs; })
            // mkServiceChecks "sonarr-v3" (import ./nix/e2e/sonarr-v3.nix { inherit pkgs; })
            // mkServiceChecks "prowlarr-v1" (import ./nix/e2e/prowlarr-v1.nix { inherit pkgs; })
            // mkServiceChecks "lidarr-v1" (import ./nix/e2e/lidarr-v1.nix { inherit pkgs; })
            // mkServiceChecks "jellyfin-v11" (import ./nix/e2e/jellyfin-v11.nix { inherit pkgs; })
            // mkServiceChecks "bazarr-v1" (import ./nix/e2e/bazarr-v1.nix { inherit pkgs; })
            // mkServiceChecks "autobrr-v1" (import ./nix/e2e/autobrr-v1.nix { inherit pkgs; })
            // mkServiceChecks "lazylibrarian-v1" (import ./nix/e2e/lazylibrarian-v1.nix { inherit pkgs; })
            // mkServiceChecks "cleanuparr-v1" (
              import ./nix/e2e/cleanuparr-v1.nix {
                inherit pkgs;
                cleanuparr = inputs'.cleanuparr-flake.packages.cleanuparr;
                cleanuparrModule = inputs.cleanuparr-flake.nixosModules.cleanuparr;
              }
            );

            formatter = pkgs.nixfmt-tree;

            apps.generate-docs = {
              type = "app";
              program = "${pkgs.writeShellScript "generate-docs" ''
                echo "==> Copying generated NixOS and Home Manager options docs..."
                cp -f ${configuratarr.docs}/nixos_options.md docs/nixos_options.md
                cp -f ${configuratarr.docs}/home_manager_options.md docs/home_manager_options.md
                echo "==> Generating service config docs..."
                ${configDocGen}/bin/config-doc-gen --output-dir docs
                echo "==> Generating CLI command docs..."
                ${cmdDocGen}/bin/cmd-doc-gen > docs/commands.md
                echo "==> Done!"
              ''}";
            };

            devShells = import ./nix/shells.nix {
              inherit pkgs rustToolchain tools;
              cleanuparr = inputs'.cleanuparr-flake.packages.cleanuparr;
            };
          };

        flake = {
          nixosModules.default = moduleWithSystem (
            { config, ... }:
            { lib, ... }:
            {
              imports = [ ./modules/nixos.nix ];
              services.configuratarr.package = lib.mkDefault config.packages.default;
            }
          );

          homeManagerModules.default = moduleWithSystem (
            { config, ... }:
            { lib, ... }:
            {
              imports = [ ./modules/home-manager.nix ];
              services.configuratarr.package = lib.mkDefault config.packages.default;
            }
          );
        };
      }
    );
}
