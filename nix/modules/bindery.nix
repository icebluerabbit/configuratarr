# Minimal NixOS module for Bindery, which is not in nixpkgs.
#
# It exists so the e2e VM test has something to boot; it is intentionally small
# (a systemd unit, a state dir, and the handful of env vars the app reads) and
# is not exported as a general-purpose module for users.
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.bindery;
in
{
  options.services.bindery = {
    enable = lib.mkEnableOption "Bindery, an automated book library manager";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ../pkgs/bindery.nix { };
      defaultText = lib.literalExpression "pkgs.callPackage ../pkgs/bindery.nix { }";
      description = "The Bindery package to run.";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 8787;
      description = "TCP port Bindery listens on.";
    };

    stateDir = lib.mkOption {
      type = lib.types.str;
      default = "/var/lib/bindery";
      description = "Directory holding Bindery's database and data.";
    };

    apiKey = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = ''
        Instance API key, passed as `BINDERY_API_KEY`. World-readable in the
        Nix store — acceptable for the e2e VM, not for a real deployment.
      '';
    };

    environment = lib.mkOption {
      type = lib.types.attrsOf lib.types.str;
      default = { };
      description = "Extra `BINDERY_*` environment variables.";
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.bindery = {
      isSystemUser = true;
      group = "bindery";
    };
    users.groups.bindery = { };

    systemd.services.bindery = {
      description = "Bindery book library manager";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      environment = {
        BINDERY_PORT = toString cfg.port;
        BINDERY_DB_PATH = "${cfg.stateDir}/bindery.db";
        BINDERY_DATA_DIR = cfg.stateDir;
        BINDERY_LIBRARY_DIR = "${cfg.stateDir}/books";
        BINDERY_DOWNLOAD_DIR = "${cfg.stateDir}/downloads";
      }
      // lib.optionalAttrs (cfg.apiKey != null) { BINDERY_API_KEY = cfg.apiKey; }
      // cfg.environment;

      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        # A static user rather than `DynamicUser`: that option implies
        # `PrivateTmp`, which systemd keeps even if the unit sets
        # `PrivateTmp = false`. Bindery stats a root folder's path before
        # accepting it, so under a private `/tmp` it cannot see a directory the
        # e2e test created and every root-folder create 400s.
        User = "bindery";
        Group = "bindery";
        StateDirectory = "bindery";
        Restart = "on-failure";
      };
    };
  };
}
