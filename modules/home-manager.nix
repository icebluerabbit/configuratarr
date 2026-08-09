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
    enableDescription = "Configuratarr user configuration service";
  };

  config = lib.mkIf cfg.enable {
    systemd.user.services.configuratarr = {
      Unit = {
        Description = "Configuratarr Stack Configuration Sync";
        After = [ "network.target" ];
      };

      Service = {
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
            # Non-interactive: the user service has no TTY for the confirm prompt.
            "--auto-approve"
          ]
          ++ lib.optional cfg.prune "--prune"
        );
      };
    };
  };
}
