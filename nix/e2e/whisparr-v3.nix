# nixosTest for whisparr-v3. Called with the pre-built e2e test binary.
#
# Whisparr eros (V3) is not in nixpkgs — `pkgs.whisparr` is the V2 Sonarr fork —
# so both the package (`nix/pkgs/whisparr-eros.nix`) and a minimal module
# (`nix/modules/whisparr-eros.nix`) live in this repo. See the package header
# for why there is no prebuilt artifact to fetch.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "whisparr-v3-e2e";
  nodes.machine = {
    imports = [ ../modules/whisparr-eros.nix ];
    services.whisparr-eros.enable = true;
    environment.systemPackages = [ e2eBin ];
  };
  testScript = ''
    from datetime import timedelta

    machine.wait_for_unit("whisparr-eros.service")
    # Building the app is cached, but the first boot still runs 241 FluentMigrator
    # migrations against a fresh SQLite database before the listener opens.
    machine.wait_for_open_port(6969, timeout=timedelta(seconds=180))
    api_key = machine.wait_until_succeeds(
      "grep -oP '(?<=<ApiKey>)[^<]+' /var/lib/whisparr/.config/Whisparr/config.xml",
      timeout=timedelta(seconds=30),
    ).strip()
    machine.succeed(
      f"WHISPARR_URL=http://localhost:6969 WHISPARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (global tag list,
      # prune deletes all); parallel runs race. Run serially.
      f"whisparr-v3-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
