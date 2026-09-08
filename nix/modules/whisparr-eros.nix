# Minimal NixOS module for Whisparr eros (V3), which is not in nixpkgs.
#
# nixpkgs ships `services.whisparr`, but its package is 2.0.0.2151 — Whisparr
# **V2**, a Sonarr fork with a series/episodes API. Enabling that module would
# boot the wrong API under the `whisparr-v3` e2e test, so this drives the
# from-source eros build in `nix/pkgs/whisparr-eros.nix` instead.
#
# Like `nix/modules/bindery.nix` it exists only so the e2e VM has something to
# boot; it is intentionally small and is not exported as a general-purpose
# module for users.
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.whisparr-eros;
in
{
  options.services.whisparr-eros = {
    enable = lib.mkEnableOption "Whisparr eros, the V3 (Radarr-derived) line";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ../pkgs/whisparr-eros.nix { };
      defaultText = lib.literalExpression "pkgs.callPackage ../pkgs/whisparr-eros.nix { }";
      description = "The Whisparr eros package to run.";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 6969;
      description = "TCP port Whisparr listens on.";
    };

    dataDir = lib.mkOption {
      type = lib.types.str;
      default = "/var/lib/whisparr/.config/Whisparr";
      description = ''
        Directory holding `config.xml` and the SQLite databases. The nested
        `.config/Whisparr` layout is Servarr's own, and nixpkgs' V2 module uses
        the same default — keeping it means the e2e test reads the generated API
        key from the same shape of path as every other *arr here.
      '';
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "whisparr";
      description = "User account under which Whisparr runs.";
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "whisparr";
      description = "Group under which Whisparr runs.";
    };

    environment = lib.mkOption {
      type = lib.types.attrsOf lib.types.str;
      default = { };
      description = ''
        Extra `WHISPARR__SECTION__KEY` environment variables. `Bootstrap` binds
        the `Whisparr:Server`, `Whisparr:Auth`, `Whisparr:Log`, `Whisparr:App`,
        `Whisparr:Update` and `Whisparr:Postgres` configuration sections, and
        .NET maps `__` to `:`, so these override the matching `config.xml` keys.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.tmpfiles.rules = [ "d '${cfg.dataDir}' 0700 ${cfg.user} ${cfg.group} - -" ];

    systemd.services.whisparr-eros = {
      description = "Whisparr eros (V3) adult movie collection manager";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      environment = {
        WHISPARR__SERVER__PORT = toString cfg.port;
        # There is no in-place updater to point at a store path; the built-in
        # one would only ever fail a health check.
        WHISPARR__UPDATE__MECHANISM = "external";
        WHISPARR__UPDATE__AUTOMATICALLY = "false";
        WHISPARR__LOG__ANALYTICSENABLED = "false";
      }
      // cfg.environment;

      serviceConfig = {
        Type = "simple";
        User = cfg.user;
        Group = cfg.group;
        # `-data` is what makes `AppFolderInfo` resolve `config.xml`, the
        # databases and `logs/` under a fixed directory rather than `$HOME`.
        ExecStart = "${lib.getExe cfg.package} -nobrowser -data='${cfg.dataDir}'";
        Restart = "on-failure";
      };
    };

    users.users = lib.mkIf (cfg.user == "whisparr") {
      whisparr = {
        group = cfg.group;
        home = cfg.dataDir;
        isSystemUser = true;
      };
    };

    users.groups = lib.mkIf (cfg.group == "whisparr") { whisparr = { }; };
  };
}
