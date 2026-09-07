{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.configuratarr;
  yamlFormat = pkgs.formats.yaml { };
  configFile = yamlFormat.generate "configuratarr.yaml" cfg.settings;
in
{
  options.services.configuratarr = import ./options.nix {
    inherit lib pkgs;
    enableDescription = "Configuratarr declarative configuration service";
  };

  config = lib.mkIf cfg.enable {
    systemd.services.configuratarr = {
      description = "Configuratarr Stack Configuration Sync";
      wantedBy = [ "multi-user.target" ];
      # Order after the apps we actually support, so they're up to receive
      # config. (Ordering only — a unit that isn't on this host is ignored, and
      # remote apps configured by URL have no local unit anyway.) Add a line here
      # as each new service crate lands.
      after = [
        "network.target"
        "radarr.service"
        "sonarr.service"
        "prowlarr.service"
        "lidarr.service"
        "jellyfin.service"
        "bazarr.service"
        "autobrr.service"
        "lazylibrarian.service"
        "cleanuparr.service"
        "komga.service"
        "bindery.service"
        "audiobookshelf.service"
        # eros/V3 specifically — nixpkgs' `services.whisparr` unit is V2, whose
        # API the whisparr-v3 crate does not model. See nix/modules/whisparr-eros.nix.
        "whisparr-eros.service"
      ];

      serviceConfig = {
        Type = "oneshot";
        ExecStart = lib.concatStringsSep " " (
          [
            "${cfg.package}/bin/configuratarr"
            "--config ${configFile}"
          ]
          # Global flags precede the subcommand.
          ++ lib.optional cfg.waitForHealthy "--wait-for-healthy"
          ++ lib.optional cfg.waitForHealthy "--wait-timeout ${toString cfg.waitTimeout}"
          ++ [
            "apply"
            # Non-interactive: systemd has no TTY for the confirmation prompt.
            "--auto-approve"
          ]
          ++ lib.optional cfg.prune "--prune"
        );
        RemainAfterExit = true;
      };
    };
  };
}
