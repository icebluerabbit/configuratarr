{
  pkgs,
  lib ? pkgs.lib,
  ...
}:

let
  # Extend pkgs to include configuratarr, which is used as the default for options.services.configuratarr.package
  pkgsDocs = pkgs.extend (
    final: prev: {
      configuratarr = final.callPackage ../nix/package.nix { };
    }
  );

  nixosEval = pkgsDocs.lib.evalModules {
    modules = [
      ./nixos.nix
      {
        options.systemd.services = pkgsDocs.lib.mkOption {
          type = pkgsDocs.lib.types.attrsOf pkgsDocs.lib.types.attrs;
          default = { };
        };
      }
    ];
    specialArgs = {
      pkgs = pkgsDocs;
    };
  };
  nixosDocs = pkgsDocs.nixosOptionsDoc {
    options = builtins.removeAttrs nixosEval.options [
      "_module"
      "systemd"
    ];
  };

  hmEval = pkgsDocs.lib.evalModules {
    modules = [
      ./home-manager.nix
      {
        options.systemd.user.services = pkgsDocs.lib.mkOption {
          type = pkgsDocs.lib.types.attrsOf pkgsDocs.lib.types.attrs;
          default = { };
        };
      }
    ];
    specialArgs = {
      pkgs = pkgsDocs;
    };
  };
  hmDocs = pkgsDocs.nixosOptionsDoc {
    options = builtins.removeAttrs hmEval.options [
      "_module"
      "systemd"
    ];
  };
in
pkgsDocs.runCommand "configuratarr-options-docs" { } ''
    mkdir -p $out

    # 1. Generate NixOS Options with clean relative paths and header
    cat << 'EOF' > $out/nixos_options.md
  # NixOS Module Options

  This document details the configuration options available for the Configuratarr NixOS module.

  EOF
    sed -E \
      -e 's|\(file:///nix/store/[a-z0-9]{32}-source/|(../|g' \
      -e 's|/nix/store/[a-z0-9]{32}-source/|../|g' \
      -e 's|\\\.|\.|g' \
      ${nixosDocs.optionsCommonMark} >> $out/nixos_options.md

    # 2. Generate Home Manager Options with clean relative paths and header
    cat << 'EOF' > $out/home_manager_options.md
  # Home Manager Module Options

  This document details the configuration options available for the Configuratarr Home Manager module.

  EOF
    sed -E \
      -e 's|\(file:///nix/store/[a-z0-9]{32}-source/|(../|g' \
      -e 's|/nix/store/[a-z0-9]{32}-source/|../|g' \
      -e 's|\\\.|\.|g' \
      ${hmDocs.optionsCommonMark} >> $out/home_manager_options.md
''
